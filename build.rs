fn main() {
    if !cfg!(feature = "codegen") {
        return;
    }

    // Windows LLVM-C.dll omits target-init symbols; Unix/macOS link full libLLVM.
    if cfg!(all(target_os = "windows", target_env = "msvc")) {
        cc::Build::new()
            .file("src/codegen/llvm_stubs.c")
            .compile("vpp_llvm_stubs");

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

        let prefix = std::env::var("LLVM_SYS_221_PREFIX").unwrap_or_else(|_| {
            "C:\\Program Files\\LLVM".to_string()
        });
        let lib_dir = std::path::Path::new(&prefix).join("lib");
        if lib_dir.exists() {
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
            println!("cargo:rustc-link-lib=dylib=LLVM-C");
        }
    }

    println!("cargo:rerun-if-env-changed=LLVM_SYS_221_PREFIX");
    println!("cargo:rerun-if-changed=src/codegen/llvm_stubs.c");
}
