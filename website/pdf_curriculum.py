"""Parse vpp_20_course_deep_curriculum.pdf into structured course content."""

from __future__ import annotations

import json
import re
from dataclasses import asdict, dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
WEBSITE = Path(__file__).resolve().parent
PDF_PATH = ROOT / "vpp_20_course_deep_curriculum.pdf"
JSON_PATH = WEBSITE / "curriculum.json"

HEADER_RE = re.compile(r"v\+\+ 20 Course Deep Curriculum\s+\d+\s*", re.I)
PROJECT_SPLIT_RE = re.compile(r"(?=Project \d{2}: )")
PROJECT_HEAD_RE = re.compile(r"^Project (\d{2}): (.+)$", re.M)
STEP_HEAD_RE = re.compile(r"^Step (\d+): (.+)$", re.M)

MARKERS = [
    "Level:",
    "Project goal:",
    "Core concepts",
    "How the learner should approach this project",
    "Code for this step",
    "After typing the snippet",
    "Before moving forward",
    "Complete source",
    "Expected behavior",
    "Run the project",
    "What the learner should understand after this project",
    "Common mistakes to teach",
    "Practice extension",
    "A strong learner should be able to explain the program",
]

CODE_STARTERS = (
    "fn ",
    "struct ",
    "enum ",
    "trait ",
    "import ",
    "impl ",
    "let ",
    "print(",
    "return ",
    "if ",
    "while ",
    "for ",
    "match ",
    "pub ",
    "}",
    "{",
)


@dataclass
class CurriculumSection:
    section_id: str
    title: str
    paragraphs: list[str] = field(default_factory=list)
    code: str = ""
    list_items: list[str] = field(default_factory=list)


@dataclass
class CurriculumProject:
    num: int
    title: str
    goal: str
    sections: list[CurriculumSection]
    complete_source: str
    expected_output: str
    run_cmd: str


def _pdf_text() -> str:
    from pypdf import PdfReader

    reader = PdfReader(str(PDF_PATH))
    raw = "\n".join((page.extract_text() or "") for page in reader.pages)
    raw = HEADER_RE.sub("", raw)
    raw = re.sub(r"\n{3,}", "\n\n", raw)
    return raw.strip()


def _is_code_line(line: str) -> bool:
    stripped = line.strip()
    if not stripped:
        return False
    if stripped in ("{", "}"):
        return True
    if re.match(r"\w+\s*:", stripped) and not stripped.startswith("Step "):
        return True
    return stripped.startswith(CODE_STARTERS)


def _extract_code_block(lines: list[str], start: int) -> tuple[str, int]:
    collected: list[str] = []
    i = start
    in_string = False
    quote_char = ""

    def update_string_state(text: str) -> None:
        nonlocal in_string, quote_char
        for ch in text:
            if in_string:
                if ch == quote_char:
                    in_string = False
                    quote_char = ""
            elif ch in ('"', "'"):
                in_string = True
                quote_char = ch

    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        if not stripped:
            if collected:
                if in_string:
                    collected.append("")
                    i += 1
                    continue
                break
            i += 1
            continue

        if collected:
            continuing_string = in_string
            update_string_state(line)
            if (
                continuing_string
                or in_string
                or _is_code_line(line)
                or line.startswith(("    ", "\t"))
                or stripped in ("{", "}")
            ):
                collected.append(line.rstrip())
                i += 1
                continue
            break

        if _is_code_line(line):
            collected.append(line.rstrip())
            update_string_state(line)
            i += 1
            continue

        i += 1

    return "\n".join(collected).strip(), i


def _split_marker(line: str) -> tuple[str, str] | None:
    for marker in MARKERS:
        if line.startswith(marker):
            rest = line[len(marker) :].strip()
            return marker.rstrip(":"), rest
        if line == marker.rstrip(":"):
            return marker.rstrip(":"), ""
    step = STEP_HEAD_RE.match(line)
    if step:
        return f"step-{step.group(1)}", f"Step {step.group(1)}: {step.group(2)}"
    return None


def _slugify_title(title: str) -> str:
    return re.sub(r"[^a-z0-9]+", "-", title.lower()).strip("-")


def _should_merge_paragraph(previous: str, nxt: str) -> bool:
    if not previous or not nxt:
        return False
    if nxt.startswith(("Step ", "Code for", "Complete source", "Expected behavior", "Run the project")):
        return False
    if re.match(r"^\d+\.\s", nxt):
        return False
    if previous.endswith((".", "!", "?", ":", ";")):
        return False
    return True


