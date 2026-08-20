#!/usr/bin/env python3
"""Generate documentation HTML pages for the V++ website."""

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

BRAND = "V++"

GITHUB_SVG = (
    '<svg width="20" height="20" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">'
    '<path fill-rule="evenodd" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38'
    ' 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53'
    ' .63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95'
    ' 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.18.82.63-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27'
    ' 1.51-1.04 2.18-.82 2.18-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48'
    ' 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"/>'
    '</svg>'
)

TREE_CHEVRON = (
    '<svg class="tree-chevron" width="12" height="12" viewBox="0 0 16 16" aria-hidden="true">'
    '<path fill="currentColor" d="M6.22 3.22a.75.75 0 0 1 1.06 0l4.25 4.25a.75.75 0 0 1 0 1.06l-4.25 4.25a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042L9.94 8 6.22 4.28a.75.75 0 0 1 0-1.06Z"/>'
    '</svg>'
)

TREE_FOLDER = (
    '<svg class="tree-icon tree-icon-folder" width="16" height="16" viewBox="0 0 16 16" aria-hidden="true">'
    '<path fill="currentColor" d="M1.75 1A1.75 1.75 0 0 0 0 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0 0 16 13.25V6.75A1.75 1.75 0 0 0 14.25 5H8.06l-.72-1.44A1.75 1.75 0 0 0 5.68 2H1.75Z"/>'
    '</svg>'
)

TREE_SEARCH = (
    '<svg class="tree-search-icon" width="14" height="14" viewBox="0 0 16 16" aria-hidden="true">'
    '<path fill="currentColor" d="M10.68 11.74a6 6 0 0 1-7.922-8.982 6 6 0 0 1 8.982 7.922l3.04 3.04a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215ZM11.5 7a4.499 4.499 0 1 0-8.997 0A4.499 4.499 0 0 0 11.5 7Z"/>'
    '</svg>'
)

TREE_FILE = (
    '<svg class="tree-icon tree-icon-file" width="16" height="16" viewBox="0 0 16 16" aria-hidden="true">'
    '<path fill="currentColor" d="M2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v9.586A1.75 1.75 0 0 1 12.25 16h-8.5A1.75 1.75 0 0 1 2 14.25Zm1.75-.25a.25.25 0 0 0-.25.25v12.5c0 .138.112.25.25.25h8.5a.25.25 0 0 0 .25-.25V6h-2.75A1.75 1.75 0 0 1 8 4.25V1.5Z"/>'
    '</svg>'
)

EXTRA_MD = [
    ROOT / "SPEC.md",
    ROOT / "ARCHITECTURE.md",
    ROOT / "MEMORY_MODEL.md",
    ROOT / "CHANGELOG.md",
    ROOT / "CONTRIBUTING.md",
    ROOT / "README.md",
]


MIN_CODE_LINES = 5

COMMENT_LINES: dict[str, list[str]] = {
    "bash": [
        "# Verify install: vpp doctor",
        "# Restart the terminal after PATH changes",
        "# Type-check: vpp check main.vpp",
        "# Format source: vpp fmt main.vpp",
        "# Native build: vpp build src/main.vpp -o app.exe",
    ],
    "vpp": [
        "// Run: vpp run main.vpp",
        "// Type-check only: vpp check main.vpp",
        "// Format: vpp fmt main.vpp",
        "// Tests: vpp test",
        "// vpp calls main() when defined",
    ],
    "toml": [
        "# Package manifest for vpp run / vpp test / vpp build",
        "# Add dependencies with: vpp add <name> --version <ver>",
        "# entry points to src/main.vpp by default",
    ],
    "rust": [
        "// V++ compiler source (Rust)",
        "// See ARCHITECTURE.md for the pipeline overview",
    ],
    "text": [
        "# ...",
        "# See docs for full examples",
    ],
}


def comment_pad(plang: str, index: int) -> str:
    pool = COMMENT_LINES.get(plang, COMMENT_LINES["text"])
    return pool[index % len(pool)]


