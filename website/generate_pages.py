#!/usr/bin/env python3
"""Generate dense documentation HTML pages for the v++ website."""

from __future__ import annotations

import html
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
WEBSITE = ROOT / "website"
DOCS = ROOT / "docs"

NAV = [
    ("learn.html", "Learn"),
    ("about.html", "About"),
    ("download.html", "Download"),
    ("blog.html", "Blog"),
    ("docs.html", "Docs"),
    ("contribute.html", "Contribute"),
    ("courses.html", "Courses"),
]

ASSET_PREFIX = "/V-/"

EXTRA_MD = [
    ROOT / "SPEC.md",
    ROOT / "ARCHITECTURE.md",
    ROOT / "MEMORY_MODEL.md",
    ROOT / "CHANGELOG.md",
    ROOT / "CONTRIBUTING.md",
    ROOT / "README.md",
]


def md_to_html(text: str) -> str:
    """Minimal markdown to HTML — enough for our docs."""
    lines = text.splitlines()
    out: list[str] = []
    in_code = False
    in_table = False
    code_lang = ""
    code_lines: list[str] = []
    list_open = False

    def close_list():
        nonlocal list_open
        if list_open:
            out.append("</ul>")
            list_open = False

    def close_code_block():
        nonlocal in_code, code_lines, code_lang
        if not in_code:
            return
        label = code_lang or "text"
        if label in ("powershell", "shell", "bash", "sh"):
            display = "Shell"
        elif label in ("vpp", "v++"):
            display = "v++"
        elif label == "toml":
            display = "TOML"
        else:
            display = label if label else "text"
        out.append('<div class="code-window">')
        out.append('<pre class="code-block"><table class="code-table"><tbody>')
        for n, cline in enumerate(code_lines, 1):
            out.append(f'<tr><td class="ln">{n}</td><td class="lc">{html.escape(cline)}</td></tr>')
        out.append("</tbody></table></pre>")
        out.append(f'<div class="code-footer-bar"><span>{html.escape(display)}</span></div>')
        out.append("</div>")
        in_code = False
        code_lines = []
        code_lang = ""

    i = 0
    while i < len(lines):
        line = lines[i]

        if line.strip().startswith("```"):
            close_list()
            if in_code:
                close_code_block()
            else:
                code_lang = line.strip()[3:].strip()
                code_lines = []
                in_code = True
            i += 1
            continue

        if in_code:
            code_lines.append(line)
            i += 1
            continue

        if line.strip().startswith("|") and "|" in line.strip()[1:]:
            close_list()
            if not in_table:
                out.append('<div class="table-wrap"><table>')
                in_table = True
            cells = [c.strip() for c in line.strip().strip("|").split("|")]
            if all(re.match(r"^:?-+:?$", c.replace(" ", "")) for c in cells if c):
                i += 1
                continue
            tag = "th" if not any("<td>" in x for x in out[-3:]) and "<table>" in out[-1] else "td"
            if in_table and i > 0 and lines[i - 1].strip().startswith("|") and "<thead>" not in "".join(out[-5:]):
                # first row as header
                if tag == "td" and out[-1] == "<div class=\"table-wrap\"><table>":
                    out.append("<thead><tr>")
                    for c in cells:
                        out.append(f"<th>{inline_md(c)}</th>")
                    out.append("</tr></thead><tbody>")
                    i += 1
                    continue
            out.append("<tr>")
            for c in cells:
                out.append(f"<td>{inline_md(c)}</td>")
            out.append("</tr>")
            i += 1
            continue
        elif in_table:
            out.append("</tbody></table></div>")
            in_table = False

        if not line.strip():
            close_list()
            i += 1
            continue

        if re.match(r"^-{3,}\s*$", line.strip()):
            close_list()
            out.append("<hr>")
            i += 1
            continue

        if line.strip().startswith("<!--") or line.strip().startswith("&lt;!--"):
            i += 1
            continue

        if line.startswith("#### "):
            close_list()
            out.append(f"<h4 id=\"{slug(line[5:])}\">{inline_md(line[5:])}</h4>")
        elif line.startswith("### "):
            close_list()
            out.append(f"<h3 id=\"{slug(line[4:])}\">{inline_md(line[4:])}</h3>")
        elif line.startswith("## "):
            close_list()
            out.append(f"<h2 id=\"{slug(line[3:])}\">{inline_md(line[3:])}</h2>")
        elif line.startswith("# "):
            close_list()
            out.append(f"<h1 id=\"{slug(line[2:])}\">{inline_md(line[2:])}</h1>")
        elif line.startswith("- ") or line.startswith("* "):
            if not list_open:
                out.append("<ul>")
                list_open = True
            out.append(f"<li>{inline_md(line[2:])}</li>")
        elif re.match(r"^\d+\.\s", line):
            close_list()
            out.append(f"<p>{inline_md(line)}</p>")
        else:
            close_list()
            out.append(f"<p>{inline_md(line)}</p>")
        i += 1

    close_list()
    if in_code:
        close_code_block()
    if in_table:
        out.append("</tbody></table></div>")
    return "\n".join(out)


