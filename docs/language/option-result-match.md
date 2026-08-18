# Option, Result, and match

## Option

```vpp
let some = Some(42)
let none: Option[int] = None
```

## Result

```vpp
fn divide(a: int, b: int) -> Result[int, string] {
    if b == 0 {
        return Err("division by zero")
    }
    return Ok(a / b)
}
```

## Exhaustive match

The compiler rejects non-exhaustive matches on enums, `Option`, and `Result`:

```vpp
match opt {
    Some(x) => print(x),
    None => print("empty"),
}
```

Missing `None` → **E0107**.

Use `_` for a catch-all when appropriate.
