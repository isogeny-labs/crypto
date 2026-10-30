#!/usr/bin/env python3
"""Render README.md and program.svg into REPORT.pdf with reportlab.

Fallback used because pandoc has no PDF engine in the container that wrote
this directory (see README.md "About the PDF"). It understands only the
Markdown this README uses (headings, paragraphs, pipe tables, bullet and
numbered lists, **bold**, `code`, a single image line) and only the SVG subset
program.svg uses (rect, line, path with M/L, circle, text, two arrow markers).
The light colour scheme is used.

Usage: python3 build_pdf.py [README.md] [REPORT.pdf]
"""
import math
import os
import re
import sys
import xml.etree.ElementTree as ET

from reportlab.lib import colors
from reportlab.lib.enums import TA_LEFT
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib.units import mm
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (Flowable, HRFlowable, Paragraph, SimpleDocTemplate,
                                Spacer, Table, TableStyle)

HERE = os.path.dirname(os.path.abspath(__file__))
FONT_DIR = "/usr/share/fonts/truetype/freefont"
pdfmetrics.registerFont(TTFont("Sans", os.path.join(FONT_DIR, "FreeSans.ttf")))
pdfmetrics.registerFont(TTFont("Sans-Bold", os.path.join(FONT_DIR, "FreeSansBold.ttf")))
pdfmetrics.registerFont(TTFont("Mono", os.path.join(FONT_DIR, "FreeMono.ttf")))
pdfmetrics.registerFontFamily("Sans", normal="Sans", bold="Sans-Bold",
                              italic="Sans", boldItalic="Sans-Bold")

BODY = ParagraphStyle("body", fontName="Sans", fontSize=9.5, leading=12.5,
                      alignment=TA_LEFT, spaceAfter=5)
CELL = ParagraphStyle("cell", parent=BODY, fontSize=8, leading=10, spaceAfter=0)
H = {1: ParagraphStyle("h1", parent=BODY, fontName="Sans-Bold", fontSize=16, leading=20,
                       spaceBefore=4, spaceAfter=8),
     2: ParagraphStyle("h2", parent=BODY, fontName="Sans-Bold", fontSize=12.5, leading=16,
                       spaceBefore=8, spaceAfter=5),
     3: ParagraphStyle("h3", parent=BODY, fontName="Sans-Bold", fontSize=10.5, leading=14,
                       spaceBefore=6, spaceAfter=4)}


def inline(text):
    text = text.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")
    text = re.sub(r"`([^`]+)`", r'<font face="Mono">\1</font>', text)
    text = re.sub(r"\*\*([^*]+)\*\*", r"<b>\1</b>", text)
    text = re.sub(r"(?<![\w*])\*([^*\s][^*]*)\*(?![\w*])", r"\1", text)
    return text


# ---------------------------------------------------------------- SVG drawing
LIGHT = {
    "bg": dict(fill="#ffffff"), "lane": dict(fill="#f2f4f7"),
    "box": dict(fill="#ffffff", stroke="#4a5568", sw=1.2),
    "out": dict(fill="#eaf2fd", stroke="#2b6cb0", sw=1.6),
    "own": dict(fill="#fff8e6", stroke="#b7791f", sw=1.4),
    "a": dict(stroke="#2d3748", sw=1.6), "ad": dict(stroke="#2d3748", sw=1.4, dash=(6, 4)),
    "ar": dict(stroke="#c53030", sw=1.5, dash=(4, 3)), "dot": dict(fill="#c53030"),
}
TEXT = {None: ("Sans", 12, "#1a202c"), "h": ("Sans-Bold", 13, "#1a202c"),
        "lab": ("Sans-Bold", 13, "#2d3748"), "m": ("Sans", 11, "#4a5568"),
        "blk": ("Sans-Bold", 11.5, "#c53030"), "ttl": ("Sans-Bold", 17, "#1a202c"),
        "doti": ("Sans-Bold", 11, "#ffffff")}
NS = "{http://www.w3.org/2000/svg}"


