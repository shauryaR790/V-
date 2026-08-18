//! Package manager tests.

use std::fs;

use vpp::{parse_manifest_toml, resolve_dependencies, DependencySpec, Manifest};

#[test]
fn parses_dependencies_in_manifest() {
    let m = parse_manifest_toml(
        r#"name = "demo"
version = "0.1.0"
entry = "src/main.vpp"

[dependencies]
helper = { path = "../helper" }
"#,
    )
    .unwrap();
    assert!(m.dependencies.contains_key("helper"));
}

#[test]
fn resolves_local_path_dependency() {
    let dir = std::env::temp_dir().join(format!("vpp-pkg-test-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::create_dir_all(dir.join("src")).unwrap();

    let helper = dir.join("helper");
    fs::create_dir_all(&helper).unwrap();
    fs::write(
        helper.join("vpp.toml"),
        r#"name = "helper"
version = "1.0.0"
entry = "src/main.vpp"
"#,
    )
    .unwrap();

    let manifest = Manifest {
        name: "demo".to_string(),
        version: "0.1.0".to_string(),
        entry: "src/main.vpp".into(),
        dependencies: [(
            "helper".to_string(),
            DependencySpec::from_path("helper"),
        )]
        .into(),
    };

    let lock = resolve_dependencies(&dir, &manifest).unwrap();
    assert_eq!(lock.packages.len(), 1);
    assert!(lock.packages[0].root.as_ref().unwrap().exists());

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn registry_version_dependency() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest = Manifest {
        name: "demo".to_string(),
        version: "0.1.0".to_string(),
        entry: "src/main.vpp".into(),
        dependencies: [(
            "hello-lib".to_string(),
            DependencySpec::Version("0.1.0".into()),
        )]
        .into(),
    };
    let lock = resolve_dependencies(&root, &manifest).unwrap();
    assert_eq!(lock.packages[0].name, "hello-lib");
    let _ = fs::remove_dir_all(root.join(".vpp"));
}
