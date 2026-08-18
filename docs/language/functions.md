# Functions

```vpp
fn greet(name: string) -> int {
    print(name)
    return 0
}
```

## Main

```vpp
fn main() -> int {
    print("starting")
    return 0
}
```

## Generic functions (v0.4+)

```vpp
fn id[T](x: T) -> T {
    return x
}

print(id[int](42))
```

Type arguments are required at call sites (monomorphization).

See [generics.md](generics.md).
