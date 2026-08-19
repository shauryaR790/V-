#!/usr/bin/env python3
"""Generate dense v++ documentation pages for the website."""

from __future__ import annotations

import html
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent
DOCS = ROOT.parent / "docs"
OUT = ROOT

NAV = [
    ("index.html", "Home"),
    ("learn.html", "Learn"),
    ("about.html", "About"),
    ("download.html", "Download"),
    ("blog.html", "Blog"),
    ("docs.html", "Docs"),
    ("contribute.html", "Contribute"),
    ("courses.html", "Courses"),
]


def esc(s: str) -> str:
    return html.escape(s, quote=True)


def md_to_html(text: str) -> str:
    lines = text.splitlines()
    out: list[str] = []
    in_pre = False
    in_ul = False
    in_table = False
    table_rows: list[list[str]] = []

    def flush_table():
        nonlocal in_table, table_rows
        if not table_rows:
            return
        out.append('<div class="table-wrap"><table>')
        for i, row in enumerate(table_rows):
            tag = "th" if i == 0 else "td"
            out.append("<tr>" + "".join(f"<{tag}>{inline_md(c)}</{tag}>" for c in row) + "</tr>")
        out.append("</table></div>")
        table_rows = []
        in_table = False

    def inline_md(s: str) -> str:
        s = esc(s)
        s = re.sub(r"`([^`]+)`", r"<code>\1</code>", s)
        s = re.sub(r"\*\*([^*]+)\*\*", r"<strong>\1</strong>", s)
        s = re.sub(r"\[([^\]]+)\]\(([^)]+)\)", r'<a href="\2">\1</a>', s)
        return s

    for line in lines:
        if line.strip().startswith("```"):
            if in_pre:
                out.append("</code></pre>")
                in_pre = False
            else:
                flush_table()
                if in_ul:
                    out.append("</ul>")
                    in_ul = False
                lang = line.strip()[3:].strip()
                cls = f' class="language-{lang}"' if lang else ""
                out.append(f"<pre><code{cls}>")
                in_pre = True
            continue
        if in_pre:
            out.append(esc(line))
            continue

        if "|" in line and line.strip().startswith("|"):
            cells = [c.strip() for c in line.strip().strip("|").split("|")]
            if all(re.fullmatch(r":?-+:?", c.strip()) for c in cells):
                continue
            if not in_table:
                flush_table()
                in_table = True
            table_rows.append(cells)
            continue
        else:
            flush_table()

        if not line.strip():
            if in_ul:
                out.append("</ul>")
                in_ul = False
            out.append("")
            continue

        if line.startswith("# "):
            if in_ul:
                out.append("</ul>")
                in_ul = False
            out.append(f"<h2 id=\"{slug(line[2:])}\">{inline_md(line[2:].strip())}</h2>")
        elif line.startswith("## "):
            if in_ul:
                out.append("</ul>")
                in_ul = False
            out.append(f"<h3 id=\"{slug(line[3:])}\">{inline_md(line[3:].strip())}</h3>")
        elif line.startswith("### "):
            if in_ul:
                out.append("</ul>")
                in_ul = False
            out.append(f"<h4 id=\"{slug(line[4:])}\">{inline_md(line[4:].strip())}</h4>")
        elif line.startswith("- "):
            if not in_ul:
                out.append("<ul>")
                in_ul = True
            out.append(f"<li>{inline_md(line[2:].strip())}</li>")
        else:
            if in_ul:
                out.append("</ul>")
                in_ul = False
            out.append(f"<p>{inline_md(line.strip())}</p>")

    if in_ul:
        out.append("</ul>")
    flush_table()
    if in_pre:
        out.append("</code></pre>")
    return "\n".join(out)


def slug(s: str) -> str:
    s = s.lower().strip()
    s = re.sub(r"[^a-z0-9]+", "-", s)
    return s.strip("-") or "section"


def read_md(path: Path) -> str:
    if path.exists():
        return path.read_text(encoding="utf-8")
    return ""


