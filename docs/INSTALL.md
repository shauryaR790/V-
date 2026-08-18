# Installing v++

## Requirements

- **Rust** (stable) — [rustup.rs](https://rustup.rs) (only if building from source)
- **LLVM 22 + Clang** — required for native compilation (`vpp build`)
- **Git** — required for git dependencies (`vpp add --git …`)

## Prebuilt binaries (recommended)

Download the archive for your platform from [GitHub Releases](https://github.com/shauryaR790/V-/releases) (tag `v0.3.0` or later). Each bundle includes:

- `vpp` / `vpp.exe` — compiler and interpreter
- `vppls` / `vppls.exe` — language server
- `std/` — standard library
- `registry/` — package index for semver deps

Extract, add the directory to your `PATH`, and set `VPP_HOME` to that directory (so `import std.*` resolves).

```powershell
$env:VPP_HOME = "C:\path\to\vpp-0.3.0"
$env:PATH = "$env:VPP_HOME;$env:PATH"
vpp doctor
```

On Windows you still need **LLVM/Clang** on `PATH` for `vpp build` (native compile).

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

## VS Code / Cursor extension

```powershell
cd editor/vscode-vpp
npm install
cd ../..
.\setup.ps1
```

Enable the v++ extension. The language server (`vppls`) starts automatically when `vpp.enableLanguageServer` is true.

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
