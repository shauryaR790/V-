# v++ — 30 second start

## Do this once

Open PowerShell in this folder and run:

```powershell
.\setup.ps1
```

That's it. It builds the compiler and runs `hello.vpp`.

## Run your code

```powershell
.\vpp.ps1 run examples\hello.vpp
.\vpp.ps1 run examples\option_result.vpp
.\vpp.ps1 check examples\structs.vpp
```

## If `cargo` not found

Paste this first, then run setup again:

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;" + $env:Path
```

## Native .exe (optional, later)

Need Visual Studio Build Tools installed, then:

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo build --release --features codegen
cargo run --features codegen -- build examples\hello.vpp
```

Until then, `.\vpp.ps1 run` works fine — it runs your code through the built-in interpreter.