class SvgFlowable(Flowable):
    def __init__(self, path, width):
        super().__init__()
        self.root = ET.parse(path).getroot()
        vb = [float(v) for v in self.root.get("viewBox").split()]
        self.vw, self.vh = vb[2], vb[3]
        self.scale = width / self.vw
        self.width, self.height = width, self.vh * self.scale

    def _arrow(self, c, x0, y0, x1, y1, color):
        ang = math.atan2(y1 - y0, x1 - x0)
        ln, hw = 9.0, 4.0
        bx, by = x1 - ln * math.cos(ang), y1 - ln * math.sin(ang)
        px, py = -math.sin(ang) * hw, math.cos(ang) * hw
        c.setFillColor(colors.HexColor(color))
        p = c.beginPath()
        p.moveTo(x1, y1)
        p.lineTo(bx + px, by + py)
        p.lineTo(bx - px, by - py)
        p.close()
        c.drawPath(p, stroke=0, fill=1)

    def draw(self):
        c = self.canv
        c.saveState()
        c.translate(0, self.height)
        c.scale(self.scale, -self.scale)  # SVG coordinates from here on
        for el in self.root.iter():
            tag = el.tag.replace(NS, "")
            cls = el.get("class")
            st = LIGHT.get(cls, {})
            if tag in ("rect", "circle"):
                fill, stroke = st.get("fill"), st.get("stroke")
                if fill:
                    c.setFillColor(colors.HexColor(fill))
                if stroke:
                    c.setStrokeColor(colors.HexColor(stroke))
                    c.setLineWidth(st.get("sw", 1))
                if tag == "rect":
                    x, y = float(el.get("x")), float(el.get("y"))
                    w, h = float(el.get("width")), float(el.get("height"))
                    r = float(el.get("rx", 0))
                    c.roundRect(x, y, w, h, r, stroke=1 if stroke else 0, fill=1 if fill else 0)
                else:
                    c.circle(float(el.get("cx")), float(el.get("cy")), float(el.get("r")),
                             stroke=0, fill=1)
            elif tag in ("line", "path") and cls in ("a", "ad", "ar"):
                if tag == "line":
                    pts = [(float(el.get("x1")), float(el.get("y1"))),
                           (float(el.get("x2")), float(el.get("y2")))]
                else:
                    nums = [float(v) for v in re.findall(r"-?\d+(?:\.\d+)?", el.get("d"))]
                    pts = list(zip(nums[0::2], nums[1::2]))
                c.setStrokeColor(colors.HexColor(st["stroke"]))
                c.setLineWidth(st["sw"])
                c.setDash(*st["dash"]) if "dash" in st else c.setDash()
                p = c.beginPath()
                p.moveTo(*pts[0])
                for q in pts[1:]:
                    p.lineTo(*q)
                c.drawPath(p, stroke=1, fill=0)
                c.setDash()
                if el.get("marker-end"):
                    self._arrow(c, *pts[-2], *pts[-1], st["stroke"])
                if el.get("marker-start"):
                    self._arrow(c, *pts[1], *pts[0], st["stroke"])
            elif tag == "text" and el.text:
                font, size, color = TEXT.get(cls, TEXT[None])
                x, y = float(el.get("x")), float(el.get("y"))
                c.saveState()
                c.translate(x, y)
                c.scale(1, -1)
                c.setFillColor(colors.HexColor(color))
                c.setFont(font, size)
                if el.get("text-anchor") == "middle":
                    c.drawCentredString(0, 0, el.text)
                else:
                    c.drawString(0, 0, el.text)
                c.restoreState()
        c.restoreState()


