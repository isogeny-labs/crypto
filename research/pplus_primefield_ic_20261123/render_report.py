"""Document rendering only: render the authored Markdown and existing figures.

No cryptographic experiment, mathematical result generation, or benchmark is
performed by this script. Native evidence is produced by the Rust qualifiers.
"""
from pathlib import Path
import re
import html
import io
import hashlib
from reportlab.pdfgen import canvas
from reportlab.platypus import (
    SimpleDocTemplate, Paragraph, Spacer, Image, Table, TableStyle,
    KeepTogether, PageBreak, HRFlowable,
)
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.enums import TA_LEFT, TA_CENTER
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from matplotlib.mathtext import math_to_image
from matplotlib.font_manager import FontProperties
from PIL import Image as PILImage

ROOT = Path(__file__).resolve().parent
REPORT = ROOT if (ROOT / 'REPORT.md').exists() else ROOT / 'publication/research/pplus_primefield_ic_20261123'
TMP = ROOT / 'tmp/pdfs'
TMP.mkdir(parents=True, exist_ok=True)
OUT = REPORT / 'Pplus_PrimeField_ECDLP_Research.pdf'
FONT = Path('/usr/share/fonts/truetype/dejavu')
for name, file in [('DVS','DejaVuSans.ttf'),('DVS-Bold','DejaVuSans-Bold.ttf'),('DVS-Oblique','DejaVuSans-Oblique.ttf'),('DVSMono','DejaVuSansMono.ttf')]:
    pdfmetrics.registerFont(TTFont(name, str(FONT/file)))
pdfmetrics.registerFontFamily('DVS',normal='DVS',bold='DVS-Bold',italic='DVS-Oblique',boldItalic='DVS-Bold')
navy=colors.HexColor('#17354b')
teal=colors.HexColor('#007f81')
ink=colors.HexColor('#213747')
muted=colors.HexColor('#526778')
light=colors.HexColor('#edf4f7')
styles = getSampleStyleSheet()
styles.add(ParagraphStyle('BodyX',fontName='DVS',fontSize=9.1,leading=13.4,textColor=ink,spaceAfter=5))
styles.add(ParagraphStyle('TitleX',fontName='DVS-Bold',fontSize=24,leading=29,textColor=navy,spaceAfter=13))
styles.add(ParagraphStyle('MetaX',fontName='DVS',fontSize=8.3,leading=12,textColor=muted,spaceAfter=12))
styles.add(ParagraphStyle('H2X',fontName='DVS-Bold',fontSize=13.6,leading=18,textColor=navy,spaceBefore=13,spaceAfter=8,keepWithNext=True))
styles.add(ParagraphStyle('H3X',fontName='DVS-Bold',fontSize=11,leading=15,textColor=teal,spaceBefore=10,spaceAfter=5,keepWithNext=True))
styles.add(ParagraphStyle('ListX',parent=styles['BodyX'],leftIndent=15,firstLineIndent=-13,spaceAfter=4))
styles.add(ParagraphStyle('CellX',fontName='DVS',fontSize=8,leading=11,textColor=ink,spaceAfter=0))
styles.add(ParagraphStyle('CellHeadX',parent=styles['CellX'],fontName='DVS-Bold',textColor=colors.white))
styles.add(ParagraphStyle('CaptionX',fontName='DVS',fontSize=8,leading=11,textColor=muted,spaceAfter=9))
styles.add(ParagraphStyle('SourceX',parent=styles['BodyX'],fontSize=8,leading=11.3,spaceAfter=5))
styles.add(ParagraphStyle('CalloutX',parent=styles['BodyX'],fontSize=9.2,leading=13.5,spaceAfter=0))
WIDTH=486

def normalize(s):
    return s.replace('\u2011','-').replace('\u2013','-').replace('\u2014',' - ')

def inline(s):
    s=normalize(s)
    links=[]
    def stash(m):
        links.append((m.group(1),m.group(2)))
        return f'ZZLINKTOKEN{len(links)-1}ZZ'
    s=re.sub(r'\[([^\]]+)\]\(([^)]+)\)',stash,s)
    s=html.escape(s)
    s=re.sub(r'\*\*(.+?)\*\*',r'<b>\1</b>',s)
    s=re.sub(r'`([^`]+)`',r'<font name="DVSMono" size="8">\1</font>',s)
    for i,(label,url) in enumerate(links):
        s=s.replace(f'ZZLINKTOKEN{i}ZZ',f'<link href="{html.escape(url,quote=True)}" color="#007f81">{html.escape(normalize(label))}</link>')
    return s

def p(s,style='BodyX'):
    return Paragraph(inline(s),styles[style])

def equation(src):
    src=src.replace('\\hbox{over }','\\mathrm{over}\\ ')
    src=src.replace('\\mathbb F','\\mathbb{F}')
    src=' '.join(src.split())
    path=TMP/('eq_'+hashlib.sha256(src.encode()).hexdigest()[:12]+'.png')
    math_to_image('$'+src+'$',path,prop=FontProperties(size=12),dpi=210,format='png',color='#17354b')
    with PILImage.open(path) as im:
        w,h=im.size
    width=min(w*72/210,WIDTH-12)
    height=h*width/w
    return KeepTogether([Spacer(1,4),Image(str(path),width=width,height=height,hAlign='CENTER'),Spacer(1,9)])

