pub mod runtime;

#[cfg(feature = "codegen")]
mod emit;

#[cfg(feature = "codegen")]
mod llvm;

#[cfg(feature = "codegen")]
pub use llvm::compile;

#[cfg(not(feature = "codegen"))]
use crate::error::{VppError, VppResult};

#[cfg(not(feature = "codegen"))]
use crate::types::TypedProgram;

#[cfg(not(feature = "codegen"))]
use std::path::Path;

#[cfg(not(feature = "codegen"))]
pub fn compile(
    _program: &TypedProgram,
    _source_path: &Path,
    _output: Option<&Path>,
    _emit_ir: Option<&Path>,
) -> VppResult<()> {
    Err(VppError::Other {
        message: "codegen is disabled; rebuild with `--features codegen`".to_string(),
    })
}
