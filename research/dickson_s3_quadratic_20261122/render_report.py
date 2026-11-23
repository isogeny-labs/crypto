#!/usr/bin/env python3
"""Lay out the existing Markdown report and SVG; do no research computation.

This renderer reads already-written prose, equations, tables, and diagram labels.
It never invokes a solver, constructs a field, verifies a mathematical claim,
calculates a scientific result, or changes REPORT.md. Hashes below identify the
layout inputs/outputs only. Inkscape and Poppler are document-rendering helpers.

Example, from the repository root:
  python3 research/dickson_s3_quadratic_20261122/render_report.py --draft --render-pages

After the report author records actual results, rerun without --draft and inspect
every new page. The script's layout receipt is not a correctness-test receipt.
"""

from __future__ import annotations

import argparse
import hashlib
import html
import json
import posixpath
import re
import shutil
import subprocess
from pathlib import Path
from xml.etree import ElementTree

import fitz
from reportlab import Version as REPORTLAB_VERSION
from reportlab.lib import colors
from reportlab.lib.enums import TA_LEFT
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import ParagraphStyle
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (
    Flowable,
    KeepTogether,
    LongTable,
    Paragraph,
    SimpleDocTemplate,
    Spacer,
    Table,
    TableStyle,
)


HERE = Path(__file__).resolve().parent
PAGE_W, PAGE_H = A4
MARGIN_X = 47
MARGIN_TOP = 48
MARGIN_BOTTOM = 48
TEXT_W = PAGE_W - 2 * MARGIN_X
INK = colors.HexColor("#182738")
MUTED = colors.HexColor("#546477")
TEAL = colors.HexColor("#1b756c")
RULE = colors.HexColor("#d4dfe5")
TINT = colors.HexColor("#edf5f3")
FONT_ROOT = Path("/usr/share/fonts/truetype/dejavu")
FONT_FILES = {
    "ReportSans": "DejaVuSans.ttf",
    "ReportSans-Bold": "DejaVuSans-Bold.ttf",
    "ReportSans-Italic": "DejaVuSans-Oblique.ttf",
    "ReportSans-BoldItalic": "DejaVuSans-BoldOblique.ttf",
    "ReportMono": "DejaVuSansMono.ttf",
    "ReportMono-Bold": "DejaVuSansMono-Bold.ttf",
}


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def typography(text: str) -> str:
    """Normalize dash/space typography without interpreting any equation."""
    return text.translate(str.maketrans({
        "\u2010": "-", "\u2011": "-", "\u2012": "-", "\u2013": "-",
        "\u2014": " - ", "\u2212": "-", "\u00a0": " ", "\u202f": " ",
    }))


def register_fonts() -> None:
    for name, filename in FONT_FILES.items():
        path = FONT_ROOT / filename
        if not path.is_file():
            raise SystemExit(f"Missing embedded font: {path}")
        pdfmetrics.registerFont(TTFont(name, str(path)))
    pdfmetrics.registerFontFamily(
        "ReportSans", normal="ReportSans", bold="ReportSans-Bold",
        italic="ReportSans-Italic", boldItalic="ReportSans-BoldItalic",
    )
    pdfmetrics.registerFontFamily(
        "ReportMono", normal="ReportMono", bold="ReportMono-Bold",
        italic="ReportMono", boldItalic="ReportMono-Bold",
    )


