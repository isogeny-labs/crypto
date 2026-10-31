#!/usr/bin/env python3
"""Render README.md + fig-ratios.png into REPORT.pdf (layout only, no computation)."""
import os, re
from reportlab.lib.pagesizes import A4, landscape
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib import colors
from reportlab.lib.units import mm
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, Image, Preformatted
here = os.path.dirname(os.path.abspath(__file__))
ss = getSampleStyleSheet()
body = ParagraphStyle("b", parent=ss["BodyText"], fontSize=8.5, leading=11)
cell = ParagraphStyle("c", parent=body, fontSize=6.8, leading=8.2)
h1 = ParagraphStyle("h1", parent=ss["Heading1"], fontSize=14); h2 = ParagraphStyle("h2", parent=ss["Heading2"], fontSize=11)
def inline(t):
    t = t.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")
    t = re.sub(r"\*\*(.+?)\*\*", r"<b>\1</b>", t)
    t = re.sub(r"`([^`]+)`", r"<font face='Courier'>\1</font>", t)
    return t
lines = open(os.path.join(here, "README.md")).read().split("\n")
fl = []; i = 0; W = landscape(A4)[0] - 30 * mm
while i < len(lines):
    L = lines[i]
    if L.startswith("# "): fl.append(Paragraph(inline(L[2:]), h1))
    elif L.startswith("## "):
        fl.append(Paragraph(inline(L[3:]), h2))
        if L.startswith("## Result by arm"):
            fl.append(Image(os.path.join(here, "fig-ratios.png"), width=W * 0.85, height=W * 0.85 * 0.42))
            fl.append(Paragraph("Figure: LA and whole-solve cost ratios to the incumbent, 95% bootstrap CI over curves. Source raw/analysis.json, item 15 (CARD-BLINDSPOT-15-01..09).", cell))
    elif L.startswith("### "): fl.append(Paragraph(inline(L[4:]), h2))
    elif L.startswith("|"):
        rows = []
        while i < len(lines) and lines[i].startswith("|"):
            if not re.match(r"^\|[-| ]+\|$", lines[i]): rows.append([Paragraph(inline(c.strip()), cell) for c in lines[i].strip("|").split("|")])
            i += 1
        t = Table(rows, repeatRows=1); t.setStyle(TableStyle([("GRID", (0, 0), (-1, -1), .3, colors.grey), ("BACKGROUND", (0, 0), (-1, 0), colors.lightgrey), ("VALIGN", (0, 0), (-1, -1), "TOP")]))
        fl.append(t); fl.append(Spacer(1, 4)); continue
    elif L.startswith("* "):
        txt = L[2:]
        while i + 1 < len(lines) and lines[i + 1].startswith("  "): i += 1; txt += " " + lines[i].strip()
        fl.append(Paragraph("&bull; " + inline(txt), body))
    elif L.strip():
        txt = L
        while i + 1 < len(lines) and lines[i + 1].strip() and not re.match(r"^(#|\||\* )", lines[i + 1]): i += 1; txt += " " + lines[i].strip()
        fl.append(Paragraph(inline(txt), body))
    i += 1
SimpleDocTemplate(os.path.join(here, "REPORT.pdf"), pagesize=landscape(A4), leftMargin=15 * mm, rightMargin=15 * mm, topMargin=12 * mm, bottomMargin=12 * mm, title="Item 15 genus-3 LA exponent").build(fl)
