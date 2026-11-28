#!/usr/bin/env python3
"""Render recorded data and the Markdown report. No research computation."""
from pathlib import Path
import html, json, re, subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from reportlab.lib import colors
from reportlab.lib.enums import TA_LEFT
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.utils import ImageReader
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, Image, Preformatted, KeepTogether, CondPageBreak

HERE = Path(__file__).resolve().parent
BASE_URL = "https://github.com/isogeny-labs/crypto/blob/lane/vitse-prime31-compatibility/research/vitse_prime31_compatibility_20261123/"
FONTDIR = Path("/usr/share/fonts/truetype/dejavu")
for name, file in [
    ("DV", "DejaVuSans.ttf"), ("DV-Bold", "DejaVuSans-Bold.ttf"),
    ("DV-Italic", "DejaVuSans-Oblique.ttf"), ("DV-Mono", "DejaVuSansMono.ttf")
]:
    pdfmetrics.registerFont(TTFont(name, str(FONTDIR / file)))
pdfmetrics.registerFontFamily("DV", normal="DV", bold="DV-Bold", italic="DV-Italic", boldItalic="DV-Bold")
INK = colors.HexColor("#153247")
ACCENT = colors.HexColor("#1e6a72")
LIGHT = colors.HexColor("#eef4f6")
GRAY = colors.HexColor("#526672")
WIDTH = A4[0] - 86

def make_figures():
    # These are pre-recorded Rust outputs. No crypto or new experiment here.
    rows = [json.loads(line) for line in
            (HERE / "evidence/qualification-01/prime31-audit/stdout.txt").read_text().splitlines()]
    audits = [r for r in rows if r["type"] == "selector_map_audit"]
    labels = {"identity": "Identity", "coordinate_negation": "−x",
              "reciprocal": "1/x", "negative_reciprocal": "−1/x"}
    names = [labels[r["map"]] for r in audits]
    counts = [r["retained"]["escaped_count"] for r in audits]
    population = audits[0]["retained"]["cardinality"]
    plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 11,
                         "axes.spines.top": False, "axes.spines.right": False})
    fig, ax = plt.subplots(figsize=(9.0, 3.5), facecolor="white")
    bars = ax.barh(names[::-1], counts[::-1],
                   color=["#b76d43", "#b76d43", "#b76d43", "#3f8371"], height=.56)
    ax.set_xlim(0, population)
    ax.set_xlabel("Escaping retained coordinates (exact count)")
    ax.set_title("Raw-selector symmetry fails after curve-point filtering", loc="left",
                 color="#153247", fontsize=13, fontweight="bold", pad=14)
    for bar, count in zip(bars, counts[::-1]):
        ax.text(max(count, 0) + 9, bar.get_y() + bar.get_height()/2,
                str(count), va="center", color="#153247", fontweight="bold")
    ax.grid(axis="x", color="#e4ebef", linewidth=.7)
    ax.set_axisbelow(True)
    ax.spines["left"].set_visible(False)
    ax.spines["bottom"].set_color("#b9cbd3")
    fig.text(.12, .005,
             f"qualification-01 · full retained population n={population} · no sampling uncertainty; no timing claim",
             fontsize=8, color="#526672")
    fig.tight_layout(rect=(0,.06,1,1))
    fig.savefig(HERE/"closure_counts.svg", metadata={"Date": None})
    svg = HERE / "closure_counts.svg"
    svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
    fig.savefig(HERE/"closure_counts.png", dpi=180)
    plt.close(fig)
    subprocess.run(["dot", "-Tsvg", str(HERE/"diagram.dot"), "-o", str(HERE/"diagram.svg")], check=True)
    subprocess.run(["dot", "-Tpng", "-Gdpi=160", str(HERE/"diagram.dot"), "-o", str(HERE/"diagram.png")], check=True)

styles = getSampleStyleSheet()
styles.add(ParagraphStyle("BodyDV", fontName="DV", fontSize=9.3, leading=14.2,
                          textColor=INK, spaceAfter=8, splitLongWords=True))
styles.add(ParagraphStyle("TitleDV", parent=styles["BodyDV"], fontName="DV-Bold",
                          fontSize=24, leading=29, spaceAfter=14))
styles.add(ParagraphStyle("HeadingDV", parent=styles["BodyDV"], fontName="DV-Bold",
                          fontSize=14, leading=19, spaceBefore=14, spaceAfter=8, keepWithNext=True))
styles.add(ParagraphStyle("SmallDV", parent=styles["BodyDV"], fontSize=7.6, leading=11, textColor=GRAY))
styles.add(ParagraphStyle("CellDV", parent=styles["BodyDV"], fontSize=7.6, leading=10.5, spaceAfter=0))
styles.add(ParagraphStyle("CellHeadDV", parent=styles["CellDV"], fontName="DV-Bold", textColor=colors.white))
styles.add(ParagraphStyle("CodeDV", fontName="DV-Mono", fontSize=7.2, leading=10,
                          backColor=LIGHT, borderPadding=8, spaceBefore=5, spaceAfter=9))

