use std::path::PathBuf;

use vpp::check;

fn example_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples").join(name)
}

#[test]
fn hello_typechecks() {
    let source = std::fs::read_to_string(example_path("hello.vpp")).unwrap();
    check(&source).unwrap();
}

#[test]
fn fib_typechecks() {
    let source = std::fs::read_to_string(example_path("fib.vpp")).unwrap();
    check(&source).unwrap();
}

#[test]
fn arrays_typechecks() {
    let source = std::fs::read_to_string(example_path("arrays.vpp")).unwrap();
    check(&source).unwrap();
}

#[cfg(feature = "codegen")]
#[test]
fn hello_compiles_and_runs() {
    let path = example_path("hello.vpp");
    let source = std::fs::read_to_string(&path).unwrap();
    vpp::run(&source, &path).unwrap();
}