# ------------------------------------------------------------ Markdown blocks
def build(md_path, out_path):
    lines = open(md_path, encoding="utf-8").read().splitlines()
    doc = SimpleDocTemplate(out_path, pagesize=A4, leftMargin=16 * mm, rightMargin=16 * mm,
                            topMargin=16 * mm, bottomMargin=16 * mm,
                            title="Prime-field IC program, 2026-10-29",
                            author="prime_ic_program_20261029 (rendered by build_pdf.py)")
    fw = doc.width
    story, para, i = [], [], 0

    def flush():
        if para:
            story.append(Paragraph(inline(" ".join(para)), BODY))
            para.clear()

    while i < len(lines):
        ln = lines[i]
        s = ln.strip()
        if not s:
            flush(); i += 1; continue
        m = re.match(r"^(#{1,3}) (.*)", ln)
        if m:
            flush(); story.append(Paragraph(inline(m.group(2)), H[len(m.group(1))])); i += 1; continue
        if s == "---":
            flush(); story.append(HRFlowable(width="100%", thickness=0.6, color=colors.grey,
                                             spaceBefore=4, spaceAfter=6)); i += 1; continue
        m = re.match(r"^!\[[^\]]*\]\(([^)]+)\)", s)
        if m:
            flush(); story.append(SvgFlowable(os.path.join(os.path.dirname(md_path), m.group(1)), fw))
            story.append(Spacer(1, 6)); i += 1; continue
        if s.startswith("|"):
            flush()
            rows = []
            while i < len(lines) and lines[i].strip().startswith("|"):
                cells = [x.strip() for x in lines[i].strip().strip("|").split("|")]
                if not all(re.fullmatch(r":?-+:?", x) for x in cells):
                    rows.append(cells)
                i += 1
            ncol = len(rows[0])
            data = [[Paragraph(("<b>%s</b>" if r == 0 else "%s") % inline(x), CELL) for x in row]
                    for r, row in enumerate(rows)]
            lens = [max(len(row[k]) for row in rows) for k in range(ncol)]
            tot = float(sum(min(L, 120) for L in lens))
            widths = [fw * max(0.08, min(L, 120) / tot) for L in lens]
            widths = [w * fw / sum(widths) for w in widths]
            t = Table(data, colWidths=widths, repeatRows=1)
            t.setStyle(TableStyle([
                ("GRID", (0, 0), (-1, -1), 0.4, colors.HexColor("#a0aec0")),
                ("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#edf2f7")),
                ("VALIGN", (0, 0), (-1, -1), "TOP"),
                ("LEFTPADDING", (0, 0), (-1, -1), 3), ("RIGHTPADDING", (0, 0), (-1, -1), 3)]))
            story.append(t); story.append(Spacer(1, 6)); continue
        m = re.match(r"^(\s*)(-|\d+\.) (.*)", ln)
        if m:
            flush()
            indent = len(m.group(1))
            bullet = "•" if m.group(2) == "-" else m.group(2)
            text = [m.group(3)]
            i += 1
            while i < len(lines) and lines[i].strip() and not re.match(r"^\s*(-|\d+\.) ", lines[i]) \
                    and lines[i].startswith(" " * (indent + 2)):
                text.append(lines[i].strip()); i += 1
            level = 1 + indent // 3
            style = ParagraphStyle("li%d" % level, parent=BODY, leftIndent=12 * level,
                                   bulletIndent=12 * level - 10, spaceAfter=2)
            story.append(Paragraph(inline(" ".join(text)), style, bulletText=bullet))
            continue
        if ln.startswith("   ") and story and isinstance(story[-1], Paragraph) \
                and story[-1].style.name.startswith("li"):
            flush()
            text = []
            while i < len(lines) and lines[i].strip() and lines[i].startswith("   "):
                text.append(lines[i].strip()); i += 1
            story.append(Paragraph(inline(" ".join(text)),
                                   ParagraphStyle("lic", parent=BODY, leftIndent=12)))
            continue
        para.append(s); i += 1
    flush()

    def footer(canvas, d):
        canvas.saveState()
        canvas.setFont("Sans", 7.5)
        canvas.setFillColor(colors.grey)
        canvas.drawString(16 * mm, 9 * mm, "prime_ic_program_20261029/REPORT.pdf - rendered from README.md "
                          "by build_pdf.py (reportlab, not pandoc) - preparation only")
        canvas.drawRightString(A4[0] - 16 * mm, 9 * mm, str(d.page))
        canvas.restoreState()

    doc.build(story, onFirstPage=footer, onLaterPages=footer)


if __name__ == "__main__":
    md = sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "README.md")
    out = sys.argv[2] if len(sys.argv) > 2 else os.path.join(HERE, "REPORT.pdf")
    build(md, out)
    print("wrote", out)
