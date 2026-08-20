"""Parse the course curriculum PDF into structured docs page content."""

from __future__ import annotations

import json
import re
from dataclasses import asdict, dataclass, field
from pathlib import Path

from pdf_curriculum import (
    CurriculumProject,
    HEADER_RE,
    load_curriculum,
    load_curriculum_from_pdf,
)

ROOT = Path(__file__).resolve().parent.parent
WEBSITE = Path(__file__).resolve().parent
PDF_CANDIDATES = (
    ROOT / "course.pdf",
    ROOT / "vpp_20_course_deep_curriculum.pdf",
)
JSON_PATH = WEBSITE / "docs.json"
CURRICULUM_JSON = WEBSITE / "curriculum.json"

LEVELS = {"Beginner", "Intermediate", "Advanced"}


@dataclass
class CurriculumMapRow:
    num: int
    course: str
    level: str
    ideas: str


@dataclass
class ConventionItem:
    label: str
    text: str


@dataclass
class DocsDocument:
    title: str
    intro_paragraphs: list[str]
    map_intro: str
    map_rows: list[CurriculumMapRow]
    teaching_rule: str
    conventions: list[ConventionItem]
    projects: list[CurriculumProject]
    implementation_intro: str
    implementation_items: list[str]
    consistency_intro: str
    consistency_items: list[str]
    end_state_paragraphs: list[str]
    footer: str = ""


def _find_pdf() -> Path | None:
    for path in PDF_CANDIDATES:
        if path.exists():
            return path
    return None


def _pdf_text() -> str:
    from pypdf import PdfReader

    pdf_path = _find_pdf()
    if pdf_path is None:
        raise FileNotFoundError("No curriculum PDF found (course.pdf or vpp_20_course_deep_curriculum.pdf).")
    reader = PdfReader(str(pdf_path))
    raw = "\n".join((page.extract_text() or "") for page in reader.pages)
    raw = HEADER_RE.sub("", raw)
    raw = re.sub(r"\n{3,}", "\n\n", raw)
    return raw.strip()


def _merge_lines(lines: list[str]) -> str:
    return " ".join(part.strip() for part in lines if part.strip())


def _parse_intro(lines: list[str]) -> tuple[str, list[str]]:
    title = "Twenty Course Deep Curriculum"
    paragraphs: list[str] = []
    i = 0
    if i < len(lines) and lines[i].strip().lower() == "v++":
        i += 1
    if i < len(lines) and "twenty course deep curriculum" in lines[i].lower():
        title = lines[i].strip()
        i += 1
    while i < len(lines):
        line = lines[i].strip()
        if line == "Curriculum Map":
            break
        if line:
            paragraphs.append(line)
        i += 1
    return title, paragraphs


def _parse_map(lines: list[str], start: int) -> tuple[str, list[CurriculumMapRow], str, int]:
    map_intro_parts: list[str] = []
    rows: list[CurriculumMapRow] = []
    teaching_rule = ""
    i = start
    if i < len(lines) and lines[i].strip() == "Curriculum Map":
        i += 1
    while i < len(lines):
        line = lines[i].strip()
        if line.startswith("Teaching rule for every page:"):
            teaching_parts = [line]
            i += 1
            while i < len(lines):
                nxt = lines[i].strip()
                if nxt.startswith("Global v++"):
                    break
                if nxt:
                    teaching_parts.append(nxt)
                i += 1
            teaching_rule = _merge_lines(teaching_parts)
            continue
        if line in ("Project", "Course", "Level", "Primary ideas"):
            i += 1
            continue
        if re.fullmatch(r"\d+", line) and int(line) <= 20:
            num = int(line)
            i += 1
            course = lines[i].strip()
            i += 1
            level = lines[i].strip()
            i += 1
            ideas = lines[i].strip()
            i += 1
            rows.append(CurriculumMapRow(num=num, course=course, level=level, ideas=ideas))
            continue
        if line:
            map_intro_parts.append(line)
        i += 1
    return _merge_lines(map_intro_parts), rows, teaching_rule, i


def _parse_conventions(lines: list[str], start: int) -> tuple[list[ConventionItem], int]:
    items: list[ConventionItem] = []
    i = start
    if i < len(lines) and lines[i].startswith("Global v++ Conventions"):
        i += 1
    current_label = ""
    current_parts: list[str] = []
    for j in range(i, len(lines)):
        line = lines[j].strip()
        if line.startswith("Project "):
            i = j
            break
        match = re.match(r"^([A-Z][A-Za-z ]+): (.+)$", line)
        if match:
            if current_label:
                items.append(ConventionItem(current_label, _merge_lines(current_parts)))
            current_label = match.group(1).strip()
            current_parts = [match.group(2).strip()]
        elif line and current_label:
            current_parts.append(line)
    else:
        i = len(lines)
    if current_label:
        items.append(ConventionItem(current_label, _merge_lines(current_parts)))
    return items, i


def _parse_numbered_items(lines: list[str], start: int, stop_prefixes: tuple[str, ...]) -> tuple[list[str], int]:
    items: list[str] = []
    i = start
    while i < len(lines):
        line = lines[i].strip()
        if any(line.startswith(prefix) for prefix in stop_prefixes):
            break
        match = re.match(r"^(\d+)\.\s*(.+)$", line)
        if match:
            item_parts = [match.group(2).strip()]
            i += 1
            while i < len(lines):
                nxt = lines[i].strip()
                if not nxt:
                    i += 1
                    break
                if re.match(r"^\d+\.\s", nxt) or any(nxt.startswith(prefix) for prefix in stop_prefixes):
                    break
                item_parts.append(nxt)
                i += 1
            items.append(_merge_lines(item_parts))
            continue
        if line:
            break
        i += 1
    return items, i


