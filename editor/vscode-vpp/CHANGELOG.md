## 1.2.0

**Recommended for Reddit / new users** — stable debugger + tests + formatter fix.

> **Version pairing:** Extension **1.2.0** (this VSIX) + Compiler **v1.0.4** ([GitHub Releases](https://github.com/shauryaR790/VPP/releases/latest)). Extension and compiler use **separate version numbers** — install both.

### Fixed
- **Format on save** no longer corrupts code (`let mut x` stayed broken as `let mutx`; ranges like `1..6` were split incorrectly). Rebuild or install `vpp` ≥ 1.0.1 before relying on format-on-save.

### Extension
- **F5 Debug** — breakpoints, step, next, locals (DAP via `vpp debug --dap`)
- **Ctrl+F5 Run** — run without debugging
- **Test Explorer** — sidebar discovery via `vpp test --list`
- **Watch / REPL / Bench** — toolbar + command palette
- **Registry search** — `vpp search` from palette
- **LSP** — diagnostics, completion, go-to-definition (`vppls`)

### Compiler (install separately from GitHub Releases)
- **v1.0.4** — CMake modules bundled in installer; frozen SPEC v1.0 + Parity Promise
- Full CLI: run, repl, watch, debug, bench, build, test, search, doctor, packages, fmt

### Known limits (honest)
- **Windows** — full native build + GitHub Releases installer (primary platform)
- **Linux / macOS** — interpreter + LSP work; native codegen bundles still improving on CI
