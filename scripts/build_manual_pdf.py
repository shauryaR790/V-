#!/usr/bin/env python3
"""Generate a clean black-and-white PDF from the v++ manual markdown."""

from __future__ import annotations

import re
import sys
from pathlib import Path

try:
    from fpdf import FPDF
except ImportError:
    print("Installing fpdf2...")
    import subprocess

    subprocess.check_call([sys.executable, "-m", "pip", "install", "fpdf2", "-q"])
    from fpdf import FPDF

ROOT = Path(__file__).resolve().parent.parent
MD_PATH = ROOT / "docs" / "VPP_COMPLETE_MANUAL_v0.1.0.md"
OUT_PATH = ROOT / "docs" / "VPP_Complete_Manual_v0.1.0.pdf"

# Layout constants (millimetres)
LEFT = 22
RIGHT = 22
TOP = 32          # body text always starts here
BOTTOM = 28       # body text must stop above this from page bottom
HEADER_TEXT_Y = 9
HEADER_RULE_Y = 15
FOOTER_RULE_Y = 22   # mm from bottom edge
FOOTER_TEXT_Y = 14   # mm from bottom edge

BODY_SIZE = 10
LINE_H = 5.0


class ManualPDF(FPDF):
    def __init__(self):
        super().__init__()
        self.set_margins(LEFT, TOP, RIGHT)
        self.set_auto_page_break(auto=True, margin=BOTTOM)
        self.alias_nb_pages()

    def add_page(self, orientation: str = "", format: str = "", same: bool = False):
        super().add_page(orientation, format, same)
        # Guarantee body never starts inside the header band after any page break.
        self.set_xy(self.l_margin, self.t_margin)

    def header(self):
        if self.page_no() == 1:
            return

        self.set_font("Times", "", 8)
        self.set_xy(self.l_margin, HEADER_TEXT_Y)
        self.cell(self.epw * 0.55, 4, "v++ Complete Manual", align="L")
        self.cell(self.epw * 0.45, 4, "Version 0.1.0", align="R")

        self.set_draw_color(0, 0, 0)
        self.set_line_width(0.25)
        self.line(self.l_margin, HEADER_RULE_Y, self.w - self.r_margin, HEADER_RULE_Y)

        # Header drawing uses absolute coords; always reset cursor to body zone.
        self.set_xy(self.l_margin, self.t_margin)

    def footer(self):
        if self.page_no() == 1:
            return

        rule_y = self.h - FOOTER_RULE_Y
        text_y = self.h - FOOTER_TEXT_Y

        self.set_draw_color(0, 0, 0)
        self.set_line_width(0.25)
        self.line(self.l_margin, rule_y, self.w - self.r_margin, rule_y)

        self.set_xy(self.l_margin, text_y)
        self.set_font("Times", "", 8)
        self.cell(self.epw, 4, f"Page {self.page_no()} of {{nb}}", align="C")


def clean(text: str) -> str:
    text = text.replace("\u2014", " - ").replace("\u2013", "-")
    text = text.replace("\u2018", "'").replace("\u2019", "'")
    text = text.replace("\u201c", '"').replace("\u201d", '"')
    text = text.replace("\u2192", "->").replace("\u2190", "<-")
    text = text.replace("\u2022", "-")
    text = text.replace("\u2713", "(yes)")
    text = text.replace("\u26a0", "Note:")
    return text.encode("latin-1", "replace").decode("latin-1")


def strip_md_inline(text: str) -> str:
    text = re.sub(r"\*\*([^*]+)\*\*", r"\1", text)
    text = re.sub(r"`([^`]+)`", r"\1", text)
    text = re.sub(r"\[x\]", "(done)", text, flags=re.IGNORECASE)
    text = re.sub(r"\[ \]", "(todo)", text)
    return clean(text)


def body_cursor(pdf: FPDF):
    """Keep writes inside the body band, never in header/footer zones."""
    y = pdf.get_y()
    if y < pdf.t_margin:
        y = pdf.t_margin
    max_y = pdf.h - BOTTOM
    if y > max_y:
        y = max_y
    pdf.set_xy(pdf.l_margin, y)


def ensure_space(pdf: FPDF, needed: float = 12):
    if pdf.get_y() + needed > pdf.h - BOTTOM:
        pdf.add_page()
    body_cursor(pdf)


