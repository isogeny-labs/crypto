#!/usr/bin/env python3
"""Document layout only: render recorded prose and the supplied SVG; no research computation."""

from html import escape
from pathlib import Path
import re

import fitz
from reportlab.lib import colors
from reportlab.lib.enums import TA_LEFT
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (
    BaseDocTemplate, PageTemplate, Frame, Paragraph, Spacer, Table, TableStyle,
    Image, Preformatted, KeepTogether,
)


HERE = Path(__file__).resolve().parent
font_root = Path("/usr/share/fonts/truetype/dejavu")
for name, filename in [("ReportSans", "DejaVuSans.ttf"),
                       ("ReportSansBold", "DejaVuSans-Bold.ttf"),
                       ("ReportMono", "DejaVuSansMono.ttf")]:
    pdfmetrics.registerFont(TTFont(name, str(font_root / filename)))
pdfmetrics.registerFontFamily("ReportSans", normal="ReportSans", bold="ReportSansBold",
                              italic="ReportSans", boldItalic="ReportSansBold")
source = (HERE / "REPORT.md").read_text()
if "ROOT_GATE_STATUS" in source:
    raise SystemExit("Refusing to render an unfinished report")

# PyMuPDF renders the exact editable SVG. This is illustration layout only.
svg = fitz.open(HERE / "architecture.svg")
svg[0].get_pixmap(matrix=fitz.Matrix(1.5, 1.5), alpha=False).save(HERE / "architecture.png")

PAGE_W, PAGE_H = A4
WIDTH = PAGE_W - 82
styles = getSampleStyleSheet()
styles.add(ParagraphStyle("ReportTitle", fontName="ReportSansBold", fontSize=22,
                          leading=27, textColor=colors.HexColor("#182c43"), spaceAfter=10))
styles.add(ParagraphStyle("ReportBody", fontName="ReportSans", fontSize=9.15,
                          leading=12.6, textColor=colors.HexColor("#25384b"),
                          spaceAfter=7, alignment=TA_LEFT, splitLongWords=True,
                          allowWidows=False, allowOrphans=False))
styles.add(ParagraphStyle("ReportHeading", fontName="ReportSansBold", fontSize=12.6,
                          leading=17, textColor=colors.HexColor("#1e567d"),
                          spaceBefore=13, spaceAfter=6, keepWithNext=True))
styles.add(ParagraphStyle("ReportCell", fontName="ReportSans", fontSize=8.3,
                          leading=11, textColor=colors.HexColor("#25384b")))
styles.add(ParagraphStyle("ReportHeaderCell", parent=styles["ReportCell"],
                          fontName="ReportSansBold", textColor=colors.white))
styles.add(ParagraphStyle("ReportCaption", parent=styles["ReportBody"],
                          fontSize=8.1, leading=11, textColor=colors.HexColor("#536779")))
styles.add(ParagraphStyle("ReportCode", fontName="ReportMono", fontSize=7.4,
                          leading=10, backColor=colors.HexColor("#edf3f8"),
                          borderPadding=8, spaceBefore=5, spaceAfter=9))


def inline(text):
    text = escape(text)
    text = re.sub(r"`([^`]+)`", r'<font name="ReportMono">\1</font>', text)
    text = re.sub(r"\*\*([^*]+)\*\*", r"<b>\1</b>", text)
    return text


def footer(canvas, doc):
    canvas.saveState()
    canvas.setStrokeColor(colors.HexColor("#d6e0e9"))
    canvas.line(41, PAGE_H - 30, PAGE_W - 41, PAGE_H - 30)
    canvas.setFillColor(colors.HexColor("#60778b"))
    canvas.setFont("ReportSans", 8)
    canvas.drawString(41, PAGE_H - 23, "ISOGENY LABS  /  M31 IMPLEMENTATION")
    canvas.drawRightString(PAGE_W - 41, PAGE_H - 23, "PR #5120")
    canvas.drawString(41, 26, "Unsigned development evidence | 23 November 2026 Pacific")
    canvas.drawRightString(PAGE_W - 41, 26, f"{doc.page}")
    canvas.restoreState()