def inline(text):
    text = html.escape(text)
    text = re.sub(r"\[([^\]]+)\]\(([^)]+)\)",
                  lambda m: '<link href="'+html.escape(m.group(2) if "://" in m.group(2)
                                                       else BASE_URL+m.group(2), quote=True)
                            +'" color="#1e6a72">'+m.group(1)+'</link>', text)
    text = re.sub(r"\*\*(.+?)\*\*", r"<b>\1</b>", text)
    text = re.sub(r"`([^`]+)`", r'<font name="DV-Mono">\1</font>', text)
    return text

def table(rows):
    columns = len(rows[0])
    fractions = {
        2: [.30,.70], 3: [.25,.48,.27], 4: [.24,.26,.27,.23],
        5: [.21,.16,.18,.19,.26]
    }.get(columns, [1/columns]*columns)
    data = [[Paragraph(inline(c), styles["CellHeadDV" if i==0 else "CellDV"])
             for c in row] for i,row in enumerate(rows)]
    t = Table(data, colWidths=[WIDTH*x for x in fractions], repeatRows=1, hAlign="LEFT")
    t.setStyle(TableStyle([
        ("BACKGROUND",(0,0),(-1,0),ACCENT),
        ("VALIGN",(0,0),(-1,-1),"TOP"),
        ("LEFTPADDING",(0,0),(-1,-1),7),("RIGHTPADDING",(0,0),(-1,-1),7),
        ("TOPPADDING",(0,0),(-1,-1),7),("BOTTOMPADDING",(0,0),(-1,-1),7),
        ("ROWBACKGROUNDS",(0,1),(-1,-1),[colors.white,LIGHT]),
        ("LINEBELOW",(0,0),(-1,0),.6,ACCENT),
        ("LINEBELOW",(0,-1),(-1,-1),.4,colors.HexColor("#b7c9d0")),
    ]))
    return t

def story_from_markdown(text):
    lines = text.splitlines()
    story = []
    i = 0
    while i < len(lines):
        line=lines[i]
        if not line.strip():
            i+=1
            continue
        if line.startswith("# "):
            story.append(Paragraph("RESEARCH NOTE  /  ORDINARY PRIME31", styles["SmallDV"]))
            story.append(Paragraph(inline(line[2:]), styles["TitleDV"]))
            i+=1
        elif line.startswith("## "):
            story.append(CondPageBreak(250))
            story.append(Paragraph(inline(line[3:]), styles["HeadingDV"]))
            i+=1
        elif line.startswith("```"):
            code=[]; i+=1
            while i<len(lines) and not lines[i].startswith("```"):
                code.append(lines[i]); i+=1
            story.append(Preformatted("\n".join(code),styles["CodeDV"]))
            i+=1
        elif line.startswith("|"):
            rows=[]
            while i<len(lines) and lines[i].startswith("|"):
                cells=[c.strip() for c in lines[i].strip().strip("|").split("|")]
                if not all(re.fullmatch(r"[: -]+", c) for c in cells):
                    rows.append(cells)
                i+=1
            t = table(rows)
            story.append(KeepTogether([t]) if len(rows) <= 6 else t)
            story.append(Spacer(1,9))
        elif line.startswith("!["):
            match=re.match(r"!\[([^\]]*)\]\(([^)]+)\)",line)
            png=(HERE/match.group(2)).with_suffix(".png")
            iw,ih=ImageReader(str(png)).getSize()
            width=WIDTH; height=width*ih/iw
            if height>260:
                width*=260/height; height=260
            im=Image(str(png),width=width,height=height)
            im.hAlign="CENTER"
            caption=Paragraph(inline(match.group(1)),styles["SmallDV"])
            story.append(KeepTogether([im,Spacer(1,5),caption,Spacer(1,7)])); i+=1
        else:
            para=[line]; i+=1
            while i<len(lines) and lines[i].strip() and not lines[i].startswith(("#","|","![","```")):
                para.append(lines[i]); i+=1
            story.append(Paragraph(inline(" ".join(para)),styles["BodyDV"]))
    return story

def page(canvas,doc):
    canvas.saveState()
    canvas.setStrokeColor(colors.HexColor("#d5e2e7"))
    canvas.line(43,42,A4[0]-43,42)
    canvas.setFont("DV",7)
    canvas.setFillColor(GRAY)
    canvas.drawString(43,29,"Vitse / Prime31 compatibility • 23 Nov 2026 • unsigned development evidence")
    canvas.drawRightString(A4[0]-43,29,str(doc.page))
    canvas.restoreState()

def main():
    make_figures()
    doc=SimpleDocTemplate(str(HERE/"REPORT.pdf"),pagesize=A4,
                          rightMargin=43,leftMargin=43,topMargin=40,bottomMargin=56,
                          title="Vitse techniques and the ordinary Prime31 target",
                          author="Isogeny Labs research audit")
    doc.build(story_from_markdown((HERE/"REPORT.md").read_text()),onFirstPage=page,onLaterPages=page)
    print(HERE/"REPORT.pdf")

if __name__=="__main__":
    main()
