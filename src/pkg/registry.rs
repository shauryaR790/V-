//! Central package registry lookup for semver dependencies.

use std::path::{Path, PathBuf};

use semver::{Version, VersionReq};

use super::manifest::DetailedDependency;
use crate::error::{VppError, VppResult};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RegistryIndex {
    pub packages: Vec<RegistryPackage>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RegistryPackage {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub path: Option<PathBuf>,
    #[serde(default)]
    pub git: Option<String>,
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(default)]
    pub branch: Option<String>,
}

pub fn registry_search_paths(project_root: Option<&Path>) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Ok(url) = std::env::var("VPP_REGISTRY") {
        paths.push(PathBuf::from(url));
    }

    if let Some(root) = project_root {
        paths.push(root.join("registry").join("index.toml"));
    }

    if let Ok(vpp_home) = std::env::var("VPP_HOME") {
        paths.push(PathBuf::from(vpp_home).join("registry").join("index.toml"));
    }

    paths.push(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("registry")
            .join("index.toml"),
    );

    paths.retain(|p| p.exists());
    paths.dedup();
    paths
}

pub fn load_index(path: &Path) -> VppResult<RegistryIndex> {
    let text = std::fs::read_to_string(path).map_err(|e| VppError::Other {
        message: format!("failed to read registry `{}`: {e}", path.display()),
    })?;
    toml::from_str(&text).map_err(|e| VppError::Other {
        message: format!("invalid registry index `{}`: {e}", path.display()),
    })
}

pub fn resolve_from_registry(
    project_root: &Path,
    name: &str,
    version_req: &str,
) -> VppResult<DetailedDependency> {
    let req = VersionReq::parse(version_req).map_err(|e| VppError::Other {
        message: format!("invalid version requirement `{version_req}`: {e}"),
    })?;

    for index_path in registry_search_paths(Some(project_root)) {
        let index = load_index(&index_path)?;
        let registry_root = index_path.parent().unwrap_or(Path::new("."));

        let mut candidates: Vec<&RegistryPackage> = index
            .packages
            .iter()
            .filter(|p| p.name == name)
            .collect();
        candidates.sort_by(|a, b| {
            Version::parse(&b.version)
                .unwrap_or_else(|_| Version::new(0, 0, 0))
                .cmp(&Version::parse(&a.version).unwrap_or_else(|_| Version::new(0, 0, 0)))
        });

        for pkg in candidates {
            let ver = Version::parse(&pkg.version).map_err(|e| VppError::Other {
                message: format!("registry package `{name}` has invalid version: {e}"),
            })?;
            if !req.matches(&ver) {
                continue;
            }
            if let Some(rel) = &pkg.path {
                let abs = if rel.is_absolute() {
                    rel.clone()
                } else {
                    registry_root.join(rel)
                };
                return Ok(DetailedDependency {
                    version: Some(version_req.to_string()),
                    path: Some(abs),
                    git: None,
                    tag: None,
                    branch: None,
                    rev: None,
                });
            }
            if let Some(git) = &pkg.git {
                return Ok(DetailedDependency {
                    version: Some(version_req.to_string()),
                    path: None,
                    git: Some(git.clone()),
                    tag: pkg.tag.clone(),
                    branch: pkg.branch.clone(),
                    rev: None,
                });
            }
        }
    }

    Err(VppError::Other {
        message: format!(
            "no registry package `{name}` matching `{version_req}` (set VPP_REGISTRY or add to registry/index.toml)"
        ),
    })
}
