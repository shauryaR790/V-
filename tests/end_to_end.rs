#[cfg(feature = "codegen")]
mod native {
    use std::path::PathBuf;
    use std::process::Command;

    use vpp::driver::CompileOptions;

    fn example(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join(name)
    }

    #[test]
    fn hello_native_build_and_run() {
        let path = example("hello.vpp");
        let source = std::fs::read_to_string(&path).unwrap();
        let exe = std::env::temp_dir().join(format!("vpp-hello-{}.exe", std::process::id()));

        vpp::compile(
            &source,
            &path,
            CompileOptions {
                output: Some(exe.clone()),
                emit_ir: None,
            },
        )
        .expect("hello should compile natively");

        let output = Command::new(&exe).output().expect("run hello.exe");
        assert!(output.status.success(), "hello.exe crashed");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Hello, v++!"));
        assert!(stdout.contains("Shaurya"));
        let _ = std::fs::remove_file(&exe);
    }

    #[test]
    fn arrays_native_build_and_run() {
        let path = example("arrays.vpp");
        let source = std::fs::read_to_string(&path).unwrap();
        let exe = std::env::temp_dir().join(format!("vpp-arrays-{}.exe", std::process::id()));

        vpp::compile(
            &source,
            &path,
            CompileOptions {
                output: Some(exe.clone()),
                emit_ir: None,
            },
        )
        .expect("arrays.vpp should compile natively");

        let output = Command::new(&exe).output().expect("run arrays.exe");
        assert!(output.status.success(), "arrays.exe crashed");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Alex"));
        assert!(stdout.contains("15"));
        let _ = std::fs::remove_file(&exe);
    }
}