def enrich_code_lines(lines: list[str], plang: str) -> list[str]:
    """Expand very short shell snippets with real related commands."""
    non_empty = [ln for ln in lines if ln.strip()]
    if len(non_empty) >= MIN_CODE_LINES:
        return non_empty

    if not non_empty:
        return lines

    first = non_empty[0].strip()

    if plang == "bash":
        if first.startswith("vpp run "):
            target = first[8:].strip()
            return [
                first,
                f"vpp check {target}",
                "vpp --version",
                "# Verify your install",
                "vpp doctor",
            ]
        if first.startswith("vpp check "):
            target = first[10:].strip()
            return [
                first,
                f"vpp run {target}",
                f"vpp fmt {target}",
                "vpp test",
                "vpp build src/main.vpp -o app.exe",
            ]
        if first in ("vpp --version", "vpp doctor"):
            return [
                "vpp --version",
                "vpp doctor",
                "vpp run examples/hello.vpp",
                "vpp check examples/hello.vpp",
                "vpp fmt examples/hello.vpp",
            ]
        if first.startswith("vpp build"):
            merged = list(non_empty)
            for extra in ("./app.exe", "vpp run app.vpp", "vpp test", "# Release binary"):
                if len(merged) >= MIN_CODE_LINES:
                    break
                if extra not in merged:
                    merged.append(extra)
            return merged
        if first.startswith("vpp new"):
            return [
                first,
                "cd myapp",
                "vpp run",
                "vpp test",
                "vpp build src/main.vpp -o myapp.exe",
            ]
        if first.startswith("cd ") and any("vpp run" in ln for ln in non_empty):
            merged = list(non_empty)
            for extra in ("vpp check main.vpp", "vpp test", "vpp --version", "# Module project"):
                if len(merged) >= MIN_CODE_LINES:
                    break
                if extra not in merged:
                    merged.append(extra)
            return merged
        if first.startswith("$") or "$env:" in first or "[Environment]" in first:
            merged = list(non_empty)
            for extra in (
                "# Restart terminal after updating PATH",
                "vpp --version",
                "vpp doctor",
                "vpp run examples/hello.vpp",
            ):
                if len(merged) >= MIN_CODE_LINES:
                    break
                if extra not in merged:
                    merged.append(extra)
            return merged
        if first.startswith("vpp add"):
            merged = list(non_empty)
            for extra in ("vpp install", "vpp run", "vpp test", "# Resolves deps from vpp.toml"):
                if len(merged) >= MIN_CODE_LINES:
                    break
                if extra not in merged:
                    merged.append(extra)
            return merged
        if first.startswith("vpp test"):
            merged = list(non_empty)
            for extra in ("vpp run", "vpp check src/main.vpp", "vpp --version", "# Inline test blocks supported"):
                if len(merged) >= MIN_CODE_LINES:
                    break
                if extra not in merged:
                    merged.append(extra)
            return merged
        if first.startswith("vpp fmt"):
            merged = list(non_empty)
            for extra in ("vpp check main.vpp", "vpp run main.vpp", "# Writes formatted source in place"):
                if len(merged) >= MIN_CODE_LINES:
                    break
                if extra not in merged:
                    merged.append(extra)
            return merged
        if any("vpp" in ln for ln in non_empty):
            merged = list(non_empty)
            for extra in (
                "vpp --version",
                "vpp doctor",
                "vpp run examples/hello.vpp",
                "vpp check examples/hello.vpp",
            ):
                if len(merged) >= MIN_CODE_LINES:
                    break
                if extra not in merged:
                    merged.append(extra)
            return merged

    if plang == "vpp" and len(non_empty) < MIN_CODE_LINES:
        merged = list(non_empty)
        for extra in (
            "fn main() -> int {",
            "    return 0",
            "}",
        ):
            if len(merged) >= MIN_CODE_LINES:
                break
            if extra not in merged:
                merged.append(extra)
        if len(merged) < MIN_CODE_LINES:
            i = 0
            while len(merged) < MIN_CODE_LINES:
                merged.append(comment_pad("vpp", i))
                i += 1
        return merged

    return non_empty


