#!/usr/bin/env python3
"""Document layout only: render recorded native values; no research computation."""
from pathlib import Path
import json
import re
from html import escape

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from reportlab.lib import colors
from reportlab.lib.enums import TA_LEFT
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.units import inch
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (
    SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle,
    Preformatted, Image, KeepTogether,
)

ROOT = Path(__file__).resolve().parent
FONT_ROOT = Path('/usr/share/fonts/truetype/dejavu')
for name, filename in [('ReportSans', 'DejaVuSans.ttf'),
                       ('ReportSans-Bold', 'DejaVuSans-Bold.ttf'),
                       ('ReportMono', 'DejaVuSansMono.ttf')]:
    pdfmetrics.registerFont(TTFont(name, str(FONT_ROOT / filename)))
pdfmetrics.registerFontFamily('ReportSans', normal='ReportSans',
                              bold='ReportSans-Bold', italic='ReportSans',
                              boldItalic='ReportSans-Bold')
data = json.loads((ROOT / "QUALIFICATION.json").read_text())
rows = [data["baseline"]] + data["sectors"]
labels = ["Coupled"] + [f"k = {r['dickson_degree']}" for r in data["sectors"]]
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10})
fig, ax = plt.subplots(figsize=(9.0, 3.6), layout="constrained")
x = list(range(len(rows)))
ax.bar([i - .18 for i in x], [r["variables"] for r in rows], .34,
       label="Variables", color="#184e77")
ax.bar([i + .18 for i in x], [r["equations"] for r in rows], .34,
       label="Equations", color="#2a9d8f")
for i, row in enumerate(rows):
    ax.text(i - .18, row["variables"] + 2, str(row["variables"]), ha="center", fontsize=9)
    ax.text(i + .18, row["equations"] + 2, str(row["equations"]), ha="center", fontsize=9)
ax.set(xticks=x, xticklabels=labels, ylim=(0, 143), ylabel="Input-system count",
       title="Smaller systems; all five sectors cover the supported RR5 chart")
ax.legend(loc="upper right", frameon=False, ncol=2)
ax.spines[["top", "right"]].set_visible(False)
ax.grid(axis="y", color="#dfe5eb", linewidth=.6)
ax.set_axisbelow(True)
fig.savefig(ROOT / "construction_inventory.svg")
fig.savefig(ROOT / "construction_inventory.png", dpi=180)
plt.close(fig)

styles = getSampleStyleSheet()
styles.add(ParagraphStyle(name="ReportTitle", fontName="ReportSans-Bold", fontSize=22,
                          leading=27, textColor=colors.HexColor("#123047"), spaceAfter=13))
styles.add(ParagraphStyle(name="ReportBody", fontName="ReportSans", fontSize=9.4,
                          leading=13.2, spaceAfter=7, splitLongWords=True))
styles.add(ParagraphStyle(name="ReportSmall", fontName="ReportSans", fontSize=7.4,
                          leading=10, spaceAfter=4, splitLongWords=True))
styles.add(ParagraphStyle(name="ReportCode", fontName="ReportMono", fontSize=8,
                          leading=10, leftIndent=10, spaceAfter=8))
styles["Heading2"].fontSize = 13
styles["Heading2"].fontName = 'ReportSans-Bold'
styles["Heading2"].keepWithNext = True
styles["Heading2"].leading = 17
styles["Heading2"].spaceBefore = 12
styles["Heading2"].spaceAfter = 7
styles["Heading2"].textColor = colors.HexColor("#184e77")
BASE = "https://github.com/isogeny-labs/crypto/blob/lane/prime31-rr5-coprime-sectors-20261123/research/prime31_rr5_coprime_sectors_20261123/"

def rich(text):
    text = escape(text)
    text = re.sub(r"\[([^\]]+)\]\(([^)]+)\)",
                  lambda m: '<link href="' + (m[2] if m[2].startswith("https:") else BASE + m[2]) +
                  '" color="#126782">' + m[1] + '</link>', text)
    text = re.sub(r"`([^`]+)`", r'<font name="ReportMono">\1</font>', text)
    text = re.sub(r"\*\*([^*]+)\*\*", r'<b>\1</b>', text)
    return text