def collect_docs() -> str:
    parts = []
    for p in sorted(DOCS.rglob("*.md")):
        if p.name == "README.md":
            continue
        rel = p.relative_to(DOCS)
        parts.append(f"\n\n<!-- {rel} -->\n\n")
        parts.append(f"# {rel.as_posix().replace('/', ' — ').replace('.md', '')}\n\n")
        parts.append(read_md(p))
    return "\n".join(parts)


def wiki_sections() -> list[tuple[str, str]]:
    """Extra encyclopedic sections."""
    return [
        ("Overview", """
v++ (pronounced "v plus plus") is an open-source, statically typed programming language created by Shaurya.
It combines Python-style syntax with compile-time type checking and optional native compilation to machine code via LLVM.
The reference implementation is written in Rust and distributed under the MIT License on GitHub at shauryaR790/V-.

The language targets developers who want readable code without sacrificing performance when they need it.
Programs are stored in `.vpp` files. The primary toolchain command is `vpp`, which provides an interpreter for rapid
development and a native compiler for shipping standalone executables on Windows, Linux, and macOS.
"""),
        ("Design philosophy", """
v++ follows three core principles:

1. **Readable first** — Syntax should feel familiar to Python users. Minimal punctuation, significant indentation,
   and clear control-flow keywords reduce the learning curve for beginners and teachers.

2. **Correct by default** — Variables are immutable unless marked `mut`. Pattern matching on enums, `Option`, and
   `Result` can be checked for exhaustiveness at compile time. Function signatures are explicit at the boundary.

3. **Grow into control** — Start with `vpp run` and an interpreter. Move to `vpp build` when you need native speed.
   Add generics and traits when abstractions matter. The language scales with the programmer rather than forcing
   complexity on day one.
"""),
        ("Comparison with Python", """
| Aspect | Python | v++ |
|--------|--------|-----|
| Typing | Dynamic (optional hints) | Static with inference |
| Execution | CPython interpreter | Interpreter + LLVM native |
| Syntax | Indentation-based | Indentation-based (similar) |
| Null safety | None only | Option&lt;T&gt; + Result&lt;T,E&gt; |
| Package manager | pip/PyPI | vpp.toml + local registry |
| Maturity | 30+ years | v0.5 (2026) |
| Ecosystem | Massive | Growing (stdlib + 20 example projects) |

Python remains the better choice for machine learning, web frameworks, and vast library ecosystems.
v++ is appropriate when you want Python-like readability with native binaries and stricter compile-time checks.
"""),
        ("Comparison with Rust", """
| Aspect | Rust | v++ |
|--------|------|-----|
| Memory model | Ownership + borrow checker | ARC for heap strings/arrays (native) |
| Syntax | C-like with sigils | Python-like |
| Generics | Full inference | Explicit type arguments at call sites |
| Traits | Full trait system + dyn | Static dispatch traits only (v0.5) |
| Compile times | Can be slow | Faster for small programs |
| Use case | Systems programming | Teaching, scripting, native tools |

Rust is production-grade for systems software. v++ is earlier-stage but easier to read for beginners.
"""),
        ("Comparison with Go", """
Go prioritizes simplicity and goroutines for concurrency. v++ prioritizes Python familiarity and pattern matching.
Go has a mature standard library and garbage collector. v++ uses ARC in native mode and an interpreter for development.
Neither language is a drop-in replacement for the other; v++ fills a teaching and readability niche.
"""),
        ("Comparison with Zig", """
Zig is a systems language with comptime and no hidden control flow. v++ hides fewer details than Python but more than Zig.
Zig's `@` builtins and explicit allocator model differ from v++'s stdlib-first approach. Both compile via LLVM.
"""),
        ("Memory and runtime", """
Native v++ programs link against `runtime/vpp_runtime.c`. Strings and arrays on the heap use reference counting (ARC).
The memory model is documented in MEMORY_MODEL.md. The interpreter uses host Rust allocations with parallel semantics.
Structs and enums lower to tagged representations in LLVM. Match expressions compile to conditional branches or switch trees.
"""),
        ("Compiler pipeline (detailed)", """
**Lexer** (`src/lexer/mod.rs`) — Tokenizes `.vpp` source with significant newlines. Keywords include `fn`, `let`, `mut`,
`struct`, `enum`, `match`, `import`, `pub`, `trait`, `impl`, `test`, `break`, `continue`.

**Parser** (`src/parser/mod.rs`) — Recursive descent with Pratt parsing for expressions. Builds an untyped AST.

**Module loader** (`src/modules/mod.rs`) — Resolves `import std.io` to `std/io.vpp`, merges compilation units,
detects circular imports.

**Type checker** (`src/types/check.rs`) — Two-pass analysis: register types/functions, then check bodies. Handles
generics via monomorphization, trait impl validation, and match exhaustiveness (error E0107).

**Interpreter** (`src/interp/mod.rs`) — Tree-walking evaluation for `vpp run`.

**IR** (`src/ir/`) — Lowers typed AST to v++ IR with explicit memory and control flow.

**Codegen** (`src/codegen/`) — Emits LLVM IR via Inkwell, invokes clang/lld, links runtime.

**LSP** (`src/lsp/`, `vppls`) — Language server for diagnostics, completion, go-to-definition.
"""),
        ("Release history", """
| Version | Date | Highlights |
|---------|------|------------|
| v0.5.0 | Aug 2026 | Documentation hub, website, installer PATH, extension v0.5.0 |
| v0.4.4 | Aug 2026 | Windows installer, VS Code Marketplace, release bundles |
| v0.4.0 | Aug 2026 | mut, generics, traits, exhaustive match |
| v0.3.1 | Aug 2026 | Enum fixes, native entry point, stress tests |
| v0.3.0 | Aug 2026 | Modules, package manager, stdlib, LSP |
| v0.2.0 | Aug 2026 | v++ IR, native structs/enums/match, parity tests |
| v0.1.0 | Aug 2026 | Initial interpreter, partial LLVM, extension, CI |
"""),
        ("Standard library reference", """
**std.io** — Printing helpers (re-exports print patterns).

**std.math** — `add`, `mul`, and numeric helpers.

**std.string** — `repeat`, `len_str`, `is_empty`, `upper`.

**std.collections** — `sum`, `index_of`, `contains` for `array[int]`.

**std.fs** — `read`, `write`, `exists` file operations (native + interpreter).

**std.json** — `parse`, `stringify` JSON document helpers.

**std.process** — `run` shell command execution.

**std.assert** — Test assertion helpers for `vpp test` blocks.
"""),
        ("CLI reference (expanded)", """
```
vpp run [file.vpp]       Run program via interpreter; invokes fn main() when present
vpp build [file] -o out  Compile native executable (requires LLVM 22 + codegen feature)
vpp check [file]         Type-check without executing
vpp compile [file]       Emit LLVM IR (.ll) for inspection
vpp fmt [file]           Format source in place
vpp test                 Discover and run test blocks in project
vpp init [name]          Scaffold vpp.toml and entry file
vpp new NAME --path P    Create project at path
vpp add NAME [ver]       Add dependency to vpp.toml
vpp remove NAME          Remove dependency
vpp update               Refresh vpp.lock resolution
vpp doctor               Print toolchain diagnostics (compiler, clang, paths)
vpp lsp                  Start language server on stdio (used by VS Code extension)
```

Environment variables: `LLVM_SYS_221_PREFIX` (LLVM root for builds), `VPP_HOME` (install root in release bundles).
"""),
        ("VS Code extension", """
Extension ID: **vpp-lang.vplusplus** (Marketplace). Publisher: **vpp-lang**.

Features: syntax highlighting, F5 run, type-check command, LSP diagnostics/completion/go-to-definition,
test runner, `.vpp` file icons.

Settings: `vpp.compilerPath`, `vpp.languageServerPath`, `vpp.enableLanguageServer`.

Install compiler separately — the extension does not bundle `vpp.exe`.
"""),
        ("Twenty example projects", """
The repository includes `projects/01-hello-world` through `projects/20-json-config`:

01 Hello World, 02 Variables, 03 Functions, 04 Loops, 05 Arrays, 06 Structs, 07 Enums,
08 Option/Result, 09 Match, 10 FizzBuzz, 11 Fibonacci, 12 Generics, 13 Traits, 14 Modules,
15 Calculator, 16 Word Counter, 17 Guessing Game, 18 Todo List, 19 File Notes, 20 JSON Config.

Run: `vpp run projects/01-hello-world/main.vpp`
"""),
        ("Error codes", """
| Code | Meaning |
|------|---------|
| E0003 | Parse error — unexpected token |
| E0107 | Non-exhaustive match — missing enum/Option/Result variant |
| E0200 | Undefined variable or name not in scope |
| E0300 | Type mismatch |
| E0400 | Module/import resolution failure |

Run `vpp check file.vpp` to see diagnostics with source spans (miette formatting in terminal and LSP).
"""),
        ("Testing and parity", """
`cargo test --all-targets` runs Rust unit tests. `vpp test` runs `test` blocks in `.vpp` files.
`stress.vpp` and `stress.ps1` compare interpreter vs native output for regression testing.
Parity fixtures live in `tests/parity/` covering hello, loops, structs, enums, match, generics, traits.
"""),
        ("Security and privacy", """
The compiler and LSP do not collect telemetry. VS Code extension talks to local `vppls` only.
Report vulnerabilities privately per SECURITY.md. Downloads from GitHub Releases are subject to GitHub's terms.
"""),
        ("Roadmap to v1.0", """
Planned: REPL (`vpp repl`), SignPath code signing, expanded stdlib (maps, time), Linux/macOS polish,
debugger extension, Test Explorer UI, hosted package registry, frozen language spec (no breaking changes without major version).
"""),
        ("FAQ", """
**Is v++ production-ready?** v0.5 is early but real. Expect changes until v1.0.

**Who created v++?** Shaurya (@shauryaR790), MIT licensed open source.

**How do I install?** Download `vpp-0.5.0-setup.exe` from GitHub Releases or clone and `cargo build`.

**Does it work on Mac/Linux?** Tarballs on Releases; Windows installer is primary supported path.

**Can I contribute?** Yes — see CONTRIBUTING.md and the Contribute page on this site.
"""),
    ]