def make_styles() -> dict[str, ParagraphStyle]:
    body = ParagraphStyle(
        "Body", fontName="ReportSans", fontSize=9.6, leading=14.0,
        textColor=INK, spaceAfter=7.5, splitLongWords=True,
        allowWidows=0, allowOrphans=0, alignment=TA_LEFT,
    )
    return {
        "body": body,
        "title": ParagraphStyle(
            "Title", parent=body, fontName="ReportSans-Bold", fontSize=23,
            leading=28, spaceAfter=10, keepWithNext=True,
        ),
        "subtitle": ParagraphStyle(
            "Subtitle", parent=body, fontSize=10.2, leading=14.8,
            textColor=MUTED, spaceAfter=7, keepWithNext=True,
        ),
        "meta": ParagraphStyle(
            "Metadata", parent=body, fontSize=8.3, leading=11.8,
            textColor=MUTED, spaceAfter=11, keepWithNext=True,
        ),
        "h2": ParagraphStyle(
            "Heading2", parent=body, fontName="ReportSans-Bold", fontSize=13.2,
            leading=18, textColor=TEAL, spaceBefore=11.5, spaceAfter=6.5,
            keepWithNext=True,
        ),
        "h3": ParagraphStyle(
            "Heading3", parent=body, fontName="ReportSans-Bold", fontSize=11,
            leading=15, textColor=TEAL, spaceBefore=9, spaceAfter=5,
            keepWithNext=True,
        ),
        "caption": ParagraphStyle(
            "Caption", parent=body, fontSize=8.2, leading=11.5,
            textColor=MUTED, spaceBefore=4, spaceAfter=10,
        ),
        "table": ParagraphStyle(
            "TableCell", parent=body, fontSize=8.6, leading=12.2, spaceAfter=0,
            allowWidows=1, allowOrphans=1,
        ),
        "table_head": ParagraphStyle(
            "TableHeader", parent=body, fontName="ReportSans-Bold",
            fontSize=8.5, leading=12, textColor=colors.white, spaceAfter=0,
            allowWidows=1, allowOrphans=1,
        ),
        "code": ParagraphStyle(
            "Code", parent=body, fontName="ReportMono", fontSize=8.6,
            leading=13, spaceAfter=0, allowWidows=1, allowOrphans=1,
        ),
        "bullet": ParagraphStyle(
            "Bullet", parent=body, leftIndent=13, firstLineIndent=0,
            bulletIndent=1.5, spaceAfter=5,
        ),
    }


class Inline:
    """Translate a small Markdown inline vocabulary to ReportLab markup."""

    def __init__(self, repository_ref: str):
        self.repository_ref = repository_ref
        self.links: dict[str, str] = {}

    def href(self, destination: str) -> str:
        if re.match(r"^(https?://|mailto:|#)", destination):
            return destination
        relative = posixpath.normpath(
            "research/dickson_s3_quadratic_20261122/" + destination
        )
        return f"https://github.com/isogeny-labs/crypto/blob/{self.repository_ref}/{relative}"

    def __call__(self, text: str, *, code_size: float = 8.65) -> str:
        text = typography(text)
        protected: list[str] = []

        def reserve(markup: str) -> str:
            protected.append(markup)
            return f"\x01{len(protected) - 1}\x02"

        def link(match: re.Match) -> str:
            label, destination = match.groups()
            target = self.href(destination)
            self.links[target] = re.sub(r"[`*]", "", label)
            display = self(label, code_size=code_size)
            return reserve(
                f'<link href="{html.escape(target, quote=True)}" color="#1b756c">'
                f'{display}</link>'
            )

        text = re.sub(r"\[([^\]]+)\]\(([^)]+)\)", link, text)
        text = re.sub(
            r"`([^`]+)`",
            lambda m: reserve(
                f'<font name="ReportMono" size="{code_size}">'
                f'{html.escape(m.group(1))}</font>'
            ),
            text,
        )
        text = html.escape(text)
        text = re.sub(r"\*\*(.+?)\*\*", r"<b>\1</b>", text)
        text = re.sub(r"(?<!\*)\*([^*]+)\*(?!\*)", r"<i>\1</i>", text)
        text = text.replace(
            "[pcc:unverified]",
            '<font size="7.7" color="#657384">[pcc:unverified]</font>',
        )
        for i, markup in enumerate(protected):
            text = text.replace(f"\x01{i}\x02", markup)
        return text


class SvgSlot(Flowable):
    """Reserve space; the original vector PDF is overlaid after text layout."""

    def __init__(self, source: Path, width: float, height: float, slots: list[dict]):
        super().__init__()
        self.source = source
        self.width = width
        self.height = height
        self.slots = slots

    def draw(self) -> None:
        x, y = self.canv.absolutePosition(0, 0)
        self.slots.append({
            "source": str(self.source), "page": self.canv.getPageNumber() - 1,
            "rectangle": [x, PAGE_H - y - self.height, x + self.width, PAGE_H - y],
        })


class ReportDocument(SimpleDocTemplate):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.heading_number = 0

    def afterFlowable(self, flowable: Flowable) -> None:
        if isinstance(flowable, Paragraph) and flowable.style.name in ("Heading2", "Heading3"):
            label = flowable.getPlainText()
            self.heading_number += 1
            key = f"section-{self.heading_number}"
            self.canv.bookmarkPage(key)
            self.canv.addOutlineEntry(label, key, level=0, closed=False)


