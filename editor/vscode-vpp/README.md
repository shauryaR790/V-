# v++ Language

Write and run [v++](https://github.com/shauryaR790/V-) programs in VS Code.

![v++ file icon](icons/vpp-file.png)

## Install the compiler (one time)

The extension runs `vpp` for you — it does not include the compiler.

**Windows (easiest):** download the latest release from [GitHub Releases](https://github.com/shauryaR790/V-/releases), unzip, add the folder to your PATH.

**From source:**

```powershell
git clone https://github.com/shauryaR790/V-.git vpp
cd vpp
.\setup.ps1
```

## Use

1. Open a folder with `.vpp` files
2. Open a file — language mode should show **v++**
3. Press **F5** to run

| Action | Shortcut |
|--------|----------|
| Run file | **F5** or **Ctrl+Shift+R** |
| Type-check | Right-click → **v++: Check File** |
| Run tests | Command palette → **v++: Run Tests** |

## Features

- Syntax highlighting
- Yellow **v** file icon
- Run button in the editor toolbar
- Language server: errors, completion, go-to-definition (needs `vppls` on PATH or in workspace)

## Settings

| Setting | Default | Description |
|---------|---------|-------------|
| `vpp.compilerPath` | *(auto)* | Path to `vpp.exe` |
| `vpp.languageServerPath` | `vppls` | Language server binary |
| `vpp.enableLanguageServer` | `true` | Diagnostics and completion |

## Links

- [GitHub](https://github.com/shauryaR790/V-)
- [Install guide](https://github.com/shauryaR790/V-/blob/main/docs/INSTALL.md)

## License

MIT
