# Package manager

## vpp.toml

```toml
name = "myapp"
version = "0.1.0"
entry = "src/main.vpp"

[dependencies]
hello-lib = "0.1.0"
helper = { path = "../helper" }
remote = { git = "https://github.com/example/lib", tag = "v1.0.0" }
```

## Commands

```powershell
vpp new myapp --path myapp
vpp add hello-lib --version 0.1.0
vpp add helper --path ../helper
vpp add lib --git https://github.com/example/lib --tag v1.0.0
vpp update
vpp remove hello-lib
```

## Lockfile

`vpp.lock` pins resolved versions. Commit it for reproducible builds.

## Registry

Built-in index: [`registry/index.toml`](../../registry/index.toml).
