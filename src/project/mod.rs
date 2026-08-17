use std::path::{Path, PathBuf};

use crate::error::{VppError, VppResult};

#[derive(Debug, Clone)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub entry: PathBuf,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            name: "app".to_string(),
            version: "0.1.0".to_string(),
            entry: PathBuf::from("src/main.vpp"),
        }
    }
}

pub fn find_project_root(start: &Path) -> Option<PathBuf> {
    let mut dir = if start.is_file() {
        start.parent()?.to_path_buf()
    } else {
        start.to_path_buf()
    };

    loop {
        if dir.join("vpp.toml").exists() {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
}

pub fn load_manifest(project_root: &Path) -> VppResult<Manifest> {
    let path = project_root.join("vpp.toml");
    let text = std::fs::read_to_string(&path).map_err(|e| VppError::Other {
        message: format!("failed to read `{}`: {e}", path.display()),
    })?;
    parse_manifest(&text)
}

pub fn parse_manifest(text: &str) -> VppResult<Manifest> {
    let mut manifest = Manifest::default();

    for line in text.lines() {
        let line = line.split('#').next().unwrap_or(line).trim();
        if line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim().trim_matches('"').trim_matches('\'');
        match key {
            "name" => manifest.name = value.to_string(),
            "version" => manifest.version = value.to_string(),
            "entry" => manifest.entry = PathBuf::from(value),
            _ => {}
        }
    }

    Ok(manifest)
}

pub fn std_search_paths(project_root: Option<&Path>) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(root) = project_root {
        paths.push(root.join("std"));
    }

    if let Ok(vpp_home) = std::env::var("VPP_HOME") {
        paths.push(PathBuf::from(vpp_home).join("std"));
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            paths.push(dir.join("std"));
            if let Some(parent) = dir.parent() {
                paths.push(parent.join("std"));
            }
        }
    }

    // When developing v++ from source
    paths.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("std"));

    paths.retain(|p| p.exists());
    paths
}

pub fn init_project(dir: &Path, name: &str) -> VppResult<()> {
    if dir.join("vpp.toml").exists() {
        return Err(VppError::Other {
            message: format!("`{}` already has a vpp.toml", dir.display()),
        });
    }

    std::fs::create_dir_all(dir.join("src")).map_err(|e| VppError::Other {
        message: format!("failed to create src/: {e}"),
    })?;
    std::fs::create_dir_all(dir.join("tests")).map_err(|e| VppError::Other {
        message: format!("failed to create tests/: {e}"),
    })?;

    let manifest = format!(
        r#"name = "{name}"
version = "0.1.0"
entry = "src/main.vpp"
"#
    );
    std::fs::write(dir.join("vpp.toml"), manifest).map_err(|e| VppError::Other {
        message: format!("failed to write vpp.toml: {e}"),
    })?;

    let main_vpp = format!(
        r#"import "std/io.vpp"

fn main() -> int {{
    greet("{name}")
    return 0
}}
"#
    );
    std::fs::write(dir.join("src/main.vpp"), main_vpp).map_err(|e| VppError::Other {
        message: format!("failed to write src/main.vpp: {e}"),
    })?;

    let test_vpp = r#"import "std/math.vpp"

test "math works" {
    assert_eq(add(2, 2), 4)
    assert_eq(pow(2, 3), 8)
}

test "strings work" {
    assert(len("hi") == 2)
}
"#;
    std::fs::write(dir.join("tests/smoke.vpp"), test_vpp).map_err(|e| VppError::Other {
        message: format!("failed to write tests/smoke.vpp: {e}"),
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_manifest() {
        let m = parse_manifest(r#"name = "demo"
version = "1.0.0"
entry = "src/main.vpp"
"#)
        .unwrap();
        assert_eq!(m.name, "demo");
        assert_eq!(m.entry, PathBuf::from("src/main.vpp"));
    }
}
