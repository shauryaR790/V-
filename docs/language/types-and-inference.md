# Types and inference

## Built-in types

| Type | Example |
|------|---------|
| `int` | `42` |
| `float` | `3.14` |
| `bool` | `true` |
| `string` | `"hello"` |
| `array[T]` | `[1, 2, 3]` |
| `Option[T]` | `Some(1)`, `None` |
| `Result[T, E]` | `Ok(1)`, `Err("msg")` |

## Local inference

```vpp
let x = 10        // int
let name = "v++"  // string
let active = true // bool
```

## Explicit annotations

```vpp
let count: int = 0
let items: array[string] = ["a", "b"]
```

## Function signatures

Parameters and return types are **always explicit**:

```vpp
fn add(a: int, b: int) -> int {
    return a + b
}
```

## Entry point

```vpp
fn main() -> int {
    return 0
}
```

`vpp run` calls `main()` when defined.