def _parse_project(num: int, title: str, body: str) -> CurriculumProject:
    lines = body.splitlines()
    goal = ""
    sections: list[CurriculumSection] = []
    complete_source = ""
    expected_output = ""
    run_cmd = ""

    i = 0
    current: CurriculumSection | None = None
    pending_list: list[str] | None = None
    expected_intro_done = False

    def flush() -> None:
        nonlocal current, pending_list
        if current is not None:
            if pending_list:
                current.list_items = pending_list
                pending_list = None
            sections.append(current)
            current = None

    def add_paragraph(text: str) -> None:
        if current is None:
            return
        text = text.strip()
        if not text:
            return
        if current.paragraphs and _should_merge_paragraph(current.paragraphs[-1], text):
            current.paragraphs[-1] = current.paragraphs[-1] + " " + text
        else:
            current.paragraphs.append(text)

    def start_section(section_id: str, heading: str, lead: str = "") -> None:
        nonlocal current, pending_list, expected_intro_done
        flush()
        paras = [lead.strip()] if lead and lead.strip() else []
        current = CurriculumSection(section_id=section_id, title=heading, paragraphs=paras)
        pending_list = None
        expected_intro_done = section_id != "expected-behavior"

    while i < len(lines):
        line = lines[i].strip()
        if not line:
            i += 1
            continue

        if line.startswith("Level:"):
            i += 1
            continue

        if line.startswith("Project goal:"):
            goal_parts = [line[len("Project goal:") :].strip()]
            i += 1
            while i < len(lines):
                nxt = lines[i].strip()
                if not nxt:
                    i += 1
                    break
                if nxt.startswith("Core concepts") or _split_marker(nxt):
                    break
                goal_parts.append(nxt)
                i += 1
            goal = " ".join(part for part in goal_parts if part)
            continue

        if line.startswith("After typing the snippet") or line.startswith("Before moving forward"):
            add_paragraph(line)
            i += 1
            continue

        marker = _split_marker(line)
        if marker:
            key, rest = marker
            if key.startswith("step-"):
                start_section(key, rest)
                i += 1
                continue
            if key == "Core concepts":
                start_section("core-concepts", "Core concepts", rest)
                i += 1
                continue
            if key == "How the learner should approach this project":
                start_section(
                    "approach",
                    "How the learner should approach this project",
                    rest,
                )
                i += 1
                continue
            if key == "Code for this step":
                code, i = _extract_code_block(lines, i + 1)
                if current:
                    current.code = code
                continue
            if key == "Complete source":
                code, i = _extract_code_block(lines, i + 1)
                complete_source = code
                start_section("complete-source", "Complete source")
                if current:
                    current.code = code
                continue
            if key == "Expected behavior":
                start_section("expected-behavior", "Expected behavior", rest)
                i += 1
                continue
            if key == "Run the project":
                start_section("run-the-project", "Run the project")
                i += 1
                if i < len(lines) and lines[i].strip().startswith("vpp "):
                    run_cmd = lines[i].strip()
                    if current:
                        current.code = run_cmd
                    i += 1
                continue
            if key == "What the learner should understand after this project":
                start_section("understanding", "What the learner should understand after this project", rest)
                i += 1
                continue
            if key == "Common mistakes to teach":
                start_section("common-mistakes", "Common mistakes to teach", rest)
                pending_list = []
                i += 1
                continue
            if key == "Practice extension":
                start_section("practice", "Practice extension", rest)
                i += 1
                continue
            if key.startswith("After typing") or key.startswith("Before moving"):
                add_paragraph(line)
                i += 1
                continue
            if key.startswith("A strong learner"):
                add_paragraph(line)
                i += 1
                continue

        if pending_list is not None and re.match(r"^\d+\.\s", line):
            pending_list.append(re.sub(r"^\d+\.\s*", "", line))
            i += 1
            continue

        if current and current.section_id == "expected-behavior":
            if line.startswith("Run the project"):
                start_section("run-the-project", "Run the project")
                i += 1
                if i < len(lines) and lines[i].strip().startswith("vpp "):
                    run_cmd = lines[i].strip()
                    if current:
                        current.code = run_cmd
                    i += 1
                continue
            if not expected_intro_done:
                add_paragraph(line)
                if "runtime." in line:
                    expected_intro_done = True
                i += 1
                continue
            expected_output += (expected_output and "\n" or "") + line
            i += 1
            continue

        if current:
            add_paragraph(line)
        i += 1

    flush()

    if not run_cmd:
        run_cmd = f"vpp run projects/{num:02d}-{_slugify_title(title)}/main.vpp"

    return CurriculumProject(
        num=num,
        title=title,
        goal=goal,
        sections=sections,
        complete_source=complete_source,
        expected_output=expected_output.strip(),
        run_cmd=run_cmd,
    )


def load_curriculum_from_pdf() -> list[CurriculumProject]:
    text = _pdf_text()
    chunks = PROJECT_SPLIT_RE.split(text)
    projects: list[CurriculumProject] = []
    for chunk in chunks:
        chunk = chunk.strip()
        if not chunk.startswith("Project "):
            continue
        head = PROJECT_HEAD_RE.match(chunk)
        if not head:
            continue
        num = int(head.group(1))
        title = head.group(2).strip()
        body = chunk[head.end() :].strip()
        projects.append(_parse_project(num, title, body))
    projects.sort(key=lambda p: p.num)
    return projects


def export_curriculum_json(path: Path = JSON_PATH) -> None:
    projects = load_curriculum_from_pdf()
    payload = [asdict(project) for project in projects]
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def load_curriculum_from_json(path: Path = JSON_PATH) -> list[CurriculumProject]:
    raw = json.loads(path.read_text(encoding="utf-8"))
    projects: list[CurriculumProject] = []
    for item in raw:
        sections = [
            CurriculumSection(
                section_id=section["section_id"],
                title=section["title"],
                paragraphs=section.get("paragraphs", []),
                code=section.get("code", ""),
                list_items=section.get("list_items", []),
            )
            for section in item["sections"]
        ]
        projects.append(
            CurriculumProject(
                num=item["num"],
                title=item["title"],
                goal=item["goal"],
                sections=sections,
                complete_source=item["complete_source"],
                expected_output=item["expected_output"],
                run_cmd=item["run_cmd"],
            )
        )
    projects.sort(key=lambda p: p.num)
    return projects


def load_curriculum() -> list[CurriculumProject]:
    """Load curriculum for site generation (JSON in CI, PDF when refreshing content)."""
    if JSON_PATH.exists():
        return load_curriculum_from_json()
    if PDF_PATH.exists():
        projects = load_curriculum_from_pdf()
        export_curriculum_json()
        return projects
    raise FileNotFoundError(
        f"Missing curriculum data. Add {JSON_PATH.name} or {PDF_PATH.name} to the repository."
    )


if __name__ == "__main__":
    export_curriculum_json()
    print(f"Wrote {JSON_PATH}")
