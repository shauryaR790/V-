# v++

**v++** is a compiled programming language — Python-style readability, static typing, native performance.

> Write it simply. Compile it natively. Grow into control when you need it.

**Author:** [Shaurya](https://github.com/shauryaR790) · **License:** MIT · **Version:** 0.5.0

---

## Get started in 2 minutes

1. **Download** [`vpp-0.5.0-setup.exe`](https://github.com/shauryaR790/V-/releases/latest) (Windows installer)
2. **Install** [v++ Language](https://marketplace.visualstudio.com/items?itemName=vpp-lang.vplusplus) in VS Code (publisher: **vpp-lang**)
3. **Run:**

```powershell
vpp run examples\hello.vpp
```

Full guide: **[docs/getting-started/hello-world.md](docs/getting-started/hello-world.md)**

---

## Documentation

| | |
|---|---|
| **[Documentation hub](docs/README.md)** | Index of all guides |
| **[Website](https://shauryaR790.github.io/V-/)** | Learn, projects, downloads, history |
| [Install](docs/getting-started/install.md) | Installer, PATH, extension |
| [Language reference](docs/language/README.md) | Types, control flow, generics, traits |
| [CLI reference](docs/guides/cli-reference.md) | Every `vpp` command |
| [Standard library](docs/stdlib/README.md) | std.io, fs, json, … |
| [FAQ](docs/project/faq.md) | Common questions |
| [SPEC.md](SPEC.md) | Formal language spec |

---

## Features

- Local **type inference** + explicit function signatures
- **Structs**, **enums**, **Option/Result**, exhaustive **match**
- **Generics**, **traits**, **`mut`** (v0.4)
- **Modules** + **package manager** (`vpp.toml`)
- **Interpreter** (`vpp run`) and **native build** (`vpp build`)
- **LSP** — diagnostics, completion, go-to-definition
- **VS Code extension** on the Marketplace

---

## Links

| Resource | URL |
|----------|-----|
| Releases | https://github.com/shauryaR790/V-/releases |
| VS Code extension | https://marketplace.visualstudio.com/items?itemName=vpp-lang.vplusplus |
| Issues | https://github.com/shauryaR790/V-/issues |
| Contributing | [CONTRIBUTING.md](CONTRIBUTING.md) |
| Security | [SECURITY.md](SECURITY.md) |
| Privacy | [docs/PRIVACY.md](docs/PRIVACY.md) |

---

## For developers

```powershell
git clone https://github.com/shauryaR790/V-.git
cd V-
cargo build --release --features codegen,lsp
cargo test --all-targets
```

See [Building from source](docs/contributing/building-from-source.md).

---

## Roadmap

**v0.5** — website, REPL, SignPath signing, expanded stdlib  
**v1.0** — stable spec, debugger, test explorer

Details: [docs/project/roadmap.md](docs/project/roadmap.md)