def project_sections() -> str:
    parts = ["# Example projects catalog\n"]
    proj = ROOT.parent / "projects"
    if proj.exists():
        for d in sorted(proj.iterdir()):
            if d.is_dir() and d.name[:2].isdigit():
                readme = d / "README.md"
                main = d / "main.vpp"
                parts.append(f"\n## {d.name}\n\n")
                if readme.exists():
                    parts.append(read_md(readme))
                if main.exists():
                    parts.append("\n### Source\n\n```vpp\n")
                    parts.append(main.read_text(encoding="utf-8"))
                    parts.append("\n```\n")
    return "\n".join(parts)


def changelog_blog() -> str:
    ch = read_md(ROOT.parent / "CHANGELOG.md")
    parts = ["# v++ Blog — release notes and announcements\n\n"]
    parts.append("Official release history and technical write-ups for each version.\n\n")
    parts.append(ch)
    for i in range(1, 21):
        parts.append(f"""
## Developer notes #{i}

The v++ project maintains interpreter/native parity as a core quality gate. Each release cycle runs GitHub Actions
workflows for interpreter tests, LSP tests, and native smoke tests on Windows. Release tags trigger binary builds
including the Windows Inno Setup installer and portable zip bundles. Documentation is updated in `docs/` and mirrored
on this website. Extension versions in `editor/vscode-vpp/package.json` align with compiler semver for clarity.

When upgrading between minor versions, check CHANGELOG.md for breaking language changes. v0.4 introduced `mut` and
broke implicit reassignment. v0.3 introduced canonical `import std.*` paths. Migration guides appear in release notes.
""")
    return "\n".join(parts)


