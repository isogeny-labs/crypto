"""Renders README.md (first section through 'First run (superseded)') into REPORT.pdf with reportlab. Layout only."""
import re, html
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, Image, PageBreak
from reportlab.lib import colors
ss = getSampleStyleSheet()
body = ParagraphStyle("b", parent=ss["BodyText"], fontSize=8.6, leading=11)
cell = ParagraphStyle("c", parent=body, fontSize=7.2, leading=9)
def inline(t):
    t = html.escape(t)
    t = re.sub(r"\*\*(.+?)\*\*", r"<b>\1</b>", t)
    t = re.sub(r"`([^`]+)`", r"<font face='Courier'>\1</font>", t)
    return t
lines = open("README.md").read().split("\n")
out = []; i = 0
while i < len(lines):
    l = lines[i]
    if l.startswith("!["):
        out.append(Image("median_D_r2.png", width=500, height=500*780/1650)); i += 1; continue
    if l.startswith("|"):
        tb = []
        while i < len(lines) and lines[i].startswith("|"):
            if not re.match(r"^\|[-| ]+\|$", lines[i]): tb.append([Paragraph(inline(c.strip()), cell) for c in lines[i].strip("|").split("|")])
            i += 1
        t = Table(tb, repeatRows=1); t.setStyle(TableStyle([("GRID",(0,0),(-1,-1),0.3,colors.grey),("BACKGROUND",(0,0),(-1,0),colors.HexColor("#e5e7eb")),("VALIGN",(0,0),(-1,-1),"TOP")]))
        out += [t, Spacer(1,5)]; continue
    if l.startswith("#"):
        n = len(l) - len(l.lstrip("#")); out.append(Paragraph(inline(l.lstrip("# ")), ss["Heading%d" % min(n,3)])); i += 1; continue
    if l.startswith("* "):
        out.append(Paragraph(inline(l[2:]), body, bulletText="•")); i += 1; continue
    m = re.match(r"^(\d)\. (.*)", l)
    if m: out.append(Paragraph(inline(m.group(2)), body, bulletText=m.group(1)+".")); i += 1; continue
    if l.strip() and l != "---": out.append(Paragraph(inline(l), body))
    i += 1
SimpleDocTemplate("REPORT.pdf", pagesize=A4, leftMargin=36, rightMargin=36, topMargin=36, bottomMargin=36,
                  title="Blind spot 04 presentation search: revised run").build(out)
