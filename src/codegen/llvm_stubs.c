// v++ uses inkwell to emit LLVM IR, then compiles with clang.
// Windows LLVM-C.dll does not export target-init symbols; provide no-op stubs.
// Note: inkwell links against the LLVM_* (underscore) symbol names.

void LLVM_InitializeNativeTarget(void) {}
void LLVM_InitializeNativeAsmPrinter(void) {}
void LLVM_InitializeNativeAsmParser(void) {}
void LLVM_InitializeNativeDisassembler(void) {}

void LLVM_InitializeAllTargets(void) {}
void LLVM_InitializeAllTargetInfos(void) {}
void LLVM_InitializeAllTargetMCs(void) {}
void LLVM_InitializeAllAsmPrinters(void) {}
void LLVM_InitializeAllAsmParsers(void) {}
void LLVM_InitializeAllDisassemblers(void) {}

void vpp_force_llvm_stubs(void) {}