def finalize_code_lines(lines: list[str], plang: str) -> list[str]:
    """No blank lines — pad short blocks with comments to MIN_CODE_LINES."""
    cleaned = [ln for ln in lines if ln.strip()]
    if not cleaned:
        return [comment_pad(plang, i) for i in range(MIN_CODE_LINES)]

    result = enrich_code_lines(cleaned, plang)
    result = [ln for ln in result if ln.strip()]

    i = 0
    while len(result) < MIN_CODE_LINES:
        result.append(comment_pad(plang, i))
        i += 1
    return result


def code_filename(display: str, plang: str, lines: list[str]) -> str:
    if plang == "vpp":
        return "main.vpp"
    if plang == "toml":
        return "vpp.toml"
    if plang == "rust":
        return "lib.rs"
    if plang == "bash":
        for ln in lines:
            s = ln.strip()
            if s.startswith("vpp run "):
                path = s[8:].strip().replace("\\", "/")
                name = path.split("/")[-1]
                return name if name else "terminal"
        return "terminal"
    return display.lower()


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
            display = BRAND
        elif label == "toml":
            display = "TOML"
        else:
            display = label if label else "text"
        plang = {
            "powershell": "bash", "shell": "bash", "sh": "bash",
            "vpp": "vpp", "v++": "vpp",
            "toml": "toml", "bash": "bash", "rust": "rust",
        }.get(label, label or "text")
        filename = code_filename(display, plang, code_lines)
        code_lines = finalize_code_lines(code_lines, plang)
        code_text = html.escape("\n".join(code_lines))
        out.append('<div class="code-block-wrap">')
        out.append(
            f'<div class="code-block-header">'
            f'<span class="code-block-filename">{html.escape(filename)}</span>'
            f"</div>"
        )
        out.append(
            f'<pre class="language-{plang}">'
            f'<code class="language-{plang}">{code_text}</code></pre>'
        )
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


def faq_md_to_html(text: str) -> str:
    """Render FAQ markdown as a bordered Q&A card list."""
    lines = text.splitlines()
    out: list[str] = []
    i = 0

    if i < len(lines) and lines[i].startswith("# "):
        title = lines[i][2:].strip()
        out.append(f'<h1 id="{slug(title)}">{inline_md(title)}</h1>')
        i += 1

    out.append('<div class="faq-list">')
    question: str | None = None
    section_lines: list[str] = []

    def flush_section() -> None:
        nonlocal question, section_lines
        if question is None:
            return
        body = "\n".join(section_lines).strip()
        body_html = md_to_html(body) if body else ""
        qid = slug(question)
        out.append(
            f'<section class="faq-item" id="{qid}">'
            f'<h2 class="faq-question">{inline_md(question)}</h2>'
            f'<div class="faq-answer">{body_html}</div>'
            f"</section>"
        )
        question = None
        section_lines = []

    while i < len(lines):
        line = lines[i]
        if line.startswith("## "):
            flush_section()
            question = line[3:].strip()
            section_lines = []
        elif question is not None:
            section_lines.append(line)
        i += 1

    flush_section()
    out.append("</div>")
    return "\n".join(out)


def render_paths(
    paths: list[Path],
    use_sources: bool = False,
    strip_h1_from: set[str] | None = None,
) -> str:
    parts: list[str] = []
    strip = strip_h1_from or set()
    for p in paths:
        if not p.exists():
            continue
        if p.name == "faq.md":
            parts.append(faq_md_to_html(p.read_text(encoding="utf-8")))
            continue
        if use_sources and p.suffix in (".vpp", ".toml", ".rs"):
            parts.append(md_to_html(source_to_md(p)))
            continue
        text = p.read_text(encoding="utf-8")
        if p.name in strip:
            lines = text.splitlines()
            if lines and lines[0].startswith("# "):
                text = "\n".join(lines[1:]).lstrip("\n")
        parts.append(md_to_html(text))
    return "\n\n".join(parts)


def brand_text(s: str) -> str:
    return re.sub(r"v\+\+", BRAND, s)


