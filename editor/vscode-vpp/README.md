# v++ Language — official VS Code extension

**Write it simply. Compile it natively.**

Official language support for **[v++](https://github.com/shauryaR790/V-)** — a compiled language that reads like Python but ships native binaries. Not a transpiler, not a toy highlighter: a real `vpp` toolchain wired into your editor.

> Install the **compiler** separately — this extension connects VS Code to `vpp`. [Quick start ↓](#quick-start)

---

## Why v++ is different

| | Scripting (Python) | Systems (Rust/C++) | **v++** |
|---|-------------------|-------------------|---------|
| Readability | High | Moderate | **Python-style syntax** |
| Output | Interpreted / VM | Native binary | **Native binary** |
| Learning curve | Gentle | Steep | **Gentle → grow into control** |
| Toolchain in editor | Mature | Mature | **Run · check · fmt · test · LSP** |

**One sentence:** v++ is the language for people who want Python's clarity without giving up native speed — with generics, traits, and compile-time checks when you're ready.

**Official extension:** publisher **`vpp-lang`** · ID **`vpp-lang.vplusplus`**  
Other Marketplace entries named "V++" are unrelated.

---

## Quick start

1. **Compiler** — [GitHub Releases](https://github.com/shauryaR790/V-/releases) (`vpp-*-setup.exe` on Windows) or build from source.
2. **Extension** — install **v++ Language** by **vpp-lang** from the Marketplace.
3. **Run** — open a `.vpp` file, press **F5**.

```powershell
vpp run examples\hello.vpp
vpp check examples\hello.vpp
vpp fmt examples\hello.vpp
vpp test
```

Docs: [github.com/shauryaR790/V-/tree/main/docs](https://github.com/shauryaR790/V-/tree/main/docs)

---

## Features (v0.6)

| Feature | What you get |
|---------|----------------|
| **Syntax + snippets** | Keywords, types, `mut`, generics, traits; tab completions for `fn`, `struct`, `match`, `test` |
| **Run (F5)** | Executes the active file via `vpp run` |
| **Type-check** | `vpp check` without running |
| **Format** | `vpp fmt` — format-on-save (default) or **Shift+Alt+F** |
| **Tests** | `vpp test` in integrated terminal |
| **Language server** | Diagnostics, completion, go-to-definition via `vppls` (starts when you open a `.vpp` file) |
| **Status bar** | Shows active compiler + LSP state |
| **File icons** | Official transparent **V++** wordmark for `.vpp` files |

---

## Commands

`Ctrl+Shift+P` → type `v++`:

| Command | Shortcut |
|---------|----------|
| **v++: Run File** | F5, Ctrl+Shift+R |
| **v++: Check File** | — |
| **v++: Format Document** | Shift+Alt+F |
| **v++: Run Tests** | — |
| **v++: Show Output** | — |
| **v++: Open Documentation** | — |

---

## Settings

| Setting | Default | Description |
|---------|---------|-------------|
| `vpp.compilerPath` | *(auto)* | Path to `vpp`. Auto-detects `vpp.ps1`, `target/debug/vpp.exe`, etc. |
| `vpp.languageServerPath` | `vppls` | Language server binary |
| `vpp.enableLanguageServer` | `true` | Diagnostics, completion, go-to-definition |
| `vpp.formatOnSave` | `true` | Run `vpp fmt` when saving `.vpp` files |

Click the **v++** item in the status bar to jump to settings.

---

## Requirements

| Component | Used for |
|-----------|----------|
| **vpp** | Run, check, format, test |
| **vppls** | IntelliSense (included in release bundle or `cargo build --features lsp --bin vppls`) |
| **LLVM/clang** | `vpp build` native codegen only |

---

## Roadmap

| Version | Focus |
|---------|--------|
| **v0.6** *(now)* | Format-on-save, status bar, snippets, faster lazy LSP |
| v0.7 | Linux/macOS releases, signed Windows builds |
| v0.8 | Debugger integration |
| v1.0 | Frozen spec, Test Explorer, hosted registry |

[Full roadmap](https://github.com/shauryaR790/V-/blob/main/docs/project/roadmap.md)

---

## Support

- [Documentation](https://github.com/shauryaR790/V-/tree/main/docs)
- [VS Code setup guide](https://github.com/shauryaR790/V-/blob/main/docs/getting-started/vscode-setup.md)
- [Report issues](https://github.com/shauryaR790/V-/issues)

MIT — see [LICENSE](https://github.com/shauryaR790/V-/blob/main/LICENSE).
