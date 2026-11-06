"""Typeset Markdown and plot recorded native counts; no research computation."""
import csv
import html
import json
import os
import posixpath
import re
from pathlib import Path
os.environ.setdefault('MPLCONFIGDIR', '/tmp/nist-period-matplotlib')
os.environ.setdefault('XDG_CACHE_HOME', '/tmp/nist-period-render-cache')
import fitz
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch, Patch
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import SimpleDocTemplate, Paragraph, Image, Table, TableStyle, Spacer, Preformatted
ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / 'reports'
INK, BLUE, GOLD, GRAY = '#17384B', '#2F6F8F', '#A36B32', '#B8C3CB'
URL = 'https://github.com/isogeny-labs/crypto/blob/codex/nist-period-ecpp-20261026/'
REPO_PATH = 'research/nist_skip_criterion_20261025/period_ecpp_20261026'
plt.rcParams.update({'font.family': 'DejaVu Sans', 'font.size': 9, 'svg.fonttype': 'none'})
def save(fig, target):
    fig.savefig(target.with_suffix('.svg'), bbox_inches='tight')
    fig.savefig(target.with_suffix('.png'), dpi=150, bbox_inches='tight')
    plt.close(fig)
    target.with_suffix('.svg').write_text('\n'.join(s.rstrip() for s in target.with_suffix('.svg').read_text().splitlines()) + '\n')
def outcomes(summary, title, target, note):
    record = json.loads(summary.read_text())
    roles = ['published', 'skipped']
    if not all('raw_cli_prime_labels' in record['roles'][r] for r in roles):
        fig, ax = plt.subplots(figsize=(8, 2.8), layout='constrained')
        for y, role in enumerate(roles):
            for key, color in [('accepted', BLUE), ('indeterminate', GRAY)]:
                n = record['roles'][role][key]
                left = 0 if key == 'accepted' else record['roles'][role]['accepted']
                ax.barh(y, n, left=left, color=color, height=.55)
                if n: ax.text(left + n/2, y, str(n), va='center', ha='center', color='white' if key == 'accepted' else INK)
        ax.set_yticks([0, 1], ['Published', 'Skipped']); ax.invert_yaxis()
        ax.set_xlim(0, 12); ax.set_xticks([0, 3, 6, 9, 12]); ax.set_xlabel('Recorded orders (count), initial v1 reader')
        ax.spines[['top', 'right']].set_visible(False)
        fig.suptitle(title, x=.02, ha='left', weight='bold', fontsize=12)
        fig.legend(handles=[Patch(color=BLUE, label='Accepted'), Patch(color=GRAY, label='Indeterminate')], loc='lower right', frameon=False, ncol=2, fontsize=8)
        fig.get_layout_engine().set(rect=(0, .13, 1, .72))
        fig.text(.02, .025, note, fontsize=7)
        save(fig, target)
        return
    fig, axes = plt.subplots(1, 2, figsize=(8, 2.8), layout='constrained')
    raw = record['roles']
    if all('raw_cli_prime_labels' in raw[r] for r in roles):
        for y, role in enumerate(roles):
            n = raw[role]['raw_cli_prime_labels']
            axes[0].barh(y, n, color=[BLUE, GOLD][y], height=.55)
            axes[0].text(n + .15, y, str(n), va='center')
        axes[0].set_title('Original CLI prime labels', loc='left', fontsize=10)
    else:
        for y, role in enumerate(roles):
            for key, color in [('accepted', BLUE), ('indeterminate', GRAY)]:
                n = raw[role][key]
                left = 0 if key == 'accepted' else raw[role]['accepted']
                axes[0].barh(y, n, left=left, color=color, height=.55)
                if n: axes[0].text(left + n/2, y, str(n), va='center', ha='center', color='white' if key == 'accepted' else INK)
        axes[0].set_title('Initial admission (v1 reader)', loc='left', fontsize=10)
    for y, role in enumerate(roles):
        for key, color in [('accepted', BLUE), ('indeterminate', GRAY)]:
            n = raw[role][key]
            left = 0 if key == 'accepted' else raw[role]['accepted']
            axes[1].barh(y, n, left=left, color=color, height=.55)
            if n: axes[1].text(left + n/2, y, str(n), va='center', ha='center', color='white' if key == 'accepted' else INK)
    axes[1].set_title('Certificate admission (separate)', loc='left', fontsize=10)
    for ax in axes:
        ax.set_yticks([0, 1], ['Published', 'Skipped']); ax.invert_yaxis(); ax.set_xlim(0, 12)
        ax.set_xticks([0, 3, 6, 9, 12]); ax.set_xlabel('Recorded orders (count)')
        ax.spines[['top', 'right']].set_visible(False)
    fig.suptitle(title, x=.02, ha='left', weight='bold', fontsize=12)
    fig.legend(handles=[Patch(color=BLUE, label='Accepted'), Patch(color=GRAY, label='Indeterminate')], loc='lower right', frameon=False, ncol=2, fontsize=8)
    fig.get_layout_engine().set(rect=(0, .13, 1, .72))
    fig.text(.02, .025, note, fontsize=7)
    save(fig, target)
