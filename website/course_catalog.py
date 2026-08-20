"""Course project metadata and lesson step generation for the V++ website."""

from __future__ import annotations

import html
import json
import re
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PROJECTS = ROOT / "projects"

LEVEL_BY_NUM: dict[int, str] = {
    **{i: "Beginner" for i in range(1, 6)},
    **{i: "Beginner" for i in (10,)},
    **{i: "Intermediate" for i in range(6, 12)},
    **{i: "Intermediate" for i in (14, 15, 16, 17)},
    **{i: "Advanced" for i in (12, 13, 18, 19, 20)},
}

OUTPUT_BY_SLUG: dict[str, str] = {
    "01-hello-world": "Hello, v++!",
    "02-variables": "13\n7\n30\nHello, v++!",
    "03-functions": "30\nHello, Shaurya",
    "04-loops": "0\n1\n2\n3\n4",
    "05-arrays": "15",
    "06-structs": "3\n4\n50",
    "07-enums": "Stop\nGo",
    "08-option-result": "5\nErr: division by zero",
    "09-match": "Midweek\nWeekend",
    "10-fizzbuzz": "\n".join(
        [
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz",
            "11", "Fizz", "13", "14", "FizzBuzz", "16", "17", "Fizz", "19", "Buzz",
        ]
    ),
    "11-fibonacci": "0\n1\n1\n2\n3\n5\n8\n13\n21\n34",
    "12-generics": "42\nhello",
    "13-traits": "Circle(5)\narea=78",
    "14-modules": "12",
    "15-calculator": "42",
    "16-word-counter": "11",
    "17-guessing-game": "too low\ntoo high\ncorrect",
    "18-todo-list": "Buy milk\nWrite code\nWalk dog",
    "19-file-notes": "note saved",
    "20-json-config": "my-app\n0.5.0",
}

def join_paragraphs(paragraphs: list[str]) -> str:
    return "\n\n".join(p.strip() for p in paragraphs if p and p.strip())


def preview_output(output: str, max_lines: int = 3) -> str:
    lines = [ln for ln in output.splitlines() if ln.strip()]
    if not lines:
        return "program output"
    sample = lines[:max_lines]
    text = ", ".join(f"`{ln}`" for ln in sample)
    if len(lines) > max_lines:
        text += f", and {len(lines) - max_lines} more lines"
    return text


def concept_list_text(concepts: list[str]) -> str:
    if not concepts:
        return "core V++ syntax"
    cleaned = [c.strip().strip("`") for c in concepts]
    if len(cleaned) == 1:
        return cleaned[0]
    return ", ".join(cleaned[:-1]) + f", and {cleaned[-1]}"


def build_overview_theory(project: CourseProject) -> str:
    concepts = concept_list_text(project.concepts)
    return join_paragraphs(
        [
            (
                f"**{project.title}** is project {project.num:02d} in the V++ curriculum ({project.level} level). "
                f"{project.summary}"
            ),
            (
                f"In this walkthrough you will not paste the whole file at once. Each step introduces one idea, "
                f"explains what the lines mean, and shows only the new snippet. By the end you will combine "
                f"everything into a complete program that produces {preview_output(project.output)}."
            ),
            (
                f"The key ideas here are {concepts}. Read the prose under every heading before you look at code. "
                f"When you reach the playground, run the full source with `{project.run_cmd}` on your machine "
                f"or use the Run button to compare against the expected output."
            ),
        ]
    )


def build_goals_theory(project: CourseProject) -> str:
    level_hint = {
        "Beginner": (
            "This is an early project: focus on syntax, naming, and how the compiler reads your file top to bottom."
        ),
        "Intermediate": (
            "You should already be comfortable with variables, functions, and loops from projects 01 through 05."
        ),
        "Advanced": (
            "This project combines multiple features. Revisit earlier lessons if a concept feels unfamiliar."
        ),
    }.get(project.level, "Work at your own pace and experiment locally.")
    concepts = concept_list_text(project.concepts)
    return join_paragraphs(
        [
            (
                f"After finishing **{project.title}**, you should be able to explain {concepts} in your own words "
                f"and reproduce the program without copying blindly."
            ),
            level_hint,
            (
                "Use `vpp check` on your file when something fails to compile. The type checker reports "
                "signature mismatches early. Fix one error at a time, then re-run."
            ),
            (
                "Tip: type out each snippet yourself instead of copy-pasting. Muscle memory matters for "
                "braces, commas, and return types."
            ),
        ]
    )


