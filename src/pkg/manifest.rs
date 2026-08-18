use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{VppError, VppResult};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub entry: PathBuf,
    #[serde(default)]
    pub dependencies: HashMap<String, DependencySpec>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            name: "app".to_string(),
            version: "0.1.0".to_string(),
            entry: PathBuf::from("src/main.vpp"),
            dependencies: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DependencySpec {
    Version(String),
    Detailed(DetailedDependency),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DetailedDependency {
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub path: Option<PathBuf>,
    #[serde(default)]
    pub git: Option<String>,
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub rev: Option<String>,
}

impl DependencySpec {
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self::Detailed(DetailedDependency {
            version: None,
            path: Some(path.into()),
            git: None,
            tag: None,
            branch: None,
            rev: None,
        })
    }

    pub fn from_git(url: impl Into<String>, tag: Option<String>, branch: Option<String>) -> Self {
        Self::Detailed(DetailedDependency {
            version: None,
            path: None,
            git: Some(url.into()),
            tag,
            branch,
            rev: None,
        })
    }
}

pub fn parse_manifest_toml(text: &str) -> VppResult<Manifest> {
    toml::from_str(text).map_err(|e| VppError::Other {
        message: format!("invalid vpp.toml: {e}"),
    })
}

pub fn write_manifest(project_root: &Path, manifest: &Manifest) -> VppResult<()> {
    let text = toml::to_string_pretty(manifest).map_err(|e| VppError::Other {
        message: format!("failed to serialize vpp.toml: {e}"),
    })?;
    std::fs::write(project_root.join("vpp.toml"), text).map_err(|e| VppError::Other {
        message: format!("failed to write vpp.toml: {e}"),
    })
}