def diagram(spec, target):
    fig, ax = plt.subplots(figsize=(8, 2.5), layout='constrained')
    ax.axis('off'); ax.set_xlim(0, 1); ax.set_ylim(0, 1)
    for i, step in enumerate(spec['steps']):
        x=.02 + .335*i
        ax.add_patch(FancyBboxPatch((x, .28), .285, .43, boxstyle='round,pad=.008', facecolor='#EDF2F5', edgecolor=INK))
        ax.text(x+.1425, .495, step, ha='center', va='center', fontsize=10, color=INK)
        if i<2: ax.annotate('', xy=(x+.325,.495), xytext=(x+.294,.495), arrowprops={'arrowstyle':'->','color':INK})
    ax.set_title(spec['title'], loc='left', weight='bold', fontsize=12)
    fig.text(.02, .02, spec['note'], fontsize=7)
    save(fig, target)
for specfile in sorted(OUT.glob('run-*/visual.json')):
    spec=json.loads(specfile.read_text())
    if spec['kind']=='diagram': diagram(spec, specfile.parent/'visual')
    else: outcomes((specfile.parent/spec['summary']).resolve(), spec['title'], specfile.parent/'visual', spec['note'])
outcomes(ROOT/'results/summary.json', 'Period ECPP: no separation of historical labels', OUT/'outcomes', 'replay004 / validation005 | all 21 retained | no composite CLI labels | exact counts, no sampling interval')
(OUT/'graph-source.json').write_text(json.dumps({'kind':'recorded-native-values-only/v1','source':'../results/summary.json','cohort':'../results/cohort.tsv','population':'All 10 published and 11 skipped orders','uncertainty':'Exact finite software outcomes; no sampled interval or historical attribution'}, indent=2)+'\n')
font=Path('/usr/share/fonts/truetype/dejavu')
for name, file in [('DVS','DejaVuSans.ttf'),('DVS-Bold','DejaVuSans-Bold.ttf'),('DVM','DejaVuSansMono.ttf')]: pdfmetrics.registerFont(TTFont(name,str(font/file)))
pdfmetrics.registerFontFamily('DVS',normal='DVS',bold='DVS-Bold',italic='DVS',boldItalic='DVS-Bold')
styles=getSampleStyleSheet()
for k in ['Title','BodyText','Heading1','Heading2']:
    styles[k].fontName='DVS'; styles[k].textColor=colors.HexColor(INK)
styles['Title'].fontSize=21; styles['Title'].leading=26; styles['Title'].spaceAfter=12
styles['BodyText'].fontSize=9; styles['BodyText'].leading=12; styles['BodyText'].spaceAfter=7
styles['Heading1'].fontName='DVS-Bold'; styles['Heading1'].fontSize=12; styles['Heading1'].leading=15; styles['Heading1'].spaceBefore=10
styles['Heading2'].fontName='DVS-Bold'; styles['Heading2'].fontSize=10; styles['Heading2'].leading=13
cell=ParagraphStyle('Cell',parent=styles['BodyText'],fontSize=7.8,leading=10,spaceAfter=0)
mono=ParagraphStyle('Mono',parent=styles['BodyText'],fontName='DVM',fontSize=7,leading=10)
def inline(s, source):
    s=html.escape(s)
    def link(m):
        url=html.unescape(m[2])
        if not url.startswith(('https://','http://')):
            url=URL+posixpath.normpath(REPO_PATH+'/'+source.parent.relative_to(ROOT).as_posix()+'/'+url)
        return '<link href="'+html.escape(url,quote=True)+'" color="#235E80">'+m[1]+'</link>'
    s=re.sub(r'\[([^\]]+)\]\(([^)]+)\)',link,s)
    s=re.sub(r'`([^`]+)`',r'<font name="DVM" size="7.5">\1</font>',s)
    s=re.sub(r'\*\*([^*]+)\*\*',r'<b>\1</b>',s)
    return s

