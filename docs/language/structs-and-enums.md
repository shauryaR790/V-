# Structs and enums

## Struct

```vpp
struct User {
    name: string,
    age: int,
}

let u = User { name: "Alex", age: 30 }
print(u.name)
```

## Enum

```vpp
enum Status {
    Active,
    Idle,
}

let s = Active
match s {
    Active => print("on"),
    Idle => print("off"),
}
```

## Enum with payloads

```vpp
enum Message {
    Text(string),
    Count(int),
}
```

All variants must be handled in `match` or use `_`.
