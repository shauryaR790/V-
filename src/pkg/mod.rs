//! Package manager: manifest dependencies, lockfile, and resolution.

mod lockfile;
mod manifest;
mod registry;
mod resolver;

pub use lockfile::{load_lockfile, write_lockfile, Lockfile, LockedPackage};
pub use registry::{load_index, registry_search_paths, resolve_from_registry, search_registry, RegistryIndex, RegistryPackage};
pub use manifest::{parse_manifest_toml, write_manifest, DependencySpec, Manifest};
pub use resolver::{resolve_dependencies, vendor_dir, VendorLayout};

use std::path::{Path, PathBuf};

use crate::error::{VppError, VppResult};

pub fn deps_dir(project_root: &Path) -> PathBuf {
    project_root.join(".vpp").join("deps")
}

pub fn add_dependency(
    project_root: &Path,
    name: &str,
    spec: DependencySpec,
) -> VppResult<()> {
    let manifest_path = project_root.join("vpp.toml");
    let text = std::fs::read_to_string(&manifest_path).map_err(|e| VppError::Other {
        message: format!("failed to read `{}`: {e}", manifest_path.display()),
    })?;
    let mut manifest = parse_manifest_toml(&text)?;
    if manifest.dependencies.contains_key(name) {
        return Err(VppError::Other {
            message: format!("dependency `{name}` already exists in vpp.toml"),
        });
    }
    manifest.dependencies.insert(name.to_string(), spec);
    write_manifest(project_root, &manifest)?;
    resolve_and_lock(project_root, &manifest)?;
    Ok(())
}

pub fn remove_dependency(project_root: &Path, name: &str) -> VppResult<()> {
    let manifest_path = project_root.join("vpp.toml");
    let text = std::fs::read_to_string(&manifest_path).map_err(|e| VppError::Other {
        message: format!("failed to read `{}`: {e}", manifest_path.display()),
    })?;
    let mut manifest = parse_manifest_toml(&text)?;
    if manifest.dependencies.remove(name).is_none() {
        return Err(VppError::Other {
            message: format!("dependency `{name}` not found in vpp.toml"),
        });
    }
    write_manifest(project_root, &manifest)?;
    resolve_and_lock(project_root, &manifest)?;
    Ok(())
}

pub fn update_dependencies(project_root: &Path) -> VppResult<()> {
    let manifest = crate::project::load_manifest(project_root)?;
    resolve_and_lock(project_root, &manifest)?;
    Ok(())
}

pub fn resolve_and_lock(project_root: &Path, manifest: &Manifest) -> VppResult<Lockfile> {
    let lock = resolve_dependencies(project_root, manifest)?;
    write_lockfile(project_root, &lock)?;
    Ok(lock)
}

pub fn dependency_search_paths(project_root: &Path) -> VppResult<Vec<PathBuf>> {
    let mut paths = Vec::new();
    let lock_path = project_root.join("vpp.lock");
    if lock_path.exists() {
        let lock = load_lockfile(&lock_path)?;
        for pkg in lock.packages {
            if let Some(root) = pkg.root {
                paths.push(root);
            }
        }
    }
    Ok(paths)
}
