# Testing

## Inline test blocks

```vpp
test "addition works" {
    assert_eq(add(2, 3), 5)
}
```

## Project tests

Place files under `tests/` or use inline blocks, then:

```powershell
vpp test
```

## VS Code

Command Palette → **v++: Run Tests**

## CI

```powershell
cargo test --all-targets
cargo test --features codegen parity
```

See [Running tests](../contributing/running-tests.md).
