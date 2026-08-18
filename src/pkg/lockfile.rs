use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{VppError, VppResult};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Lockfile {
    pub version: u32,
    pub packages: Vec<LockedPackage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LockedPackage {
    pub name: String,
    pub version: String,
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<PathBuf>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>,
}

pub fn load_lockfile(path: &Path) -> VppResult<Lockfile> {
    let text = std::fs::read_to_string(path).map_err(|e| VppError::Other {
        message: format!("failed to read `{}`: {e}", path.display()),
    })?;
    toml::from_str(&text).map_err(|e| VppError::Other {
        message: format!("invalid vpp.lock: {e}"),
    })
}

pub fn write_lockfile(project_root: &Path, lock: &Lockfile) -> VppResult<()> {
    let text = toml::to_string_pretty(lock).map_err(|e| VppError::Other {
        message: format!("failed to serialize vpp.lock: {e}"),
    })?;
    std::fs::write(project_root.join("vpp.lock"), text).map_err(|e| VppError::Other {
        message: format!("failed to write vpp.lock: {e}"),
    })
}
