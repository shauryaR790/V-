# v++ Parity Promise (v1.0)

**The unique contract:** one readable `.vpp` file works the same across every execution mode.

| Mode | Command | Engine |
|------|---------|--------|
| Run | `vpp run file.vpp` | Interpreter |
| REPL | `vpp repl` | Interpreter (persistent session) |
| Watch | `vpp watch file.vpp` | Interpreter (re-run on save) |
| Debug | `vpp debug file.vpp` / F5 in VS Code | Interpreter (breakpoints, step, locals) |
| Bench | `vpp bench file.vpp` | Interpreter (timing) |
| Ship | `vpp build` | Native (LLVM + clang) |

No second syntax. No “tutorial subset.” Learn with the interpreter; ship with native codegen when ready.

## Why it matters

- **Python** optimizes for approachability but sacrifices static control and native speed by default.
- **Rust/C++** optimize for control but steepen the learning curve.
- **v++** keeps Python-style readability while guaranteeing the **same source** runs in REPL, watch, debug, and production build.

## v1.0 stability

From v1.0 onward, breaking language changes require a **major** version bump. Compatibility is enforced in CI against frozen examples and the SPEC.
