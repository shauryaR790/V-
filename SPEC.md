# v++ Language Specification (v0.2 draft)

This document describes v++ as implemented in v0.2. If code and spec disagree, parity tests and native execution are authoritative.

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
x = x + 1            // reassignment (v0.2: let is mutable; mut keyword planned)
```

## Functions

```
fn add(a: int, b: int) -> int {
    return a + b
}
```

## Control flow

`if` / `else`, `while`, `for i in start..end` (half-open), `for item in arr`, `break`, `continue`, `match`.

## Builtins

| Name | Signature | Behavior |
|------|-----------|----------|
| print | variadic printable | Print line per argument |
| len | array or string → int | Length |
| assert | bool → void | Fail if false |
| assert_eq | T, T → void | Fail if not equal |

## Modules (v0.2)

```
import "relative/path.vpp"
import "std/io.vpp"
```

Flat merge into one namespace. Redesign planned for v0.3.

## Execution

- `vpp run file.vpp` — interpreter
- `vpp build file.vpp` — native executable (requires codegen feature)

Interpreter and native must produce identical stdout for supported programs.
