"""Render the review qualification; performs no scientific computation."""
from pathlib import Path
import html
import re

from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.enums import TA_LEFT
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer


def main():
    root = Path(__file__).resolve().parent
    styles = getSampleStyleSheet()
    styles.add(ParagraphStyle("ReviewBody", fontName="Helvetica", fontSize=10,
                              leading=14, spaceAfter=9, alignment=TA_LEFT))
    story = []
    for block in (root / "README.md").read_text().split("\n\n"):
        block = block.strip()
        if not block:
            continue
        style = styles["ReviewBody"]
        if block.startswith("# "):
            style, block = styles["Title"], block[2:]
        elif block.startswith("## "):
            style, block = styles["Heading2"], block[3:]
        block = re.sub(r"\[([^]]+)\]\(([^)]+)\)", r"\1", block)
        block = html.escape(block.replace("`", ""))
        story.append(Paragraph(block, style))
    doc = SimpleDocTemplate(str(root / "qualification.pdf"),
                            title="Validation sweep D review qualification",
                            author="Repository review", leftMargin=45,
                            rightMargin=45, topMargin=40, bottomMargin=40)
    def footer(canvas, document):
        canvas.saveState()
        canvas.setFont("Helvetica", 8)
        canvas.setFillColor(colors.HexColor("#555555"))
        canvas.drawString(45, 24, "Review qualification | 2026-10-20 UTC")
        canvas.drawRightString(document.pagesize[0] - 45, 24, str(document.page))
        canvas.restoreState()
    doc.build(story, onFirstPage=footer, onLaterPages=footer)


if __name__ == "__main__":
    main()
