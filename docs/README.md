# v++ Documentation

Welcome to the v++ documentation. Start with [Install](getting-started/install.md) or [Hello, v++](getting-started/hello-world.md).

## Getting started

| Guide | Description |
|-------|-------------|
| [Install](getting-started/install.md) | Windows installer, VS Code extension, PATH |
| [Hello, v++](getting-started/hello-world.md) | First program in 5 minutes |
| [VS Code setup](getting-started/vscode-setup.md) | Run, debug, LSP settings |
| [Your first project](getting-started/first-project.md) | `vpp new`, layout, tests |

## Language reference

| Guide | Description |
|-------|-------------|
| [Overview](language/README.md) | Syntax and index |
| [Types & inference](language/types-and-inference.md) | `int`, `string`, `let`, signatures |
| [Functions](language/functions.md) | `fn`, `return`, `main` |
| [Control flow](language/control-flow.md) | `if`, `while`, `for`, `match` |
| [Structs & enums](language/structs-and-enums.md) | Custom types |
| [Option, Result, match](language/option-result-match.md) | Safe patterns + exhaustiveness |
| [Generics](language/generics.md) | `fn id[T](x: T)` |
| [Traits & impls](language/traits.md) | Interfaces with static dispatch |
| [`mut` & immutability](language/mut-and-immutability.md) | Reassignment rules |
| [Modules](language/modules.md) | `import`, `pub`, `std.*` |

## Guides

| Guide | Description |
|-------|-------------|
| [CLI reference](guides/cli-reference.md) | Every `vpp` command |
| [Package manager](guides/package-manager.md) | `vpp.toml`, deps, lockfile |
| [Testing](guides/testing.md) | `test` blocks, `vpp test` |
| [Native compilation](guides/native-compilation.md) | `vpp build`, LLVM, `.exe` |
| [Formatting](guides/formatting.md) | `vpp fmt` |
| [Language server](guides/language-server.md) | `vppls`, diagnostics, completion |
| [Troubleshooting](guides/troubleshooting.md) | Common errors and fixes |

## Standard library

| Module | Guide |
|--------|-------|
| Overview | [stdlib/README.md](stdlib/README.md) |
| `std.io` | [stdlib/io.md](stdlib/io.md) |
| `std.math` | [stdlib/math.md](stdlib/math.md) |
| `std.string` | [stdlib/string.md](stdlib/string.md) |
| `std.collections` | [stdlib/collections.md](stdlib/collections.md) |
| `std.fs` | [stdlib/fs.md](stdlib/fs.md) |
| `std.json` | [stdlib/json.md](stdlib/json.md) |
| `std.process` | [stdlib/process.md](stdlib/process.md) |

## Project

| Doc | Description |
|-----|-------------|
| [Roadmap](project/roadmap.md) | Planned features |
| [FAQ](project/faq.md) | Common questions |
| [SPEC](../SPEC.md) | Language specification |
| [CHANGELOG](../CHANGELOG.md) | Version history |

## Contributing

| Doc | Description |
|-----|-------------|
| [Contributing](../CONTRIBUTING.md) | How to help |
| [Build from source](contributing/building-from-source.md) | `cargo build`, features |
| [Run tests](contributing/running-tests.md) | CI parity, local tests |

## Legal

| Doc | Description |
|-----|-------------|
| [Privacy](PRIVACY.md) | Data collection policy |
| [Security](../SECURITY.md) | Vulnerability reporting |
| [Code of Conduct](../CODE_OF_CONDUCT.md) | Community standards |
