# v++ extension for Visual Studio Code

A Visual Studio Code extension with rich support for the **v++** programming language, providing syntax highlighting, integrated run and debug, static diagnostics, language-server completion and navigation, and project test discovery for `.vpp` files.

v++ is a compiled language with Python-style readability and native performance. This extension is the official editor integration published by **vpp-lang**.

> **Note:** This extension does not bundle the compiler. Install `vpp` separately — see [Quick start](#quick-start).

---

## Quick start

**Step 1.** Install the v++ compiler from [GitHub Releases](https://github.com/shauryaR790/V-/releases) or build from source.

**Step 2.** Install **v++ Language** from the Marketplace (publisher: **vpp-lang**, id: `vpp-lang.vplusplus`).

**Step 3.** Open a folder containing `.vpp` files, open a file, and press **F5** to run.

```powershell
# Windows — after extracting a release zip:
$env:VPP_HOME = "C:\vpp"
$env:PATH = "$env:VPP_HOME;$env:PATH"
vpp run examples\hello.vpp
```

---

## Set up your environment

1. Clone or download the [v++ repository](https://github.com/shauryaR790/V-).
2. Build the toolchain: `cargo build --release --features codegen,lsp`
   - Or use a [prebuilt release](https://github.com/shauryaR790/V-/releases) (no Rust required).
3. Ensure `vpp` is on your `PATH`, or set `vpp.compilerPath` in VS Code settings.
4. Reload VS Code after installing this extension.
5. Open any `.vpp` file — the editor activates v++ language support automatically.

For language-server features (diagnostics, completion, go-to-definition), build `vppls`:

```powershell
cargo build --release --features lsp --bin vppls
```

---

## Useful commands

Open the Command Palette (`Ctrl+Shift+P` on Windows/Linux, `Cmd+Shift+P` on macOS):

| Command | Description |
|---------|-------------|
| **v++: Run File** | Run the active `.vpp` file (`F5`, `Ctrl+Shift+R`) |
| **v++: Check File** | Type-check the current file without executing |
| **v++: Run Tests** | Run inline `test` blocks via `vpp test` in the workspace |

To see all v++ commands, open the Command Palette and type `v++`.

---

## Feature details

| Feature | Description |
|---------|-------------|
| **Syntax highlighting** | Keywords, types, literals, comments, and builtins for `.vpp` |
| **Run & debug** | **F5** or the ▶ toolbar runs the current file through `vpp run` |
| **Diagnostics** | Red squiggles from the v++ language server (`vppls`) |
| **IntelliSense** | Completion and go-to-definition when `vppls` is available |
| **Type-check** | **v++: Check File** reports errors without running code |
| **Project tests** | **v++: Run Tests** executes test blocks in the workspace |
| **File icon** | Distinct yellow **v** icon in the Explorer |

---

## Extension settings

| Setting | Default | Description |
|---------|---------|-------------|
| `vpp.compilerPath` | *(auto)* | Path to `vpp` / `vpp.exe`. Auto-detects `vpp.ps1` in the workspace root. |
| `vpp.languageServerPath` | `vppls` | Path to the v++ language server binary |
| `vpp.enableLanguageServer` | `true` | Enable diagnostics, completion, and go-to-definition |

---

## Requirements

| Component | Required for | Notes |
|-----------|--------------|-------|
| **vpp** compiler | Run, check, build | On `PATH`, or set `vpp.compilerPath` |
| **vppls** language server | IntelliSense, diagnostics | Build with `--features lsp` |
| **LLVM / clang** | `vpp build` (native `.exe`) | Not required for `vpp run` (interpreter) |

---

## Which extension is mine?

Search for **v++ Language** published by **vpp-lang** (`vpp-lang.vplusplus`).

Other Marketplace extensions named "V++" are unrelated projects. Always verify the publisher is **vpp-lang**.

---

## Links

- [GitHub repository](https://github.com/shauryaR790/V-)
- [Install guide](https://github.com/shauryaR790/V-/blob/main/docs/INSTALL.md)
- [VS Code setup](https://github.com/shauryaR790/V-/blob/main/docs/VSCODE.md)
- [Language specification](https://github.com/shauryaR790/V-/blob/main/SPEC.md)
- [Report issues](https://github.com/shauryaR790/V-/issues)

---

## Questions, issues, and contributions

If you run into a problem, please [file an issue](https://github.com/shauryaR790/V-/issues). Feature requests and pull requests are welcome on the main repository.

---

## License

MIT — see [LICENSE](https://github.com/shauryaR790/V-/blob/main/LICENSE).
