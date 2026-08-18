# Changelog

All notable changes to v++ are documented here.

## [0.3.0] — 2026-08-18 (Usable Language / Ecosystem)

### Added

- **Module system**: `import std.io` canonical paths, `pub` exports, namespaced calls (`math.add`), legacy `import "file.vpp"`, circular import detection, duplicate import errors
- **Package manager**: TOML `vpp.toml` with `[dependencies]`, `vpp.lock`, `vpp new`/`add`/`remove`/`update`, local path + git deps, semver checks
- **Central registry**: `registry/index.toml` with semver resolution (`hello-lib = "0.1.0"`)
- **Standard library**: `std.io`, `std.math`, `std.string`, `std.collections`, `std.fs`, `std.json`, `std.process`
- **Native fs/json/process**: runtime C helpers + LLVM codegen for `read_file`, `write_file`, `file_exists`, `json_parse`, `json_stringify`, `process_run`
- **CLI**: `vpp doctor` for toolchain/project health
- **LSP**: diagnostic spans from miette; extension wires `vppls` via vscode-languageclient
- **Distribution**: GitHub Actions release workflow (linux/windows/macos) on version tags
- **Syntax highlighting**: struct, enum, match, import, pub, test, break, continue, builtins
- **Tests**: `tests/modules.rs`, `tests/pkg.rs`, `tests/stdlib.rs`, `examples/std_builtins.vpp` parity; all v0.2 parity preserved

### Changed

- `vpp init` scaffolds `import std.io` and `import std.math`
- LSP `check_with_index` uses in-memory buffer when file is unsaved
- Interpreter invokes `fn main()` when present (matches native entry point)
- Native codegen: correct scope cleanup on `return`, `if`/`match` arms, and heap release paths

### Fixed

- Native `file_exists` bool codegen (i32 → i1 compare)
- LLVM invalid IR when `return` preceded function epilogue cleanup
- Function parameters lost after `return` inside `if` branches during codegen
- Empty stdout under piped test runs (`fflush` in runtime prints; staged link output)

## [0.2.0] — 2026-08-18

### Added

- v++ IR (`src/ir/`) between typed AST and LLVM
- Shared builtin registry (`src/builtins/`)
- `ARCHITECTURE.md`, `MEMORY_MODEL.md`, `SPEC.md`
- Native string ABI using `VppString*` end-to-end
- **Native array ABI using `VppArray*` end-to-end**
- Scoped locals in native codegen (correct shadowing)
- Float equality/compare via LLVM float instructions
- **Native structs** — literals, field access, fn params/returns
- **Native enums / Option / Result** — tagged struct representation
- **Native match** — statement and expression forms
- **Native break/continue** — loop stack for while/for loops
- Differential parity tests (`tests/parity/`) — hello, lesson01, scope_shadow, arrays, arrays_fn, **structs, option_result, match_test, lesson03_loops**
- `tests/arrays.rs` — IR lowering tests for arrays
- `examples/arrays_fn.vpp` — arrays as function params/returns
- Native `vpp_array_index_ptr` bounds checking
- CI job for native codegen (where LLVM available)

### Fixed

- Native crash on `print("...")` (i8* vs VppString* mismatch)
- Broken block scope in native codegen
- Integer compare used for float equality

### Changed

- LLVM backend lowers from v++ IR, not directly from typed AST
- Inkwell dependency pinned for reproducible builds
- Runtime uses portable `strdup` wrapper
- Struct/enum/match codegen in `src/codegen/struct_enum.rs`

### Known limitations (v0.2)

- Module imports: interpreter only until native parity tests pass
- Full ARC at all scope exits not yet complete for heap values inside structs/enums
- Match exhaustiveness is runtime-checked only (non-exhaustive match calls `vpp_assert_fail`)

## [0.1.0] — 2026-08

Initial release: interpreter-complete language, partial LLVM backend, CLI, extension, stdlib, CI.
