"""Lay out README.md as REPORT.pdf (reportlab platypus; no LaTeX/typst/wkhtmltopdf engine on the host, so `pandoc -o x.pdf`
could not produce a PDF). Handles the Markdown subset the README uses: # headings, paragraphs, - bullets, | tables, ``` code,
`code`, **bold**, and ![](image) (PNG). Computes nothing. Usage: python3 -I build_pdf.py <README.md> <out.pdf>"""
import re, sys, os
from xml.sax.saxutils import escape
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.units import cm
from reportlab.lib import colors
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, Preformatted, Image, ListFlowable, ListItem
from PIL import Image as PI
src, out = sys.argv[1], sys.argv[2]; base = os.path.dirname(os.path.abspath(src))
ss = getSampleStyleSheet()
body = ParagraphStyle("b", parent=ss["BodyText"], fontSize=9, leading=12)
cell = ParagraphStyle("c", parent=body, fontSize=7, leading=8.6)
code = ParagraphStyle("code", fontName="Courier", fontSize=7.2, leading=9, backColor=colors.HexColor("#f4f3ee"))
H = {1: ParagraphStyle("h1", parent=ss["Heading1"], fontSize=15), 2: ParagraphStyle("h2", parent=ss["Heading2"], fontSize=12), 3: ParagraphStyle("h3", parent=ss["Heading3"], fontSize=10)}
def inline(t):
    t = escape(t)
    t = re.sub(r"`([^`]+)`", r'<font face="Courier" size="8">\1</font>', t)
    t = re.sub(r"\*\*([^*]+)\*\*", r"<b>\1</b>", t)
    t = re.sub(r"\[([^\]]+)\]\(([^)]+)\)", r"\1", t)
    return t
lines = open(src, encoding="utf-8").read().split("\n"); story = []; i = 0
while i < len(lines):
    l = lines[i]
    if l.startswith("```"):
        j = i + 1; buf = []
        while not lines[j].startswith("```"): buf.append(lines[j]); j += 1
        story += [Preformatted("\n".join(buf), code), Spacer(1, 4)]; i = j + 1; continue
    m = re.match(r"^(#{1,3}) (.*)", l)
    if m: story.append(Paragraph(inline(m.group(2)), H[len(m.group(1))])); i += 1; continue
    m = re.match(r"^!\[[^\]]*\]\(([^)]+)\)", l)
    if m:
        p = os.path.join(base, m.group(1)); w, h = PI.open(p).size; W = 17.4 * cm
        story += [Image(p, width=W, height=W * h / w), Spacer(1, 4)]; i += 1; continue
    if l.startswith("|"):
        rows = []
        while i < len(lines) and lines[i].startswith("|"):
            if not re.match(r"^\|[\s\-|:]+\|$", lines[i]): rows.append([Paragraph(inline(c.strip()), cell) for c in lines[i].strip().strip("|").split("|")])
            i += 1
        n = max(len(r) for r in rows); rows = [r + [""] * (n - len(r)) for r in rows]
        t = Table(rows, colWidths=[17.4 * cm / n] * n, repeatRows=1)
        t.setStyle(TableStyle([("GRID", (0, 0), (-1, -1), 0.3, colors.grey), ("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#e9e8e1")), ("VALIGN", (0, 0), (-1, -1), "TOP")]))
        story += [t, Spacer(1, 6)]; continue
    if re.match(r"^[-*] ", l):
        items = []
        while i < len(lines) and re.match(r"^[-*] |^  \S", lines[i]):
            if re.match(r"^[-*] ", lines[i]): items.append(lines[i][2:])
            else: items[-1] += " " + lines[i].strip()
            i += 1
        story += [ListFlowable([ListItem(Paragraph(inline(x), body)) for x in items], bulletType="bullet", bulletFontSize=6, leftIndent=12), Spacer(1, 4)]; continue
    if l.strip() == "": i += 1; continue
    buf = [l]; i += 1
    while i < len(lines) and lines[i].strip() and not re.match(r"^(#|\||```|[-*] |!\[)", lines[i]): buf.append(lines[i]); i += 1
    story += [Paragraph(inline(" ".join(buf)), body), Spacer(1, 4)]
SimpleDocTemplate(out, pagesize=A4, leftMargin=1.8 * cm, rightMargin=1.8 * cm, topMargin=1.6 * cm, bottomMargin=1.6 * cm, title="GHS cover census n = 4").build(story)
print("wrote", out)