def describe_let(line: str) -> str:
    mut = "mut " if "let mut " in line else ""
    match = re.match(r"let\s+(?:mut\s+)?(\w+)\s*=\s*(.+)", line.strip())
    if not match:
        return f"The binding `{line.strip()}` introduces a new local name in the current scope."
    name, value = match.group(1), match.group(2).rstrip()
    if mut:
        return (
            f"`let mut {name} = {value}` creates a **mutable** binding. You can reassign `{name}` later "
            f"(for example in a loop). The compiler still tracks its type after each assignment."
        )
    if value.startswith('"') or ("+" in value and '"' in value):
        return (
            f"`let {name} = {value}` stores a string value. V++ checks that `{name}` stays a string if you "
            f"use it again in concatenation or printing."
        )
    return (
        f"`let {name} = {value}` binds the name `{name}` to a value. V++ infers the type from the right "
        f"hand side so you rarely need to write it explicitly for simple literals."
    )


def describe_print(line: str) -> str:
    inner = line.strip()[6:-1] if line.strip().endswith(")") else line.strip()
    if any(op in inner for op in (" + ", " - ", " * ", " / ")):
        return (
            f"`print({inner})` evaluates the expression first, then prints the result on its own line. "
            f"Arithmetic operators work on integers the way you expect from math class."
        )
    if "+" in inner and '"' in inner:
        return (
            f"`print({inner})` demonstrates string concatenation: `+` joins string pieces when both sides are strings."
        )
    return (
        f"`print({inner})` writes to standard output followed by a newline. "
        f"`print` accepts values such as integers and strings."
    )


def describe_return(line: str) -> str:
    value = line.strip().removeprefix("return").strip()
    if value == "0":
        return (
            "`return 0` exits `main` with status code 0, which operating systems treat as success. "
            "Non-zero values signal an error."
        )
    return f"`return {value}` sends a value back to the caller and ends the current function."


def describe_if(line: str) -> str:
    return (
        f"`{line.strip()}` starts a conditional block. The condition must be `bool`. "
        f"Only the matching branch runs; other branches are skipped."
    )


def describe_while(line: str) -> str:
    return (
        f"`{line.strip()}` repeats its body while the condition stays true. "
        f"Make sure something inside the loop eventually changes the condition to avoid infinite loops."
    )


def describe_match_line(line: str) -> str:
    stripped = line.strip()
    if stripped.startswith("match "):
        return (
            f"`{stripped}` inspects a value and selects the first arm whose pattern fits. "
            f"V++ requires match expressions to be exhaustive over the type."
        )
    if "=>" in stripped:
        return f"The arm `{stripped}` handles one case. When it matches, the code after `=>` runs."
    return ""


def describe_line(line: str) -> str | None:
    stripped = line.strip()
    if not stripped or stripped in ("{", "}"):
        return None
    if stripped.startswith("let "):
        return describe_let(stripped)
    if stripped.startswith("print("):
        return describe_print(stripped)
    if stripped.startswith("return "):
        return describe_return(stripped)
    if stripped.startswith("if "):
        return describe_if(stripped)
    if stripped.startswith("while "):
        return describe_while(stripped)
    if stripped.startswith("match ") or "=>" in stripped:
        return describe_match_line(stripped)
    if stripped.startswith("import "):
        mod = stripped.removeprefix("import").strip()
        return f"`import {mod}` brings that module's public symbols into this file so you can call its functions."
    if re.match(r"\w+\s*=", stripped) and not stripped.startswith("let "):
        name = stripped.split("=")[0].strip()
        return f"`{stripped}` reassigns `{name}`. This only works when `{name}` was declared with `let mut`."
    return None


def explain_import_block(block: str) -> list[str]:
    imports = [ln.strip() for ln in block.splitlines() if ln.strip().startswith("import")]
    names = [ln.removeprefix("import").strip() for ln in imports]
    joined = ", ".join(f"`{n}`" for n in names)
    return [
        f"This file begins with {joined}. Imports must appear before functions and types.",
        (
            "The compiler resolves standard library paths like `std.fs` or `std.json` automatically when "
            "you run `vpp run`. If an import is misspelled, you will get an unknown module error at compile time."
        ),
    ]


