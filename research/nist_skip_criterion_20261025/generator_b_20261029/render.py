# Document rendering only. No curve/field arithmetic or experiment execution.
import json, re, html
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
matplotlib.rcParams['svg.fonttype']='none'
import matplotlib.pyplot as plt
from reportlab.pdfgen import canvas
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, Image, PageBreak
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib import colors
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.lib.enums import TA_LEFT
import fitz
b=Path(__file__).resolve().parent
def normalize_svg(file):
    file.write_text('\n'.join(line.rstrip() for line in file.read_text().splitlines())+'\n')
for name,file in [('DejaVu','DejaVuSans.ttf'),('DejaVu-Bold','DejaVuSans-Bold.ttf')]:
    pdfmetrics.registerFont(TTFont(name,'/usr/share/fonts/truetype/dejavu/'+file))
pdfmetrics.registerFontFamily('DejaVu',normal='DejaVu',bold='DejaVu-Bold',italic='DejaVu',boldItalic='DejaVu-Bold')
# Counts copied from the already frozen prior measurement inventory, not newly measured.
graph={'kind':'existing-feature-display/v1','source':'evidence/existing-b-character.json','new_cohort_execution':False,'series':[{'label':'Published','b_square':9,'total':10},{'label':'Skipped','b_square':9,'total':11}]}
(b/'b-character.json').write_text(json.dumps(graph,indent=2)+'\n')
fig,ax=plt.subplots(figsize=(6.7,2.0)); fig.set_facecolor('white')
ax.barh(['Published (10 curves)','Skipped (11 curves)'],[9,9],color=['#256c8d','#759d49'],label='b is square')
ax.barh(['Published (10 curves)','Skipped (11 curves)'],[1,2],left=[9,9],color='#d9dee2',label='b is nonsquare')
ax.set_xlim(0,11.8);ax.set_xticks([0,3,6,9,11]);ax.set_xlabel('Curve choices · existing historical-b measurements');ax.invert_yaxis()
for y,t in enumerate(['9 / 10','9 / 11']):ax.text(4.5,y,t,ha='center',va='center',color='white',fontweight='bold')
ax.spines[['top','right']].set_visible(False);ax.legend(loc='lower right',bbox_to_anchor=(1,1.02),ncol=2,frameon=False,fontsize=8)
fig.tight_layout();fig.savefig(b/'b-character.svg');normalize_svg(b/'b-character.svg');fig.savefig(b/'b-character.png',dpi=150);plt.close(fig)

def diagram(file,rows,caption):
    from matplotlib.patches import FancyBboxPatch
    fig,ax=plt.subplots(figsize=(7.5,1.71))
    ax.set_xlim(0,720);ax.set_ylim(164,0);ax.axis('off')
    for i,(title,lines,status) in enumerate(rows):
        x=10+i*238;stroke='#ad6c27' if status=='blocked' else '#366c85';fill='#fff4e8' if status=='blocked' else '#eef6fa'
        ax.add_patch(FancyBboxPatch((x,8),224,118,boxstyle='round,pad=0.1,rounding_size=6',facecolor=fill,edgecolor=stroke,linewidth=.8))
        ax.text(x+11,31,title,fontsize=9.4,fontweight='bold',color='#16384c')
        for j,line in enumerate(lines):ax.text(x+11,53+j*18,line,fontsize=8,color='#263745')
    ax.text(10,151,caption,fontsize=7,color='#465766')
    fig.subplots_adjust(left=0,right=1,top=1,bottom=0)
    fig.savefig(file);normalize_svg(file);fig.savefig(file.with_suffix('.png'),dpi=150);plt.close(fig)
    file.with_suffix('.json').write_text(json.dumps({'kind':'evidence-boundary-diagram/v1','boxes':rows,'caption':caption},indent=2)+'\n')
diagram(b/'visual.svg',[
 ('Existing measurements',['All 21 original choices retained','b square: published 9/10','b square: skipped 9/11'],'verified'),
 ('Conditional consequences',['x = 0 implies y² = b','x = 1 implies y² = b − 2','Five Lean proofs checked'],'verified'),
 ('Actual new comparison',['171 point traits prepared','Synthetic checks pass','0 production attempts'],'blocked')],
 'G–b connection · 29 Oct 2026 · No historical selector or attack established')
diagram(b/'reports/development/visual.svg',[
 ('Native development',['PARI producer + GMP verifier','Six tiny fields; 21 parser rows','Dummy inherited features'],'verified'),
 ('Independent checking',['100,548 synthetic trait cells','All 9 corruption controls reject','5 Lean proofs; audit passes'],'verified'),
 ('Production admission',['Actual NIST cohort not run','Checkpoint identity unavailable','Preparation remains local'],'blocked')],
 'Development controls only · No benchmark clock or real-curve result')
