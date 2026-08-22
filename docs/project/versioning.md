# Version numbers

v++ uses **two independent version lines**. This confuses people if we do not say it plainly.

| Product | Current | Where |
|---------|---------|--------|
| **Compiler** (`vpp`) | **v1.0.4** | [GitHub Releases](https://github.com/shauryaR790/VPP/releases) — installer, portable zip |
| **VS Code extension** | **1.2.0** | [Marketplace](https://marketplace.visualstudio.com/items?itemName=vpp-lang.vplusplus) — debug, LSP, Test Explorer |

## What to install

1. **Compiler v1.0.4** from GitHub Releases (`vpp-1.0.4-setup.exe` or zip).
2. **Extension 1.2.0** from VS Code Marketplace (publisher: `vpp-lang`).

Extension **1.2.0** is tested with compiler **v1.0.4**. Older compilers (≥ 1.0.1) work for most features; use the latest compiler for CMake bundles and fixes.

## Why two numbers?

- **Compiler** semver tracks the language, CLI, and native toolchain (SPEC, `vpp build`, releases).
- **Extension** semver tracks VS Code UI, DAP, Test Explorer, and Marketplace packaging.

They do not bump in lockstep. Changelog entries always state the **paired compiler release** for each extension version.

## Releasing (maintainers)

1. Bump `Cargo.toml` version → commit → `git tag vX.Y.Z` → `git push origin vX.Y.Z`
2. CI (`.github/workflows/release.yml`) builds installer, zip, and VSIX → creates GitHub Release
3. Upload VSIX to Marketplace separately (or use `publish-extension.yml` with VSCE PAT)

Local fallback: `.\scripts\publish-release.ps1 -Version X.Y.Z` → `manual-releases/vX.Y.Z/`
