# v++ Language Specification (v0.3)

This document describes v++ as implemented in v0.3. If code and spec disagree, parity tests and native execution are authoritative.

## Types

| Type | Syntax | Notes |
|------|--------|-------|
| int | `int` | 64-bit signed |
| float | `float` | 64-bit IEEE |
| bool | `bool` | `true` / `false` |
| string | `string` | UTF-8 heap string (ARC native) |
| array | `array[T]` | Homogeneous array |
| struct | `Name` | User-defined product type |
| enum | `Name` | User-defined sum type |
| Option | `Option[T]` | `Some(x)` / `None` |
| Result | `Result[T, E]` | `Ok(x)` / `Err(e)` |

## Variables

```
let x = 10           // inferred int
let name: string = "hi"
x = x + 1            // reassignment (mut keyword planned for v0.4)
```

## Functions

```
fn add(a: int, b: int) -> int {
    return a + b
}

fn main() -> int {
    // entry point when present; interpreter and native both invoke main()
    return 0
}
```

## Control flow

`if` / `else`, `while`, `for i in start..end` (half-open), `for item in arr`, `break`, `continue`, `match`.

Match arms use blocks:

```
match status {
    Active => {
        print("active")
    }
    Inactive => {
        print("inactive")
    }
}
```

User enum variants in expressions use bare names when the expected type is known (e.g. struct field `status: Active`).

## Builtins

| Name | Signature | Behavior |
|------|-----------|----------|
| print | printable values | Print line per argument |
| len | array or string → int | Length |
| assert | bool → void | Fail if false |
| assert_eq | T, T → void | Fail if not equal |
| read_file / write_file / file_exists | fs | File I/O (also via `std.fs`) |
| json_parse / json_stringify | json | JSON helpers (also via `std.json`) |
| process_run | process | Run shell command (also via `std.process`) |

## Modules (v0.3)

```
import std.io
import std.fs
import "legacy/path.vpp"   // still supported
```

- Canonical paths: `import std.io` → `std/io.vpp`
- `pub fn` / `pub struct` / `pub enum` for exports
- Namespaced calls: `math.add(1, 2)`
- Circular imports and duplicate imports are errors

## Projects and packages

`vpp.toml` manifest, `vpp.lock`, dependencies via path, git, or registry semver (`hello-lib = "0.1.0"`).

## Standard library

`std/io`, `std/math`, `std/string`, `std/collections`, `std/fs`, `std/json`, `std/process`.

## Execution

- `vpp run file.vpp` — interpreter (calls `fn main()` when defined)
- `vpp build file.vpp -o out.exe` — native executable (requires LLVM + codegen)
- `.\stress.ps1` — compare interpreter vs native output for `stress.vpp`

Interpreter and native must produce identical stdout for supported programs.

## Known limitations (v0.3)

- No generics or traits (planned v0.4)
- Match exhaustiveness checked at runtime only
- Hosted package registry is local (`registry/index.toml`); no remote publish yet
