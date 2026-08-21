fn main() {
    if !cfg!(feature = "codegen") {
        return;
    }

    let prefix = std::env::var("LLVM_SYS_221_PREFIX").unwrap_or_else(|_| {
        if cfg!(target_os = "windows") {
            "C:\\Program Files\\LLVM".to_string()
        } else {
            "/usr/lib/llvm-22".to_string()
        }
    });

    cc::Build::new()
        .file("src/codegen/llvm_stubs.c")
        .compile("vpp_llvm_stubs");

    if cfg!(all(target_os = "windows", target_env = "msvc")) {
        let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");
        let stub_lib = std::path::Path::new(&out_dir).join("vpp_llvm_stubs.lib");
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
    } else {
        println!("cargo:rustc-link-lib=static=vpp_llvm_stubs");
    }

    let prefix_path = std::path::Path::new(&prefix);
    for lib_dir in [prefix_path.join("lib"), prefix_path.join("lib64")] {
        if lib_dir.exists() {
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
            println!("cargo:rustc-link-lib=dylib=LLVM-C");
        }
    }

    if cfg!(target_env = "gnu") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }

    println!("cargo:rerun-if-env-changed=LLVM_SYS_221_PREFIX");
    println!("cargo:rerun-if-changed=src/codegen/llvm_stubs.c");
}
