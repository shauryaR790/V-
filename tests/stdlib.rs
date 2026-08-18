//! Stdlib fs/json/process parity and registry tests.

#![cfg(feature = "codegen")]

use std::path::{Path, PathBuf};
use std::process::Command;

use vpp::driver::CompileOptions;

fn run_in_dir(dir: &Path, subcommand: &str, file: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_vpp"))
        .arg(subcommand)
        .arg(file)
        .current_dir(dir)
        .output()
        .expect("failed to spawn vpp")
}

fn stdlib_parity(source: &str, filename: &str) {
    let dir = std::env::temp_dir().join(format!("vpp-stdlib-{}-{}", filename, std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join(filename);
    std::fs::write(&file, source).unwrap();
    let exe = dir.join("out.exe");

    let interp = run_in_dir(&dir, "run", &file);
    assert!(
        interp.status.success(),
        "interpreter failed: {}",
        String::from_utf8_lossy(&interp.stderr)
    );

    vpp::compile(
        source,
        &file,
        CompileOptions {
            output: Some(exe.clone()),
            emit_ir: None,
        },
    )
    .expect("native compile failed");

    let native = Command::new(&exe)
        .current_dir(&dir)
        .output()
        .expect("failed to run native binary");
    assert!(
        native.status.success(),
        "native failed: {}",
        String::from_utf8_lossy(&native.stderr)
    );

    let i_out = String::from_utf8_lossy(&interp.stdout).replace("\r\n", "\n");
    let n_out = String::from_utf8_lossy(&native.stdout).replace("\r\n", "\n");
    assert_eq!(i_out, n_out, "interpreter/native stdout mismatch");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn std_fs_json_parity() {
    stdlib_parity(
        r#"import std.fs
import std.json

fn main() -> int {
    fs.write("vpp_std_fs_test.txt", "parity")
    print(fs.read("vpp_std_fs_test.txt"))
    if fs.exists("vpp_std_fs_test.txt") {
        print("yes")
    }
    let j = json.parse("{\"ok\":true}")
    print(json.stringify(j))
    return 0
}
"#,
        "std_test.vpp",
    );
}

#[test]
fn std_process_parity() {
    let cmd = if cfg!(windows) {
        "exit 0"
    } else {
        "true"
    };
    stdlib_parity(
        &format!(
            r#"import std.process

fn main() -> int {{
    let code = process.run("{cmd}")
    print(code)
    return 0
}}
"#
        ),
        "proc_test.vpp",
    );
}

#[test]
fn registry_resolves_hello_lib() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dep = vpp::resolve_from_registry(&root, "hello-lib", "0.1.0").unwrap();
    assert!(dep.path.is_some());
    let lock = vpp::resolve_dependencies(
        &root,
        &vpp::Manifest {
            name: "test".into(),
            version: "0.1.0".into(),
            entry: "src/main.vpp".into(),
            dependencies: [("hello-lib".to_string(), vpp::DependencySpec::Version("0.1.0".into()))]
                .into(),
        },
    )
    .unwrap();
    assert_eq!(lock.packages.len(), 1);
    assert_eq!(lock.packages[0].name, "hello-lib");
}
