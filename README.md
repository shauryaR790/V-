# v++

**v++** is a compiled programming language that aims for Python's approachability with Rust/C++-level control and native performance.

> Write it simply. Compile it natively. Grow into control when you need it.

## Features (MVP)

- Practical **local type inference** (`let x = 10` → `int`)
- Functions with explicit signatures
- Control flow: `if`/`else`, `while`, `for` over ranges and arrays
- Strings, arrays, `print`, `len`, `assert`, `assert_eq`
- **Structs**, **Option/Result**, **match**, **modules/imports**
- **`test` blocks** with `vpp test` runner
- **`break` / `continue`**, project manifest (`vpp.toml`)
- **Standard library** (`std/math`, `std/io`, `std/string`)
- **Numbered compiler diagnostics** with source spans and help text
- **LLVM native codegen** via `inkwell` (optional feature)
- CLI: `run`, `build`, `check`, `compile`, `fmt`, `test`, `init`, `lsp`

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

```powershell
# Create a new project
vpp init myapp
cd myapp
vpp run
vpp test

# Type-check without codegen
vpp check examples/hello.vpp

# Format source
vpp fmt examples/hello.vpp
```

### Run programs

`vpp run` uses a **built-in interpreter** — no LLVM required:

```powershell
vpp run examples\hello.vpp
vpp run examples\lesson01_basics.vpp
```

Expected output for `hello.vpp`:

```
Hello, v++!
10
Shaurya
true
```

`vpp build` (native executable via LLVM) requires `--features codegen` and MSVC + LLVM on Windows.

```powershell
# Emit LLVM IR
vpp compile examples/hello.vpp -o hello.ll
```

See [LANGUAGE.md](LANGUAGE.md) for the full language reference and teaching curriculum.

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

Only local `let` bindings are inferred. Function signatures are explicit. No generics yet.

### Memory model (bootstrap)

The bootstrap runtime uses `malloc` without `free` — **temporary only**. Heap allocation is abstracted behind a runtime shim so a hybrid memory model (stack values + ARC heap + optional unsafe) can replace it later.

## Project layout

```
src/
  lexer/       Tokenizer
  parser/      Recursive-descent parser
  ast/         Untyped AST
  types/       Local inference + type checker
  codegen/     LLVM IR generation (inkwell)
  fmt/         Basic formatter
  driver.rs    compile/check pipelines
runtime/
  vpp_runtime.c  Bootstrap C runtime
examples/
  hello.vpp, fib.vpp, arrays.vpp
```

## Roadmap

- Generics and traits
- Hybrid memory model (ARC + unsafe opt-in)
- Package registry and `vpp publish`
- Full native codegen for structs/match on all platforms
- Enhanced LSP (go-to-def, completions)

## License

MIT