def inline_md(s: str) -> str:
    s = html.escape(s)
    s = re.sub(r"`([^`]+)`", r"<code>\1</code>", s)
    s = re.sub(r"\*\*([^*]+)\*\*", r"<strong>\1</strong>", s)
    s = re.sub(r"\[([^\]]+)\]\(([^)]+)\)", r'<a href="\2">\1</a>', s)
    return brand_text(s)


def slug(text: str) -> str:
    s = re.sub(r"[^a-zA-Z0-9]+", "-", text.lower()).strip("-")
    return s or "section"


def collect_md(paths: list[Path]) -> str:
    parts = []
    for p in paths:
        if p.exists():
            parts.append(p.read_text(encoding="utf-8"))
    return "\n\n".join(parts)


def source_to_md(path: Path) -> str:
    """Wrap source files as markdown sections for the reference page."""
    rel = path.relative_to(ROOT).as_posix()
    text = path.read_text(encoding="utf-8")
    if path.suffix == ".vpp":
        fence = "vpp"
    elif path.suffix == ".toml":
        fence = "toml"
    elif path.suffix == ".rs":
        fence = "rust"
    else:
        return f"\n\n<!-- {rel} -->\n\n{text}"
    if "projects/" in rel and path.name == "main.vpp":
        return f"\n\n```{fence}\n{text}\n```\n"
    heading = f"### `{rel}`"
    return f"\n\n{heading}\n\n```{fence}\n{text}\n```\n"


def collect_sources(paths: list[Path]) -> str:
    parts: list[str] = []
    for p in paths:
        if not p.exists():
            continue
        if p.suffix in (".vpp", ".toml", ".rs"):
            parts.append(source_to_md(p))
        else:
            rel = p.relative_to(ROOT).as_posix()
            parts.append(f"\n\n<!-- file: {rel} -->\n\n{p.read_text(encoding='utf-8')}")
    return "\n".join(parts)


def dedupe_paths(paths: list[Path]) -> list[Path]:
    seen: set[Path] = set()
    out: list[Path] = []
    for p in paths:
        key = p.resolve()
        if key in seen or not p.exists():
            continue
        seen.add(key)
        out.append(p)
    return out


def all_reference_sources() -> list[Path]:
    """Every real doc and source file in the repo for the master reference page."""
    paths: list[Path] = []
    paths.extend(sorted(DOCS.rglob("*.md")))
    for name in (
        "README.md", "SPEC.md", "ARCHITECTURE.md", "MEMORY_MODEL.md",
        "CHANGELOG.md", "CONTRIBUTING.md", "SECURITY.md", "CODE_OF_CONDUCT.md",
    ):
        paths.append(ROOT / name)
    paths.extend(sorted((ROOT / "std").glob("*.vpp")))
    paths.extend(sorted((ROOT / "examples").glob("*.vpp")))
    paths.append(ROOT / "projects" / "README.md")
    for i in range(1, 21):
        for proj in sorted((ROOT / "projects").glob(f"{i:02d}-*")):
            readme = proj / "README.md"
            main_vpp = proj / "main.vpp"
            toml = proj / "vpp.toml"
            if readme.exists():
                paths.append(readme)
            if main_vpp.exists():
                paths.append(main_vpp)
            if toml.exists():
                paths.append(toml)
    for extra in (
        ROOT / "stress.vpp",
        ROOT / "editor" / "vscode-vpp" / "README.md",
        ROOT / "editor" / "vscode-vpp" / "CHANGELOG.md",
        ROOT / "registry" / "index.toml",
        ROOT / "registry" / "fixtures" / "hello-lib" / "src" / "lib.vpp",
        ROOT / "registry" / "fixtures" / "hello-lib" / "vpp.toml",
    ):
        paths.append(extra)
    staging_std = ROOT / "staging" / "std"
    if staging_std.exists():
        paths.extend(sorted(staging_std.glob("*.vpp")))
    staging_ex = ROOT / "staging" / "examples"
    if staging_ex.exists():
        paths.extend(sorted(staging_ex.glob("*.vpp")))
    fixtures = ROOT / "tests" / "fixtures"
    if fixtures.exists():
        paths.extend(sorted(fixtures.glob("*.vpp")))
    paths.extend(sorted((ROOT / "tests").glob("*.rs")))
    for rel in (
        "Cargo.toml",
        "build.rs",
        "src/lib.rs",
        "src/driver.rs",
        "src/error.rs",
        "src/bin/vppls.rs",
    ):
        paths.append(ROOT / rel)
    return dedupe_paths(paths)