def paragraph(text, small=False):
    return Paragraph(rich(text), styles["ReportSmall" if small else "ReportBody"])

story = []
lines = (ROOT / "README.md").read_text().splitlines()
paragraph_lines = []
table_lines = []
code_lines = []
inserted_graph = False

def flush_paragraph():
    if paragraph_lines:
        story.append(paragraph(" ".join(paragraph_lines)))
        paragraph_lines.clear()

def flush_code():
    if code_lines:
        story.append(KeepTogether([Preformatted("\n".join(code_lines), styles["ReportCode"])]))
        code_lines.clear()

def flush_table():
    if not table_lines:
        return
    cells = []
    for line in table_lines:
        parts = [v.strip() for v in line.strip().strip("|").split("|")]
        if all(re.fullmatch(r"[-: ]+", v) for v in parts):
            continue
        cells.append([paragraph(v, True) for v in parts])
    count = len(cells[0])
    widths = ({2:[145, 355], 3:[90, 100, 310],
               6:[75, 65, 60, 64, 83, 153]}.get(count, [500 / count] * count))
    table = Table(cells, colWidths=widths, repeatRows=1, hAlign="LEFT")
    table.setStyle(TableStyle([
        ("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#e6f1f5")),
        ("VALIGN", (0, 0), (-1, -1), "TOP"),
        ("LEFTPADDING", (0, 0), (-1, -1), 7),
        ("RIGHTPADDING", (0, 0), (-1, -1), 7),
        ("TOPPADDING", (0, 0), (-1, -1), 6),
        ("BOTTOMPADDING", (0, 0), (-1, -1), 5),
        ("LINEBELOW", (0, 0), (-1, 0), .8, colors.HexColor("#184e77")),
        ("LINEBELOW", (0, 1), (-1, -1), .3, colors.HexColor("#d7e1e8")),
    ]))
    story.extend([table, Spacer(1, 9)])
    table_lines.clear()

for line in lines + [""]:
    if line.startswith("|"):
        flush_paragraph(); flush_code(); table_lines.append(line); continue
    flush_table()
    if line.startswith("    "):
        flush_paragraph(); code_lines.append(line[4:]); continue
    flush_code()
    if not line.strip():
        flush_paragraph(); continue
    if line.startswith("# "):
        flush_paragraph(); story.append(Paragraph(rich(line[2:]), styles["ReportTitle"])); continue
    if line.startswith("## "):
        flush_paragraph()
        if line.startswith("## 5.") and not inserted_graph:
            story.append(KeepTogether([
                Image(str(ROOT / "construction_inventory.png"), width=440, height=176),
                paragraph("Recorded native constructor inventories; all equations have input degree at most two. "
                                   "Counts describe separate systems, not measured solver work. Source: library-tests-02; "
                                   "PCC-20261124-36f87f, unsigned.", True)]))
            inserted_graph = True
        story.append(Paragraph(rich(line[3:]), styles["Heading2"])); continue
    if line.startswith("- "):
        flush_paragraph(); story.append(paragraph("• " + line[2:])); continue
    if re.match(r"^\d+\. ", line):
        flush_paragraph()
    paragraph_lines.append(line)

def page_frame(canvas, document):
    canvas.saveState()
    canvas.setStrokeColor(colors.HexColor("#d7e1e8"))
    canvas.line(47, 42, 547, 42)
    canvas.setFont("ReportSans", 6.5)
    canvas.setFillColor(colors.HexColor("#526575"))
    canvas.drawString(47, 29, "ISOGENY LABS  |  M31 RR5 COPRIME SECTORS  |  CONSTRUCTION QUALIFICATION")
    canvas.drawRightString(547, 29, str(document.page))
    canvas.restoreState()

doc = SimpleDocTemplate(str(ROOT / "M31_RR5_Coprime_Selector_Sectors.pdf"),
                        pagesize=(594, 792), leftMargin=47, rightMargin=47,
                        topMargin=43, bottomMargin=55,
                        title="Five exact coprime-selector sectors for the M31 RR5 chart",
                        author="Isogeny Labs")
doc.build(story, onFirstPage=page_frame, onLaterPages=page_frame)
print(ROOT / "M31_RR5_Coprime_Selector_Sectors.pdf")
