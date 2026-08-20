"""Course project metadata loaded from the official PDF curriculum."""

from __future__ import annotations

import html
import re
from dataclasses import dataclass, field
from datetime import date, timedelta
from pathlib import Path

from pdf_curriculum import CurriculumSection, load_curriculum

ROOT = Path(__file__).resolve().parent.parent
PROJECTS = ROOT / "projects"

LEVEL_BY_NUM: dict[int, str] = {
    **{i: "Beginner" for i in range(1, 6)},
    **{i: "Beginner" for i in (10,)},
    **{i: "Intermediate" for i in range(6, 12)},
    **{i: "Intermediate" for i in (14, 15, 16, 17)},
    **{i: "Advanced" for i in (12, 13, 18, 19, 20)},
}

COURSE_PUBLISH_START = date(2026, 4, 1)


def course_publish_date(num: int) -> str:
    published = COURSE_PUBLISH_START + timedelta(days=7 * (num - 1))
    return f"{published.strftime('%b')} {published.day}, {published.year}"


@dataclass
class CourseProject:
    num: int
    slug: str
    title: str
    level: str
    summary: str
    source: str
    output: str
    run_cmd: str
    sections: list[CurriculumSection] = field(default_factory=list)

    @property
    def page_name(self) -> str:
        return f"course-{self.slug}.html"

    @property
    def level_key(self) -> str:
        return self.level.lower()

    @property
    def published_label(self) -> str:
        return course_publish_date(self.num)


def title_from_slug(slug: str) -> str:
    _, _, name = slug.partition("-")
    return name.replace("-", " ").title()


def discover_course_projects() -> list[CourseProject]:
    pdf_by_num = {p.num: p for p in load_curriculum()}
    projects: list[CourseProject] = []
    for i in range(1, 21):
        num = i
        matches = sorted(PROJECTS.glob(f"{num:02d}-*"))
        if not matches:
            continue
        folder = matches[0]
        slug = folder.name
        pdf = pdf_by_num.get(num)
        if not pdf:
            continue
        level = LEVEL_BY_NUM.get(num, "Intermediate")
        projects.append(
            CourseProject(
                num=num,
                slug=slug,
                title=pdf.title,
                level=level,
                summary=pdf.goal,
                source=pdf.complete_source,
                output=pdf.expected_output,
                run_cmd=pdf.run_cmd,
                sections=pdf.sections,
            )
        )
    return projects


def code_block_html(
    code: str,
    lang: str = "vpp",
    filename: str = "main.vpp",
    wrap_class: str = "",
    code_class: str = "",
) -> str:
    if not code.strip():
        return ""
    if code.strip().startswith("vpp "):
        lang = "bash"
        filename = "terminal"
    escaped = html.escape(code)
    wrap_cls = "code-block-wrap"
    if wrap_class:
        wrap_cls += f" {wrap_class}"
    code_cls = f"language-{lang}"
    if code_class:
        code_cls += f" {code_class}"
    return (
        f'<div class="{wrap_cls}">'
        f'<div class="code-block-header"><span class="code-block-filename">{html.escape(filename)}</span></div>'
        f'<pre class="language-{lang}"><code class="{code_cls}">{escaped}</code></pre>'
        "</div>"
    )
