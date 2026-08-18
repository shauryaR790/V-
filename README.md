# v++

**v++** is a compiled programming language that aims for Python's approachability with Rust/C++-level control and native performance.

> Write it simply. Compile it natively. Grow into control when you need it.

## Quick start (Windows)

1. **Compiler** — [Download latest release](https://github.com/shauryaR790/V-/releases) → unzip → double-click **`GO.bat`**
2. **Editor** — VS Code → Extensions → search **v++** → Install [v++ Language](https://marketplace.visualstudio.com/items?itemName=vpp-lang.vplusplus)
3. Open a `.vpp` file → press **F5**

No Rust required for prebuilt releases. See [docs/INSTALL.md](docs/INSTALL.md).

## Features (v0.4)

- Practical **local type inference** (`let x = 10` → `int`)
- Functions with explicit signatures; **`fn main() -> int`** entry point
- Control flow: `if`/`else`, `while`, `for` over ranges and arrays, `match`, `break`/`continue`
- **Structs**, **user enums**, **Option/Result**, **match**
- **Modules**: `import std.io`, `pub` exports, namespaced calls
- **Package manager**: `vpp.toml`, lockfile, path/git/registry deps
- **Stdlib**: `std/io`, `std/math`, `std/string`, `std/collections`, `std/fs`, `std/json`, `std/process`
- **Native codegen** for fs/json/process + full v0.2 feature parity
- **`test` blocks** with `vpp test` runner
- **LSP** diagnostics, completion, go-to-definition
- CLI: `run`, `build`, `check`, `compile`, `fmt`, `test`, `init`, `doctor`, `lsp`

## Prerequisites

### Windows

1. **Rust** — [rustup.rs](https://rustup.rs/)
2. **LLVM 18+** — `winget install LLVM.LLVM` or [LLVM releases](https://github.com/llvm/llvm-project/releases)
3. **clang** — included with LLVM
4. For codegen builds, set (LLVM 22 example):

```powershell
$env:LLVM_SYS_221_PREFIX = "C:\Program Files\LLVM"
```

### Build

**Default (type-check, parse, fmt — no LLVM required):**

```powershell
cd v++
cargo build --release
```

**With native codegen (requires LLVM 22 + MSVC toolchain on Windows):**

```powershell
$env:LLVM_SYS_221_PREFIX = "C:\Program Files\LLVM"
cargo build --release --features codegen
```

On Windows, use the **MSVC** Rust toolchain (`rustup default stable-x86_64-pc-windows-msvc`) with Visual Studio Build Tools. The GNU/MinGW toolchain cannot link against the official LLVM Windows libraries.

The compiler binary is `target/release/vpp.exe`.

## Syntax highlighting (Cursor / VS Code)

`.vpp` files use a bundled editor extension for keyword, string, and comment colors.

The extension lives in [`editor/vscode-vpp/`](editor/vscode-vpp/). It should already be installed under your Cursor extensions folder. If highlighting still looks like plain text:

1. **Reload the window** — `Ctrl+Shift+P` → **Developer: Reload Window**
2. **Confirm language mode** — bottom-right of the editor should say **v++** (not Plain Text)
3. **Manual install** (if needed):

```powershell
# Copy into Cursor extensions
Copy-Item -Recurse editor\vscode-vpp "$env:USERPROFILE\.cursor\extensions\vpp-lang.vpp-0.1.0"
```

Then reload Cursor again.

The extension includes a **v++ logo** (indigo badge with a `V` and `++`) shown in editor tabs. Optional: set `"workbench.iconTheme": "vpp-lang.vpp-icons"` in your user settings if you want the logo in the file explorer too (this only adds icons for `.vpp` files).

## Usage

**Recommended on Windows** — use the local wrapper (always up to date):

```powershell
.\vpp.ps1 run stress.vpp
.\vpp.ps1 build stress.vpp -o stress.exe
.\stress.ps1    # automatic interpreter vs native parity test
```

Or install globally once: `.\install.ps1` (then reopen terminal).

```powershell
# Create a new project
vpp init myapp
cd myapp
vpp run
vpp test

# Type-check
vpp check stress.vpp

# Format source
vpp fmt examples/hello.vpp
vpp doctor
```

See [docs/INSTALL.md](docs/INSTALL.md) for prebuilt binaries and LLVM setup.

### Run programs

`vpp run` uses the **interpreter** (no LLVM required). It runs top-level statements and calls `fn main()` when present.

```powershell
.\vpp.ps1 run stress.vpp
```

Expected output includes:

```
=== v++ hello ===
30
...
active
42
...
=== done ===
```

`vpp build` produces a native executable (requires LLVM 22 + `--features codegen`).

```powershell
$env:LLVM_SYS_221_PREFIX = "C:\Program Files\LLVM"
.\vpp.ps1 build stress.vpp -o stress.exe
.\stress.exe
```

See [SPEC.md](SPEC.md) for the language reference. Full manual: [docs/VPP_COMPLETE_MANUAL_v0.1.0.md](docs/VPP_COMPLETE_MANUAL_v0.1.0.md).

## Example

```vpp
let users = ["Alex", "Sam", "John"]

for user in users {
    print(user)
}

fn add(a: int, b: int) -> int {
    return a + b
}

print(add(2, 3))
```

## Architecture

```
.vpp source → lexer → parser → AST → type checker → LLVM IR → native executable
```

### Type inference (MVP)

Only local `let` bindings are inferred. Function signatures are explicit. Generics use monomorphization with explicit type arguments at call sites (`id[int](42)`).

### Memory model (v0.4)

Heap **strings** and **arrays** use ARC reference counting in native code (`runtime/vpp_runtime.c`). See [MEMORY_MODEL.md](MEMORY_MODEL.md).

## Roadmap

- **v0.5** — hosted registry, REPL, expanded stdlib (net, time, maps)
- **v1.0** — stable spec, full parity guarantee, curriculum + playground

v0.4 shipped: **generics**, **traits/impls**, **`mut`**, and **compile-time match exhaustiveness**.

## License

MIT