def table_cells(line: str) -> list[str]:
    cells, current, quoted = [], [], False
    for char in line.strip().strip("|"):
        if char == "`":
            quoted = not quoted
        if char == "|" and not quoted:
            cells.append("".join(current).strip())
            current = []
        else:
            current.append(char)
    cells.append("".join(current).strip())
    return cells


def table_widths(header: list[str]) -> list[float]:
    if len(header) == 2:
        first = 0.30
        if header[0].lower() == "bit":
            first = 0.10
        elif header[0].lower() == "arm":
            first = 0.21
        elif header[0].lower() == "requirement":
            first = 0.34
        return [TEXT_W * first, TEXT_W * (1 - first)]
    if len(header) == 3 and header[0].lower() == "representation":
        return [TEXT_W * share for share in (0.42, 0.29, 0.29)]
    return [TEXT_W / len(header)] * len(header)


def make_table(rows: list[list[str]], inline: Inline, styles: dict) -> LongTable:
    count = len(rows[0])
    if any(len(row) != count for row in rows):
        raise ValueError("Markdown table has inconsistent column counts")
    formatted = [
        [Paragraph(inline(cell, code_size=7.9), styles["table_head" if i == 0 else "table"])
         for cell in row]
        for i, row in enumerate(rows)
    ]
    table = LongTable(
        formatted, colWidths=table_widths(rows[0]), repeatRows=1,
        hAlign="LEFT", spaceBefore=4, spaceAfter=10, splitByRow=1,
    )
    table.setStyle(TableStyle([
        ("BACKGROUND", (0, 0), (-1, 0), TEAL),
        ("ROWBACKGROUNDS", (0, 1), (-1, -1), [colors.HexColor("#f1f5f7"), colors.white]),
        ("VALIGN", (0, 0), (-1, -1), "TOP"),
        ("LEFTPADDING", (0, 0), (-1, -1), 8),
        ("RIGHTPADDING", (0, 0), (-1, -1), 8),
        ("TOPPADDING", (0, 0), (-1, -1), 7),
        ("BOTTOMPADDING", (0, 0), (-1, -1), 7),
        ("LINEBELOW", (0, 0), (-1, 0), 0.6, TEAL),
        ("LINEBELOW", (0, -1), (-1, -1), 0.5, RULE),
    ]))
    return table


def svg_dimensions(path: Path) -> tuple[float, float]:
    root = ElementTree.parse(path).getroot()
    if "viewBox" in root.attrib:
        _, _, width, height = (float(n) for n in re.split(r"[ ,]+", root.attrib["viewBox"]))
        return width, height
    return tuple(float(re.sub(r"[^0-9.]", "", root.attrib[key])) for key in ("width", "height"))


