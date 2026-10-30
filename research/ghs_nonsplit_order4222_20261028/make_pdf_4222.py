#!/usr/bin/env python3
"""Render the exact-class research report to a reviewable PDF."""
import html
import re
from pathlib import Path

from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER
from reportlab.lib.pagesizes import letter
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.units import inch
from reportlab.platypus import (
    HRFlowable, KeepTogether, Paragraph, Preformatted, SimpleDocTemplate, Spacer,
)

HERE = Path(__file__).resolve().parent
SOURCE = HERE / "REPORT.md"
OUTPUT = HERE / "output/pdf/GHS_ORDER4222_CLASS_WITNESS.pdf"
OUTPUT.parent.mkdir(parents=True, exist_ok=True)

styles = getSampleStyleSheet()
styles.add(ParagraphStyle(name="DocTitle", parent=styles["Title"],
                          fontName="Helvetica-Bold", fontSize=18, leading=22,
                          textColor=colors.HexColor("#17324f"), spaceAfter=14))
styles.add(ParagraphStyle(name="Section", parent=styles["Heading2"],
                          fontName="Helvetica-Bold", fontSize=12, leading=15,
                          textColor=colors.HexColor("#174f77"), spaceBefore=10,
                          spaceAfter=6, keepWithNext=True))
styles.add(ParagraphStyle(name="BodyResearch", parent=styles["BodyText"],
                          fontName="Helvetica", fontSize=8.7, leading=11.7,
                          spaceAfter=4))
styles.add(ParagraphStyle(name="BodyLead", parent=styles["BodyResearch"],
                          keepWithNext=True))
styles.add(ParagraphStyle(name="CodeResearch", fontName="Courier", fontSize=7.5,
                          leading=10, leftIndent=10, rightIndent=6,
                          backColor=colors.HexColor("#f2f5f7"),
                          borderPadding=7, spaceBefore=3, spaceAfter=10))
styles.add(ParagraphStyle(name="Footer", fontName="Helvetica", fontSize=8,
                          leading=10, textColor=colors.HexColor("#687781"),
                          alignment=TA_CENTER))


def para_markup(raw):
    s = html.escape(raw)
    s = re.sub(r"`([^`]+)`", r'<font name="Courier">\1</font>', s)
    s = re.sub(r"\*\*([^*]+)\*\*", r"<b>\1</b>", s)
    s = re.sub(r"\[([^\]]+)\]\((https?://[^)]+)\)",
               r'<link href="\2" color="#174f77">\1</link>', s)
    return s


blocks = []
paragraph = []
code_lines = []
in_code = False


def flush_paragraph():
    if paragraph:
        raw = " ".join(paragraph).strip()
        if raw.startswith("Magma's plane-quartic line sieve ran"):
            paragraph.clear()
            return
        style = styles["BodyLead"] if raw.startswith("For the quotient map") else styles["BodyResearch"]
        blocks.append(Paragraph(para_markup(raw), style))
        paragraph.clear()


for line in SOURCE.read_text().splitlines():
    if line == "## Reproduce":
        break
    if line.startswith("```"):
        flush_paragraph()
        if in_code:
            blocks.append(Preformatted("\n".join(code_lines), styles["CodeResearch"],
                                       maxLineLength=92, splitChars=" "))
            code_lines = []
        in_code = not in_code
        continue
    if in_code:
        code_lines.append(line)
        continue
    if line.startswith("# "):
        flush_paragraph()
        blocks.append(Paragraph(para_markup(line[2:]), styles["DocTitle"]))
        blocks.append(HRFlowable(width="100%", thickness=1,
                                 color=colors.HexColor("#b8cbd8")))
        blocks.append(Spacer(1, 8))
    elif line.startswith("## "):
        flush_paragraph()
        blocks.append(Paragraph(para_markup(line[3:]), styles["Section"]))
    elif not line.strip():
        flush_paragraph()
    else:
        paragraph.append(line.strip())
flush_paragraph()
assert not in_code


def footer(canvas, doc):
    canvas.saveState()
    canvas.setStrokeColor(colors.HexColor("#b8cbd8"))
    canvas.line(0.65*inch, 0.55*inch, 7.85*inch, 0.55*inch)
    canvas.setFont("Helvetica", 7.5)
    canvas.setFillColor(colors.HexColor("#687781"))
    canvas.drawString(0.67*inch, 0.38*inch,
                      "Order-4222 cubic descent | Research receipt | 2026-10-28")
    canvas.drawRightString(7.83*inch, 0.38*inch, f"Page {doc.page}")
    canvas.restoreState()


doc = SimpleDocTemplate(str(OUTPUT), pagesize=letter,
                        leftMargin=0.68*inch, rightMargin=0.68*inch,
                        topMargin=0.60*inch, bottomMargin=0.65*inch,
                        title="Order-4222 cubic descent and catalog witness",
                        author="GHS research audit")
doc.build(blocks, onFirstPage=footer, onLaterPages=footer)
print(OUTPUT)
