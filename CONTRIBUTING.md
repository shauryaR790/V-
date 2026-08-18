# Contributing to v++

Thanks for helping improve v++!

## Quick links

- [Documentation hub](docs/README.md)
- [Build from source](docs/contributing/building-from-source.md)
- [Run tests](docs/contributing/running-tests.md)
- [Report a bug](https://github.com/shauryaR790/V-/issues/new)

## How to contribute

1. **Issues first** — open an issue for bugs or feature ideas before large PRs
2. **Fork & branch** — work on a feature branch off `main`
3. **Test** — `cargo test --all-targets`; if touching codegen, `cargo test --features codegen`
4. **Format** — `cargo fmt`, `vpp fmt` on any `.vpp` examples you change
5. **PR** — describe what changed and why; link the issue

## Code areas

| Area | Path |
|------|------|
| Lexer / parser | `src/lexer`, `src/parser` |
| Type checker | `src/types` |
| Interpreter | `src/interp` |
| Codegen | `src/codegen` |
| LSP | `src/lsp`, `src/bin/vppls.rs` |
| VS Code extension | `editor/vscode-vpp/` |
| Docs | `docs/` |

## Commit messages

Use clear summaries: `Fix …`, `Add …`, `Docs: …` — same style as existing history.

## License

By contributing, you agree your work is licensed under the project MIT license.
