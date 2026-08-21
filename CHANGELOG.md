# Changelog

All notable changes to v++ are documented here.

## [0.6.2] — 2026-08-21

- Version alignment for Marketplace (extension + compiler)

## [0.6.0] — 2026-08-21

### Added — Interactive development (v0.6 milestone)

- **`vpp repl`** — persistent read-eval-print loop using the same interpreter as `vpp run`
- **Parity promise** documented: same `.vpp` source for `run`, `repl`, and `build` ([VERSION_ROADMAP.md](docs/project/VERSION_ROADMAP.md))
- **VS Code extension 0.6.0** — format-on-save, REPL terminal command, snippets, status bar, lazy LSP

### Unique differentiator

v++ is the readable compiled language where **one file** teaches (`repl`/`run`) and ships (`build`) — with CI parity tests proving both paths match.

## [0.5.0] — 2026-08-18

### Added

- **Website** — official site at `website/` (learn, 20 projects, downloads, history, architecture, about)
- **20 example projects** — `projects/01-hello-world` through `projects/20-json-config`
- **GitHub Pages** — deploy workflow for the website
- **Documentation hub** — 30+ guides (`docs/getting-started`, `language`, `guides`, `stdlib`, `project`)
- **GitHub paperwork** — `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`
- **VS Code extension v0.5.0** — Python-style Marketplace README and CHANGELOG
- **Windows installer PATH** — adds `vpp` and bundled `clang` to user PATH on install

### Changed

- Compiler, installer, and release bundle version aligned to **0.5.0**

## [0.4.4] — 2026-08-18 (Windows installer)

### Added

- **GitHub Release bundles** — Windows zip with `GO.bat` one-click launcher; Linux/macOS tarballs with `run.sh`
- **VS Code Marketplace** — extension published as `vpp-lang.vplusplus`
- **Docs** — `docs/RELEASE.md`, `docs/MARKETPLACE.md`, simplified `docs/INSTALL.md`

### Fixed

- LSP build (`jsonrpc_core`, `CompletionItemKind::SNIPPET`)
- `setup.ps1` PowerShell encoding + builds with `codegen,lsp` by default
- VSIX packaging includes language-client dependencies

## [0.4.0] — 2026-08-18 (Language Core / Phase B)

### Added

- **`mut` keyword** — `let mut x = 1` required for reassignment; bindings are immutable by default
- **Generics** — monomorphized functions: `fn id[T](x: T) -> T`, calls like `id[int](42)` and `first[string](words)`
- **Traits and impls** — `trait Display { fn to_text(self) -> string }`, `impl Display for User { ... }`, method calls `user.to_text()`
- **Compile-time match exhaustiveness** — enums, `Option`, and `Result` must be covered or use `_` (error E0107)
- **Examples** — [`examples/generics.vpp`](examples/generics.vpp), [`examples/traits.vpp`](examples/traits.vpp)
- **Tests** — mut/immutability, exhaustiveness, generics, and traits typecheck + parity coverage

### Changed

- All examples and stdlib sources updated to use `let mut` where variables are reassigned
- [`SPEC.md`](SPEC.md) updated for v0.4 language features

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

## [0.3.1] — 2026-08-18 (Phase A polish)

### Fixed

- User-defined enum types resolved correctly in function params, struct fields, and match patterns
- Bare enum variant literals in struct fields (e.g. `status: Active`)
- Native entry point: LLVM `main` calls `vpp_user_main` instead of symbol collision
- Path lookup for `vpp run hello.vpp` searches `examples/`, `src/`, `tests/`

### Added

- [`stress.vpp`](stress.vpp) and [`stress.ps1`](stress.ps1) — one-command interpreter/native parity test
- `.gitignore` entries for `.vpp/`, test output artifacts

### Changed

- [`SPEC.md`](SPEC.md) and [`README.md`](README.md) updated for v0.3

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