def build(source, attachments=()):
    story=[]
    for block in source.read_text().split('\n\n'):
        block=block.strip()
        if not block: continue
        if block.startswith('!['):
            image=(source.parent/re.search(r'\]\(([^)]+)\)',block)[1]).resolve()
            pix=fitz.Pixmap(str(image)); width=510; height=width*pix.height/pix.width
            story.extend([Image(str(image),width=width,height=height),Spacer(1,8)])
        elif block.startswith('```'):
            story.append(Preformatted('\n'.join(block.splitlines()[1:-1]),mono))
        elif block.startswith('|'):
            lines=[x for x in block.splitlines() if not re.match(r'^\|[\s:|\-]+\|$',x)]
            data=[[Paragraph(inline(c.strip(),source),cell) for c in x.strip('|').split('|')] for x in lines]
            n=len(data[0]); widths=[140,80,290] if n==3 else [145,365]
            table=Table(data,colWidths=widths,repeatRows=1,hAlign='LEFT')
            table.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#E9EFF3')),('VALIGN',(0,0),(-1,-1),'TOP'),('LEFTPADDING',(0,0),(-1,-1),5),('RIGHTPADDING',(0,0),(-1,-1),5),('TOPPADDING',(0,0),(-1,-1),5),('BOTTOMPADDING',(0,0),(-1,-1),5),('LINEBELOW',(0,0),(-1,0),.5,colors.HexColor(GRAY))]))
            story.extend([table,Spacer(1,8)])
        elif block.startswith('#'):
            level=len(block)-len(block.lstrip('#'))
            story.append(Paragraph(inline(block.lstrip('#').strip(),source),styles['Title' if level==1 else 'Heading1' if level==2 else 'Heading2']))
        elif block.startswith('- '):
            for line in block.splitlines(): story.append(Paragraph('• '+inline(line[2:],source),styles['BodyText']))
        else: story.append(Paragraph(inline(block.replace('\n',' '),source),styles['BodyText']))
    def footer(canvas, doc):
        canvas.setFont('DVS',7); canvas.setFillColor(colors.HexColor('#607581'))
        canvas.drawString(38,22,'Period ECPP functional trial · 26 October 2026 · no historical-use or DLP claim')
        canvas.drawRightString(557,22,str(doc.page))
    pdfpath=source.with_suffix('.pdf'); raw=source.with_suffix('.rendering.pdf')
    SimpleDocTemplate(str(raw),pagesize=(595.276,841.89),leftMargin=38,rightMargin=38,topMargin=35,bottomMargin=39,title=source.read_text().splitlines()[0][2:]).build(story,onFirstPage=footer,onLaterPages=footer)
    doc=fitz.open(raw)
    for name,file in attachments:
        doc.embfile_add(name,file.read_bytes(),filename=name,desc=str(file.relative_to(ROOT)) if file.is_relative_to(ROOT) else file.name)
    doc.set_metadata({'title':source.read_text().splitlines()[0][2:],'subject':'Complete21 software outcomes; unknowns retained; no attack, security or historical selection claim'})
    doc.save(pdfpath,garbage=4,deflate=True); doc.close(); raw.unlink()
    return pdfpath
runpdfs=[]
for source in sorted(OUT.glob('run-*/REPORT.md')): runpdfs.append(build(source,[('visual-source.json',source.with_name('visual.json'))]))
attachments=[(name,ROOT/file) for name,file in [('PROTOCOL.md','PROTOCOL.md'),('native.cpp','native.cpp'),('cohort.tsv','results/cohort.tsv'),('summary.json','results/summary.json'),('initial-004-summary.json','results/initial-004-summary.json'),('archive-manifest.json','evidence/manifest.json'),('archive-delivery.json','evidence/archive-delivery.json'),('provenance.json','evidence/provenance.json'),('runs.tar.xz.sha256','runs.tar.xz.sha256'),('run.sh','run.sh'),('validate_saved.sh','validate_saved.sh')]]
attachments.append(('historical.tsv',ROOT.parent/'inputs/historical.tsv'))
attachments.extend((p.parent.name+'.pdf',p) for p in runpdfs)
build(ROOT/'REPORT.md',attachments)