def parse_report(source: Path, inline: Inline, styles: dict, slots: list[dict]) -> tuple[list, dict]:
    lines = source.read_text().splitlines()
    story = []
    inventory = {"headings": [], "paragraphs": 0, "tables": 0, "code_blocks": 0, "figures": []}
    i = 0
    before_sections = True
    while i < len(lines):
        line = lines[i].strip()
        if not line:
            i += 1
            continue
        if line.startswith("```:"):
            raise ValueError("Unexpected malformed code fence")
        if line.startswith("```"):
            i += 1
            code = []
            while i < len(lines) and not lines[i].strip().startswith("```"):
                code.append(html.escape(typography(lines[i])))
                i += 1
            if i == len(lines):
                raise ValueError("Unclosed code block")
            paragraph = Paragraph("<br/>".join(code), styles["code"])
            box = Table([[paragraph]], colWidths=[TEXT_W], hAlign="LEFT", spaceBefore=3, spaceAfter=10)
            box.setStyle(TableStyle([
                ("BACKGROUND", (0, 0), (-1, -1), TINT),
                ("BOX", (0, 0), (-1, -1), 0.5, RULE),
                ("LEFTPADDING", (0, 0), (-1, -1), 12),
                ("RIGHTPADDING", (0, 0), (-1, -1), 12),
                ("TOPPADDING", (0, 0), (-1, -1), 10),
                ("BOTTOMPADDING", (0, 0), (-1, -1), 10),
            ]))
            story.append(box)
            inventory["code_blocks"] += 1
            i += 1
            continue
        image = re.fullmatch(r"!\[([^\]]*)\]\(([^)]+)\)", line)
        if image:
            image_path = (source.parent / image.group(2)).resolve()
            if image_path.suffix.lower() != ".svg":
                raise ValueError(f"This report renderer expects SVG figures, got {image_path}")
            width, height = svg_dimensions(image_path)
            figure = SvgSlot(image_path, TEXT_W, TEXT_W * height / width, slots)
            group = [Spacer(1, 3), figure]
            j = i + 1
            while j < len(lines) and not lines[j].strip():
                j += 1
            if j < len(lines) and lines[j].strip().startswith("*") and not lines[j].strip().startswith("**"):
                group.append(Paragraph(inline(lines[j].strip()), styles["caption"]))
                i = j
            story.append(KeepTogether(group))
            inventory["figures"].append({"path": str(image_path), "caption": image.group(1)})
            i += 1
            continue
        heading = re.match(r"^(#{1,3})\s+(.+)$", line)
        if heading:
            level, title = len(heading.group(1)), heading.group(2)
            style = "title" if level == 1 else "h2" if level == 2 else "h3"
            story.append(Paragraph(inline(title), styles[style]))
            inventory["headings"].append(title)
            if level > 1:
                before_sections = False
            i += 1
            continue
        if line.startswith("|") and i + 1 < len(lines):
            separators = table_cells(lines[i + 1])
            if all(re.fullmatch(r":?-{2,}:?", cell) for cell in separators):
                rows = [table_cells(line)]
                i += 2
                while i < len(lines) and lines[i].strip().startswith("|"):
                    rows.append(table_cells(lines[i]))
                    i += 1
                story.append(make_table(rows, inline, styles))
                inventory["tables"] += 1
                continue
        bullet = re.match(r"^(?:[-*]\s+|(\d+)\.\s+)(.+)$", line)
        if bullet:
            label = f"{bullet.group(1)}." if bullet.group(1) else "-"
            story.append(Paragraph(inline(bullet.group(2)), styles["bullet"], bulletText=label))
            inventory["paragraphs"] += 1
            i += 1
            continue
        paragraph_lines = [line]
        i += 1
        while i < len(lines) and lines[i].strip():
            if re.match(r"^(#{1,3}\s|```|\||!\[|[-*]\s+|\d+\.\s+)", lines[i].strip()):
                break
            paragraph_lines.append(lines[i].strip())
            i += 1
        text = " ".join(paragraph_lines)
        style = "body"
        if before_sections:
            style = "subtitle" if text.startswith("**") else "meta"
        story.append(Paragraph(inline(text), styles[style]))
        inventory["paragraphs"] += 1
    return story, inventory


def page_decorations(draft: bool):
    def draw(canvas, doc):
        canvas.saveState()
        canvas.setStrokeColor(RULE)
        canvas.setLineWidth(0.65)
        canvas.line(MARGIN_X, PAGE_H - 32, PAGE_W - MARGIN_X, PAGE_H - 32)
        canvas.setFont("ReportSans-Bold", 7.5)
        canvas.setFillColor(TEAL)
        canvas.drawString(MARGIN_X, PAGE_H - 24, "ISOGENY LABS / PRIME-FIELD RESEARCH")
        canvas.setFont("ReportSans", 7.2)
        canvas.setFillColor(MUTED)
        canvas.drawRightString(PAGE_W - MARGIN_X, PAGE_H - 24, "DRAFT LAYOUT" if draft else "DICKSON / S3")
        canvas.line(MARGIN_X, 34, PAGE_W - MARGIN_X, 34)
        canvas.setFont("ReportSans", 7.0)
        canvas.drawString(MARGIN_X, 22, "PR #4946  |  Source: REPORT.md  |  Evidence standing is stated in the text")
        canvas.drawRightString(PAGE_W - MARGIN_X, 22, f"{doc.page}")
        canvas.restoreState()
    return draw


