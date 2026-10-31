"""Plot recorded counts and typeset the study report; document rendering only."""
import csv
import html
import json
import re
from pathlib import Path

import fitz
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.colors import ListedColormap
from reportlab.lib import colors
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import Image, Paragraph, SimpleDocTemplate, Spacer, Table, TableStyle, XPreformatted

ROOT=Path(__file__).resolve().parent.parent
OUT=ROOT/'reports'
rows=list(csv.DictReader((ROOT/'results/criterion-truth-table.tsv').open(),delimiter='\t'))
matrix=[[1 if row['role']=='published' else 0,int(row['gate1']),int(row['gate2']),int(row['two_test_accept'])] for row in rows]
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':9})
fig,ax=plt.subplots(figsize=(8.0,6.0),layout='constrained')
ax.imshow(matrix,cmap=ListedColormap(['#EEF1F3','#17334B']),aspect='auto',vmin=0,vmax=1)
ax.set_yticks(range(len(rows)),[row['label']+' ('+row['role']+')' for row in rows],fontsize=8)
ax.set_xticks(range(4),['Published\nlabel','Twist-order\ncondition','Coefficient\ncondition','Combined\ncriterion'])
ax.tick_params(axis='both',length=0)
for i,row in enumerate(matrix):
    for j,value in enumerate(row):ax.text(j,i,str(value),ha='center',va='center',color='white' if value else '#637785',fontsize=8)
ax.spines[:].set_visible(False)
ax.set_title('Recorded fit: 10 published and 11 skipped curves',loc='left',weight='bold',pad=12)
fig.savefig(OUT/'rule-fit.svg');fig.savefig(OUT/'rule-fit.png',dpi=150);plt.close(fig)

extended=json.loads((ROOT/'results/extended-generation.json').read_text())
primality=json.loads((ROOT/'results/primality-generation.json').read_text())
union=json.loads((ROOT/'results/calibration-union.json').read_text())
series=[('Generation features (1,100)',extended['short_rule_search_as_good_count']),
        ('With primality-gate models',primality['short_rule_search_as_good_count']),
        ('Union of 15 search families',union['assignments_with_some_exact_rule_of_at_most_two_tests'])]
fig,ax=plt.subplots(figsize=(8.0,2.6),layout='constrained')
for i,(label,count) in enumerate(series):
    ax.barh(i,count,color='#17334B',height=.55)
    ax.text(count+65,i,str(count)+' / 4,480',va='center',fontsize=9)
ax.set_yticks(range(3),[label for label,_ in series]);ax.invert_yaxis()
ax.set_xlim(0,5700);ax.set_xticks([0,1120,2240,3360,4480])
ax.set_xlabel('Fixed-count labelings admitting an exact rule with at most two tests')
ax.spines[['top','right','left']].set_visible(False);ax.tick_params(axis='y',length=0)
fig.savefig(OUT/'calibration.svg');fig.savefig(OUT/'calibration.png',dpi=150);plt.close(fig)
for name in ['rule-fit.svg','calibration.svg']:
    p=OUT/name;p.write_text('\n'.join(line.rstrip() for line in p.read_text().splitlines())+'\n')
(OUT/'graph-source.json').write_text(json.dumps({
    'kind':'recorded-values-only/v1','rule_source':'results/criterion-truth-table.tsv',
    'population':'21 recorded eligible curves; 10 published, 11 skipped',
    'calibration_population':4480,'calibration_series':[{'label':label,'count':count} for label,count in series],
    'calibration_sources':['results/extended-generation.json','results/primality-generation.json','results/calibration-union.json'],
    'uncertainty':'complete finite enumeration; no sampling interval inferred; exchangeability is an explicit model assumption'
},indent=2)+'\n')

fonts=Path('/usr/share/fonts/truetype/dejavu')
for name,file in [('DVS','DejaVuSans.ttf'),('DVS-Bold','DejaVuSans-Bold.ttf'),('DVS-Italic','DejaVuSans-Oblique.ttf'),('DVM','DejaVuSansMono.ttf')]:
    pdfmetrics.registerFont(TTFont(name,str(fonts/file)))
pdfmetrics.registerFontFamily('DVS',normal='DVS',bold='DVS-Bold',italic='DVS-Italic',boldItalic='DVS-Bold')
styles=getSampleStyleSheet()
for key in ['BodyText','Title','Heading1','Heading2']:
    styles[key].fontName='DVS';styles[key].textColor=colors.HexColor('#17334B')
