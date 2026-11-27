#!/usr/bin/env python3
"""Document layout only: plot recorded native ranks and render the report."""
from pathlib import Path
from html import escape
import json
import re
import tempfile

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
from reportlab.lib import colors
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (
    SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle,
    Preformatted, Image, KeepTogether,
)

ROOT = Path(__file__).resolve().parent
TEMP = Path(tempfile.mkdtemp(prefix="m31-polar-report-"))
DATA = json.loads((ROOT / "evidence/attempt-01/check/stdout.txt").read_text())
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10,
                     "svg.fonttype": "none"})
BLUE, TEAL, GRAY = "#173e5b", "#007f82", "#596a77"

fig, ax = plt.subplots(figsize=(9.2, 4.8), layout="constrained")
ax.set(xlim=(0, 10), ylim=(-.12, 5.4))
ax.axis("off")

def box(x, y, w, h, title, body, color=BLUE):
    ax.add_patch(FancyBboxPatch((x, y), w, h,
                 boxstyle="round,pad=0.08,rounding_size=0.08",
                 facecolor="#f2f7fa", edgecolor=color, linewidth=1.3))
    ax.text(x+w/2, y+h-.24, title, ha="center", va="center",
            weight="bold", color=color, fontsize=11)
    ax.text(x+w/2, y+.29, body, ha="center", va="center",
            color=GRAY, fontsize=9.5)

def arrow(a, b):
    ax.annotate("", xy=b, xytext=a,
                arrowprops={"arrowstyle":"-|>", "color":GRAY, "lw":1.3})

box(2.35,4.24,5.3,.95,"Original regular RR5 chart",
    "Three parameters; four distinct original abscissae")
box(2.35,2.91,5.3,.95,"Ten quartic Graeffe stages",
    "Intermediate images may collide; terminal value is fixed")
box(.22,1.43,4.45,1.05,"Keep the original equations",
    "F1 = F2 = F3 = F4 = 0\nOriginal Jacobian rank at most one")
box(5.18,1.43,4.45,1.05,"Add six polar equations",
    "All total second partials of centered F3\nTheir Jacobian has rank three",TEAL)
box(2.35,.05,5.3,.92,"Same regular solution set",
    "Local multiplicity removed; solver cost still to be tested",TEAL)
arrow((5,4.13),(5,3.97))
arrow((4.1,2.8),(2.6,2.59))
arrow((5.9,2.8),(7.3,2.59))
arrow((2.6,1.31),(4.1,1.08))
arrow((7.3,1.31),(5.9,1.08))
fig.savefig(ROOT/"mechanism.svg")
fig.savefig(TEMP/"mechanism.png",dpi=160)
plt.close(fig)

cases=DATA["cases"]
fig, axes=plt.subplots(1,2,figsize=(9.2,3.45),layout="constrained",
                       gridspec_kw={"width_ratios":[1.65,1]})
xs=[r["case"] for r in cases]
axes[0].plot(xs,[r["original_collapsed_rank"] for r in cases],"o-",
             color=BLUE,label="Original equations",markersize=4)
axes[0].plot(xs,[r["f3_hessian_jacobian_rank"] for r in cases],"s-",
             color=TEAL,label="Six polar equations",markersize=4)
axes[0].set(title="Sixteen prescribed M31 quartets",xlabel="Fixed case ID",
            ylabel="Exact Jacobian rank",xticks=[0,3,6,9,12,15],
            yticks=[0,1,2,3],ylim=(-.1,3.55))
axes[0].legend(frameon=False,loc="center right",fontsize=8.5)
selected=[next(r for r in DATA["synthetic_controls"] if r["name"]==name)
          for name in ["f2_rank_failure_nonzero_normal","zero_normal_coordinates_out_of_scope"]]
axes[1].bar([-.18,.82],[r["f2_plus_f1_rank"] for r in selected],.34,
            color=BLUE,label="F1 + three F2 derivatives")
axes[1].bar([.18,1.18],[r["f3_hessian_jacobian_rank"] for r in selected],.34,
            color=TEAL,label="Six polar equations")
axes[1].set(title="Synthetic boundary controls",xticks=[0,1],
            xticklabels=["Nonzero\nexceptional normal","Out-of-scope\nzero coordinates"],
            yticks=[0,1,2,3],ylim=(-.1,4.1))
axes[1].legend(frameon=False,loc="upper right",fontsize=7)
axes[1].tick_params(axis="x",labelsize=8)
for a in axes:
    a.spines[["top","right"]].set_visible(False)
    a.grid(axis="y",color="#dce4e9",linewidth=.6)
    a.set_axisbelow(True)
fig.savefig(ROOT/"rank_controls.svg")
fig.savefig(TEMP/"rank_controls.png",dpi=160)
plt.close(fig)

fontroot=Path('/usr/share/fonts/truetype/dejavu')
for name, filename in [('ReportSans','DejaVuSans.ttf'),
                       ('ReportSans-Bold','DejaVuSans-Bold.ttf'),
                       ('ReportMono','DejaVuSansMono.ttf')]:
    pdfmetrics.registerFont(TTFont(name,str(fontroot/filename)))
pdfmetrics.registerFontFamily('ReportSans',normal='ReportSans',
    bold='ReportSans-Bold',italic='ReportSans',boldItalic='ReportSans-Bold')
styles=getSampleStyleSheet()
styles.add(ParagraphStyle(name="ReportTitle",fontName="ReportSans-Bold",
    fontSize=21,leading=26,textColor=colors.HexColor(BLUE),spaceAfter=13))
