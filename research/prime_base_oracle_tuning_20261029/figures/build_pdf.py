# Layout only: renders a Markdown report and the recorded-value figures (PNG) into a PDF.
# Usage, from the study directory: python3 figures/build_pdf.py README.md README.pdf
import os, re, sys
from reportlab.platypus import SimpleDocTemplate, Paragraph, Image, Table, TableStyle
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet
from reportlab.lib.pagesizes import A4

src, dst = sys.argv[1], sys.argv[2]
base = os.path.dirname(os.path.abspath(src))
st = getSampleStyleSheet(); body = st["BodyText"]; body.fontSize = 8.5; body.leading = 11
cell = st["BodyText"].clone("cell"); cell.fontSize = 7; cell.leading = 8.5
out, tbl = [], []


def flush():
    global tbl
    if tbl:
        rows = [[Paragraph(inline(c.strip()), cell) for c in r.strip().strip("|").split("|")]
                for r in tbl if not re.match(r"^\|[\s:|-]+\|$", r.strip())]
        t = Table(rows, colWidths=[523 / len(rows[0])] * len(rows[0]), repeatRows=1)
        t.setStyle(TableStyle([("GRID", (0, 0), (-1, -1), 0.3, colors.grey),
                               ("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#eef2f7")),
                               ("VALIGN", (0, 0), (-1, -1), "TOP")]))
        out.append(t); tbl = []


def inline(t):
    t = t.replace("&", "&amp;").replace("<", "&lt;")
    t = re.sub(r"\*\*([^*]+)\*\*", r"<b>\1</b>", t)
    t = re.sub(r"\[([^\]]+)\]\([^)]+\)", r"\1", t)
    return re.sub(r"`([^`]*)`", r'<font face="Courier">\1</font>', t)


for ln in open(src).read().split("\n"):
    if ln.startswith("|"):
        tbl.append(ln); continue
    flush()
    m = re.match(r"!\[[^\]]*\]\(([^)]+)\.svg\)", ln)
    if m:
        png = os.path.normpath(os.path.join(base, m.group(1) + ".png"))
        img = Image(png); s = 520 / img.drawWidth
        img.drawWidth *= s; img.drawHeight *= s
        out.append(img)
    elif ln.startswith("# "):
        out.append(Paragraph(inline(ln[2:]), st["Title"]))
    elif ln.startswith("#"):
        out.append(Paragraph(inline(ln.lstrip("# ")), st["Heading3"]))
    elif ln.strip():
        out.append(Paragraph(inline(ln), body))
flush()
SimpleDocTemplate(dst, pagesize=A4, leftMargin=36, rightMargin=36, topMargin=36, bottomMargin=36).build(out)
