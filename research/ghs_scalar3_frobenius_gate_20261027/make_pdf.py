#!/usr/bin/env python3
"""Render the checked research note as a compact PDF for review."""
import html
import re
from pathlib import Path

from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER
from reportlab.lib.pagesizes import letter
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (BaseDocTemplate, Frame, KeepTogether, PageTemplate,
                                Paragraph, Spacer, Table, TableStyle)

HERE = Path(__file__).resolve().parent
SRC = HERE / "REPORT.md"
OUT = HERE / "output" / "pdf" / "SCALAR_3_TORSION_GHS_REPORT.pdf"

pdfmetrics.registerFont(TTFont("ArialUnicode", "/System/Library/Fonts/Supplemental/Arial Unicode.ttf"))
pdfmetrics.registerFont(TTFont("ArialBold", "/System/Library/Fonts/Supplemental/Arial Bold.ttf"))
pdfmetrics.registerFontFamily("ArialUnicode", normal="ArialUnicode", bold="ArialBold")


def inline(source):
    source = source.translate(str.maketrans({
        "¹": "^1", "²": "^2", "³": "^3", "⁴": "^4", "⁵": "^5",
        "⁶": "^6", "⁷": "^7", "–": "-", "—": "-", "‑": "-",
    }))
    s = html.escape(source)
    s = re.sub(r"\[([^]]+)\]\(([^)]+)\)",
               lambda m: (f'<link href="{html.escape(m.group(2), quote=True)}" color="#1c5789">'
                          f'{m.group(1)}</link>') if m.group(2).startswith("http") else m.group(1), s)
    s = re.sub(r"`([^`]+)`", r'<font name="Courier" size="8.5">\1</font>', s)
    s = re.sub(r"\*\*([^*]+)\*\*", r"<b>\1</b>", s)
    return s.replace("–", "-").replace("—", "-").replace("‑", "-")


styles = getSampleStyleSheet()
body = ParagraphStyle("BodyGHS", parent=styles["BodyText"], fontName="ArialUnicode",
                      fontSize=9, leading=12, spaceAfter=5, textColor=colors.HexColor("#152535"))
h1 = ParagraphStyle("H1GHS", parent=body, fontName="ArialBold", fontSize=14,
                    leading=17, spaceBefore=12, spaceAfter=7, keepWithNext=True,
                    textColor=colors.HexColor("#123e62"))
h2 = ParagraphStyle("H2GHS", parent=body, fontName="ArialBold", fontSize=11,
                    leading=13, spaceBefore=8, spaceAfter=5, keepWithNext=True,
                    textColor=colors.HexColor("#1b547c"))
title = ParagraphStyle("TitleGHS", parent=h1, fontSize=20, leading=24, alignment=TA_CENTER,
                       spaceBefore=0, spaceAfter=18)
small = ParagraphStyle("SmallGHS", parent=body, fontSize=7.8, leading=10, spaceAfter=0)
code = ParagraphStyle("CodeGHS", parent=body, fontName="Courier", fontSize=7.5,
                      leading=10, leftIndent=12, spaceAfter=2)


def page(canvas, doc):
    canvas.saveState()
    width, height = letter
    canvas.setStrokeColor(colors.HexColor("#afbfcc"))
    canvas.line(48, height - 45, width - 48, height - 45)
    canvas.setFont("ArialUnicode", 8)
    canvas.setFillColor(colors.HexColor("#506477"))
    canvas.drawString(48, 34, "Scalar 3-torsion GHS route | 27 October 2026")
    canvas.drawRightString(width - 48, 34, str(doc.page))
    canvas.restoreState()


def render():
    lines = SRC.read_text().splitlines()
    flow = []
    paragraph = []
    table = []
    in_code = False

    def flush_paragraph():
        if paragraph:
            flow.append(Paragraph(inline(" ".join(paragraph)), body))
            paragraph.clear()

    def flush_table():
        if not table:
            return
        rows = [[cell.strip() for cell in line.strip().strip("|").split("|")]
                for line in table if not re.fullmatch(r"\|?[\s|:\-]+\|?", line)]
        if rows:
            n = len(rows[0])
            widths = [500 / n] * n
            if n == 5:
                widths = [92, 91, 77, 102, 138]
            data = [[Paragraph(inline(cell), small) for cell in row] for row in rows]
            grid = Table(data, colWidths=widths, repeatRows=1, hAlign="LEFT")
            grid.setStyle(TableStyle([
                ("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#dfeaf2")),
                ("ROWBACKGROUNDS", (0, 1), (-1, -1), [colors.white, colors.HexColor("#f5f8fa")]),
                ("VALIGN", (0, 0), (-1, -1), "TOP"),
                ("TOPPADDING", (0, 0), (-1, -1), 4),
                ("BOTTOMPADDING", (0, 0), (-1, -1), 4),
                ("LEFTPADDING", (0, 0), (-1, -1), 6),
                ("RIGHTPADDING", (0, 0), (-1, -1), 6),
                ("LINEBELOW", (0, 0), (-1, 0), 0.6, colors.HexColor("#7c9ab0")),
            ]))
            flow.extend([Spacer(1, 4), grid, Spacer(1, 8)])
        table.clear()

    for line in lines:
        if line.startswith("```"):
            flush_paragraph()
            flush_table()
            in_code = not in_code
            continue
        if in_code:
            flow.append(Paragraph(html.escape(line).replace(" ", "&nbsp;"), code))
            continue
        if line.startswith("|"):
            flush_paragraph()
            table.append(line)
            continue
        flush_table()
        if not line.strip():
            flush_paragraph()
            continue
        if line.startswith("# "):
            flush_paragraph()
            flow.append(Paragraph(inline(line[2:]), title))
        elif line.startswith("## "):
            flush_paragraph()
            flow.append(Paragraph(inline(line[3:]), h1))
        elif line.startswith("### "):
            flush_paragraph()
            flow.append(Paragraph(inline(line[4:]), h2))
        elif re.match(r"\d+\. ", line):
            flush_paragraph()
            flow.append(Paragraph(inline(line), body))
        else:
            paragraph.append(line.strip())
    flush_paragraph()
    flush_table()
    OUT.parent.mkdir(parents=True, exist_ok=True)
    doc = BaseDocTemplate(str(OUT), pagesize=letter, leftMargin=48, rightMargin=48,
                          topMargin=58, bottomMargin=50, title="Scalar 3-torsion GHS route",
                          author="Cryptanalysis research audit")
    frame = Frame(48, 50, 516, 684, leftPadding=0, rightPadding=0,
                  topPadding=0, bottomPadding=0)
    doc.addPageTemplates(PageTemplate(id="Main", frames=[frame], onPage=page))
    doc.build(flow)
    print(OUT)


if __name__ == "__main__":
    render()