def headings_from_html(content: str) -> list[tuple[str, str, str]]:
    toc = []
    for m in re.finditer(r'<h([1234]) id="([^"]+)">([^<]+)</h\1>', content):
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
  <title>{html.escape(title)} — {BRAND}</title>
  {meta}
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Sans:wght@400;500;600;700&family=IBM+Plex+Mono:wght@400;500&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="{ASSET_PREFIX}css/style.css">
  <link rel="stylesheet" href="{ASSET_PREFIX}css/prism-vpp.css">
  <link rel="icon" href="{ASSET_PREFIX}assets/favicon.png">
  <link rel="apple-touch-icon" href="{ASSET_PREFIX}assets/logo-header.png">
</head>
<body class="page-docs">
  <header class="site-header">
    <div class="header-inner">
      <a href="{ASSET_PREFIX}index.html" class="brand"><img src="{ASSET_PREFIX}assets/logo-header.png" alt="{BRAND}" class="brand-logo"></a>
      <nav class="top-nav">{nav_items}</nav>
      <div class="header-actions">
        <a href="https://github.com/shauryaR790/V-" class="icon-btn" aria-label="GitHub" target="_blank" rel="noopener">
          {GITHUB_SVG}
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
      <nav class="toc-tree" aria-label="On this page">{toc_html}</nav>
    </aside>
  </div>
  <script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/prism.min.js"></script>
  <script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-clike.min.js"></script>
  <script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-javascript.min.js"></script>
  <script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-bash.min.js"></script>
  <script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-toml.min.js"></script>
  <script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-rust.min.js"></script>
  <script src="{ASSET_PREFIX}js/prism-vpp.js"></script>
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
    parts = [
        '<div class="tree-search-wrap">',
        '<div class="tree-search-field">',
        TREE_SEARCH,
        '<input type="search" class="tree-search" placeholder="Go to page" aria-label="Filter navigation">',
        "</div></div>",
        '<nav class="sidebar-tree" aria-label="Documentation">',
    ]
    for group, links in groups.items():
        parts.append('<details class="tree-folder" open>')
        parts.append(
            f'<summary class="tree-folder-label">{TREE_CHEVRON}{TREE_FOLDER}'
            f"{html.escape(group)}</summary>"
        )
        parts.append('<ul class="tree-list">')
        for href, label in links:
            parts.append(
                f'<li><a href="{doc_href(href)}" class="tree-link">'
                f"{TREE_FILE}{html.escape(label)}</a></li>"
            )
        parts.append("</ul></details>")
    parts.append("</nav>")
    return "\n".join(parts)


def build_toc(headings: list[tuple[str, str, str]]) -> str:
    if not headings:
        return f'<ul><li><a href="#top" class="tree-link">{TREE_FILE}Top</a></li></ul>'
    parts = ["<ul>"]
    for level, hid, title in headings[:120]:
        if level == "4":
            cls = "toc-h4"
        elif level == "3":
            cls = "toc-h3"
        elif level == "2":
            cls = "toc-h2"
        else:
            cls = ""
        title_clean = html.unescape(title)
        parts.append(
            f'<li class="{cls}"><a href="#{hid}" class="tree-link">{TREE_FILE}{title_clean}</a></li>'
        )
    parts.append("</ul>")
    return "\n".join(parts)


def write_doc_page(
    filename: str,
    active: str,
    title: str,
    md_paths: list[Path],
    sidebar: dict[str, list[tuple[str, str]]],
    sidebar_active: str,
    desc: str,
    use_sources: bool = False,
    page_h1: str | None = None,
    strip_h1_from: set[str] | None = None,
) -> None:
    if use_sources:
        body = render_paths(md_paths, use_sources=True, strip_h1_from=strip_h1_from)
    else:
        body = render_paths(md_paths, use_sources=False, strip_h1_from=strip_h1_from)
    if page_h1:
        body = (
            f'<div id="top"></div>\n'
            f'<h1 id="{slug(page_h1)}">{html.escape(page_h1)}</h1>\n{body}'
        )
    else:
        body = f'<div id="top"></div>\n' + body
    headings = headings_from_html(body)
    toc = build_toc(headings)
    page = shell(active, title, body, build_sidebar(sidebar, sidebar_active), toc, desc)
    (WEBSITE / filename).write_text(page, encoding="utf-8")
    lines = page.count("\n")
    print(f"Wrote {filename}: {lines} lines")


