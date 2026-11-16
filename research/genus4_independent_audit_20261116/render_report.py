#!/usr/bin/env python3
"""Layout only: plot Rust-produced values and render existing report text."""
from pathlib import Path
import html
import json
import re
import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "none"  # text as text: keeps the SVG under the artifact store threshold
import matplotlib.pyplot as plt
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Image, Table, TableStyle, KeepTogether
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib import colors
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont

root=Path(__file__).resolve().parent
original=json.loads((root/"out/original-recalculation.json").read_text())
later=json.loads((root/"out/followup-recalculation.json").read_text())
def cell(data, mode):
    return next(x for p in data["per_p"] if p["p"]==8209 for x in p["prices"] if x["mode"]==mode and x["w"]==1)
panels=[("Original frozen instances: 3 targets x 4 seeds",original,
         ["Q1","strictmul","Q2"],["Frozen model","Rho native work unpriced","IC additions; rho as frozen"]),
        ("Later follow-up: 9 targets x 2 seeds",later,
         ["Q3","lean_floor","lean_actual"],["Registered rho + window","Plain rho, no window; C floored","Plain rho, no window; actual C"])]
fig,axes=plt.subplots(2,1,figsize=(9.4,6.7))
for ax,(title,data,modes,labels) in zip(axes,panels):
    records=[cell(data,m) for m in modes]
    vals=[x["ratio"] for x in records]
    ax.barh(labels,vals,color=["#225b73","#a96937","#a96937"],height=.56)
    for j,x in enumerate(records):
        lo,hi=x["paired_cluster_bootstrap95"]
        ax.plot([lo,hi],[j,j],color="#222222",lw=1.1)
        ax.text(x["ratio"]+.04,j+.24,f'{x["ratio"]:.3f}',fontsize=10)
    ax.axvline(1,color="#a33b3b",linestyle="--",lw=1)
    ax.set_xlim(0,3.05)
    ax.invert_yaxis()
    ax.set_title(title,loc="left",fontsize=11,fontweight="bold",pad=12)
    ax.set_xlabel("IC / rho model cost at weight 1; parity = 1")
    ax.spines[["top","right"]].set_visible(False)
fig.suptitle("Genus-4 advantage depends on pricing and reference",x=.02,ha="left",fontsize=14,fontweight="bold")
fig.text(.02,.018,"Rust-derived ratios and paired bootstrap intervals. Orange rows are sensitivities with omitted native work;\nnone certifies a complete calibrated price. Different panels are separate studies. Audit 2026-11-16.",fontsize=9)
fig.subplots_adjust(left=.37,right=.98,top=.89,bottom=.17,hspace=.7)
fig.savefig(root/"pricing.svg")
fig.savefig(root/"pricing.png",dpi=150)
from PIL import Image as _PIL  # 256-colour palette: keeps the PNG under the artifact store threshold
_PIL.open(root/"pricing.png").convert("RGB").quantize(256).save(root/"pricing.png", optimize=True)
plt.close(fig)

fontdir=Path("/usr/share/fonts/truetype/dejavu")
pdfmetrics.registerFont(TTFont("DV",str(fontdir/"DejaVuSans.ttf")))
pdfmetrics.registerFont(TTFont("DVB",str(fontdir/"DejaVuSans-Bold.ttf")))
pdfmetrics.registerFontFamily("DV",normal="DV",bold="DVB",italic="DV",boldItalic="DVB")
styles=getSampleStyleSheet()
styles.add(ParagraphStyle(name="BodyAudit",fontName="DV",fontSize=9,leading=11.5,spaceAfter=6,allowWidows=0,allowOrphans=0))
styles.add(ParagraphStyle(name="TitleAudit",fontName="DVB",fontSize=19,leading=23,spaceAfter=14,textColor=colors.HexColor("#173f55")))
styles.add(ParagraphStyle(name="HeadAudit",fontName="DVB",fontSize=12,leading=16,spaceBefore=11,spaceAfter=7,textColor=colors.HexColor("#173f55"),keepWithNext=True))
styles.add(ParagraphStyle(name="CellAudit",fontName="DV",fontSize=7.3,leading=10,spaceAfter=2))
def markup(s):
    s=html.escape(s.replace("`",""))
    s=re.sub(r"\[([^\]]+)\]\((https?://[^)]+)\)",r'<link href="\2" color="#225b73">\1</link>',s)
    s=re.sub(r"\*\*([^*]+)\*\*",r"<b>\1</b>",s)
    return s
lines=(root/"REPORT.md").read_text().splitlines()
story=[]; paragraph=[]; index=0; inserted=False
def flush():
    if paragraph:
        story.append(Paragraph(markup(" ".join(paragraph)),styles["BodyAudit"]))
        paragraph.clear()
while index<len(lines):
    line=lines[index]
    if line.startswith("# "):
        flush();story.append(Paragraph(markup(line[2:]),styles["TitleAudit"]))
    elif line.startswith("## "):
        flush()
        if not inserted and line=="## Evidence and method":
            story.append(Image(str(root/"pricing.png"),width=510,height=364))
            inserted=True
        story.append(Paragraph(markup(line[3:]),styles["HeadAudit"]))
    elif line.startswith("|"):
        flush();table=[]
        while index<len(lines) and lines[index].startswith("|"):
            cells=[x.strip() for x in lines[index].strip().strip("|").split("|")]
            if not all(re.fullmatch(r"[-: ]+",x) for x in cells):
                table.append([Paragraph(markup(x),styles["CellAudit"]) for x in cells])
            index+=1
        count=len(table[0]); widths=([150,360] if count==2 else [260,125,125] if count==3 else [225,95,95,95] if count==4 else [35,110,110,125,130])
        t=Table(table,colWidths=widths,repeatRows=1,hAlign="LEFT")
        t.setStyle(TableStyle([("BACKGROUND",(0,0),(-1,0),colors.HexColor("#e5eef3")),("VALIGN",(0,0),(-1,-1),"TOP"),("LINEBELOW",(0,0),(-1,0),.5,colors.HexColor("#9bb2bf")),("LINEBELOW",(0,1),(-1,-1),.2,colors.HexColor("#d7dfe4")),("LEFTPADDING",(0,0),(-1,-1),5),("RIGHTPADDING",(0,0),(-1,-1),5),("TOPPADDING",(0,0),(-1,-1),5),("BOTTOMPADDING",(0,0),(-1,-1),5)]))
        story.append(KeepTogether([t,Spacer(1,9)]));continue
    elif not line.strip():flush()
    elif line.startswith("- "):
        flush();story.append(Paragraph(markup(line[2:]),styles["BodyAudit"],bulletText="-"))
    else:paragraph.append(line.strip())
    index+=1
flush()
def footer(canvas,doc):
    canvas.setFont("DV",7)
    canvas.setFillColor(colors.HexColor("#61727c"))
    canvas.drawString(42,25,"Independent genus-4 audit | 2026-11-16 | PCC standing: unverified")
    canvas.drawRightString(552,25,str(doc.page))
SimpleDocTemplate(str(root/"REPORT.pdf"),pagesize=(612,792),rightMargin=51,leftMargin=51,topMargin=43,bottomMargin=43).build(story,onFirstPage=footer,onLaterPages=footer)