def explain_type_def_block(block: str, kind: str, title: str) -> list[str]:
    first = block.lstrip().splitlines()[0].strip()
    paragraphs = []
    if kind == "enum":
        paragraphs.append(
            f"We introduce **{title}** with `{first}`. An enum lists every legal variant up front, "
            f"which prevents impossible states later when you `match` on it."
        )
        variants = [ln.strip() for ln in block.splitlines()[1:] if ln.strip() and ln.strip() not in ("{", "")]
        if variants:
            paragraphs.append(
                f"Variants {', '.join(f'`{v}`' for v in variants)} are the only allowed values. "
                f"Adding a new variant later means updating every `match` that uses this enum."
            )
    elif kind == "struct":
        paragraphs.append(
            f"**{title}** defines a product type: `{first}`. Structs group fields that belong together "
            f"so you pass one value instead of many separate parameters."
        )
        fields = [ln.strip().rstrip(",") for ln in block.splitlines()[1:] if ":" in ln]
        if fields:
            paragraphs.append(
                "Fields " + ", ".join(f"`{f}`" for f in fields) + " are checked at compile time. "
                "You construct values with struct literal syntax `{ field: value }`."
            )
    elif kind == "trait":
        paragraphs.append(
            f"**{title}** declares a trait with `{first}`. Traits describe behavior that multiple types "
            f"can share. Implementations are resolved statically at compile time."
        )
    return paragraphs


def explain_fn_block(block: str, title: str, is_main: bool) -> list[str]:
    first = block.lstrip().splitlines()[0].strip()
    paragraphs = []
    if is_main:
        paragraphs.append(
            "`fn main() -> int` is the program entry point. The runtime calls it after your definitions "
            "are loaded. Everything inside the braces runs in order."
        )
    else:
        paragraphs.append(
            f"**{title}** adds `{first}`. Parameters and return types are required on function signatures; "
            f"the body must return a value compatible with that return type on every path."
        )
    body_lines = [
        ln
        for ln in block.splitlines()[1:]
        if ln.strip() and ln.strip() not in ("{", "}")
    ]
    for ln in body_lines:
        desc = describe_line(ln)
        if desc:
            paragraphs.append(desc)
    if is_main and not any("return" in ln for ln in body_lines):
        paragraphs.append(
            "Remember to `return 0` from `main` when the program finishes successfully."
        )
    return paragraphs


def explain_block_theory(
    block: str,
    kind: str,
    title: str,
    project: CourseProject,
    step_index: int,
    total_blocks: int,
) -> str:
    intro = (
        f"Step {step_index} of {total_blocks} for **{project.title}**. "
        f"We now add **{title}** to the program."
    )
    paragraphs = [intro]
    if kind == "import":
        paragraphs.extend(explain_import_block(block))
    elif kind in ("enum", "struct", "trait"):
        paragraphs.extend(explain_type_def_block(block, kind, title))
    elif kind in ("fn", "main"):
        paragraphs.extend(explain_fn_block(block, title, kind == "main"))
    else:
        paragraphs.append(THEORY_FOR_KIND.get(kind, THEORY_FOR_KIND["fn"]))
    paragraphs.append(
        "Study the snippet below. It is only the new piece for this step; earlier definitions are assumed "
        "to exist above it in your file."
    )
    return join_paragraphs(paragraphs)


THEORY_FOR_KIND: dict[str, str] = {
    "import": "Imports connect your file to the standard library or other modules. The compiler resolves paths and merges exported symbols.",
    "enum": "Enums model a fixed set of named variants. They make invalid states unrepresentable and pair naturally with exhaustive match.",
    "struct": "Structs group related data into one typed value. Field names document intent and the type checker enforces shape at compile time.",
    "trait": "Traits describe behavior that types can implement. V++ uses static dispatch: the compiler knows the concrete type at compile time.",
    "fn": "Functions encapsulate reusable logic with explicit parameter and return types. V++ requires a typed signature; locals may be inferred.",
    "main": "Every executable program defines fn main() -> int. The integer return value becomes the process exit code (0 means success).",
}


@dataclass
class LessonStep:
    step_id: str
    title: str
    theory: str
    code: str


@dataclass
class CourseProject:
    num: int
    slug: str
    title: str
    level: str
    summary: str
    concepts: list[str]
    source: str
    output: str
    run_cmd: str
    steps: list[LessonStep] = field(default_factory=list)

    @property
    def page_name(self) -> str:
        return f"course-{self.slug}.html"

    @property
    def level_key(self) -> str:
        return self.level.lower()


def title_from_slug(slug: str) -> str:
    _, _, name = slug.partition("-")
    return name.replace("-", " ").title()


def parse_concepts(readme: str) -> list[str]:
    match = re.search(r"\*\*Concepts:\*\*\s*(.+)", readme)
    if not match:
        return []
    return [c.strip() for c in re.split(r"[,;]", match.group(1)) if c.strip()]


