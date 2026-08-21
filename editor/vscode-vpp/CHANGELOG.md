## 1.0.0

**Stable release** — Parity Promise frozen. Same `.vpp` for run, repl, watch, debug, and native build.

### Compiler
- Frozen SPEC v1.0 + compatibility CI on all examples
- Full CLI: run, repl, watch, debug, bench, build, test, search, doctor, packages

### Extension
- **F5 Debug** — interpreter breakpoints, step, locals (DAP via `vpp debug --dap`)
- **Test Explorer** — discovers `test` blocks via `vpp test --list`
- **Registry search** — `vpp search` from command palette
- Marketplace category: Debuggers

## 0.9.0

### Ecosystem
- Test Explorer UI (`vpp test --list` JSON)
- `vpp search` for package registry
- SPEC v1.0 release candidate

## 0.8.0

### Insight — debugger
- **`vpp debug`** CLI — step, break, locals (interpreter)
- **VS Code debug launch** — F5 starts DAP session (`vpp debug --dap`)
- Ctrl+F5 run without debugging

## 0.7.0

**Live development** — v0.7's differentiator: compiled languages rarely offer instant save-to-run loops.

### Compiler
- **`vpp watch`** — re-runs your file every time you save (live dev loop; same interpreter as run/repl)
- **`vpp bench`** — time repeated interpreter runs; see iteration speed before native build
- **`vpp doctor`** — shows OS/arch, cross-platform LLVM install hints

### Extension
- **v++: Watch File** — eye icon in editor toolbar; opens live watch terminal
- **v++: Benchmark File** — runs `vpp bench` on active file
- Marketplace description updated for watch/REPL/bench workflow

## 0.6.2