def write_paragraph(pdf: FPDF, text: str, size: int = BODY_SIZE, style: str = ""):
    ensure_space(pdf, LINE_H * 2)
    body_cursor(pdf)
    pdf.set_font("Times", style, size)
    pdf.multi_cell(pdf.epw, LINE_H * (size / BODY_SIZE), strip_md_inline(text))


def write_heading(pdf: FPDF, text: str, level: int):
    sizes = {1: 17, 2: 13, 3: 11.5, 4: 11}
    before = {1: 8, 2: 7, 3: 5, 4: 4}
    after = {1: 3, 2: 2, 3: 2, 4: 1}
    size = sizes.get(level, 11)
    pdf.ln(before.get(level, 4))
    ensure_space(pdf, size + 8)
    body_cursor(pdf)
    pdf.set_font("Times", "B", size)
    pdf.multi_cell(pdf.epw, size * 0.52, strip_md_inline(text))
    pdf.ln(after.get(level, 2))


def write_bullet(pdf: FPDF, text: str):
    indent = 5
    ensure_space(pdf, LINE_H)
    body_cursor(pdf)
    pdf.set_x(pdf.l_margin + indent)
    pdf.set_font("Times", "", BODY_SIZE)
    pdf.multi_cell(pdf.epw - indent, LINE_H, "- " + strip_md_inline(text.lstrip("-* ").strip()))


def write_numbered(pdf: FPDF, text: str):
    ensure_space(pdf, LINE_H)
    body_cursor(pdf)
    pdf.set_x(pdf.l_margin + 3)
    pdf.set_font("Times", "", BODY_SIZE)
    pdf.multi_cell(pdf.epw - 3, LINE_H, strip_md_inline(text))


def write_code_block(pdf: FPDF, lines: list[str]):
    if not lines:
        return
    block = "\n".join(lines)
    line_count = max(1, len(lines))
    box_h = line_count * 4.4 + 8
    ensure_space(pdf, box_h + 6)
    body_cursor(pdf)

    x = pdf.l_margin
    y = pdf.get_y()
    pdf.set_draw_color(0, 0, 0)
    pdf.set_line_width(0.2)
    pdf.rect(x, y, pdf.epw, box_h)

    pdf.set_xy(x + 4, y + 4)
    pdf.set_font("Courier", "", 8.5)
    pdf.multi_cell(pdf.epw - 8, 4.4, clean(block))
    pdf.set_y(y + box_h + 5)


def parse_table_row(line: str) -> list[str]:
    return [strip_md_inline(c.strip()) for c in line.strip().strip("|").split("|")]


def is_table_line(line: str) -> bool:
    s = line.strip()
    return s.startswith("|") and s.count("|") >= 2


def is_table_separator(line: str) -> bool:
    s = line.strip().strip("|")
    return bool(s) and all(re.fullmatch(r":?-{1,}:?", part.strip()) for part in s.split("|"))


def write_table(pdf: FPDF, rows: list[list[str]]):
    if not rows:
        return

    cols = len(rows[0])
    col_widths = [0.0] * cols
    pdf.set_font("Times", "", 9)
    for row in rows:
        for i, cell in enumerate(row):
            if i < cols:
                col_widths[i] = max(col_widths[i], pdf.get_string_width(cell) + 8)

    total = sum(col_widths)
    if total > pdf.epw:
        scale = pdf.epw / total
        col_widths = [w * scale for w in col_widths]
    elif total < pdf.epw:
        extra = (pdf.epw - total) / cols
        col_widths = [w + extra for w in col_widths]

    row_h = 7.0
    ensure_space(pdf, row_h * min(len(rows), 3))

    pdf.set_draw_color(0, 0, 0)
    pdf.set_line_width(0.2)

    for r_idx, row in enumerate(rows):
        x_start = pdf.l_margin
        y_start = pdf.get_y()
        if y_start + row_h > pdf.h - BOTTOM:
            pdf.add_page()
            body_cursor(pdf)
            y_start = pdf.get_y()

        for i in range(cols):
            cell = row[i] if i < len(row) else ""
            x = x_start + sum(col_widths[:i])
            w = col_widths[i]
            pdf.rect(x, y_start, w, row_h)
            pdf.set_xy(x + 3, y_start + 2)
            pdf.set_font("Times", "B" if r_idx == 0 else "", 9)
            pdf.cell(w - 6, row_h - 3, cell, align="L")

        pdf.set_y(y_start + row_h)

    pdf.ln(5)


