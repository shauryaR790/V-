use std::path::Path;
use std::process::Command;

fn main() {
    if !cfg!(feature = "codegen") {
        return;
    }

    if cfg!(all(target_os = "windows", target_env = "msvc")) {
        link_windows_stubs();
    } else {
        link_llvm_via_config();
    }

    println!("cargo:rerun-if-env-changed=LLVM_SYS_221_PREFIX");
    println!("cargo:rerun-if-changed=src/codegen/llvm_stubs.c");
}

fn link_windows_stubs() {
    cc::Build::new()
        .file("src/codegen/llvm_stubs.c")
        .compile("vpp_llvm_stubs");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");
    let stub_lib = Path::new(&out_dir).join("vpp_llvm_stubs.lib");
    println!("cargo:rustc-link-arg={}", stub_lib.display());
    for sym in [
        "LLVM_InitializeAllTargets",
        "LLVM_InitializeAllTargetInfos",
        "LLVM_InitializeAllTargetMCs",
        "LLVM_InitializeAllAsmPrinters",
        "LLVM_InitializeAllAsmParsers",
        "LLVM_InitializeAllDisassemblers",
        "LLVM_InitializeNativeTarget",
        "LLVM_InitializeNativeAsmPrinter",
        "LLVM_InitializeNativeAsmParser",
        "LLVM_InitializeNativeDisassembler",
    ] {
        println!("cargo:rustc-link-arg=/INCLUDE:{sym}");
    }

    let prefix = std::env::var("LLVM_SYS_221_PREFIX")
        .unwrap_or_else(|_| "C:\\Program Files\\LLVM".to_string());
    let lib_dir = Path::new(&prefix).join("lib");
    if lib_dir.exists() {
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=dylib=LLVM-C");
    }
}

fn link_llvm_via_config() {
    let config = find_llvm_config();
    emit_link_flags(&run_llvm_config(&config, &["--ldflags"]));
    emit_link_flags(&run_llvm_config(&config, &["--libs"]));
    emit_link_flags(&run_llvm_config(&config, &["--system-libs"]));
    if cfg!(target_env = "gnu") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}

fn find_llvm_config() -> String {
    let mut candidates = Vec::new();
    if let Ok(prefix) = std::env::var("LLVM_SYS_221_PREFIX") {
        candidates.push(format!("{prefix}/bin/llvm-config"));
        candidates.push(format!("{prefix}/bin/llvm-config-22"));
    }
    candidates.extend([
        "llvm-config-22".to_string(),
        "llvm-config".to_string(),
    ]);

    for candidate in candidates {
        if Path::new(&candidate).exists()
            || Command::new(&candidate)
                .arg("--version")
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        {
            return candidate;
        }
    }

    panic!(
        "llvm-config not found; set LLVM_SYS_221_PREFIX to your LLVM 22 install prefix"
    );
}

fn run_llvm_config(config: &str, args: &[&str]) -> String {
    let output = Command::new(config)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("failed to run {config} {:?}: {e}", args));
    if !output.status.success() {
        panic!(
            "{config} {:?} failed: {}",
            args,
            String::from_utf8_lossy(&output.stderr)
        );
    }
    String::from_utf8(output.stdout)
        .expect("llvm-config output must be utf-8")
        .trim()
        .to_string()
}

fn emit_link_flags(flags: &str) {
    for flag in flags.split_whitespace() {
        if let Some(path) = flag.strip_prefix("-L") {
            println!("cargo:rustc-link-search=native={path}");
        } else if let Some(lib) = flag.strip_prefix("-l") {
            if lib.starts_with(':') {
                println!("cargo:rustc-link-arg=-l{lib}");
            } else {
                println!("cargo:rustc-link-lib=dylib={lib}");
            }
        } else if flag.starts_with('-') {
            println!("cargo:rustc-link-arg={flag}");
        }
    }
}