def all_doc_links() -> dict[str, list[tuple[str, str]]]:
    getting = [
        ("learn.html#introduction", "Introduction"),
        ("learn.html#install", "Install"),
        ("learn.html#first-program", "First program"),
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
        ("about.html", f"About {BRAND}"),
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
        DOCS / "getting-started" / "introduction.md",
        DOCS / "getting-started" / "install.md",
        DOCS / "language" / "README.md",
        DOCS / "project" / "faq.md",
        DOCS / "getting-started" / "hello-world.md",
        DOCS / "getting-started" / "first-project.md",
        DOCS / "getting-started" / "vscode-setup.md",
    ]
    write_doc_page("learn.html", "learn.html", "Learn", learn_paths, sidebar, "learn.html#introduction",
                   f"Learn {BRAND} — installation, syntax, and your first programs.")

    docs_paths = all_reference_sources()
    write_doc_page("docs.html", "docs.html", "Documentation", docs_paths, sidebar, "docs.html",
                   f"Complete {BRAND} language and toolchain documentation.", use_sources=True)

    about_paths = [ROOT / "README.md", ROOT / "ARCHITECTURE.md", ROOT / "MEMORY_MODEL.md",
                   ROOT / "SPEC.md", DOCS / "project" / "roadmap.md", DOCS / "PRIVACY.md"]
    write_doc_page(
        "about.html", "about.html", "About The Language", about_paths, sidebar, "about.html",
        "About The Language — design, architecture, memory model, and roadmap.",
        page_h1="About The Language",
        strip_h1_from={"README.md"},
    )

    blog_paths = [ROOT / "CHANGELOG.md", DOCS / "project" / "roadmap.md"]
    write_doc_page("blog.html", "blog.html", "Blog", blog_paths, sidebar, "blog.html",
                   f"{BRAND} release notes and development blog.")

    contrib_paths = [ROOT / "CONTRIBUTING.md", DOCS / "contributing" / "building-from-source.md",
                     DOCS / "contributing" / "running-tests.md", ROOT / "SECURITY.md",
                     ROOT / "CODE_OF_CONDUCT.md"]
    write_doc_page("contribute.html", "contribute.html", "Contribute", contrib_paths, sidebar, "contribute.html",
                   f"Contribute to the {BRAND} compiler, docs, and ecosystem.")

    courses_paths = [ROOT / "projects" / "README.md"]
    for i in range(1, 21):
        n = f"{i:02d}"
        for p in (ROOT / "projects").glob(f"{n}-*"):
            courses_paths.append(p / "README.md")
            if (p / "main.vpp").exists():
                courses_paths.append(p / "main.vpp")
    write_doc_page("courses.html", "courses.html", "Courses", courses_paths, sidebar, "courses.html",
                   f"Twenty {BRAND} projects from beginner to advanced.", use_sources=True)

    # Download page
    download_md = collect_md([DOCS / "getting-started" / "install.md"]) + "\n\n" + collect_md([ROOT / "CHANGELOG.md"])
    download_body = md_to_html(download_md)
    download_body = f'<div id="top"></div>\n<h1>Download {BRAND}</h1>\n{download_body}'
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
    headings = headings_from_html(download_body)
    page = shell("download.html", "Download", download_body, build_sidebar(sidebar, "download.html"),
                 build_toc(headings), f"Download {BRAND} prebuilt binaries and VS Code extension.")
    (WEBSITE / "download.html").write_text(page, encoding="utf-8")
    print(f"Wrote download.html: {page.count(chr(10))} lines")


if __name__ == "__main__":
    main()