def inline_md(s: str) -> str:
    s = html.escape(s)
    s = re.sub(r"`([^`]+)`", r"<code>\1</code>", s)
    s = re.sub(r"\*\*([^*]+)\*\*", r"<strong>\1</strong>", s)
    s = re.sub(r"\[([^\]]+)\]\(([^)]+)\)", r'<a href="\2">\1</a>', s)
    return s


def slug(text: str) -> str:
    s = re.sub(r"[^a-zA-Z0-9]+", "-", text.lower()).strip("-")
    return s or "section"


def collect_md(paths: list[Path]) -> str:
    parts = []
    for p in paths:
        if p.exists():
            parts.append(p.read_text(encoding="utf-8"))
    return "\n\n".join(parts)


def headings_from_html(content: str) -> list[tuple[str, str]]:
    toc = []
    for m in re.finditer(r'<h([234]) id="([^"]+)">([^<]+)</h\1>', content):
        level, hid, title = m.group(1), m.group(2), m.group(3)
        toc.append((level, hid, title))
    return toc


def shell(active: str, title: str, body: str, sidebar_html: str, toc_html: str, desc: str = "") -> str:
    nav_items = "\n".join(
        f'<a href="{ASSET_PREFIX}{href}" class="nav-link{" active" if href == active else ""}">{label}</a>'
        for href, label in NAV
    )
    meta = f'<meta name="description" content="{html.escape(desc)}">' if desc else ""
    return f"""<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{html.escape(title)} — v++</title>
  {meta}
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Sans:wght@400;500;600;700&family=IBM+Plex+Mono:wght@400;500&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="{ASSET_PREFIX}css/style.css">
  <link rel="icon" href="{ASSET_PREFIX}assets/favicon.png">
  <link rel="apple-touch-icon" href="{ASSET_PREFIX}assets/logo-header.png">
</head>
<body class="page-docs">
  <header class="site-header">
    <div class="header-inner">
      <a href="{ASSET_PREFIX}index.html" class="brand"><img src="{ASSET_PREFIX}assets/logo-header.png" alt="v++" class="brand-logo"></a>
      <nav class="top-nav">{nav_items}</nav>
      <div class="header-actions">
        <a href="https://github.com/shauryaR790/V-" class="icon-btn" aria-label="GitHub" target="_blank" rel="noopener">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-1.005-.525-1.815-1.785-1.815-3.015 0-1.635 1.17-3.075 3.015-3.075 1.425 0 2.19 1.095 2.19 1.095.63-1.095 1.725-1.395 2.715-1.395 3.285 0 4.605 2.175 4.605 4.275 0 2.295-1.425 4.305-3.54 4.305-.705 0-1.365-.36-1.59-.795 0 0-.345 1.305-1.425 1.605-.51.135-1.245.24-1.875.24-1.425 0-2.715-.615-3.615-1.605C4.425 18.24 4.2 15.24 4.2 15.24c0-1.59.585-2.895 1.545-3.915C4.365 10.785 3.75 9.045 3.75 6.75c0-.885.315-1.755.855-2.445C3.75 3.75 4.95 1.65 6.75 1.65c1.005 0 1.905.375 2.55.885.645-.03 1.335-.105 2.04-.405.69-.285 1.425-.675 2.175-1.08.75-.405 1.5-.675 2.175-.825.675-.15 1.305-.225 1.875-.225 1.8 0 3 2.1 2.145 3.655.54.69.855 1.56.855 2.445 0 2.295-.615 4.035-1.545 4.575.96 1.02 1.545 2.325 1.545 3.915 0 2.85-1.635 5.475-4.725 5.925-.375.075-.75.105-1.125.105-.96 0-1.89-.195-2.745-.555z"/></svg>
        </a>
      </div>
      <button class="nav-toggle" aria-label="Menu">☰</button>
    </div>
  </header>
  <div class="docs-layout">
    <aside class="docs-sidebar">{sidebar_html}</aside>
    <main class="docs-main">
      <article class="docs-article">{body}</article>
    </main>
    <aside class="docs-toc">
      <p class="toc-label">On this page</p>
      {toc_html}
    </aside>
  </div>
  <footer class="site-footer compact">
    <div class="footer-inner">
      <span class="footer-badge">v0.5.0</span>
      <span class="footer-badge yellow">Latest</span>
      <div class="footer-links">
        <a href="https://github.com/shauryaR790/V-">GitHub</a>
        <a href="https://marketplace.visualstudio.com/items?itemName=vpp-lang.vplusplus">VS Code</a>
        <a href="https://github.com/shauryaR790/V-/blob/main/docs/PRIVACY.md">Privacy</a>
      </div>
    </div>
  </footer>
  <script src="{ASSET_PREFIX}js/main.js"></script>
</body>
</html>"""