story = []
lines = source.splitlines()
i = 0
while i < len(lines):
    line = lines[i].strip()
    if not line:
        i += 1
        continue
    if line.startswith("# "):
        story.append(Paragraph(inline(line[2:]), styles["ReportTitle"]))
        i += 1
    elif line.startswith("## "):
        story.append(Paragraph(inline(line[3:]), styles["ReportHeading"]))
        i += 1
    elif line.startswith("!["):
        figure = Image(str(HERE / "architecture.png"), width=WIDTH, height=WIDTH * 0.65)
        story.append(KeepTogether([Spacer(1, 5), figure, Spacer(1, 7)]))
        i += 1
    elif line.startswith("```"):
        code = []
        i += 1
        while i < len(lines) and not lines[i].startswith("```"):
            code.append(lines[i])
            i += 1
        story.append(Preformatted("\n".join(code), styles["ReportCode"], maxLineLength=105))
        i += 1
    elif line.startswith("|"):
        rows = []
        while i < len(lines) and lines[i].strip().startswith("|"):
            cells = [c.strip() for c in lines[i].strip().strip("|").split("|")]
            if not all(re.fullmatch(r"[:\- ]+", c) for c in cells):
                rows.append(cells)
            i += 1
        widths = [WIDTH * 0.17, WIDTH * 0.29, WIDTH * 0.54] if len(rows[0]) == 3 else [WIDTH * 0.31, WIDTH * 0.69]
        cooked = [[Paragraph(inline(cell), styles["ReportHeaderCell" if r == 0 else "ReportCell"])
                   for cell in row] for r, row in enumerate(rows)]
        table = Table(cooked, colWidths=widths, repeatRows=1, hAlign="LEFT")
        table.setStyle(TableStyle([
            ("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#234a6c")),
            ("ROWBACKGROUNDS", (0, 1), (-1, -1), [colors.HexColor("#f1f5f9"), colors.white]),
            ("VALIGN", (0, 0), (-1, -1), "TOP"),
            ("LEFTPADDING", (0, 0), (-1, -1), 8),
            ("RIGHTPADDING", (0, 0), (-1, -1), 8),
            ("TOPPADDING", (0, 0), (-1, -1), 7),
            ("BOTTOMPADDING", (0, 0), (-1, -1), 7),
            ("LINEBELOW", (0, -1), (-1, -1), 0.5, colors.HexColor("#d6e0e9")),
        ]))
        story.extend([table, Spacer(1, 9)])
    elif line.startswith("- "):
        story.append(Paragraph(inline(line[2:]), styles["ReportBody"], bulletText="-"))
        i += 1
    else:
        paragraph = [line]
        i += 1
        while i < len(lines) and lines[i].strip() and not lines[i].startswith(("#", "|", "```", "![", "- ")):
            paragraph.append(lines[i].strip())
            i += 1
        story.append(Paragraph(inline(" ".join(paragraph)), styles["ReportBody"]))

doc = BaseDocTemplate(str(HERE / "m31-integrated-build.pdf"), pagesize=A4,
                      rightMargin=41, leftMargin=41, topMargin=44, bottomMargin=42,
                      title="M31 integrated implementation report", author="Isogeny Labs")
frame = Frame(doc.leftMargin, doc.bottomMargin, doc.width, doc.height, id="Body")
doc.addPageTemplates(PageTemplate(id="Report", frames=[frame]))
doc.build(story)
# Stamp every completed page after layout, avoiding inherited text/clip state
# from split flowables. This also makes header/footer placement deterministic.
pdf_path = HERE / "m31-integrated-build.pdf"
pdf = fitz.open(pdf_path)
stamp_font = fitz.Font(fontfile=str(font_root / "DejaVuSans.ttf"))
for index, page in enumerate(pdf):
    page.insert_font(fontname="ReportStamp", fontfile=str(font_root / "DejaVuSans.ttf"))
    ink = (0.376, 0.467, 0.545)
    page.draw_line((41, 30), (PAGE_W - 41, 30), color=(0.839, 0.878, 0.914), width=0.5)
    page.insert_text((41, 23), "ISOGENY LABS  /  M31 IMPLEMENTATION", fontname="ReportStamp", fontsize=8, color=ink)
    for value, baseline in [("PR #5120", 23), (str(index + 1), PAGE_H - 26)]:
        text_width = stamp_font.text_length(value, fontsize=8)
        page.insert_text((PAGE_W - 41 - text_width, baseline), value, fontname="ReportStamp", fontsize=8, color=ink)
    page.insert_text((41, PAGE_H - 26), "Unsigned development evidence | 23 November 2026 Pacific",
                     fontname="ReportStamp", fontsize=8, color=ink)
pdf.subset_fonts()
temporary = HERE / "report-layout.tmp.pdf"
pdf.save(temporary, garbage=4, deflate=True)
pdf.close()
temporary.replace(pdf_path)
print(HERE / "m31-integrated-build.pdf")