styles.add(ParagraphStyle(name="ReportBody",fontName="ReportSans",fontSize=9.1,
    leading=12.8,spaceAfter=6.5,splitLongWords=True))
styles.add(ParagraphStyle(name="ReportSmall",fontName="ReportSans",fontSize=7.5,
    leading=10.3,spaceAfter=4,splitLongWords=True))
styles.add(ParagraphStyle(name="ReportCode",fontName="ReportMono",fontSize=7.3,
    leading=10,leftIndent=9,spaceAfter=8))
for name,size,leading in [("Heading2",13,17),("Heading3",10.5,14)]:
    styles[name].fontName="ReportSans-Bold"
    styles[name].fontSize=size
    styles[name].leading=leading
    styles[name].textColor=colors.HexColor(BLUE)
    styles[name].keepWithNext=True
    styles[name].spaceBefore=11
    styles[name].spaceAfter=6
BASE="https://github.com/isogeny-labs/crypto/blob/lane/m31-structural-lead-search-20261123/research/m31_graeffe_polar_deflation_20261123/"

def rich(text):
    text=escape(text)
    text=re.sub(r"\[([^\]]+)\]\(([^)]+)\)",lambda m:
        '<link href="'+(m[2] if m[2].startswith("https:") else BASE+m[2])+
        '" color="#007f82">'+m[1]+'</link>',text)
    text=re.sub(r"`([^`]+)`",r'<font name="ReportMono">\1</font>',text)
    return re.sub(r"\*\*([^*]+)\*\*",r'<b>\1</b>',text)

def para(text,small=False):
    return Paragraph(rich(text),styles["ReportSmall" if small else "ReportBody"])

story=[]
paragraphs=[]
tablelines=[]
codelines=[]

def flush_para():
    if paragraphs:
        story.append(para(" ".join(paragraphs)))
        paragraphs.clear()

def flush_code():
    if codelines:
        story.append(KeepTogether([Preformatted("\n".join(codelines),styles["ReportCode"])]))
        codelines.clear()

def flush_table():
    if not tablelines: return
    cells=[]
    for line in tablelines:
        parts=[v.strip() for v in line.strip().strip("|").split("|")]
        if all(re.fullmatch(r"[-: ]+",v) for v in parts): continue
        cells.append([para(v,True) for v in parts])
    widths={2:[151,337],3:[151,127,210]}[len(cells[0])]
    table=Table(cells,colWidths=widths,repeatRows=1,hAlign="LEFT")
    table.setStyle(TableStyle([
        ("BACKGROUND",(0,0),(-1,0),colors.HexColor("#e6f1f5")),
        ("VALIGN",(0,0),(-1,-1),"TOP"),
        ("LEFTPADDING",(0,0),(-1,-1),7),
        ("RIGHTPADDING",(0,0),(-1,-1),7),
        ("TOPPADDING",(0,0),(-1,-1),6),
        ("BOTTOMPADDING",(0,0),(-1,-1),5),
        ("LINEBELOW",(0,0),(-1,0),.8,colors.HexColor(BLUE)),
        ("LINEBELOW",(0,1),(-1,-1),.3,colors.HexColor("#d7e1e8")),
    ]))
    story.extend([table,Spacer(1,9)])
    tablelines.clear()

for line in (ROOT/"REPORT.md").read_text().splitlines()+[""]:
    if line.startswith("|"):
        flush_para(); flush_code(); tablelines.append(line); continue
    flush_table()
    if line.startswith("    "):
        flush_para(); codelines.append(line[4:]); continue
    flush_code()
    if not line.strip(): flush_para(); continue
    if line.startswith("!["):
        flush_para()
        match=re.fullmatch(r"!\[(.*?)\]\((.*?)\.svg\)",line)
        name=match[2]
        height=244 if name=="mechanism" else 176
        caption=("Theoretical construction on the regular RR5 chart. Total derivatives retain the original parameters; repeated original roots remain excluded."
                 if name=="mechanism" else
                 "Exact recorded ranks, attempt-01; 16 fixed quartets and 2 selected synthetic controls. No timing or population estimate. PCC-20261124-f381e2, unsigned.")
        story.append(KeepTogether([Image(str(TEMP/(name+".png")),width=470,height=height),para(caption,True)]))
        continue
    if line.startswith("### "):
        flush_para(); story.append(Paragraph(rich(line[4:]),styles["Heading3"])); continue
    if line.startswith("## "):
        flush_para(); story.append(Paragraph(rich(line[3:]),styles["Heading2"])); continue
    if line.startswith("# "):
        flush_para(); story.append(Paragraph(rich(line[2:]),styles["ReportTitle"])); continue
    if line.startswith("- "):
        flush_para(); paragraphs.append("• "+line[2:]); continue
    paragraphs.append(line)

def frame(canvas,doc):
    canvas.saveState()
    canvas.setStrokeColor(colors.HexColor("#d7e1e8"))
    canvas.line(47,42,547,42)
    canvas.setFont("ReportSans",6.6)
    canvas.setFillColor(colors.HexColor(GRAY))
    canvas.drawString(47,29,"ISOGENY LABS | M31 GRAEFFE POLAR DEFLATION | THEORY + FIXED CONTROLS")
    canvas.drawRightString(547,29,str(doc.page))
    canvas.restoreState()

path=ROOT/"M31_Graeffe_Polar_Deflation.pdf"
doc=SimpleDocTemplate(str(path),pagesize=(594,792),leftMargin=47,rightMargin=47,
    topMargin=43,bottomMargin=55,title="Repairing collapsed Graeffe equations with six polar derivatives",
    author="Isogeny Labs")
doc.build(story,onFirstPage=frame,onLaterPages=frame)
print(path)