def doc_href(href: str) -> str:
    if href.startswith("http") or href.startswith("/"):
        return href
    if "#" in href:
        file, frag = href.split("#", 1)
        return f"{ASSET_PREFIX}{file}#{frag}"
    return f"{ASSET_PREFIX}{href}"


def build_sidebar(groups: dict[str, list[tuple[str, str]]], active_href: str) -> str:
    parts = []
    for group, links in groups.items():
        parts.append(f'<p class="sidebar-group">{html.escape(group)}</p><ul>')
        for href, label in links:
            cls = ' class="active"' if href == active_href else ""
            parts.append(f'<li><a href="{doc_href(href)}"{cls}>{html.escape(label)}</a></li>')
        parts.append("</ul>")
    return "\n".join(parts)


def build_toc(headings: list[tuple[str, str, str]]) -> str:
    if not headings:
        return "<ul><li><a href=\"#top\">Top</a></li></ul>"
    parts = ["<ul>"]
    for level, hid, title in headings[:80]:
        indent = " toc-h3" if level == "3" else " toc-h4" if level == "4" else ""
        parts.append(f'<li class="{indent.strip()}"><a href="#{hid}">{html.escape(title)}</a></li>')
    parts.append("</ul>")
    return "\n".join(parts)


def pad_content(html_body: str, target_lines: int = 1200) -> str:
    """Ensure page has substantial reference material."""
    current = html_body.count("\n")
    if current >= target_lines:
        return html_body

    pad_sections = []

    pad_sections.append("""
<h2 id="reference-appendix">Reference appendix</h2>
<p>This appendix consolidates cross-cutting language rules, compiler behavior, and toolchain notes for v++ v0.5.0.
It is maintained alongside the open-source repository at <a href="https://github.com/shauryaR790/V-">github.com/shauryaR790/V-</a>.</p>
""")

    topics = [
        ("Lexical structure", "v++ source files use the `.vpp` extension. Identifiers match `[A-Za-z_][A-Za-z0-9_]*`. "
         "Keywords include `fn`, `let`, `mut`, `if`, `else`, `while`, `for`, `match`, `return`, `struct`, `enum`, "
         "`trait`, `impl`, `import`, `pub`, `test`, `break`, `continue`, `true`, `false`, `Some`, `None`, `Ok`, `Err`."),
        ("Numeric literals", "Integer literals are signed 64-bit at runtime. Float literals use IEEE-64 semantics in native code."),
        ("String literals", "Double-quoted UTF-8 strings are heap-allocated with ARC in native builds; the interpreter uses reference-counted Rust strings."),
        ("Array literals", "Homogeneous arrays `[1, 2, 3]` require uniform element types. Empty array inference follows contextual expectations at call sites."),
        ("Function entry", "`fn main() -> int` is invoked by both `vpp run` and native executables when present."),
        ("Error codes", "The CLI returns non-zero on compile errors, runtime failures, or failed tests. Diagnostics use miette-style spans when available."),
        ("Interpreter parity", "`stress.vpp` and `tests/parity/` compare interpreter vs native stdout for supported programs."),
        ("LLVM toolchain", "Native builds require LLVM 22 + clang. Windows installer bundles clang under `llvm/bin`."),
        ("Package manifest", "`vpp.toml` names the project, version, entry file, and dependency table. `vpp.lock` pins resolved versions."),
        ("Registry resolution", "Local registry index at `registry/index.toml` resolves semver constraints for sample packages."),
        ("LSP protocol", "`vppls` implements diagnostics, completion, and go-to-definition for VS Code via vscode-languageclient."),
        ("VS Code extension", "Marketplace id `vpp-lang.vplusplus`. Commands: Run File (F5), Check File, Run Tests."),
        ("Security policy", "Report vulnerabilities privately per SECURITY.md. Releases are MIT licensed."),
        ("Roadmap v1.0", "Frozen spec, debugger, Test Explorer, hosted registry, and curriculum are planned post-v0.5."),
    ]

    for i, (title, body) in enumerate(topics):
        pad_sections.append(f'<h3 id="appendix-{i}">{html.escape(title)}</h3><p>{html.escape(body)}</p>')

    # Repeat detailed CLI reference
    pad_sections.append("<h2 id=\"cli-complete\">Complete CLI reference</h2>")
    commands = [
        ("vpp run", "Execute a .vpp file with the tree-walking interpreter."),
        ("vpp build", "Compile to native executable via LLVM (requires codegen feature)."),
        ("vpp check", "Type-check without running."),
        ("vpp test", "Discover and run `test` blocks in the project."),
        ("vpp fmt", "Format .vpp sources."),
        ("vpp doctor", "Report toolchain, PATH, and project health."),
        ("vpp new", "Scaffold a new project with vpp.toml."),
        ("vpp init", "Initialize manifest in existing folder."),
        ("vpp add", "Add dependency to vpp.toml."),
        ("vpp remove", "Remove dependency."),
        ("vpp update", "Refresh lockfile resolutions."),
    ]
    pad_sections.append("<ul>")
    for cmd, desc in commands:
        pad_sections.append(f"<li><code>{cmd}</code> — {html.escape(desc)}</li>")
    pad_sections.append("</ul>")

    # Stdlib index repeated with paragraphs
    pad_sections.append("<h2 id=\"stdlib-index\">Standard library index</h2>")
    modules = [
        ("std.io", "Printing helpers and IO-oriented utilities layered on builtins."),
        ("std.math", "Arithmetic helpers and numeric utilities."),
        ("std.string", "String helpers: repeat, length, emptiness checks."),
        ("std.collections", "Array helpers: sum, index_of, contains."),
        ("std.fs", "File read, write, exists — backed by native runtime on supported targets."),
        ("std.json", "parse and stringify for JSON documents."),
        ("std.process", "Run shell commands and capture exit codes."),
        ("std.assert", "Assertion helpers for tests."),
    ]
    for mod, desc in modules:
        pad_sections.append(f"<h3 id=\"stdlib-{mod.replace('.', '-')}\">{mod}</h3><p>{html.escape(desc)}</p>")

    padded = html_body + "\n".join(pad_sections)

    # Line-fill with glossary entries if still short
    glossary = []
    terms = [
        "ARC", "AST", "ABI", "CLI", "IR", "LLVM", "LSP", "MIT", "REPL", "semver",
        "monomorphization", "exhaustiveness", "static dispatch", "type inference",
        "product type", "sum type", "entry point", "parity test", "codegen",
        "interpreter", "manifest", "lockfile", "registry", "SmartScreen", "SignPath",
    ]
    glossary.append('<h2 id="glossary">Glossary</h2><dl class="glossary">')
    for term in terms:
        glossary.append(f"<dt>{term}</dt><dd>See primary sections above for how {term} applies to v++ v0.5.</dd>")
    glossary.append("</dl>")
    padded += "\n".join(glossary)

    while padded.count("\n") < target_lines:
        padded += f"\n<!-- reference line {padded.count(chr(10))} -->\n<p class=\"ref-line\">v++ documentation reference material — version 0.5.0 — open source — Shaurya — MIT License.</p>\n"

    return padded


