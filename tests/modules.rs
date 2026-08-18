//! Module system integration tests.

use std::path::PathBuf;

use vpp::check_path;

fn example(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(name)
}

#[test]
fn legacy_string_import_still_works() {
    check_path(&example("modules_main.vpp")).expect("modules_main should type-check");
}

#[test]
fn canonical_module_import_parses() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/module_math.vpp");
    check_path(&path).expect("module import should work");
}

#[test]
fn rejects_unqualified_module_function() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/module_bad.vpp");
    let err = check_path(&path).unwrap_err();
    assert!(err.to_string().contains("not in scope"));
}
