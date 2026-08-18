# Installing v++

## Requirements

- **Rust** (stable) — [rustup.rs](https://rustup.rs) (only if building from source)
- **LLVM 22 + Clang** — required for native compilation (`vpp build`)
- **Git** — required for git dependencies (`vpp add --git …`)

## Prebuilt binaries (recommended)

**Download:** [GitHub Releases](https://github.com/shauryaR790/V-/releases) (latest `v0.4.x`)

### Windows (like Python — use the installer)

1. Download **`vpp-x.y.z-setup.exe`** from Releases
2. Run the installer (Next → Install → Finish)
3. Open a **new** terminal and run:

```powershell
vpp run examples\hello.vpp
vpp doctor
```

4. Install VS Code extension: Extensions → **v++ Language** (publisher: **vpp-lang**)

No Rust. No zip. No `GO.bat`. The installer adds v++ to your user PATH.

**Security prompts:** Once [code signing](SIGNING.md) is enabled (SignPath, free for OSS), the installer shows a verified publisher — same as Python. Until then, Windows may show one SmartScreen prompt on first install; click **More info → Run anyway**.

**Portable zip** (`vpp-x.y.z-windows-x64.zip`) is still available for advanced users.

### Linux / macOS

Download the `.tar.gz` for your platform, extract, and run `./run.sh`.

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

## Code signing

See [docs/SIGNING.md](SIGNING.md) for enabling trusted Windows installs (SignPath OSS program).
