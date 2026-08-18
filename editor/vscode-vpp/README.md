# v++ Language extension for Visual Studio Code

Official language support for **[v++](https://github.com/shauryaR790/V-)** — a compiled language with Python-style simplicity and native performance. Edit `.vpp` files with syntax highlighting, integrated run, static diagnostics, language-server IntelliSense, and project test discovery.

> **Install the compiler separately** — this extension does not bundle `vpp`. See [Quick start](#quick-start).

---

## Quick start

**Step 1.** Install the v++ compiler from [GitHub Releases](https://github.com/shauryaR790/V-/releases) (`vpp-*-setup.exe` recommended).

**Step 2.** Install **v++ Language** from the Marketplace (publisher: **vpp-lang**).

**Step 3.** Open a folder with `.vpp` files, open a file, press **F5** to run.

```powershell
vpp run examples\hello.vpp
```

Full docs: [github.com/shauryaR790/V-/tree/main/docs](https://github.com/shauryaR790/V-/tree/main/docs)

---

## Feature details

| Feature | Description |
|---------|-------------|
| **Syntax highlighting** | Keywords, types, strings, comments, `mut`, generics, traits |
| **Run** | **F5** / **Ctrl+Shift+R** runs the active file via `vpp run` |
| **Type-check** | **v++: Check File** — errors without executing |
| **Language server** | Diagnostics, completion, go-to-definition via `vppls` |
| **Project tests** | **v++: Run Tests** runs `vpp test` in the workspace |
| **File icons** | Yellow **v** icon for `.vpp` files |
| **Toolbar** | Run button in the editor title bar |

---

## Useful commands

Open the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`):

| Command | Description |
|---------|-------------|
| **v++: Run File** | Run the active `.vpp` file (`F5`, `Ctrl+Shift+R`) |
| **v++: Check File** | Type-check without running |
| **v++: Run Tests** | Run test blocks in the project |

Type `v++` in the Command Palette to see all commands.

---

## Extension settings

| Setting | Default | Description |
|---------|---------|-------------|
| `vpp.compilerPath` | *(auto)* | Path to `vpp` / `vpp.exe`. Auto-detects workspace scripts. |
| `vpp.languageServerPath` | `vppls` | Path to the v++ language server |
| `vpp.enableLanguageServer` | `true` | Diagnostics, completion, go-to-definition |

---

## Requirements

| Component | Required for | Notes |
|-----------|--------------|-------|
| **vpp** | Run, check, build | [Releases](https://github.com/shauryaR790/V-/releases) or build from source |
| **vppls** | IntelliSense | Included in release bundle or `cargo build --features lsp --bin vppls` |
| **LLVM/clang** | `vpp build` only | Bundled in Windows installer under `llvm\` |

---

## Set up your environment

1. Install v++ ([install guide](https://github.com/shauryaR790/V-/blob/main/docs/getting-started/install.md))
2. Reload VS Code after installing this extension
3. Open any `.vpp` file — language mode should show **v++**
4. Optional: set `vpp.compilerPath` if `vpp` is not on PATH

---

## Which extension is official?

Search **v++ Language** published by **vpp-lang** (`vpp-lang.vplusplus`).

Other Marketplace extensions named "V++" are unrelated — verify publisher **vpp-lang**.

---

## Documentation & support

- [Documentation hub](https://github.com/shauryaR790/V-/tree/main/docs)
- [VS Code setup guide](https://github.com/shauryaR790/V-/blob/main/docs/getting-started/vscode-setup.md)
- [Language reference](https://github.com/shauryaR790/V-/tree/main/docs/language)
- [Troubleshooting](https://github.com/shauryaR790/V-/blob/main/docs/guides/troubleshooting.md)
- [Report issues](https://github.com/shauryaR790/V-/issues)

---

## Planned (road to v1.0)

- Integrated **debugger** (breakpoints, step-through)
- **Test Explorer** UI
- Format-on-save via `vpp fmt`
- Environment / toolchain picker in status bar

Track progress: [roadmap](https://github.com/shauryaR790/V-/blob/main/docs/project/roadmap.md)

---

## License

MIT — see [LICENSE](https://github.com/shauryaR790/V-/blob/main/LICENSE).
