"""Render the descriptive Markdown and recorded graph to PDF; no research computation."""
from pathlib import Path
from xml.sax.saxutils import escape
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.pagesizes import A4
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Image, Table, TableStyle

root = Path(__file__).resolve().parent / "reports"
styles = getSampleStyleSheet()
styles.add(ParagraphStyle(name="Body", fontSize=9.3, leading=13, spaceAfter=8))
styles.add(ParagraphStyle(name="Cell", fontSize=8, leading=10))
story = [Paragraph("Fp² and prime-field m4 native diagnostics", styles["Title"]),
         Paragraph("2026-10-18 · two bounded native runs · partial execution of audit items 1 and 14", styles["Body"])]
lines = (root / "REPORT.md").read_text().splitlines()
paragraph, table = [], []
def flush():
    if paragraph:
        story.append(Paragraph(escape(" ".join(paragraph).replace("`", "")), styles["Body"]))
        paragraph.clear()
def flush_table():
    if table:
        body = [[Paragraph(escape(cell), styles["Cell"]) for cell in row] for row in table]
        t = Table(body, colWidths=[137, 197, 181], repeatRows=1)
        t.setStyle(TableStyle([("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#E8EDF2")),
                              ("VALIGN", (0, 0), (-1, -1), "TOP"),
                              ("GRID", (0, 0), (-1, -1), .3, colors.lightgrey),
                              ("BOTTOMPADDING", (0, 0), (-1, -1), 6)]))
        story.extend([t, Spacer(1, 12)])
        table.clear()
for line in lines:
    if line.startswith("|"):
        flush()
        if "---" not in line:
            table.append([cell.strip().replace("`", "") for cell in line.strip("|").split("|")])
    elif line.startswith("!["):
        flush(); flush_table()
        story.extend([Image(str(root / "recorded-values.png"), width=515, height=154.5), Spacer(1, 10)])
    elif not line.strip():
        flush(); flush_table()
    else:
        paragraph.append(line)
flush(); flush_table()
def footer(canvas, doc):
    canvas.setFont("Helvetica", 8)
    canvas.drawString(40, 25, "Source: targets_prime/reports/REPORT.md; native evidence: targets_prime/runs/")
    canvas.drawRightString(555, 25, str(doc.page))
SimpleDocTemplate(str(root / "REPORT.pdf"), pagesize=A4, leftMargin=40, rightMargin=40,
                  topMargin=38, bottomMargin=40).build(story, onFirstPage=footer, onLaterPages=footer)
