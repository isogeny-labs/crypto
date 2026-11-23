"""Plot recorded counts and typeset existing reports only."""
import html,json,os,posixpath,re
from pathlib import Path
os.environ.setdefault('MPLCONFIGDIR','/tmp/nist-lip-mpl')
os.environ.setdefault('XDG_CACHE_HOME','/tmp/nist-lip-doc-cache')
import fitz
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch,Patch
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet,ParagraphStyle
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import SimpleDocTemplate,Paragraph,Image,Table,TableStyle,Spacer,Preformatted
ROOT=Path(__file__).resolve().parent.parent; OUT=ROOT/'reports'
INK,BLUE,GOLD,GRAY='#17384B','#2F6F8F','#A36B32','#B8C3CB'
URL='https://github.com/isogeny-labs/crypto/blob/codex/nist-period-ecpp-20261026/'
RP='research/nist_skip_criterion_20261025/freelip_replay_20261027'
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':9,'svg.fonttype':'none'})
def save(fig,p):
 fig.savefig(p.with_suffix('.svg'),bbox_inches='tight');fig.savefig(p.with_suffix('.png'),dpi=150,bbox_inches='tight');plt.close(fig)
 s=p.with_suffix('.svg');s.write_text('\n'.join(l.rstrip()for l in s.read_text().splitlines())+'\n')
def outcomes(p,title,initial=False):
 record=json.loads((ROOT/'results'/('initial-summary.json'if initial else 'summary.json')).read_text())
 fig,axes=plt.subplots(1,2 if initial else 1,figsize=(8,3.1),layout='constrained');axes=[axes]if not initial else axes
 for ax,key in zip(axes,['accepted','indeterminate']):
  for i,row in enumerate(record['configurations']):
   for role,dy,col in [('published',-.17,BLUE),('skipped',.17,GOLD)]:
    n=row['roles'][role][key];ax.barh(i+dy,n,color=col,height=.30);ax.text(n+.12,i+dy,str(n),va='center',fontsize=8)
  ax.set_yticks(range(4),['v'+r['version']+' '+r['method']for r in record['configurations']]);ax.invert_yaxis();ax.set_xlim(0,12.5);ax.set_xticks([0,3,6,9,12]);ax.set_xlabel('Orders (count)');ax.set_title(key.capitalize(),loc='left',fontsize=10);ax.spines[['top','right']].set_visible(False)
 fig.suptitle(title,x=.02,ha='left',weight='bold',fontsize=12);fig.legend(handles=[Patch(color=BLUE,label='Published'),Patch(color=GOLD,label='Skipped')],loc='lower right',ncol=2,frameon=False,fontsize=8);fig.get_layout_engine().set(rect=(0,.13,1,.72));fig.text(.02,.025,'21 distinct orders reused across 4 configurations | exact counts, no sampled interval',fontsize=7);save(fig,p)
def development(p):
 d=json.loads((OUT/'development-values.json').read_text());fig,ax=plt.subplots(figsize=(8,2.7),layout='constrained');ax.barh(d['labels'],d['values'],color=[GRAY,GRAY,GRAY,BLUE],height=.55)
 for i,n in enumerate(d['values']):ax.text(n+.08,i,str(n),va='center')
 ax.invert_yaxis();ax.set_xlim(0,6);ax.set_xlabel('Development attempts; zero historical calls');ax.set_title('All failed development attempts retained',loc='left',weight='bold');ax.spines[['top','right']].set_visible(False);save(fig,p)
def sourcepath():
 d=json.loads((OUT/'source-path.json').read_text());fig,ax=plt.subplots(figsize=(8,3.2),layout='constrained');ax.axis('off');ax.set_xlim(0,1);ax.set_ylim(0,1)
 for i,text in enumerate(d['steps']):
  x=.02+i*.335;ax.add_patch(FancyBboxPatch((x,.52),.285,.30,boxstyle='round,pad=.008',facecolor='#EDF2F5',edgecolor=INK));ax.text(x+.1425,.67,text,ha='center',va='center',fontsize=9)
  if i<2:ax.annotate('',xy=(x+.325,.67),xytext=(x+.295,.67),arrowprops={'arrowstyle':'->','color':INK})
 ax.annotate('',xy=(.50,.24),xytext=(.82,.49),arrowprops={'arrowstyle':'->','color':GRAY,'linestyle':'--'})
 ax.text(.5,.15,'Possible connection to seven discarded calls\nOriginal caller and exact RNG states unverified',ha='center',va='center',fontsize=9,color=INK);ax.set_title('Source call path; dashed connection is proposed',loc='left',weight='bold',fontsize=12);save(fig,OUT/'source-path')
outcomes(OUT/'outcomes','Every fixed configuration accepts all 21 orders');sourcepath()
for vf in sorted(OUT.glob('*/visual.json')):
 s=json.loads(vf.read_text());target=vf.parent/'visual'
 if s['kind']=='development':development(target)
 else:outcomes(target,s['title'],s['kind']=='initial')