def parse_summary(readme: str) -> str:
    lines = readme.splitlines()
    for line in lines[1:]:
        text = line.strip()
        if text and not text.startswith("```") and not text.startswith("**"):
            return text
    return "Hands on V++ project walkthrough."


def block_kind(block: str) -> str:
    first = block.lstrip().splitlines()[0].strip()
    if first.startswith("import "):
        return "import"
    if first.startswith("enum "):
        return "enum"
    if first.startswith("struct "):
        return "struct"
    if first.startswith("trait "):
        return "trait"
    if first.startswith("fn main"):
        return "main"
    if first.startswith("fn "):
        return "fn"
    return "fn"


def block_title(block: str) -> str:
    kind = block_kind(block)
    first = block.lstrip().splitlines()[0].strip()
    if kind == "main":
        return "Entry point: main"
    if kind == "import":
        return "Imports"
    match = re.match(r"(?:fn|struct|enum|trait)\s+(\w+)", first)
    name = match.group(1) if match else "definition"
    if kind == "fn":
        return f"Function: {name}"
    if kind == "struct":
        return f"Struct: {name}"
    if kind == "enum":
        return f"Enum: {name}"
    if kind == "trait":
        return f"Trait: {name}"
    return name.replace("_", " ").title()


def split_vpp_blocks(source: str) -> list[str]:
    lines = source.strip().splitlines()
    blocks: list[str] = []
    current: list[str] = []
    for line in lines:
        stripped = line.lstrip()
        starts_def = stripped.startswith(("fn ", "struct ", "enum ", "trait ", "import "))
        if starts_def and current:
            blocks.append("\n".join(current).strip())
            current = [line]
        else:
            current.append(line)
    if current:
        blocks.append("\n".join(current).strip())
    return [b for b in blocks if b]


def build_lesson_steps(project: CourseProject) -> list[LessonStep]:
    steps: list[LessonStep] = [
        LessonStep(
            "overview",
            "What you will build",
            build_overview_theory(project),
            "",
        ),
        LessonStep(
            "goal",
            "Learning goals",
            build_goals_theory(project),
            "",
        ),
    ]
    blocks = split_vpp_blocks(project.source)
    total = len(blocks)
    for idx, block in enumerate(blocks, start=1):
        kind = block_kind(block)
        title = block_title(block)
        theory = explain_block_theory(block, kind, title, project, idx, total)
        steps.append(
            LessonStep(
                f"step-{idx}",
                title,
                theory,
                block,
            )
        )
    return steps


def discover_course_projects() -> list[CourseProject]:
    projects: list[CourseProject] = []
    for i in range(1, 21):
        num = i
        matches = sorted(PROJECTS.glob(f"{num:02d}-*"))
        if not matches:
            continue
        folder = matches[0]
        slug = folder.name
        readme_path = folder / "README.md"
        main_path = folder / "main.vpp"
        readme = readme_path.read_text(encoding="utf-8") if readme_path.exists() else ""
        source = main_path.read_text(encoding="utf-8") if main_path.exists() else ""
        title = title_from_slug(slug)
        if readme.startswith("#"):
            heading = readme.splitlines()[0].lstrip("# ").strip()
            if heading:
                title = re.sub(r"^\d+\s*[-–—]\s*", "", heading)
                title = re.sub(r"^\d+\s+", "", title)
        level = LEVEL_BY_NUM.get(num, "Intermediate")
        concepts = parse_concepts(readme)
        summary = parse_summary(readme)
        run_cmd = f"vpp run projects/{slug}/main.vpp"
        if slug == "14-modules":
            run_cmd = "cd projects/14-modules && vpp run main.vpp"
        output = OUTPUT_BY_SLUG.get(slug, "Program output appears in your terminal.")
        project = CourseProject(
            num=num,
            slug=slug,
            title=title,
            level=level,
            summary=summary,
            concepts=concepts,
            source=source.strip(),
            output=output,
            run_cmd=run_cmd,
        )
        project.steps = build_lesson_steps(project)
        projects.append(project)
    return projects


def code_block_html(code: str, lang: str = "vpp", filename: str = "main.vpp") -> str:
    if not code.strip():
        return ""
    plang = lang
    escaped = html.escape(code)
    return (
        '<div class="code-block-wrap">'
        f'<div class="code-block-header"><span class="code-block-filename">{html.escape(filename)}</span></div>'
        f'<pre class="language-{plang}"><code class="language-{plang}">{escaped}</code></pre>'
        "</div>"
    )
