# Your first v++ project

## Scaffold

```powershell
vpp new myapp --path myapp
cd myapp
```

Creates:

```
myapp/
  vpp.toml
  src/main.vpp
  tests/smoke.vpp
```

## Run

```powershell
vpp run
```

Runs the entry in `vpp.toml` (default `src/main.vpp`).

## Test

```powershell
vpp test
```

Runs `test` blocks in `tests/` and inline tests.

## Add a dependency

```powershell
vpp add hello-lib --version 0.1.0
vpp update
```

See [Package manager](../guides/package-manager.md).
