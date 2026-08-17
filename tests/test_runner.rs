use std::path::PathBuf;

#[test]
fn runs_inline_test_blocks() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("smoke.vpp");
    let typed = vpp::check_path(&path).expect("smoke.vpp should type-check");
    let count = vpp::interp::run_tests(&typed).expect("tests should pass");
    assert_eq!(count, 2);
}

#[test]
fn init_scaffold_typechecks() {
    use std::fs;

    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("test-init-scaffold");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    vpp::init_project(&dir, "demo").unwrap();
    let main = dir.join("src/main.vpp");
    vpp::check_path(&main).expect("init main.vpp should type-check");
    let tests = dir.join("tests/smoke.vpp");
    vpp::check_path(&tests).expect("init smoke tests should type-check");

    let _ = fs::remove_dir_all(&dir);
}