styles['BodyText'].fontSize=9;styles['BodyText'].leading=12.5;styles['BodyText'].spaceAfter=7
styles['Title'].fontSize=23;styles['Title'].leading=28
styles['Heading1'].fontName='DVS-Bold';styles['Heading1'].fontSize=14
styles['Heading2'].fontName='DVS-Bold';styles['Heading2'].fontSize=11
cell=ParagraphStyle('Cell',parent=styles['BodyText'],fontSize=8,leading=10.5,spaceAfter=0)
mono=ParagraphStyle('Mono',parent=styles['BodyText'],fontName='DVM',fontSize=8.2,leading=11)

def inline(text):
    value=html.escape(text)
    def link(match):
        label,url=match[1],html.unescape(match[2])
        if url.startswith('https://') or url.startswith('http://'):
            return '<link href="'+html.escape(url)+'" color="#235E80">'+label+'</link>'
        return label+' <font name="DVM" size="7.5">('+html.escape(url)+')</font>'
    value=re.sub(r'\[([^\]]+)\]\(([^)]+)\)',link,value)
    value=re.sub(r'`([^`]+)`',r'<font name="DVM">\1</font>',value)
    return re.sub(r'\*\*([^*]+)\*\*',r'<b>\1</b>',value)

story=[]
for block in (ROOT/'REPORT.md').read_text().split('\n\n'):
    block=block.strip()
    if not block:continue
    if block.startswith('```'):
        story.append(XPreformatted('\n'.join(html.escape(s) for s in block.splitlines()[1:-1]),mono))
    elif block.startswith('!['):
        filename='rule-fit.png' if 'rule-fit' in block else 'calibration.png'
        story.append(Image(str(OUT/filename),width=506,height=379.5 if filename=='rule-fit.png' else 164.5))
        story.append(Spacer(1,8))
    elif block.startswith('|'):
        lines=[line for line in block.splitlines() if not re.match(r'^\|[\s:|-]+\|$',line)]
        data=[[Paragraph(inline(c.strip()),cell) for c in line.strip('|').split('|')] for line in lines]
        table=Table(data,colWidths=[210,296],repeatRows=1,hAlign='LEFT')
        table.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#E9EFF3')),
            ('VALIGN',(0,0),(-1,-1),'TOP'),('LEFTPADDING',(0,0),(-1,-1),5),('RIGHTPADDING',(0,0),(-1,-1),5),
            ('TOPPADDING',(0,0),(-1,-1),6),('BOTTOMPADDING',(0,0),(-1,-1),6),
            ('LINEBELOW',(0,0),(-1,0),.6,colors.HexColor('#B7C5CF'))]))
        story.extend([table,Spacer(1,9)])
    elif block.startswith('#'):
        level=len(block)-len(block.lstrip('#'))
        story.append(Paragraph(inline(block.lstrip('#').strip()),styles['Title' if level==1 else 'Heading1' if level==2 else 'Heading2']))
    elif block.startswith('- '):
        for item in re.split(r'\n(?=- )',block):story.append(Paragraph('• '+inline(item[2:].replace('\n',' ')),styles['BodyText']))
    else:story.append(Paragraph(inline(block.replace('\n',' ')),styles['BodyText']))

def footer(canvas,doc):
    canvas.saveState();canvas.setFont('DVS',7);canvas.setFillColor(colors.HexColor('#687A87'))
    canvas.drawString(38,24,'Skipped-curve criterion search · 25–26 October 2026 · exploratory')
    canvas.drawRightString(557,24,str(doc.page));canvas.restoreState()

path=ROOT/'REPORT.pdf'
doc=SimpleDocTemplate(str(path),pagesize=(595.276,841.89),leftMargin=38,rightMargin=38,topMargin=38,bottomMargin=40,
    title='Searching for the skipped-curve criterion',author='Native curve study')
doc.build(story,onFirstPage=footer,onLaterPages=footer)
pdf=fitz.open(path)
for name,file,description in [
    ('runs.tar.xz',ROOT/'evidence/runs.tar.xz','Full native records and per-member SHA-256 manifest'),
    ('historical-inputs.tsv',ROOT/'inputs/historical.tsv','Exact normalized historical models'),
    ('criterion-truth-table.tsv',ROOT/'results/criterion-truth-table.tsv','Direct arithmetic classification of all 21 curves')]:
    pdf.embfile_add(name,file.read_bytes(),filename=name,desc=description)
temporary=ROOT/'REPORT-attached.pdf';pdf.save(temporary,deflate=True);pdf.close();temporary.replace(path)
