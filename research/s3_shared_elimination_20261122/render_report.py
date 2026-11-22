#!/usr/bin/env python3
"""Typeset the recorded Markdown and SVG only; performs no research computation."""

from html import escape
from pathlib import Path
import re
import tempfile

import fitz
from reportlab.lib import colors
from reportlab.lib.enums import TA_LEFT
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.units import mm
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (
    KeepTogether, Paragraph, Preformatted, SimpleDocTemplate, Spacer, Table, TableStyle,
)

STUDY = Path(__file__).resolve().parent
BASE_URL = "https://github.com/isogeny-labs/crypto/blob/lane/s3-shared-elimination/research/s3_shared_elimination_20261122/"
BLUE = colors.HexColor("#172c45")
TEAL = colors.HexColor("#2e7c89")
FONT_ROOT = Path("/usr/share/fonts/truetype/dejavu")
pdfmetrics.registerFont(TTFont("ReportSans", str(FONT_ROOT / "DejaVuSans.ttf")))
pdfmetrics.registerFont(TTFont("ReportSansBold", str(FONT_ROOT / "DejaVuSans-Bold.ttf")))
pdfmetrics.registerFont(TTFont("ReportSansItalic", str(FONT_ROOT / "DejaVuSans-Oblique.ttf")))
pdfmetrics.registerFont(TTFont("ReportMono", str(FONT_ROOT / "DejaVuSansMono.ttf")))
pdfmetrics.registerFontFamily("ReportSans", normal="ReportSans", bold="ReportSansBold", italic="ReportSansItalic")


def inline(text):
    text = escape(text)
    code_spans = []

    def protect_code(match):
        code_spans.append(match.group(1))
        return f"\x01{len(code_spans) - 1}\x02"

    text = re.sub(r"`([^`]+)`", protect_code, text)
    text = re.sub(r"\*\*([^*]+)\*\*", r"<b>\1</b>", text)
    text = re.sub(r"(?<!\*)\*([^*]+)\*(?!\*)", r"<i>\1</i>", text)

    def link(match):
        title, target = match.groups()
        if not target.startswith(("https://", "http://")):
            target = BASE_URL + target
        return f'<link href="{target}" color="#2e7c89">{title}</link>'

    text = re.sub(r"\[([^]]+)\]\(([^)]+)\)", link, text)
    for index, code_span in enumerate(code_spans):
        text = text.replace(f"\x01{index}\x02", f'<font name="ReportMono">{code_span}</font>')
    return text


def footer(canvas, doc):
    canvas.setStrokeColor(colors.HexColor("#cdd7e3"))
    canvas.line(18 * mm, 15 * mm, A4[0] - 18 * mm, 15 * mm)
    canvas.setFillColor(colors.HexColor("#506079"))
    canvas.setFont("ReportSans", 7.5)
    canvas.drawString(18 * mm, 10.5 * mm, "S3 shared elimination | PR #4922 | November 22, 2026")


