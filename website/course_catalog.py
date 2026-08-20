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
    concepts = ", ".join(project.concepts) if project.concepts else "core V++ syntax"
    steps: list[LessonStep] = [
        LessonStep(
            "overview",
            "What you will build",
            (
                f"{project.summary} By the end you will understand {concepts}. "
                f"We build the program in layers, run snippets mentally, then execute the full source."
            ),
            "",
        ),
        LessonStep(
            "goal",
            "Learning goals",
            (
                f"Level: {project.level}. "
                "Read each step before copying code. Types are enforced at compile time, "
                "so fix signature errors early with vpp check."
            ),
            "",
        ),
    ]
    blocks = split_vpp_blocks(project.source)
    accumulated: list[str] = []
    for idx, block in enumerate(blocks, start=1):
        accumulated.append(block)
        kind = block_kind(block)
        theory = THEORY_FOR_KIND.get(kind, THEORY_FOR_KIND["fn"])
        steps.append(
            LessonStep(
                f"step-{idx}",
                block_title(block),
                f"{theory} In this step we add {block_title(block).lower()} to the growing program.",
                "\n\n".join(accumulated),
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