(OUT/'graph-source.json').write_text(json.dumps({'kind':'recorded-native-values-only/v1','final':'../results/summary.json','initial':'../results/initial-summary.json','development':'development-values.json','population':'21 distinct original orders,84 software calls; labels already visible','uncertainty':'Exact finite outcomes, no sampled interval'},indent=2)+'\n')
font=Path('/usr/share/fonts/truetype/dejavu')
for name,f in [('DVS','DejaVuSans.ttf'),('DVS-Bold','DejaVuSans-Bold.ttf'),('DVM','DejaVuSansMono.ttf')]:pdfmetrics.registerFont(TTFont(name,str(font/f)))
pdfmetrics.registerFontFamily('DVS',normal='DVS',bold='DVS-Bold',italic='DVS',boldItalic='DVS-Bold')
styles=getSampleStyleSheet()
for key in ['Title','BodyText','Heading1','Heading2']:styles[key].fontName='DVS';styles[key].textColor=colors.HexColor(INK)
styles['Title'].fontSize=20;styles['Title'].leading=25;styles['Title'].spaceAfter=12
styles['BodyText'].fontSize=9;styles['BodyText'].leading=12;styles['BodyText'].spaceAfter=7
styles['BodyText'].allowWidows=0;styles['BodyText'].allowOrphans=0
styles['Heading1'].fontName='DVS-Bold';styles['Heading1'].fontSize=12;styles['Heading1'].leading=15;styles['Heading1'].spaceBefore=10
cell=ParagraphStyle('Cell',parent=styles['BodyText'],fontSize=7.6,leading=10,spaceAfter=0)
mono=ParagraphStyle('Mono',parent=styles['BodyText'],fontName='DVM',fontSize=7,leading=10)
def inline(s,src):
 s=html.escape(s)
 def link(m):
  url=html.unescape(m[2]);url=url if url.startswith(('https://','http://'))else URL+posixpath.normpath(RP+'/'+src.parent.relative_to(ROOT).as_posix()+'/'+url)
  return '<link href="'+html.escape(url,quote=True)+'" color="#235E80">'+m[1]+'</link>'
 s=re.sub(r'\[([^\]]+)\]\(([^)]+)\)',link,s);s=re.sub(r'`([^`]+)`',r'<font name="DVM" size="7.5">\1</font>',s);return re.sub(r'\*\*([^*]+)\*\*',r'<b>\1</b>',s)
def build(src,attachments):
 story=[]
 for block in src.read_text().split('\n\n'):
  block=block.strip()
  if not block:continue
  if block.startswith('!['):
   p=(src.parent/re.search(r'\]\(([^)]+)\)',block)[1]).resolve();pix=fitz.Pixmap(str(p));story.extend([Image(str(p),width=510,height=510*pix.height/pix.width),Spacer(1,8)])
  elif block.startswith('```'):story.append(Preformatted('\n'.join(block.splitlines()[1:-1]),mono))
  elif block.startswith('|'):
   lines=[x for x in block.splitlines()if not re.match(r'^\|[\s:|\-]+\|$',x)];data=[[Paragraph(inline(c.strip(),src),cell)for c in x.strip('|').split('|')]for x in lines];n=len(data[0]);w=[45,150,105,105,105]if n==5 else[170,340]
   t=Table(data,colWidths=w,repeatRows=1,hAlign='LEFT');t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#E9EFF3')),('VALIGN',(0,0),(-1,-1),'TOP'),('LEFTPADDING',(0,0),(-1,-1),5),('RIGHTPADDING',(0,0),(-1,-1),5),('TOPPADDING',(0,0),(-1,-1),5),('BOTTOMPADDING',(0,0),(-1,-1),5),('LINEBELOW',(0,0),(-1,0),.5,colors.HexColor(GRAY))]));story.extend([t,Spacer(1,8)])
  elif block.startswith('#'):
   lev=len(block)-len(block.lstrip('#'));story.append(Paragraph(inline(block.lstrip('#').strip(),src),styles['Title'if lev==1 else'Heading1']))
  else:story.append(Paragraph(inline(block.replace('\n',' '),src),styles['BodyText']))
 def footer(c,d):c.setFont('DVS',7);c.setFillColor(colors.HexColor('#607581'));c.drawString(38,22,'Recovered FreeLIP · 27 October 2026 · correct-order software test; no historical-use claim');c.drawRightString(557,22,str(d.page))
 raw=src.with_suffix('.rendering.pdf');out=src.with_suffix('.pdf');SimpleDocTemplate(str(raw),pagesize=(595.276,841.89),leftMargin=38,rightMargin=38,topMargin=35,bottomMargin=39,title=src.read_text().splitlines()[0][2:]).build(story,onFirstPage=footer,onLaterPages=footer);doc=fitz.open(raw)
 for name,p in attachments:doc.embfile_add(name,p.read_bytes(),filename=name,desc=str(p.relative_to(ROOT)))
 doc.set_metadata({'title':src.read_text().splitlines()[0][2:],'subject':'All84 software calls accept21 distinct original orders; no criterion or attack established'});doc.save(out,garbage=4,deflate=True);doc.close();raw.unlink();return out
runpdf=[]
for src in sorted(OUT.glob('*/REPORT.md')):runpdf.append(build(src,[('visual-source.json',src.with_name('visual.json'))]))
files=['PROTOCOL.md','protocol.json','VALIDATION_PROTOCOL.md','PREPARED.json','source-audit.json','results/cohort.tsv','results/summary.json','results/initial-summary.json','export.json','evidence/protocol-admission.json','evidence/export-admission.json','evidence/preparation-delivery.json','evidence/final-delivery.json','evidence/preparation.tar.xz','runs.tar.xz','summarize.cpp','replay.c','bridge.c','bridge.h','export.cpp','inputs/cohort.tsv']
attachments=[(name.replace('/','__'),ROOT/name)for name in files];attachments.extend((p.parent.name+'.pdf',p)for p in runpdf);build(ROOT/'REPORT.md',attachments)