def figure(file,caption,max_height=290):
    path=REPORT/file
    with PILImage.open(path) as im: w,h=im.size
    factor=min(WIDTH/w,max_height/h)
    return KeepTogether([Spacer(1,4),Image(str(path),width=w*factor,height=h*factor,hAlign='CENTER'),Spacer(1,5),p(caption,'CaptionX')])

def table(rows):
    data=[]
    for i,row in enumerate(rows):
        data.append([p(c,'CellHeadX' if i==0 else 'CellX') for c in row])
    n=len(rows[0])
    widths=[WIDTH*.29,WIDTH*.28,WIDTH*.43] if n==3 else [WIDTH*.08,WIDTH*.29,WIDTH*.31,WIDTH*.32]
    t=Table(data,colWidths=widths,repeatRows=1,hAlign='LEFT')
    t.setStyle(TableStyle([
        ('BACKGROUND',(0,0),(-1,0),navy),('VALIGN',(0,0),(-1,-1),'TOP'),
        ('LEFTPADDING',(0,0),(-1,-1),8),('RIGHTPADDING',(0,0),(-1,-1),8),
        ('TOPPADDING',(0,0),(-1,-1),7),('BOTTOMPADDING',(0,0),(-1,-1),7),
        ('ROWBACKGROUNDS',(0,1),(-1,-1),[colors.white,light]),
        ('LINEBELOW',(0,-1),(-1,-1),.6,colors.HexColor('#c5d5de')),
    ]))
    return [t,Spacer(1,10)]

def draw_page(c,doc):
    c.saveState()
    w,h=doc.pagesize
    c.setStrokeColor(colors.HexColor('#d8e2e8'))
    c.line(54,h-43,w-54,h-43)
    c.setFont('DVS-Bold',7.5)
    c.setFillColor(teal)
    c.drawString(54,h-32,'ISOGENY LABS  /  RESEARCH NOTE')
    c.setFont('DVS',7.5)
    c.setFillColor(muted)
    c.drawRightString(w-54,h-32,'23 NOVEMBER 2026')
    c.line(54,42,w-54,42)
    c.setFont('DVS',7)
    c.drawString(54,29,'Prime-field p+1 route  |  Correctness evidence; no speed claim')
    c.drawRightString(w-54,29,f'{doc.page}')
    c.restoreState()

source=(REPORT/'REPORT.md').read_text()
lines=source.splitlines()
story=[]
i=0
in_sources=False
while i<len(lines):
    line=lines[i].strip()
    if not line:
        i+=1; continue
    if line.startswith('# '):
        story.append(p(line[2:],'TitleX'));i+=1;continue
    if line.startswith('Research note'):
        story.append(p(line,'MetaX'));i+=1;continue
    if line.startswith('## '):
        label=line[3:]
        if label=='Sources':
            in_sources=True
        story.append(p(label,'H2X'));i+=1;continue
    if line.startswith('### '):
        story.append(p(line[4:],'H3X'));i+=1;continue
    if line=='$$':
        eq=[];i+=1
        while i<len(lines) and lines[i].strip()!='$$':
            eq.append(lines[i]);i+=1
        story.append(equation('\n'.join(eq)));i+=1;continue
    if line.startswith('|'):
        rows=[]
        while i<len(lines) and lines[i].strip().startswith('|'):
            row=[x.strip() for x in lines[i].strip().strip('|').split('|')]
            if not all(re.fullmatch(r':?-+:?',x) for x in row): rows.append(row)
            i+=1
        story.extend(table(rows));continue
    match=re.match(r'!\[([^\]]*)\]\(([^)]+)\)',line)
    if match:
        story.append(figure(match.group(2),match.group(1),330 if 'route' in match.group(2) else 250));i+=1;continue
    if re.match(r'^\d+\. ',line):
        story.append(p(line,'ListX'));i+=1;continue
    if line.startswith('**Outcome.') or line.startswith('**Evidence standing.'):
        cell=Table([[p(line,'CalloutX')]],colWidths=[WIDTH])
        cell.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,-1),light),('BOX',(0,0),(-1,-1),.5,colors.HexColor('#d5e3e9')),('LEFTPADDING',(0,0),(-1,-1),11),('RIGHTPADDING',(0,0),(-1,-1),11),('TOPPADDING',(0,0),(-1,-1),9),('BOTTOMPADDING',(0,0),(-1,-1),9)]))
        story.extend([cell,Spacer(1,8)]);i+=1;continue
    para=[line];i+=1
    while i<len(lines) and lines[i].strip() and not re.match(r'^(#|\||\$\$|!\[|\d+\. )',lines[i].strip()):
        para.append(lines[i].strip());i+=1
    paragraph=p(' '.join(para),'SourceX' if in_sources else 'BodyX')
    next_content=i
    while next_content<len(lines) and not lines[next_content].strip(): next_content+=1
    if next_content<len(lines) and lines[next_content].strip()=='$$': paragraph.keepWithNext=True
    story.append(paragraph)

doc=SimpleDocTemplate(str(OUT),pagesize=(594,792),rightMargin=54,leftMargin=54,topMargin=60,bottomMargin=57,title='A p+1 index-calculus route for prime-field ECDLP',author='Isogeny Labs research',subject='Dickson selectors, prime-field Semaev index calculus, and a qualified full-domain line FFT')
doc.build(story,onFirstPage=draw_page,onLaterPages=draw_page)
print(OUT)