def _parse_epilogue(text: str) -> dict[str, object]:
    idx = text.rfind("Implementation Notes for Cursor")
    if idx < 0:
        return {
            "implementation_intro": "",
            "implementation_items": [],
            "consistency_intro": "",
            "consistency_items": [],
            "end_state_paragraphs": [],
            "footer": "",
        }
    body = text[idx:]
    lines = body.splitlines()
    implementation_intro = ""
    consistency_intro = ""
    end_state: list[str] = []
    footer = ""
    i = 0
    if lines[i].startswith("Implementation Notes"):
        i += 1
    intro_parts: list[str] = []
    while i < len(lines):
        line = lines[i].strip()
        if line.startswith("1. Every project"):
            break
        if line:
            intro_parts.append(line)
        i += 1
    implementation_intro = _merge_lines(intro_parts)
    implementation_items, i = _parse_numbered_items(lines, i, ("Documentation Consistency Checks",))
    if i < len(lines) and lines[i].strip().startswith("Documentation Consistency Checks"):
        i += 1
    consistency_parts: list[str] = []
    while i < len(lines):
        line = lines[i].strip()
        if line.startswith("1. Project"):
            break
        if line:
            consistency_parts.append(line)
        i += 1
    consistency_intro = _merge_lines(consistency_parts)
    consistency_items, i = _parse_numbered_items(lines, i, ("End State",))
    if i < len(lines) and lines[i].strip().startswith("End State"):
        i += 1
    while i < len(lines):
        line = lines[i].strip()
        if line.startswith("Prepared from"):
            footer = line
            break
        if line:
            end_state.append(line)
        i += 1
    return {
        "implementation_intro": implementation_intro,
        "implementation_items": implementation_items,
        "consistency_intro": consistency_intro,
        "consistency_items": consistency_items,
        "end_state_paragraphs": end_state,
        "footer": footer,
    }


def load_docs_from_pdf() -> DocsDocument:
    text = _pdf_text()
    preamble, projects_text = re.split(r"(?=Project 01: )", text, maxsplit=1)
    preamble_lines = [line.strip() for line in preamble.splitlines()]
    title, intro = _parse_intro(preamble_lines)
    map_start = next(i for i, line in enumerate(preamble_lines) if line == "Curriculum Map")
    map_intro, map_rows, teaching_rule, conv_start = _parse_map(preamble_lines, map_start)
    conventions, _ = _parse_conventions(preamble_lines, conv_start)
    projects = load_curriculum_from_pdf()

    epilogue = _parse_epilogue(text)
    return DocsDocument(
        title=title,
        intro_paragraphs=intro,
        map_intro=map_intro,
        map_rows=map_rows,
        teaching_rule=teaching_rule,
        conventions=conventions,
        projects=projects,
        **epilogue,
    )


def export_docs_json(path: Path = JSON_PATH) -> None:
    doc = load_docs_from_pdf()
    payload = {
        "title": doc.title,
        "intro_paragraphs": doc.intro_paragraphs,
        "map_intro": doc.map_intro,
        "map_rows": [asdict(row) for row in doc.map_rows],
        "teaching_rule": doc.teaching_rule,
        "conventions": [asdict(item) for item in doc.conventions],
        "projects": [asdict(project) for project in doc.projects],
        "implementation_intro": doc.implementation_intro,
        "implementation_items": doc.implementation_items,
        "consistency_intro": doc.consistency_intro,
        "consistency_items": doc.consistency_items,
        "end_state_paragraphs": doc.end_state_paragraphs,
        "footer": doc.footer,
    }
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def load_docs_from_json(path: Path = JSON_PATH) -> DocsDocument:
    from pdf_curriculum import CurriculumSection

    raw = json.loads(path.read_text(encoding="utf-8"))

    projects = []
    for item in raw["projects"]:
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
    return DocsDocument(
        title=raw["title"],
        intro_paragraphs=raw["intro_paragraphs"],
        map_intro=raw["map_intro"],
        map_rows=[CurriculumMapRow(**row) for row in raw["map_rows"]],
        teaching_rule=raw["teaching_rule"],
        conventions=[ConventionItem(**item) for item in raw["conventions"]],
        projects=projects,
        implementation_intro=raw["implementation_intro"],
        implementation_items=raw["implementation_items"],
        consistency_intro=raw["consistency_intro"],
        consistency_items=raw["consistency_items"],
        end_state_paragraphs=raw["end_state_paragraphs"],
        footer=raw.get("footer", ""),
    )


def load_docs() -> DocsDocument:
    """Load docs page content (JSON in CI, PDF when refreshing)."""
    if JSON_PATH.exists():
        return load_docs_from_json()
    if _find_pdf() is not None:
        doc = load_docs_from_pdf()
        export_docs_json()
        return doc
    if CURRICULUM_JSON.exists():
        projects = load_curriculum()
        return DocsDocument(
            title="Twenty Course Deep Curriculum",
            intro_paragraphs=[],
            map_intro="",
            map_rows=[],
            teaching_rule="",
            conventions=[],
            projects=projects,
            implementation_intro="",
            implementation_items=[],
            consistency_intro="",
            consistency_items=[],
            end_state_paragraphs=[],
        )
    raise FileNotFoundError(
        f"Missing docs data. Add {JSON_PATH.name} or a curriculum PDF to the repository."
    )


if __name__ == "__main__":
    export_docs_json()
    print(f"Wrote {JSON_PATH}")
