# v++ Language extension for Visual Studio Code

Official language support for **[v++](https://github.com/shauryaR790/V-)** — a compiled language with Python-style simplicity and native performance. Edit `.vpp` files with syntax highlighting, diagnostics, run/debug integration, and a built-in language server.

> **Install the compiler separately** — this extension does not bundle `vpp`. See [Quick start](#quick-start) below.

---

## Quick start

**Step 1.** Install the v++ compiler ([GitHub Releases](https://github.com/shauryaR790/V-/releases) or build from source).

**Step 2.** Install **v++ Language** from the Marketplace (publisher: **vpp-lang**).

**Step 3.** Open a folder with `.vpp` files, open a file, and press **F5** to run.

```powershell
# Windows — after downloading the release zip:
$env:VPP_HOME = "C:\vpp"
$env:PATH = "$env:VPP_HOME;$env:PATH"
vpp run examples\hello.vpp
```

---

## Feature details

| Feature | Description |
|---------|-------------|
| **Syntax highlighting** | Keywords, types, strings, comments, and builtins for `.vpp` files |
| **Run & debug** | **F5** or the ▶ toolbar button runs the current file via `vpp` |
| **Type-check** | **v++: Check File** — errors without executing |
| **Language server** | Red squiggles, completion, go-to-definition (requires `vppls`) |
| **File icon** | Distinct yellow **v** icon in the explorer |
| **Project tests** | **v++: Run Tests** runs `vpp test` in the workspace |

---

## Useful commands

Open the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`):

| Command | Description |
|---------|-------------|
| **v++: Run File** | Run the active `.vpp` file (`F5`, `Ctrl+Shift+R`) |
| **v++: Check File** | Type-check without running |
| **v++: Run Tests** | Run test blocks in the project |

---

## Extension settings

| Setting | Default | Description |
|---------|---------|-------------|
| `vpp.compilerPath` | *(auto)* | Path to `vpp` / `vpp.exe`. Auto-detects `vpp.ps1` in the workspace. |
| `vpp.languageServerPath` | `vppls` | Path to the v++ language server binary |
| `vpp.enableLanguageServer` | `true` | Enable diagnostics, completion, and go-to-definition |

---

## Requirements

- **v++ compiler** (`vpp`) on your `PATH` or in the workspace (`vpp.ps1`, `target/release/vpp.exe`)
- **Language server** (`vppls`) optional but recommended — build with `cargo build --features lsp --bin vppls`
- **LLVM/clang** only required for `vpp build` (native `.exe` output), not for `vpp run` (interpreter)

---

## Set up your environment

1. Clone or download [v++](https://github.com/shauryaR790/V-)
2. Build: `cargo build --release --features codegen,lsp` (or use a [prebuilt release](https://github.com/shauryaR790/V-/releases))
3. Reload VS Code after installing this extension
4. Open any `.vpp` file — the status bar should show **v++**

---

## Which extension is mine?

Search for **v++ Language** published by **vpp-lang** (`vpp-lang.vplusplus`).

There are other unrelated extensions named "V++" on the Marketplace — always check the publisher is **vpp-lang**.

---

## Links

- [GitHub repository](https://github.com/shauryaR790/V-)
- [Install guide](https://github.com/shauryaR790/V-/blob/main/docs/INSTALL.md)
- [Language specification](https://github.com/shauryaR790/V-/blob/main/SPEC.md)
- [Report issues](https://github.com/shauryaR790/V-/issues)

---

## License

MIT — see [LICENSE](https://github.com/shauryaR790/V-/blob/main/LICENSE).