def render_markdown(md: str, pdf: ManualPDF):
    lines = md.splitlines()
    i = 0
    in_code = False
    code_lines: list[str] = []
    table_rows: list[list[str]] = []

    while i < len(lines):
        line = lines[i].rstrip()

        if line.strip().startswith("```"):
            if in_code:
                write_code_block(pdf, code_lines)
                code_lines = []
                in_code = False
            else:
                in_code = True
            i += 1
            continue

        if in_code:
            code_lines.append(line)
            i += 1
            continue

        if is_table_line(line):
            if not is_table_separator(line):
                table_rows.append(parse_table_row(line))
            i += 1
            while i < len(lines) and is_table_line(lines[i].rstrip()):
                row_line = lines[i].rstrip()
                if not is_table_separator(row_line):
                    table_rows.append(parse_table_row(row_line))
                i += 1
            write_table(pdf, table_rows)
            table_rows = []
            continue

        if not line.strip():
            pdf.ln(3)
            i += 1
            continue

        if line.startswith("# "):
            write_heading(pdf, line[2:].strip(), 1)
            i += 1
            continue

        if line.startswith("## "):
            write_heading(pdf, line[3:].strip(), 2)
            i += 1
            continue

        if line.startswith("### "):
            write_heading(pdf, line[4:].strip(), 3)
            i += 1
            continue

        if line.startswith("#### "):
            write_heading(pdf, line[5:].strip(), 4)
            i += 1
            continue

        if line.startswith("---"):
            pdf.ln(3)
            y = pdf.get_y()
            pdf.set_draw_color(0, 0, 0)
            pdf.set_line_width(0.2)
            pdf.line(pdf.l_margin, y, pdf.l_margin + pdf.epw, y)
            pdf.ln(5)
            i += 1
            continue

        if line.startswith("- ") or line.startswith("* "):
            write_bullet(pdf, line[2:])
            i += 1
            continue

        if re.match(r"^\d+\.\s", line):
            write_numbered(pdf, line)
            i += 1
            continue

        write_paragraph(pdf, line)
        i += 1


def draw_title_page(pdf: ManualPDF):
    pdf.add_page()

    mid = pdf.h / 2
    pdf.set_y(mid - 35)

    pdf.set_font("Times", "B", 28)
    pdf.cell(pdf.epw, 12, "v++ Programming Language", align="C", new_x="LMARGIN", new_y="NEXT")
    pdf.ln(8)

    pdf.set_font("Times", "", 15)
    pdf.cell(pdf.epw, 9, "Complete Technical Manual", align="C", new_x="LMARGIN", new_y="NEXT")
    pdf.ln(12)

    pdf.set_font("Times", "B", 12)
    pdf.cell(pdf.epw, 7, "Version 0.1.0", align="C", new_x="LMARGIN", new_y="NEXT")
    pdf.ln(10)

    pdf.set_font("Times", "", 10.5)
    pdf.multi_cell(
        pdf.epw,
        5.5,
        "Language reference, compiler architecture, standard library, "
        "tooling, and build guide for the v0.1.0 release.",
        align="C",
    )
    pdf.ln(18)

    pdf.set_font("Times", "I", 9.5)
    pdf.cell(pdf.epw, 5, "github.com/shauryaR790/V-", align="C", new_x="LMARGIN", new_y="NEXT")
    pdf.ln(2)
    pdf.cell(pdf.epw, 5, "MIT License  |  August 2026", align="C")


def main():
    if not MD_PATH.exists():
        print(f"Missing manual: {MD_PATH}")
        sys.exit(1)

    md = MD_PATH.read_text(encoding="utf-8")
    pdf = ManualPDF()

    draw_title_page(pdf)
    pdf.add_page()
    render_markdown(md, pdf)

    OUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    pdf.output(str(OUT_PATH))
    print(f"Wrote {OUT_PATH}")
    print(f"Pages: {pdf.page_no()}")


if __name__ == "__main__":
    main()