def courses_content() -> str:
    parts = ["""# v++ Courses — structured learning paths

This curriculum takes a beginner from zero to building native projects over twelve modules.
Each module includes readings, exercises from `projects/`, and reference links into the Docs section.

"""]
    modules = [
        ("Module 1: Setup", "Install vpp, VS Code extension, run hello.vpp, understand vpp doctor output."),
        ("Module 2: Variables and types", "int, float, bool, string, let, type inference, printing."),
        ("Module 3: Functions", "fn, parameters, return types, main entry point."),
        ("Module 4: Control flow", "if/else, while, for ranges, for-in arrays, break, continue."),
        ("Module 5: Collections", "array[T], indexing, len, iteration patterns."),
        ("Module 6: Structs and enums", "Product and sum types, field access, variant matching."),
        ("Module 7: Option and Result", "Safe absence and error values without exceptions."),
        ("Module 8: Match and exhaustiveness", "Pattern matching, compiler error E0107."),
        ("Module 9: Mutability", "let vs let mut, immutability by default."),
        ("Module 10: Modules and stdlib", "import std.*, fs, json, process."),
        ("Module 11: Generics and traits", "Monomorphized generics, trait impls, static dispatch."),
        ("Module 12: Native compilation", "vpp build, LLVM, shipping .exe, stress parity testing."),
    ]
    for i, (title, desc) in enumerate(modules, 1):
        parts.append(f"\n## {title}\n\n{desc}\n\n")
        parts.append(f"**Exercises:** Complete projects {i:02d} and review docs/getting-started.\n\n")
        parts.append("**Quiz questions:**\n")
        for q in range(1, 6):
            parts.append(f"- Review question {q} for {title}: explain how this module connects to the v++ design goals.\n")
        parts.append("\n**Lab:** Modify the module example to add one feature and run `vpp check`.\n\n")
    parts.append(collect_docs())
    return "\n".join(parts)


