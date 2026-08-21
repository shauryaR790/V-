# FAQ

## What is v++?

A statically typed language with readable syntax, an interpreter for development, and native compilation to `.exe` via LLVM. Open source (MIT), currently at **v0.5**.

## How is v++ different from Python?

| | Python | v++ |
|---|--------|-----|
| Typing | Dynamic (optional hints) | Static + inference |
| Run | Interpreter default | Interpreter + native `.exe` |
| Errors | Mostly at runtime | Types and exhaustiveness at compile time |
| Ecosystem | Huge (PyPI) | Small stdlib + growing registry |
| Best for | Scripts, ML, web backends | Learning, CLI tools, typed native programs |

Python is better when you need libraries and speed of prototyping across domains. v++ is better when you want types and a native binary without switching languages later.

## How is v++ different from Rust?

Rust enforces ownership and borrowing at compile time for maximum safety and performance. v++ uses ARC for heap values in native mode and skips the borrow checker — easier to learn, less control over allocation patterns.

Choose Rust for systems programming at scale. Choose v++ to learn typed languages or ship small native tools with Python-like syntax.

## How is v++ different from Go or TypeScript?

Go is statically typed with a large ecosystem and goroutines. TypeScript adds types to JavaScript but still runs on Node/V8. v++ is a standalone language with its own interpreter and LLVM backend — not a host-VM language.

## How do I learn v++?

1. Read [Introduction](../getting-started/introduction.md).
2. [Install](../getting-started/install.md) and write the [first program](../getting-started/hello-world.md).
3. Complete the [20 projects](../../projects/README.md).
4. Reference [language docs](../language/README.md) and [guides](../guides/README.md).

If you know Python or JavaScript, expect a few days for syntax; a few weeks for types, structs, and native builds.

## What are the main difficulties?

- **Strict types** — function signatures and match exhaustiveness are enforced.
- **Native builds** — require LLVM 22 + clang; Windows is the primary supported platform.
- **Young ecosystem** — fewer third-party packages than Python or Rust.
- **Pre-1.0** — syntax and stdlib may change; pin releases for serious work.

## Why would I choose v++?

- Readable syntax with real static typing.
- Same source for interpret (`vpp run`) and native ship (`vpp build`).
- Integrated toolchain: fmt, test, packages, LSP, VS Code extension.
- Full compiler source available to read and contribute to.

## Is v++ ready for production?

**v0.5** — suitable for learning, personal tools, and experimentation. Not yet a replacement for Python or Rust in large production systems. See [roadmap](roadmap.md).

## Where do I download?

[GitHub Releases](https://github.com/shauryaR790/V-/releases) — `vpp-1.0.0-setup.exe` for Windows (Linux/macOS tarballs on the same page).

## Which VS Code extension is official?

**v++ Language** — publisher `vpp-lang` (`vpp-lang.vplusplus`).

## Does v++ collect data?

No. See [PRIVACY.md](../PRIVACY.md).

## How do I report bugs?

[GitHub Issues](https://github.com/shauryaR790/V-/issues)

## Can I contribute?

Yes — [CONTRIBUTING.md](../../CONTRIBUTING.md).
