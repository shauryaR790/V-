# v++ Language Reference

v++ is a small, teachable language with Python-like syntax and explicit types where they matter. Programs use `.vpp` files.

## Hello world

```vpp
print("Hello, v++!")
```

## Variables

```vpp
let x = 42          // inferred int
let name: string = "Alex"
let nums = [1, 2, 3]
```

Variables are immutable by default. Reassign with `name = "new value"`.

## Types

| Type | Example |
|------|---------|
| `int` | `42` |
| `float` | `3.14` |
| `bool` | `true`, `false` |
| `string` | `"hello"` |
| `array[T]` | `[1, 2, 3]` |
| `Option<T>` | `Some(5)`, `None` |
| `Result<T, E>` | `Ok(42)`, `Err("oops")` |

## Functions

```vpp
fn add(a: int, b: int) -> int {
    return a + b
}

fn main() -> int {
    print(add(2, 3))
    return 0
}
```

## Control flow

```vpp
if x > 0 {
    print("positive")
} else {
    print("zero or negative")
}

while n > 0 {
    n = n - 1
}

for i in 0..5 {
    print(i)
}

for item in [10, 20, 30] {
    print(item)
}

break
continue
```

## Match

```vpp
match maybe {
    Some(n) => { print(n) }
    None => { print(0) }
}
```

## Structs

```vpp
struct Person {
    name: string
    age: int
}

let p = Person { name: "Alex", age: 20 }
print(p.name)
```

## Modules

```vpp
import "std/math.vpp"
import "std/io.vpp"

print(add(2, 2))
```

Standard library modules live in `std/` and are resolved automatically.

## Projects

Create a project:

```powershell
vpp init myapp
cd myapp
vpp run
vpp test
```

Project layout:

```
myapp/
  vpp.toml       # name, version, entry
  src/main.vpp   # program entry
  tests/         # test files
```

`vpp.toml`:

```toml
name = "myapp"
version = "0.1.0"
entry = "src/main.vpp"
```

## Tests

```vpp
import "std/math.vpp"

test "addition works" {
    assert_eq(add(2, 2), 4)
}

test "truth" {
    assert(1 + 1 == 2)
}
```

Run tests:

```powershell
vpp test
```

## Builtins

| Name | Description |
|------|-------------|
| `print(value)` | Print to stdout |
| `len(x)` | Length of string or array |
| `assert(cond)` | Fail if `cond` is false |
| `assert_eq(a, b)` | Fail if `a != b` |

## Standard library

| Module | Functions |
|--------|-----------|
| `std/io.vpp` | `greet`, `println` |
| `std/math.vpp` | `add`, `abs`, `max`, `min`, `pow` |
| `std/string.vpp` | `upper`, `repeat` |

## CLI

| Command | Description |
|---------|-------------|
| `vpp run [file]` | Run a file or project entry |
| `vpp check file` | Type-check only |
| `vpp test` | Run all tests in project |
| `vpp init [name]` | Create new project |
| `vpp fmt file` | Format source |
| `vpp build [file]` | Native compile (requires codegen feature) |

## Learning path

Start with the examples:

1. `examples/lesson01_basics.vpp` — variables and print
2. `examples/lesson02_functions.vpp` — functions and imports
3. `examples/lesson03_loops.vpp` — loops, break, continue
4. `examples/structs.vpp` — custom types
5. `examples/option_result.vpp` — `Option` and `Result`