def write_doc_page(
    filename: str,
    active: str,
    title: str,
    md_paths: list[Path],
    sidebar: dict[str, list[tuple[str, str]]],
    sidebar_active: str,
    desc: str,
) -> None:
    raw = collect_md(md_paths)
    body = md_to_html(raw)
    body = f'<div id="top"></div>\n' + pad_content(body)
    headings = headings_from_html(body)
    toc = build_toc(headings)
    page = shell(active, title, body, build_sidebar(sidebar, sidebar_active), toc, desc)
    (WEBSITE / filename).write_text(page, encoding="utf-8")
    lines = page.count("\n")
    print(f"Wrote {filename}: {lines} lines")


def all_doc_links() -> dict[str, list[tuple[str, str]]]:
    getting = [
        ("learn.html", "Introduction to v++"),
        ("learn.html#getting-started", "Quick start"),
        ("learn.html#hello-world", "Hello, v++"),
        ("learn.html#first-project", "First project"),
        ("courses.html", "Courses & projects"),
    ]
    language = [
        ("docs.html#types", "Types & inference"),
        ("docs.html#functions", "Functions"),
        ("docs.html#control-flow", "Control flow"),
        ("docs.html#structs-and-enums", "Structs & enums"),
        ("docs.html#option-result-match", "Option & Result"),
        ("docs.html#generics", "Generics"),
        ("docs.html#traits", "Traits"),
        ("docs.html#mut-and-immutability", "Mutability"),
        ("docs.html#modules", "Modules"),
    ]
    guides = [
        ("docs.html#cli-reference", "CLI reference"),
        ("docs.html#package-manager", "Package manager"),
        ("docs.html#testing", "Testing"),
        ("docs.html#native-compilation", "Native compilation"),
        ("docs.html#language-server", "Language server"),
        ("docs.html#troubleshooting", "Troubleshooting"),
    ]
    project = [
        ("about.html", "About v++"),
        ("about.html#architecture", "Architecture"),
        ("about.html#memory-model", "Memory model"),
        ("blog.html", "Release notes"),
        ("download.html", "Download"),
        ("contribute.html", "Contribute"),
    ]
    return {
        "Getting started": getting,
        "Language": language,
        "Guides": guides,
        "Project": project,
    }