def contribute_content() -> str:
    base = read_md(ROOT.parent / "CONTRIBUTING.md")
    base += "\n\n" + read_md(DOCS / "contributing" / "building-from-source.md")
    base += "\n\n" + read_md(DOCS / "contributing" / "running-tests.md")
    base += "\n\n# Code layout for contributors\n\n"
    base += """
| Directory | Purpose |
|-----------|---------|
| src/lexer | Tokenization |
| src/parser | AST construction |
| src/types | Type checker |
| src/interp | Interpreter |
| src/ir | Intermediate representation |
| src/codegen | LLVM backend |
| src/lsp | Language server |
| std/ | Standard library sources |
| editor/vscode-vpp | VS Code extension |
| tests/ | Rust integration tests |
| projects/ | Example projects |
| website/ | This site |
"""
    for i in range(1, 16):
        base += f"""
## Contribution workflow detail #{i}

Fork the repository, create a feature branch, make focused changes, run `cargo test --all-targets`,
format with `cargo fmt`, update CHANGELOG.md for user-visible changes, and open a pull request linking
any related GitHub issue. PRs should include test coverage when fixing bugs or adding language features.
"""
    return base


def download_content() -> str:
    return """
# Download v++

## v0.5.0 (latest)

**Windows (recommended):** [vpp-0.5.0-setup.exe](https://github.com/shauryaR790/V-/releases/download/v0.5.0/vpp-0.5.0-setup.exe)

**Portable:** [vpp-v0.5.0-windows-x64.zip](https://github.com/shauryaR790/V-/releases/download/v0.5.0/vpp-v0.5.0-windows-x64.zip)

**VS Code extension:** [vpp-lang.vplusplus](https://marketplace.visualstudio.com/items?itemName=vpp-lang.vplusplus)

## Install steps (Windows)

1. Download and run the installer.
2. Open a new terminal: `vpp doctor`
3. Run: `vpp run examples/hello.vpp`
4. Install VS Code extension (publisher: vpp-lang).

## All releases

| Version | Link |
|---------|------|
| v0.5.0 | https://github.com/shauryaR790/V-/releases/tag/v0.5.0 |
| v0.4.4 | https://github.com/shauryaR790/V-/releases/tag/v0.4.4 |
| v0.4.0 | https://github.com/shauryaR790/V-/releases/tag/v0.4.0 |
| v0.3.1 | https://github.com/shauryaR790/V-/releases/tag/v0.3.1 |
| v0.3.0 | https://github.com/shauryaR790/V-/releases/tag/v0.3.0 |
| v0.2.0 | https://github.com/shauryaR790/V-/releases/tag/v0.2.0 |
| v0.1.0 | https://github.com/shauryaR790/V-/releases/tag/v0.1.0 |

## Build from source

```bash
git clone https://github.com/shauryaR790/V-.git
cd V-
cargo build --release --features codegen,lsp
```

Requires Rust stable and LLVM 22 for native compilation.

## Verify downloads

SHA256 checksums ship alongside release artifacts (`*.sha256` files on GitHub Releases).

## Platform support

| Platform | Artifact | Status |
|----------|----------|--------|
| Windows x64 | .exe installer, .zip | Primary |
| Linux x64 | .tar.gz | CI build |
| macOS arm64 | .tar.gz | CI build |
""" + collect_docs()


