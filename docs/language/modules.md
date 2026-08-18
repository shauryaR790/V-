# Modules

## Standard library imports

```vpp
import std.io
import std.math

fn main() -> int {
    io.print("hello")
    print(math.abs(-5))
    return 0
}
```

## Public exports

In a module file, mark items with `pub`:

```vpp
pub fn helper() -> int {
    return 1
}
```

## Project layout

```
myapp/
  vpp.toml
  src/main.vpp
  src/util.vpp
```

See [Package manager](../guides/package-manager.md) for `vpp.toml` and dependencies.
