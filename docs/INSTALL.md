# Installing v++

## Requirements

- **Rust** (stable) — [rustup.rs](https://rustup.rs) (only if building from source)
- **LLVM 22 + Clang** — required for native compilation (`vpp build`)
- **Git** — required for git dependencies (`vpp add --git …`)

## Prebuilt binaries (recommended)

**Download:** [GitHub Releases](https://github.com/shauryaR790/V-/releases) (latest `v0.4.x`)

### Windows (easiest)

1. Download `vpp-v0.4.x-windows-x64.zip`
2. Unzip anywhere (e.g. `C:\vpp`)
3. Double-click **`GO.bat`**
4. Install VS Code extension: Extensions → search **v++**

No Rust required. LLVM only needed if you use `vpp build` (native compile).

```powershell
$env:VPP_HOME = "C:\vpp"
$env:PATH = "$env:VPP_HOME;$env:PATH"
vpp run examples\hello.vpp
vpp doctor
```

Each bundle includes `vpp`, `vppls`, `std/`, `registry/`, and a hello example.

## Build from source

```powershell
git clone https://github.com/shauryaR790/V-.git vpp
cd vpp
cargo build --release --features codegen,lsp
```

Set LLVM on Windows:

```powershell
$env:LLVM_SYS_221_PREFIX = "C:\Program Files\LLVM"
$env:PATH = "C:\Program Files\LLVM\bin;" + $env:PATH
```

Install binaries:

```powershell
cargo install --path . --features codegen,lsp
```

## Verify

```powershell
vpp doctor
vpp test
cargo test --features codegen -- --test-threads=1
```

## VS Code / Cursor (recommended IDE)

**Extension:** [VS Code Marketplace — v++ Language](https://marketplace.visualstudio.com/items?itemName=vpp-lang.vplusplus)  
Or run `.\setup.ps1` from this repo for a local install.

Full editor guide: [docs/VSCODE.md](VSCODE.md)

## New project

```powershell
vpp new myapp --path myapp
cd myapp
vpp run
vpp test
```

## Add a dependency

```powershell
# Registry (semver)
vpp add hello-lib --version 0.1.0

# Local path
vpp add helper --path ../helper

# Git
vpp add lib --git https://github.com/example/lib --tag v1.0.0
vpp update
```