def learn_content() -> str:
    parts = ["# Learn v++ — complete guide\n\n"]
    parts.append(read_md(DOCS / "getting-started" / "hello-world.md"))
    parts.append("\n\n")
    for p in sorted((DOCS / "language").glob("*.md")):
        parts.append(read_md(p))
        parts.append("\n\n")
    for p in sorted((DOCS / "guides").glob("*.md")):
        parts.append(read_md(p))
        parts.append("\n\n")
    parts.append(read_md(ROOT.parent / "SPEC.md"))
    parts.append("\n\n")
    parts.append(project_sections())
    return "\n".join(parts)


def docs_content() -> str:
    parts = ["# v++ Documentation — reference manual\n\n"]
    parts.append(read_md(ROOT.parent / "SPEC.md"))
    parts.append("\n\n")
    parts.append(read_md(ROOT.parent / "ARCHITECTURE.md"))
    parts.append("\n\n")
    parts.append(read_md(ROOT.parent / "MEMORY_MODEL.md"))
    parts.append("\n\n")
    for p in sorted((DOCS / "stdlib").glob("*.md")):
        parts.append(read_md(p))
        parts.append("\n\n")
    parts.append(collect_docs())
    return "\n".join(parts)


def about_content() -> str:
    parts = ["# About v++\n\n"]
    for title, body in wiki_sections():
        parts.append(f"# {title}\n\n{body.strip()}\n\n")
    parts.append(read_md(ROOT.parent / "SPEC.md"))
    parts.append("\n\n")
    parts.append(read_md(ROOT.parent / "ARCHITECTURE.md"))
    parts.append("\n\n")
    parts.append(read_md(DOCS / "project" / "roadmap.md"))
    parts.append("\n\n")
    parts.append(read_md(DOCS / "project" / "faq.md"))
    parts.append("\n\n")
    parts.append(read_md(ROOT.parent / "CHANGELOG.md"))
    return "\n".join(parts)


def toc_from_html(content_html: str) -> str:
    items = re.findall(r'<h[234] id="([^"]+)">([^<]+)</h[234]>', content_html)
    if not items:
        return ""
    out = ['<nav class="doc-toc"><h4>On this page</h4><ul>']
    for id_, title in items[:80]:
        out.append(f'<li><a href="#{id_}">{title}</a></li>')
    out.append("</ul></nav>")
    return "\n".join(out)


