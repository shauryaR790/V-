use std::path::Path;

use crate::error::VppResult;
use crate::ir::lower_program_with_enums;
use crate::types::TypedProgram;

use super::emit;

pub fn compile(
    program: &TypedProgram,
    source_path: &Path,
    output: Option<&Path>,
    emit_ir: Option<&Path>,
) -> VppResult<()> {
    let ir = lower_program_with_enums(program)?;
    emit::compile_module(&ir, source_path, output, emit_ir)
}
