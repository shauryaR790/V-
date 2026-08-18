# v++ Documentation

Welcome to the v++ documentation hub. Start here, then drill into the topic you need.

## New to v++?

| Guide | Description |
|-------|-------------|
| [Install](getting-started/install.md) | Download installer, VS Code extension, PATH |
| [Hello, v++](getting-started/hello-world.md) | First program in 5 minutes |
| [VS Code setup](getting-started/vscode-setup.md) | F5 run, LSP, settings |
| [Your first project](getting-started/first-project.md) | `vpp new`, folders, tests |

## Language

| Guide | Description |
|-------|-------------|
| [Language overview](language/README.md) | Syntax philosophy and index |
| [Types & inference](language/types-and-inference.md) | `int`, `string`, `let`, function signatures |
| [Functions](language/functions.md) | `fn`, `return`, `main` |
| [Control flow](language/control-flow.md) | `if`, `while`, `for`, `match` |
| [Structs & enums](language/structs-and-enums.md) | Custom types |
| [Option, Result, match](language/option-result-match.md) | Safe patterns + exhaustiveness |
| [Generics](language/generics.md) | `fn id[T](x: T)` |
| [Traits & impls](language/traits.md) | Interfaces with static dispatch |
| [`mut` & immutability](language/mut-and-immutability.md) | Reassignment rules |

## Guides

| Guide | Description |
|-------|-------------|
| [CLI reference](guides/cli-reference.md) | Every `vpp` command |
| [Modules](language/modules.md) | `import`, `pub`, `std.*` |
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

## Tools & distribution

| Doc | Description |
|-----|-------------|
| [Install (detailed)](INSTALL.md) | Prebuilt vs source |
| [VS Code (detailed)](VSCODE.md) | Extension + Marketplace |
| [Releases](RELEASE.md) | GitHub Releases workflow |
| [Code signing](SIGNING.md) | SignPath / trusted installs |
| [Privacy](PRIVACY.md) | Data collection policy |
| [Marketplace publish](MARKETPLACE.md) | For extension maintainers |

## Project

| Doc | Description |
|-----|-------------|
| [Roadmap](project/roadmap.md) | v0.5 → v1.0 plan |
| [FAQ](project/faq.md) | Common questions |
| [Full manual (v0.1)](VPP_COMPLETE_MANUAL_v0.1.0.md) | Legacy comprehensive manual |
| [SPEC](../SPEC.md) | Language specification |
| [CHANGELOG](../CHANGELOG.md) | Version history |

## Contributing

| Doc | Description |
|-----|-------------|
| [Contributing](../CONTRIBUTING.md) | How to help |
| [Build from source](contributing/building-from-source.md) | `cargo build`, features |
| [Run tests](contributing/running-tests.md) | CI parity, local tests |
| [Release process](contributing/release-process.md) | Tags, installer, Marketplace |