def render_vectors(slots: list[dict], qa: Path) -> dict[str, Path]:
    inkscape = shutil.which("inkscape")
    if slots and not inkscape:
        raise SystemExit("Inkscape is required to preserve the existing SVG as vector PDF")
    vectors = {}
    for slot in slots:
        source = slot["source"]
        if source in vectors:
            continue
        destination = qa / f"{Path(source).stem}-vector.pdf"
        command = [inkscape, source, "--export-type=pdf", "--export-area-page", f"--export-filename={destination}"]
        result = subprocess.run(command, capture_output=True, text=True, check=True)
        (qa / f"{Path(source).stem}-inkscape.stderr.txt").write_text(result.stderr)
        vectors[source] = destination
    return vectors


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, default=HERE / "REPORT.md")
    parser.add_argument("--output", type=Path, default=HERE / "REPORT.pdf")
    parser.add_argument("--qa-dir", type=Path, default=HERE / "qa")
    parser.add_argument("--github-ref", default="lane/dickson-s3-quadratic-implementation-20261122")
    parser.add_argument("--draft", action="store_true", help="Put a draft-layout label in the running header")
    parser.add_argument("--render-pages", action="store_true", help="Render every PDF page to PNG for human visual inspection")
    parser.add_argument("--dpi", type=int, default=120)
    args = parser.parse_args()
    source, output, qa = args.source.resolve(), args.output.resolve(), args.qa_dir.resolve()
    qa.mkdir(parents=True, exist_ok=True)
    output.parent.mkdir(parents=True, exist_ok=True)
    register_fonts()
    styles = make_styles()
    inline = Inline(args.github_ref)
    slots = []
    story, inventory = parse_report(source, inline, styles, slots)
    raw_pdf = qa / "report-text-layout.pdf"
    doc = ReportDocument(
        str(raw_pdf), pagesize=A4, leftMargin=MARGIN_X, rightMargin=MARGIN_X,
        topMargin=MARGIN_TOP, bottomMargin=MARGIN_BOTTOM,
        title="Dickson ladders with quadratic S3 coupling",
        author="Isogeny Labs", subject="Source-linked construction and implementation report",
        pageCompression=1, invariant=1,
    )
    decorations = page_decorations(args.draft)
    doc.build(story, onFirstPage=decorations, onLaterPages=decorations)
    vectors = render_vectors(slots, qa)
    with fitz.open(raw_pdf) as document:
        for slot in slots:
            with fitz.open(vectors[slot["source"]]) as figure:
                document[slot["page"]].show_pdf_page(fitz.Rect(slot["rectangle"]), figure, 0)
        document.save(output, garbage=4, deflate=True)
    with fitz.open(output) as document:
        page_text = [page.get_text() for page in document]
        pages = len(document)
        links = sum(len(page.get_links()) for page in document)
        if any(not text.strip() for text in page_text):
            raise SystemExit("A rendered page has no selectable text")
        (qa / "report-extracted.txt").write_text("\n\n".join(page_text))
    pngs = []
    render_command = None
    if args.render_pages:
        pdftoppm = shutil.which("pdftoppm")
        if not pdftoppm:
            raise SystemExit("Poppler pdftoppm is required for PNG page inspection")
        prefix = qa / "page"
        # Clear only this renderer's stale page previews, never source/evidence.
        for old in qa.glob("page-[0-9]*.png"):
            old.unlink()
        render_command = [pdftoppm, "-r", str(args.dpi), "-png", str(output), str(prefix)]
        subprocess.run(render_command, check=True, capture_output=True, text=True)
        pngs = [str(path) for path in sorted(qa.glob("page-[0-9]*.png"))]
    receipt = {
        "schema": "dickson-s3-report-layout/v1",
        "scope": "Layout and text/figure rendering only; no scientific calculation or correctness verification",
        "draft_layout": args.draft,
        "source": {"path": str(source), "sha256": digest(source)},
        "renderer": {"path": str(Path(__file__).resolve()), "sha256": digest(Path(__file__).resolve())},
        "pdf": {"path": str(output), "sha256": digest(output), "pages": pages, "bytes": output.stat().st_size},
        "figures": [{"source": path, "sha256": digest(Path(path)), "vector_pdf": str(vector)} for path, vector in vectors.items()],
        "content_inventory": inventory,
        "clickable_link_annotations": links,
        "source_link_targets": inline.links,
        "embedded_fonts": FONT_FILES,
        "reportlab_version": REPORTLAB_VERSION,
        "page_render_command": render_command,
        "page_previews": pngs,
        "human_visual_inspection": "pending; rendering and text extraction do not certify visual quality",
    }
    (qa / "render-receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps({"output": str(output), "pages": pages, "links": links, "draft": args.draft, "qa": str(qa)}))


if __name__ == "__main__":
    main()
