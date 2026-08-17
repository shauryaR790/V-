pub mod ast;
pub mod codegen;
pub mod driver;
pub mod error;
pub mod fmt;
pub mod interp;
pub mod lexer;
pub mod project;
#[cfg(feature = "lsp")]
pub mod lsp;
pub mod modules;
pub mod parser;
pub mod span;
pub mod symbols;
pub mod types;

#[cfg(feature = "codegen")]
#[link(name = "vpp_llvm_stubs", kind = "static")]
extern "C" {
    fn vpp_force_llvm_stubs();
}

#[cfg(feature = "codegen")]
pub fn ensure_llvm_stubs_linked() {
    unsafe { vpp_force_llvm_stubs() };
}

pub use driver::{
    check, check_file, check_path, check_with_index, compile, format_source, init_project,
    parse, project_entry, run, run_tests_in_project, emit_ir, CompileOptions,
};
pub use project::{find_project_root, load_manifest, Manifest};
pub use error::{VppError, VppResult};
