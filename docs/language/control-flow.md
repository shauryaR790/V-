# Control flow

## if / else

```vpp
if x > 0 {
    print("positive")
} else {
    print("non-positive")
}
```

## while

```vpp
let mut i = 0
while i < 3 {
    print(i)
    i = i + 1
}
```

## for (range)

```vpp
for i in 0..3 {
    print(i)
}
```

## for (array)

```vpp
for name in ["Alex", "Sam"] {
    print(name)
}
```

## match

```vpp
match status {
    Active => print("on"),
    Idle => print("off"),
}
```

Enums, `Option`, and `Result` matches must be **exhaustive** or use `_` (compile error E0107).

See [option-result-match.md](option-result-match.md).

## break / continue

Supported inside `while` and `for` loops.