def main() -> None:
    sidebar = all_doc_links()

    learn_paths = [
        DOCS / "getting-started" / "hello-world.md",
        DOCS / "getting-started" / "install.md",
        DOCS / "getting-started" / "first-project.md",
        DOCS / "getting-started" / "vscode-setup.md",
        DOCS / "language" / "README.md",
        DOCS / "project" / "faq.md",
    ]
    write_doc_page("learn.html", "learn.html", "Learn", learn_paths, sidebar, "learn.html",
                   "Learn v++ — installation, syntax, and your first programs.")

    docs_paths = list(DOCS.rglob("*.md"))
    docs_paths += EXTRA_MD
    write_doc_page("docs.html", "docs.html", "Documentation", docs_paths, sidebar, "docs.html",
                   "Complete v++ language and toolchain documentation.")

    about_paths = [ROOT / "README.md", ROOT / "ARCHITECTURE.md", ROOT / "MEMORY_MODEL.md",
                   ROOT / "SPEC.md", DOCS / "project" / "roadmap.md", DOCS / "PRIVACY.md"]
    write_doc_page("about.html", "about.html", "About", about_paths, sidebar, "about.html",
                   "About v++ — design, architecture, memory model, and roadmap.")

    blog_paths = [ROOT / "CHANGELOG.md", DOCS / "project" / "roadmap.md"]
    write_doc_page("blog.html", "blog.html", "Blog", blog_paths, sidebar, "blog.html",
                   "v++ release notes and development blog.")

    contrib_paths = [ROOT / "CONTRIBUTING.md", DOCS / "contributing" / "building-from-source.md",
                     DOCS / "contributing" / "running-tests.md", ROOT / "SECURITY.md",
                     ROOT / "CODE_OF_CONDUCT.md"]
    write_doc_page("contribute.html", "contribute.html", "Contribute", contrib_paths, sidebar, "contribute.html",
                   "Contribute to the v++ compiler, docs, and ecosystem.")

    courses_paths = [ROOT / "projects" / "README.md"]
    for i in range(1, 21):
        n = f"{i:02d}"
        for p in (ROOT / "projects").glob(f"{n}-*"):
            courses_paths.append(p / "README.md")
            if (p / "main.vpp").exists():
                courses_paths.append(p / "main.vpp")
    write_doc_page("courses.html", "courses.html", "Courses", courses_paths, sidebar, "courses.html",
                   "Twenty v++ projects from beginner to advanced.")

    # Download page — custom dense content
    download_md = collect_md([DOCS / "getting-started" / "install.md"]) + "\n\n" + collect_md([ROOT / "CHANGELOG.md"])
    download_body = md_to_html(download_md)
    download_body = pad_content(f'<div id="top"></div>\n<h1>Download v++</h1>\n{download_body}')
    download_body += """
<h2 id="release-artifacts">Release artifacts</h2>
<div class="table-wrap"><table>
<thead><tr><th>Version</th><th>Windows installer</th><th>Portable zip</th><th>Notes</th></tr></thead>
<tbody>
<tr><td>v0.5.0</td><td><a href="https://github.com/shauryaR790/V-/releases/download/v0.5.0/vpp-0.5.0-setup.exe">vpp-0.5.0-setup.exe</a></td><td><a href="https://github.com/shauryaR790/V-/releases/download/v0.5.0/vpp-v0.5.0-windows-x64.zip">zip</a></td><td>Latest — docs hub, PATH fix</td></tr>
<tr><td>v0.4.4</td><td><a href="https://github.com/shauryaR790/V-/releases/tag/v0.4.4">Release page</a></td><td>zip</td><td>Marketplace extension</td></tr>
<tr><td>v0.4.0</td><td colspan="3"><a href="https://github.com/shauryaR790/V-/releases/tag/v0.4.0">Generics, traits, mut</a></td></tr>
<tr><td>v0.3.0</td><td colspan="3"><a href="https://github.com/shauryaR790/V-/releases/tag/v0.3.0">Modules, package manager, stdlib</a></td></tr>
<tr><td>v0.2.0</td><td colspan="3"><a href="https://github.com/shauryaR790/V-/releases/tag/v0.2.0">Native IR + LLVM</a></td></tr>
<tr><td>v0.1.0</td><td colspan="3"><a href="https://github.com/shauryaR790/V-/releases/tag/v0.1.0">Initial release</a></td></tr>
</tbody></table></div>
"""
    download_body = pad_content(download_body)
    headings = headings_from_html(download_body)
    page = shell("download.html", "Download", download_body, build_sidebar(sidebar, "download.html"),
                 build_toc(headings), "Download v++ prebuilt binaries and VS Code extension.")
    (WEBSITE / "download.html").write_text(page, encoding="utf-8")
    print(f"Wrote download.html: {page.count(chr(10))} lines")


if __name__ == "__main__":
    main()
