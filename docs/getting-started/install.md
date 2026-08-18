# Install v++

## Windows (recommended)

1. Download **`vpp-0.5.0-setup.exe`** from [GitHub Releases](https://github.com/shauryaR790/V-/releases/latest).
2. Run the installer. If SmartScreen warns, choose **More info → Run anyway** (unsigned until SignPath approval).
3. Open a **new** terminal:

```powershell
vpp run examples\hello.vpp
vpp doctor
```

If `vpp` is not found, add the install folder to PATH:

```powershell
$dir = "$env:LOCALAPPDATA\Programs\vpp"
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";$dir;$dir\llvm\bin", "User")
```

Restart the terminal.

## VS Code extension

1. Extensions → search **v++ Language**
2. Publisher must be **vpp-lang**
3. [Marketplace link](https://marketplace.visualstudio.com/items?itemName=vpp-lang.vplusplus)

See [VS Code setup](vscode-setup.md).

## Portable zip (advanced)

Download `vpp-v0.5.0-windows-x64.zip`, extract, run `GO.bat` or add the folder to PATH manually.

## Build from source

See [Building from source](../contributing/building-from-source.md).

## Requirements

| Task | Needs |
|------|--------|
| `vpp run`, `check`, `test` | Installer only |
| `vpp build` (native `.exe`) | Bundled `clang` in installer, or LLVM 22 |
| Hack on compiler | Rust + LLVM 22 |
