//! Interpreter vs native output parity tests.
//! Run: cargo test --features codegen parity -- --nocapture

#![cfg(feature = "codegen")]

use std::path::{Path, PathBuf};
use std::process::Command;

use vpp::driver::CompileOptions;

fn example(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(name)
}

fn run_interpreter(path: &Path) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_vpp"))
        .arg("run")
        .arg(path)
        .output()
        .expect("failed to run vpp interpreter");
    assert!(
        output.status.success(),
        "interpreter failed for {}: {}",
        path.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn run_native(path: &Path, exe: &Path) -> String {
    let source = std::fs::read_to_string(path).unwrap();
    vpp::compile(
        &source,
        path,
        CompileOptions {
            output: Some(exe.to_path_buf()),
            emit_ir: None,
        },
    )
    .unwrap_or_else(|e| panic!("native compile failed for {}: {e}", path.display()));

    let output = Command::new(exe).output().expect("failed to run native binary");
    assert!(
        output.status.success(),
        "native run failed for {}: {}",
        path.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn normalize_output(s: String) -> String {
    s.replace("\r\n", "\n")
}

fn assert_parity(name: &str) {
    let path = example(name);
    let stem = path.file_stem().unwrap().to_string_lossy();
    let exe = std::env::temp_dir().join(format!("vpp-parity-{stem}-{}.exe", std::process::id()));

    let interp = normalize_output(run_interpreter(&path));
    let native = normalize_output(run_native(&path, &exe));
    assert_eq!(
        interp, native,
        "interpreter/native stdout mismatch for {}",
        path.display()
    );
    let _ = std::fs::remove_file(&exe);
}

#[test]
fn hello_parity() {
    assert_parity("hello.vpp");
}

#[test]
fn scope_shadow_parity() {
    assert_parity("scope_shadow.vpp");
}

#[test]
fn lesson01_parity() {
    assert_parity("lesson01_basics.vpp");
}

#[test]
fn arrays_parity() {
    assert_parity("arrays.vpp");
}

#[test]
fn arrays_fn_parity() {
    assert_parity("arrays_fn.vpp");
}

#[test]
fn structs_parity() {
    assert_parity("structs.vpp");
}

#[test]
fn option_result_parity() {
    assert_parity("option_result.vpp");
}

#[test]
fn match_test_parity() {
    assert_parity("match_test.vpp");
}

#[test]
fn std_builtins_example_parity() {
    assert_parity("std_builtins.vpp");
}

#[test]
fn lesson03_loops_parity() {
    assert_parity("lesson03_loops.vpp");
}
