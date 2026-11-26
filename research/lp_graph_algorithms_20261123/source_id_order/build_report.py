#!/usr/bin/env python3
"""Render the already-recorded report and diagram. Performs no research computation."""
from pathlib import Path
import html, re
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.enums import TA_LEFT
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, Image, KeepTogether
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont

here = Path(__file__).resolve().parent
font = Path("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf")
bold = Path("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf")
if font.exists() and bold.exists():
    pdfmetrics.registerFont(TTFont("Body", str(font)))
    pdfmetrics.registerFont(TTFont("BodyBold", str(bold)))
else:
    raise SystemExit("DejaVu Sans fonts are required for this report.")
styles = getSampleStyleSheet()
styles.add(ParagraphStyle(name="ReportBody", fontName="Body", fontSize=9.1, leading=13.2, spaceAfter=7, textColor=colors.HexColor("#25344A")))
styles.add(ParagraphStyle(name="ReportTitle", fontName="BodyBold", fontSize=21, leading=25, spaceAfter=14, textColor=colors.HexColor("#102B50")))
styles.add(ParagraphStyle(name="ReportHeading", fontName="BodyBold", fontSize=12.2, leading=16, spaceBefore=11, spaceAfter=7, textColor=colors.HexColor("#125780"), keepWithNext=True))
styles.add(ParagraphStyle(name="ReportCell", fontName="Body", fontSize=8, leading=11, textColor=colors.HexColor("#25344A")))
styles.add(ParagraphStyle(name="ReportSmall", fontName="Body", fontSize=7.5, leading=10, textColor=colors.HexColor("#52657A"), spaceAfter=6))
def formatted(s):
    out=html.escape(s)
    out=re.sub(r"\*\*(.+?)\*\*", r'<font name="BodyBold">\1</font>',out)
    return out
def para(s, style="ReportBody"):
    return Paragraph(formatted(s), styles[style])
def footer(c, doc):
    c.setStrokeColor(colors.HexColor("#D8E1EB")); c.line(42, 40, 570, 40)
    c.setFont("Body",7.1); c.setFillColor(colors.HexColor("#52657A"))
    c.drawString(42,27,"ISOGENY LABS  |  PR #5118  |  SOURCE-ID CORRECTNESS  |  UNSIGNED DEVELOPMENT EVIDENCE")
    c.drawRightString(570,27,str(doc.page))
lines=(here/"REPORT.md").read_text().splitlines()
story=[]; i=0
while i<len(lines):
    line=lines[i]
    if not line.strip(): i+=1; continue
    if line.startswith("# "):
        story.append(para(line[2:],"ReportTitle")); i+=1; continue
    if line.startswith("## "):
        story.append(para(line[3:],"ReportHeading")); i+=1; continue
    if line.startswith("!["):
        im=Image(str(here/"source_order.png"))
        im.drawHeight *= 510/im.drawWidth; im.drawWidth=510
        story.extend([Spacer(1,4),im,Spacer(1,7)])
        i+=1;continue
    if line.startswith("|"):
        rows=[]
        while i<len(lines) and lines[i].startswith("|"):
            row=[c.strip() for c in lines[i].strip("|").split("|")]
            if not all(re.fullmatch(r"[-: ]+",c) for c in row):
                rows.append([para(c,"ReportCell") for c in row])
            i+=1
        widths=[64,71,91,210,64] if len(rows[0])==5 else [242,258]
        t=Table(rows,colWidths=widths,repeatRows=1,hAlign="LEFT")
        t.setStyle(TableStyle([("BACKGROUND",(0,0),(-1,0),colors.HexColor("#E7EFF8")),("ROWBACKGROUNDS",(0,1),(-1,-1),[colors.white,colors.HexColor("#F5F8FC")]),("BOX",(0,0),(-1,-1),0.5,colors.HexColor("#D7E0EB")),("LINEBELOW",(0,0),(-1,0),0.7,colors.HexColor("#A9BCD1")),("VALIGN",(0,0),(-1,-1),"TOP"),("LEFTPADDING",(0,0),(-1,-1),7),("RIGHTPADDING",(0,0),(-1,-1),7),("TOPPADDING",(0,0),(-1,-1),6),("BOTTOMPADDING",(0,0),(-1,-1),6)]))
        story.extend([t,Spacer(1,9)]);continue
    block=[line];i+=1
    while i<len(lines) and lines[i].strip() and not lines[i].startswith(("#","|","![")):
        block.append(lines[i]);i+=1
    story.append(para(" ".join(block)))
doc=SimpleDocTemplate(str(here/"Graph_Witness_Source_ID_Fix.pdf"),pagesize=(612,792),leftMargin=42,rightMargin=42,topMargin=42,bottomMargin=55,title="Graph witness source-ID ordering: M31 regression and fix",author="Isogeny Labs")
doc.build(story,onFirstPage=footer,onLaterPages=footer)
print(here/"Graph_Witness_Source_ID_Fix.pdf")
