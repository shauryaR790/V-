# mut and immutability (v0.4+)

Bindings are **immutable by default**.

```vpp
let x = 1
// x = 2   // error: cannot reassign immutable binding
```

Use `mut` when you need to reassign:

```vpp
let mut count = 0
count = count + 1
```

Loop counters and accumulators typically need `mut`.