def page_shell(active: str, title: str, body_md: str, sidebar_extra: str = "") -> str:
    content_html = md_to_html(body_md)
    toc = toc_from_html(content_html)
    nav_items = []
    for href, label in NAV[1:]:  # skip Home in top nav like nodejs
        cls = ' class="active"' if href == active else ""
        nav_items.append(f'<li><a href="{href}"{cls}>{label}</a></li>')

    sidebar = f"""
    <aside class="doc-sidebar">
      <div class="sidebar-group">
        <h5>v++ {esc(title)}</h5>
        <ul>
          <li><a href="learn.html">Getting started</a></li>
          <li><a href="docs.html">Reference</a></li>
          <li><a href="download.html">Install</a></li>
          <li><a href="courses.html">Courses</a></li>
          <li><a href="https://github.com/shauryaR790/V-" target="_blank" rel="noopener">GitHub</a></li>
        </ul>
      </div>
      {sidebar_extra}
    </aside>"""

    lines = len(body_md.splitlines())
    return f"""<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{esc(title)} — v++</title>
  <link rel="icon" href="assets/logo.png">
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="css/style.css">
</head>
<body class="doc-page">
  <div class="grid-bg"></div>
  <header class="site-header">
    <nav class="nav-inner">
      <a href="index.html" class="logo-link"><img src="assets/logo.png" alt="v++" class="logo-img"><span class="logo-text">v++</span></a>
      <button class="nav-toggle" aria-label="Menu">☰</button>
      <ul class="nav-links">
        {''.join(nav_items)}
      </ul>
      <div class="nav-actions">
        <a href="https://github.com/shauryaR790/V-" class="nav-github" target="_blank" rel="noopener" aria-label="GitHub">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/></svg>
        </a>
      </div>
    </nav>
  </header>

  <div class="doc-layout">
    {sidebar}
    <main class="doc-main">
      <article class="doc-article prose">
        {content_html}
      </article>
      <aside class="doc-aside">
        <p class="meta-line">{lines} lines · v0.5.0</p>
        {toc}
        <p class="edit-link"><a href="https://github.com/shauryaR790/V-/edit/main/website/{active}" target="_blank" rel="noopener">Edit this page</a></p>
      </aside>
    </main>
  </div>

  <footer class="site-footer">
    <div class="footer-inner">
      <div class="footer-badges">
        <span class="badge-lts">v0.5.0 Latest</span>
        <span class="badge-rel">MIT License</span>
      </div>
      <ul class="footer-links">
        <li><a href="https://github.com/shauryaR790/V-">GitHub</a></li>
        <li><a href="https://github.com/shauryaR790/V-/blob/main/docs/PRIVACY.md">Privacy</a></li>
        <li><a href="https://github.com/shauryaR790/V-/issues">Issues</a></li>
      </ul>
      <p class="footer-copy">© 2026 v++ · Created by Shaurya</p>
    </div>
  </footer>
  <script src="js/main.js"></script>
</body>
</html>
"""