def render():
    styles = getSampleStyleSheet()
    body = ParagraphStyle("ReportBody", fontName="ReportSans", fontSize=9.3,
                          leading=13.2, spaceAfter=7, textColor=BLUE,
                          alignment=TA_LEFT, splitLongWords=True,
                          allowWidows=0, allowOrphans=0)
    title = ParagraphStyle("ReportTitle", parent=styles["Title"], fontName="ReportSansBold",
                           fontSize=22, leading=27, textColor=BLUE, alignment=TA_LEFT,
                           spaceAfter=14)
    h2 = ParagraphStyle("ReportH2", parent=styles["Heading2"], fontName="ReportSansBold", fontSize=14,
                        leading=18, textColor=TEAL, spaceBefore=13, spaceAfter=7,
                        keepWithNext=True)
    h3 = ParagraphStyle("ReportH3", parent=h2, fontSize=11, leading=14,
                        textColor=BLUE, spaceBefore=10)
    cell = ParagraphStyle("ReportCell", parent=body, fontSize=7.7, leading=10.4,
                          spaceAfter=0)
    code = ParagraphStyle("ReportCode", fontName="ReportMono", fontSize=8,
                          leading=11, leftIndent=10, spaceAfter=9, textColor=BLUE)
    lines = (STUDY / "REPORT.md").read_text(encoding="utf-8").splitlines()
    story = []
    paragraph = []

    def flush():
        if paragraph:
            story.append(Paragraph(inline(" ".join(paragraph)), body))
            paragraph.clear()

    i = 0
    while i < len(lines):
        line = lines[i]
        if not line.strip():
            flush()
        elif line.startswith("!["):
            flush()
            story.append(Paragraph("<b>Figure 1.</b> The shared and fresh numerical paths are shown on the following landscape figure page.", body))
        elif line.startswith("# "):
            flush()
            story.append(Paragraph(inline(line[2:]), title))
        elif line.startswith("## "):
            flush()
            story.append(Paragraph(inline(line[3:]), h2))
        elif line.startswith("### "):
            flush()
            story.append(Paragraph(inline(line[4:]), h3))
        elif re.match(r"^\d+\. ", line):
            flush()
            paragraph.append(line)
        elif line.startswith("```"):
            flush()
            code_lines = []
            i += 1
            while i < len(lines) and not lines[i].startswith("```"):
                code_lines.append(lines[i])
                i += 1
            story.append(Preformatted("\n".join(code_lines), code))
        elif line.startswith("|"):
            flush()
            rows = []
            while i < len(lines) and lines[i].startswith("|"):
                parts = [part.strip() for part in lines[i].strip().strip("|").split("|")]
                if not all(re.fullmatch(r"[-: ]+", part) for part in parts):
                    rows.append([Paragraph(inline(part), cell) for part in parts])
                i += 1
            width = A4[0] - 36 * mm
            table = Table(rows, colWidths=[width * .23, width * .35, width * .42],
                          repeatRows=1, hAlign="LEFT")
            table.setStyle(TableStyle([
                ("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#e5ebf3")),
                ("ROWBACKGROUNDS", (0, 1), (-1, -1), [colors.white, colors.HexColor("#f6f8fc")]),
                ("VALIGN", (0, 0), (-1, -1), "TOP"),
                ("BOX", (0, 0), (-1, -1), .5, colors.HexColor("#a4b4c8")),
                ("INNERGRID", (0, 0), (-1, -1), .3, colors.HexColor("#cdd7e3")),
                ("LEFTPADDING", (0, 0), (-1, -1), 6),
                ("RIGHTPADDING", (0, 0), (-1, -1), 6),
                ("TOPPADDING", (0, 0), (-1, -1), 6),
                ("BOTTOMPADDING", (0, 0), (-1, -1), 6),
            ]))
            story.extend([table, Spacer(1, 8)])
            continue
        else:
            paragraph.append(line)
        i += 1
    flush()
    with tempfile.TemporaryDirectory(prefix="s3-report-layout-") as temporary:
        text_pdf = Path(temporary) / "text.pdf"
        doc = SimpleDocTemplate(str(text_pdf), pagesize=A4, topMargin=18 * mm,
                                bottomMargin=21 * mm, leftMargin=18 * mm,
                                rightMargin=18 * mm,
                                title="Exact modular matrices with shared S3 elimination",
                                author="Isogeny Labs research / PR #4922")
        doc.build(story, onFirstPage=footer, onLaterPages=footer)
        result = fitz.open(text_pdf)
        svg = fitz.open(stream=(STUDY / "projection.svg").read_bytes(), filetype="svg")
        figure = fitz.open(stream=svg.convert_to_pdf(), filetype="pdf")
        page = result.new_page(pno=1, width=A4[1], height=A4[0])
        page.show_pdf_page(fitz.Rect(25, 68, A4[1] - 25, A4[0] - 80), figure, 0)
        page.insert_text((30, 36), "Figure 1 | Exact row-space reuse and its fresh reference",
                         fontsize=15, color=(.09, .17, .27))
        for number, page in enumerate(result, 1):
            page.insert_text((page.rect.width - 65, page.rect.height - 28),
                             f"{number} / {len(result)}", fontsize=8,
                             color=(.31, .38, .47))
        result.save(STUDY / "report.pdf", garbage=4, deflate=True)
    print(f"Rendered {STUDY / 'report.pdf'}")


if __name__ == "__main__":
    render()
