## 1.2.0

**Recommended for Reddit / new users** — stable debugger + tests + formatter fix.

### Fixed
- **Format on save** no longer corrupts code (`let mut x` stayed broken as `let mutx`; ranges like `1..6` were split incorrectly). Rebuild or install `vpp` ≥ 1.0.1 before relying on format-on-save.
- Extension version aligned with compiler release line (GitHub **v1.0.3** Windows installer).

### Extension
- **F5 Debug** — breakpoints, step, next, locals (DAP via `vpp debug --dap`)
- **Ctrl+F5 Run** — run without debugging
- **Test Explorer** — sidebar discovery via `vpp test --list`
- **Watch / REPL / Bench** — toolbar + command palette
- **Registry search** — `vpp search` from palette
- **LSP** — diagnostics, completion, go-to-definition (`vppls`)

### Compiler (bundled workflow)
- Frozen SPEC v1.0 + Parity Promise (same `.vpp` for run, repl, watch, debug, build)
- Full CLI: run, repl, watch, debug, bench, build, test, search, doctor, packages, fmt

### Known limits (honest)
- **Windows** — full native build + GitHub Releases installer (primary platform)
- **Linux / macOS** — interpreter + LSP work; native codegen bundles still improving on CI

## 1.0.0

**Stable release** — Parity Promise frozen.

### Compiler
- Frozen SPEC v1.0 + compatibility CI on all examples
- Full CLI: run, repl, watch, debug, bench, build, test, search, doctor, packages

### Extension
- Debug, Test Explorer, registry search (see 1.2.0 for consolidated list)

## 0.9.0

### Ecosystem
- Test Explorer UI (`vpp test --list` JSON)
- `vpp search` for package registry
- SPEC v1.0 release candidate

## 0.8.0

### Debugger
- **`vpp debug`** CLI — step, break, locals (interpreter)
- **VS Code debug launch** — F5 starts DAP session
- Ctrl+F5 run without debugging

## 0.7.0

**Live development** — save-to-run loops without recompile.

### Compiler
- **`vpp watch`** — re-runs on save
- **`vpp bench`** — interpreter timing
- **`vpp doctor`** — OS/arch + LLVM hints

### Extension
- Watch File, Benchmark File commands

## 0.6.2

Initial Marketplace-quality release: LSP, format-on-save, snippets, syntax highlighting.