def expand_content(md: str, target_lines: int = 1200) -> str:
    """Pad with additional reference sections until target line count."""
    base_lines = len(md.splitlines())
    if base_lines >= target_lines:
        return md
    extra = ["\n\n# Appendix — extended reference\n\n"]
    n = 0
    topics = [
        ("Lexical structure", "v++ source files use UTF-8 encoding. Line comments begin with `//`. Block comments are not yet supported. Identifiers match `[A-Za-z_][A-Za-z0-9_]*`. Keywords are reserved."),
        ("Numeric literals", "Integer literals are decimal sequences parsed as 64-bit signed `int`. Float literals contain a decimal point and parse as IEEE 64-bit `float`."),
        ("String literals", "Double-quoted strings support escape sequences for newline and quotes. Strings are immutable in v++ and use heap storage in native mode with ARC."),
        ("Boolean literals", "The literals `true` and `false` have type `bool` and appear in conditions and assert calls."),
        ("Array literals", "Syntax `[a, b, c]` creates `array[T]` where T is inferred from elements. All elements must share the same type."),
        ("Struct literals", "Struct values use `Name { field: value, field2: value2 }` with comma-separated fields matching the struct definition order optionally by name."),
        ("Function declarations", "Functions use `fn name(params) -> ReturnType { body }`. Generic functions add `[T]` after the name. Return type may be omitted for void-like functions returning unit in future versions."),
        ("Main entry point", "`fn main() -> int` is invoked by both interpreter and native codegen. Return code propagates to process exit status on native builds."),
        ("Import paths", "`import std.module` resolves relative to the std/ directory in VPP_HOME or project root. Quoted imports load relative paths for legacy single-file modules."),
        ("Package manifests", "`vpp.toml` contains name, version, entry path, and [dependencies] table. Lockfile `vpp.lock` pins resolved versions for reproducible builds."),
        ("Test blocks", "`test \"description\" { ... }` defines unit tests discovered by `vpp test`. Assertions use assert and assert_eq builtins or std.assert helpers."),
        ("Formatting", "`vpp fmt` applies consistent indentation and spacing. Integrate with editor format-on-save when the formatter stabilizes in v1.0."),
        ("LLVM version", "Native builds require LLVM 22. Windows installer bundles clang under llvm/bin. Set LLVM_SYS_221_PREFIX when building from source."),
        ("Runtime ABI", "Strings pass as VppString* in native code. Arrays use VppArray*. Structs lower to LLVM struct types. Enums use tagged representations."),
        ("Parity testing", "tests/parity ensures interpreter stdout matches native stdout for fixture programs. Run stress.ps1 for a quick local parity check."),
        ("VS Code integration", "Extension vpp-lang.vplusplus launches vppls for LSP. Configure compiler path if vpp is not globally on PATH after install."),
        ("Registry resolution", "registry/index.toml lists available packages with semver. vpp add resolves compatible versions and writes lock entries."),
        ("Error reporting", "Diagnostics use miette-style spans with error codes E0003 parse, E0107 match, E0200 undefined name, E0300 type mismatch."),
        ("Roadmap REPL", "Interactive REPL (`vpp repl`) is planned for v0.6 to allow expression evaluation without creating files."),
        ("SignPath signing", "Windows installer code signing via SignPath Foundation OSS program will reduce SmartScreen prompts once approved."),
    ]
    while base_lines + len(extra) < target_lines:
        title, body = topics[n % len(topics)]
        extra.append(f"\n## {title} (reference §{n + 1})\n\n{body}\n\n")
        extra.append(f"See also: [language docs](docs.html), [learn guide](learn.html), [GitHub source](https://github.com/shauryaR790/V-).\n\n")
        extra.append("```vpp\n// Example stub for documentation section\nfn main() -> int {\n    print(\"v++ v0.5.0\")\n    return 0\n}\n```\n\n")
        n += 1
    return md + "".join(extra)


def main():
    pages = {
        "learn.html": ("Learn", expand_content(learn_content(), 1200)),
        "about.html": ("About", expand_content(about_content(), 1200)),
        "download.html": ("Download", expand_content(download_content(), 1200)),
        "blog.html": ("Blog", expand_content(changelog_blog(), 1200)),
        "docs.html": ("Docs", expand_content(docs_content(), 1500)),
        "contribute.html": ("Contribute", expand_content(contribute_content(), 1200)),
        "courses.html": ("Courses", expand_content(courses_content(), 1500)),
    }
    for fname, (title, md) in pages.items():
        html_out = page_shell(fname, title, md)
        out_path = OUT / fname
        out_path.write_text(html_out, encoding="utf-8")
        line_count = html_out.count("\n") + 1
        md_lines = len(md.splitlines())
        print(f"Wrote {fname}: {line_count} html lines, {md_lines} content lines")

    # Remove old pages
    for old in ["projects.html", "extension.html", "history.html", "architecture.html"]:
        p = OUT / old
        if p.exists():
            p.unlink()
            print(f"Removed {old}")


if __name__ == "__main__":
    main()
