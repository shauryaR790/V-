# Change Log

## 0.6.2

- Version bump for Marketplace upload (0.6.0 VSIX filename already used)

## 0.6.1

- Fix format-on-save conflicting with VS Code save (format via temp file, no disk overwrite during save)

## 0.6.0

**Write simply. Compile natively.** — v0.6 aligns the extension with the language's interactive-development milestone.

### New
- **Format on save** — `vpp fmt` runs automatically (toggle: `vpp.formatOnSave`)
- **Format document** command and **Shift+Alt+F** keybinding
- **Status bar** — active compiler path + LSP indicator (click to open settings)
- **Code snippets** — `main`, `fn`, `struct`, `match`, `test`, `println`
- **Welcome prompt** on first install with docs link
- **v++: Show Output**, **Open Settings**, **Open Documentation** commands

### Improved
- **Lazy LSP** — language server starts only when a `.vpp` file is opened (faster startup)
- **Runner cache** — compiler path resolved once per workspace change
- **Shared output channel** — run, check, fmt, and LSP logs in one panel
- LSP skipped when `vppls` binary is not found (no silent failures)
- Honest Marketplace categories (removed Debuggers until v0.8)
- README repositioned: Python readability + native compilation + real toolchain

## 0.6.0

**Interactive development** — compiler + extension aligned at v0.6.

### Compiler
- **`vpp repl`** — persistent REPL (same interpreter as `vpp run`; definitions carry across lines)

### Extension
- Format-on-save (`vpp fmt`), **Shift+Alt+F**, temp-file fix (no save conflicts)
- **v++: Start REPL** terminal command
- Code snippets, status bar, lazy LSP, welcome prompt
- Transparent official V++ wordmark icons

## 0.5.10

- Force-refresh extension header icon (clears stale Marketplace cache from 0.5.0)

## 0.5.9

- Fix icons: regenerate from the true transparent removebg asset (previous copy had an opaque black matte)

## 0.5.8

- Extension icons use the transparent official V++ wordmark (no black background)

## 0.5.7

- Official V++ wordmark only (no generated circle badge); larger mark on black background

## 0.5.6

- Activate when workspace contains `.vpp` files

## 0.5.5

- Official wordmark icons from transparent logo asset

## 0.5.4

- Placeholder circle badge (superseded)

## 0.5.0

- Initial Marketplace release: syntax, run, check, test, LSP