styles=getSampleStyleSheet()
for k in ['Normal','BodyText','Heading1','Heading2','Heading3','Title']:
    styles[k].fontName='DejaVu';styles[k].textColor=colors.HexColor('#1b3343')
styles['Normal'].fontSize=9;styles['Normal'].leading=13;styles['Normal'].spaceAfter=7
styles['Title'].fontSize=17;styles['Title'].leading=22;styles['Title'].spaceAfter=14
styles['Heading2'].fontSize=12;styles['Heading2'].leading=16;styles['Heading2'].spaceBefore=10;styles['Heading2'].spaceAfter=7
cell=ParagraphStyle('Cell',parent=styles['Normal'],fontSize=8,leading=11,spaceAfter=2)
def fmt(t):
    t=html.escape(t)
    t=re.sub(r'\[([^\]]+)\]\(([^)]+)\)',lambda m:f'<link href="{html.escape(m.group(2),quote=True)}" color="#226989">{m.group(1)}</link>',t)
    t=re.sub(r'\*\*([^*]+)\*\*',r'<b>\1</b>',t)
    t=re.sub(r'`([^`]+)`',r'\1',t)
    return t

def pdf(md,output,graphs):
    flow=[];para=[];rows=[]
    def flush():
        if para:flow.append(Paragraph(fmt(' '.join(para)),styles['Normal']));para.clear()
    def tableflush():
        if not rows:return
        usable=[r for r in rows if not re.match(r'^\|[\s:|\-]+\|$',r)]
        cells=[[Paragraph(fmt(x.strip()),cell) for x in r.strip('|').split('|')] for r in usable]
        t=Table(cells,colWidths=[132,358],repeatRows=1,hAlign='LEFT');t.setStyle(TableStyle([('VALIGN',(0,0),(-1,-1),'TOP'),('BACKGROUND',(0,0),(-1,0),colors.HexColor('#e9f1f5')),('GRID',(0,0),(-1,-1),.4,colors.HexColor('#c4d2da')),('LEFTPADDING',(0,0),(-1,-1),6),('RIGHTPADDING',(0,0),(-1,-1),6),('TOPPADDING',(0,0),(-1,-1),5),('BOTTOMPADDING',(0,0),(-1,-1),5)]));flow.extend([t,Spacer(1,9)]);rows.clear()
    for line in md.read_text().splitlines():
        if line.startswith('|'):flush();rows.append(line);continue
        tableflush()
        if not line.strip():flush();continue
        if line.startswith('# '):flush();flow.append(Paragraph(fmt(line[2:]),styles['Title']))
        elif line.startswith('## '):flush();flow.append(Paragraph(fmt(line[3:]),styles['Heading2']))
        else:para.append(line)
    flush();tableflush()
    flow.append(Spacer(1,10))
    for g in graphs:
        im=Image(str(g.with_suffix('.png')));im.drawHeight*=490/im.drawWidth;im.drawWidth=490;flow.extend([im,Spacer(1,12)])
    def foot(c,d):
        c.setFont('DejaVu',7);c.setFillColor(colors.HexColor('#5f717e'));c.drawString(45,25,'Generator/coefficient study · 2026-10-29 · prepared, actual cohort not run');c.drawRightString(550,25,str(d.page))
    SimpleDocTemplate(str(output),pagesize=(595.28,841.89),rightMargin=45,leftMargin=45,topMargin=38,bottomMargin=43,title=md.read_text().splitlines()[0].lstrip('# '),author='Curve research').build(flow,onFirstPage=foot,onLaterPages=foot)
    d=fitz.open(output)
    out=Path('/workspace/nist-generator-document-inspection')/output.parent.name
    out.mkdir(parents=True,exist_ok=True)
    for i,p in enumerate(d):p.get_pixmap(matrix=fitz.Matrix(1.4,1.4)).save(out/f'page-{i+1}.png')
    text='\n'.join(p.get_text() for p in d);(out/'readback.txt').write_text(text)
    print(json.dumps({'pdf':str(output),'pages':len(d),'bytes':output.stat().st_size,'inspection':str(out),'text_chars':len(text)}))
pdf(b/'REPORT.md',b/'REPORT.pdf',[b/'visual.svg',b/'b-character.svg'])
pdf(b/'reports/development/REPORT.md',b/'reports/development/REPORT.pdf',[b/'reports/development/visual.svg'])
