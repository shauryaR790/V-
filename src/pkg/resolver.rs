use std::path::{Path, PathBuf};
use std::process::Command;

use semver::Version;

use super::lockfile::{Lockfile, LockedPackage};
use super::manifest::{DependencySpec, DetailedDependency, Manifest};
use crate::error::{VppError, VppResult};
use crate::pkg::deps_dir;

pub struct VendorLayout {
    pub packages: Vec<LockedPackage>,
}

pub fn vendor_dir(project_root: &Path, name: &str) -> PathBuf {
    deps_dir(project_root).join(name)
}

pub fn resolve_dependencies(project_root: &Path, manifest: &Manifest) -> VppResult<Lockfile> {
    std::fs::create_dir_all(deps_dir(project_root)).map_err(|e| VppError::Other {
        message: format!("failed to create .vpp/deps: {e}"),
    })?;

    let mut packages = Vec::new();
    for (name, spec) in &manifest.dependencies {
        let locked = resolve_one(project_root, name, spec)?;
        packages.push(locked);
    }

    Ok(Lockfile {
        version: 1,
        packages,
    })
}

fn resolve_one(project_root: &Path, name: &str, spec: &DependencySpec) -> VppResult<LockedPackage> {
    match spec {
        DependencySpec::Version(v) => {
            let detailed = super::registry::resolve_from_registry(project_root, name, v)?;
            resolve_detailed(project_root, name, &detailed)
        }
        DependencySpec::Detailed(d) => resolve_detailed(project_root, name, d),
    }
}

fn resolve_detailed(
    project_root: &Path,
    name: &str,
    dep: &DetailedDependency,
) -> VppResult<LockedPackage> {
    if let Some(path) = &dep.path {
        let abs = if path.is_absolute() {
            path.clone()
        } else {
            project_root.join(path)
        };
        let canonical = abs.canonicalize().map_err(|e| VppError::Other {
            message: format!("dependency `{name}` path `{}`: {e}", abs.display()),
        })?;
        let version = read_dep_version(&canonical).unwrap_or_else(|| "0.0.0".to_string());
        if let Some(req) = &dep.version {
            ensure_version(name, &version, req)?;
        }
        let dest = vendor_dir(project_root, name);
        link_or_copy(&canonical, &dest)?;
        return Ok(LockedPackage {
            name: name.to_string(),
            version,
            source: format!("path+{}", canonical.display()),
            root: Some(dest),
            git: None,
            rev: None,
        });
    }

    if let Some(git) = &dep.git {
        let dest = vendor_dir(project_root, name);
        let rev = fetch_git_dep(git, dep, &dest)?;
        let version = read_dep_version(&dest).unwrap_or_else(|| "0.0.0".to_string());
        if let Some(req) = &dep.version {
            ensure_version(name, &version, req)?;
        }
        return Ok(LockedPackage {
            name: name.to_string(),
            version,
            source: format!("git+{git}"),
            root: Some(dest),
            git: Some(git.clone()),
            rev: Some(rev),
        });
    }

    Err(VppError::Other {
        message: format!("dependency `{name}` must specify `path` or `git`"),
    })
}

fn read_dep_version(root: &Path) -> Option<String> {
    let text = std::fs::read_to_string(root.join("vpp.toml")).ok()?;
    let manifest = super::manifest::parse_manifest_toml(&text).ok()?;
    Some(manifest.version)
}

fn ensure_version(name: &str, found: &str, req: &str) -> VppResult<()> {
    let found_v = Version::parse(found).map_err(|e| VppError::Other {
        message: format!("dependency `{name}` has invalid version `{found}`: {e}"),
    })?;
    let req_v = semver::VersionReq::parse(req).map_err(|e| VppError::Other {
        message: format!("invalid version requirement `{req}` for `{name}`: {e}"),
    })?;
    if !req_v.matches(&found_v) {
        return Err(VppError::Other {
            message: format!(
                "dependency `{name}` version `{found}` does not satisfy requirement `{req}`"
            ),
        });
    }
    Ok(())
}

fn link_or_copy(src: &Path, dest: &Path) -> VppResult<()> {
    if dest.exists() {
        std::fs::remove_dir_all(dest).ok();
        std::fs::remove_file(dest).ok();
    }
    std::fs::create_dir_all(dest.parent().unwrap_or(Path::new("."))).map_err(|e| {
        VppError::Other {
            message: format!("failed to create vendor parent: {e}"),
        }
    })?;

    #[cfg(windows)]
    {
        let status = Command::new("cmd")
            .args([
                "/C",
                "mklink",
                "/J",
                &dest.to_string_lossy(),
                &src.to_string_lossy(),
            ])
            .status()
            .map_err(|e| VppError::Other {
                message: format!("failed to junction `{dest:?}` -> `{src:?}`: {e}"),
            })?;
        if status.success() {
            return Ok(());
        }
    }

    copy_dir_recursive(src, dest)
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> VppResult<()> {
    std::fs::create_dir_all(dest).map_err(|e| VppError::Other {
        message: format!("failed to create `{}`: {e}", dest.display()),
    })?;
    for entry in std::fs::read_dir(src).map_err(|e| VppError::Other {
        message: format!("failed to read `{}`: {e}", src.display()),
    })? {
        let entry = entry.map_err(|e| VppError::Other {
            message: e.to_string(),
        })?;
        let ty = entry.file_type().map_err(|e| VppError::Other {
            message: e.to_string(),
        })?;
        let from = entry.path();
        let to = dest.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else {
            std::fs::copy(&from, &to).map_err(|e| VppError::Other {
                message: format!("failed to copy `{}`: {e}", from.display()),
            })?;
        }
    }
    Ok(())
}

fn fetch_git_dep(git: &str, dep: &DetailedDependency, dest: &Path) -> VppResult<String> {
    if dest.exists() {
        std::fs::remove_dir_all(dest).map_err(|e| VppError::Other {
            message: format!("failed to remove old git dep `{}`: {e}", dest.display()),
        })?;
    }

    let mut clone = Command::new("git");
    clone.args(["clone", "--depth", "1", git, &dest.to_string_lossy()]);
    if let Some(branch) = &dep.branch {
        clone.args(["--branch", branch]);
    } else if let Some(tag) = &dep.tag {
        clone.args(["--branch", tag]);
    }
    let status = clone.status().map_err(|e| VppError::Other {
        message: format!("failed to run git clone for `{git}`: {e}"),
    })?;
    if !status.success() {
        return Err(VppError::Other {
            message: format!("git clone failed for dependency `{git}`"),
        });
    }

    if let Some(rev) = &dep.rev {
        let checkout = Command::new("git")
            .args(["checkout", rev])
            .current_dir(dest)
            .status()
            .map_err(|e| VppError::Other {
                message: format!("failed to checkout `{rev}`: {e}"),
            })?;
        if !checkout.success() {
            return Err(VppError::Other {
                message: format!("git checkout `{rev}` failed"),
            });
        }
        return Ok(rev.clone());
    }

    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(dest)
        .output()
        .map_err(|e| VppError::Other {
            message: format!("failed to read git HEAD: {e}"),
        })?;
    if !output.status.success() {
        return Err(VppError::Other {
            message: "failed to resolve git revision".to_string(),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
