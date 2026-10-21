"""Document rendering only: plot recorded values and typeset REPORT.md.

No curve arithmetic, statistical inference, factorization or experiment runs.
"""
import html
import json
import re
import textwrap
from pathlib import Path
from urllib.parse import urljoin

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from reportlab.lib import colors
from reportlab.lib.enums import TA_LEFT
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (
    Image, KeepTogether, PageBreak, Paragraph, SimpleDocTemplate, Spacer,
    Table, TableStyle, XPreformatted,
)

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / 'reports'
BASE = 'https://github.com/aburan28/crypto/blob/codex/curve-candidate-assessment-20261021/research/curve_candidates_dissect_20261021/hypotheses/'
rows = [json.loads((ROOT/'records'/f'curve-{i}.json').read_text()) for i in [0, 1, 2, 4, 3, 5]]
graph_source = {
    'kind': 'recorded-value-plot/v1', 'unit': 'bits of largest twist prime',
    'population': 'six eligible curves from two recorded generation streams',
    'uncertainty': 'exact integer bit lengths; no statistical interval inferred',
    'records': [{'source': f"../records/curve-{i}.json", 'label': row['label'],
                 'role': row['role'], 'bits': row['largest_twist_prime_bits']}
                for i, row in zip([0, 1, 2, 4, 3, 5], rows)],
}
(OUT/'graph-source.json').write_text(json.dumps(graph_source, indent=2)+'\n')
plt.rcParams.update({'font.family': 'DejaVu Sans', 'font.size': 10})
fig, ax = plt.subplots(figsize=(8.1, 3.7), layout='constrained')
for i, row in enumerate(rows):
    ax.barh(i, row['largest_twist_prime_bits'], height=.60,
            color='#17334B' if row['role']=='published' else '#D18A27')
    ax.text(row['largest_twist_prime_bits']+3, i, str(row['largest_twist_prime_bits']), va='center')
ax.set_yticks(range(6), [f"{r['field_bits']} / {r['candidate']} ({r['role']})" for r in rows])
ax.invert_yaxis(); ax.set_xlim(0, 265)
ax.set_xlabel('Largest prime factor of the base-field quadratic twist: bits')
ax.axhline(3.5, color='#9BA8B1', lw=.6)
ax.spines[['top', 'right', 'left']].set_visible(False)
ax.tick_params(axis='y', length=0)
fig.savefig(OUT/'twist-comparison.svg')
fig.savefig(OUT/'twist-comparison.png', dpi=150)
plt.close(fig)
svg = OUT/'twist-comparison.svg'
svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')

fonts = Path('/usr/share/fonts/truetype/dejavu')
for name, file in [('DVS', 'DejaVuSans.ttf'), ('DVS-Bold', 'DejaVuSans-Bold.ttf'),
                   ('DVS-Italic', 'DejaVuSans-Oblique.ttf'), ('DVM', 'DejaVuSansMono.ttf')]:
    pdfmetrics.registerFont(TTFont(name, str(fonts/file)))
pdfmetrics.registerFontFamily('DVS', normal='DVS', bold='DVS-Bold', italic='DVS-Italic', boldItalic='DVS-Bold')
styles = getSampleStyleSheet()
for key in ['Normal', 'BodyText', 'Title', 'Heading1', 'Heading2', 'Heading3']:
    styles[key].fontName = 'DVS'
    styles[key].textColor = colors.HexColor('#17334B')
styles['BodyText'].fontSize = 8.7; styles['BodyText'].leading = 12.3
styles['BodyText'].spaceAfter = 7
styles['Title'].fontSize = 23; styles['Title'].leading = 28
styles['Heading1'].fontName = 'DVS-Bold'; styles['Heading1'].fontSize = 14
styles['Heading2'].fontName = 'DVS-Bold'; styles['Heading2'].fontSize = 11
cell = ParagraphStyle('Cell', parent=styles['BodyText'], fontSize=7, leading=9, spaceAfter=0)
mono = ParagraphStyle('Mono', parent=styles['BodyText'], fontName='DVM', fontSize=6.8, leading=9)

def inline(value):
    value = html.escape(value)
    value = re.sub(r'\[([^\]]+)\]\(([^)]+)\)',
                   lambda m: '<link href="'+html.escape(urljoin(BASE, html.unescape(m[2])))+'" color="#235E80">'+m[1]+'</link>', value)
    value = re.sub(r'`([^`]+)`', r'<font name="DVM">\1</font>', value)
    value = re.sub(r'\*\*([^*]+)\*\*', r'<b>\1</b>', value)
    return value

story = []
blocks = (ROOT/'REPORT.md').read_text().split('\n\n')
for block in blocks:
    block = block.strip()
    if not block: continue
    if block.startswith('```'):
        lines = block.splitlines()[1:-1]
        story.append(XPreformatted('\n'.join(html.escape(line) for line in lines), mono))
    elif block.startswith('!['):
        story.append(Image(str(OUT/'twist-comparison.png'), width=506, height=231))
    elif block.startswith('|'):
        raw = [line for line in block.splitlines() if not re.match(r'^\|[\s:|-]+\|$', line)]
        data = [[Paragraph(inline(c.strip()), cell) for c in line.strip('|').split('|')] for line in raw]
        widths = [70, 54, 67, 82, 75, 75, 83] if len(data[0]) == 7 else [133, 247, 126]
        table = Table(data, colWidths=widths, repeatRows=1, hAlign='LEFT')
        table.setStyle(TableStyle([
            ('BACKGROUND', (0,0), (-1,0), colors.HexColor('#E9EFF3')),
            ('VALIGN', (0,0), (-1,-1), 'TOP'),
            ('LEFTPADDING', (0,0), (-1,-1), 5),
            ('RIGHTPADDING', (0,0), (-1,-1), 5),
            ('TOPPADDING', (0,0), (-1,-1), 6),
            ('BOTTOMPADDING', (0,0), (-1,-1), 6),
            ('LINEBELOW', (0,0), (-1,0), .6, colors.HexColor('#B7C5CF')),
        ]))
        story.extend([table, Spacer(1, 9)])
    elif block.startswith('#'):
        level = len(block)-len(block.lstrip('#'))
        story.append(Paragraph(inline(block.lstrip('#').strip()), styles['Title' if level==1 else 'Heading1' if level==2 else 'Heading2']))
    elif block.startswith('- '):
        items = re.split(r'\n(?=- )', block)
        for item in items:
            story.append(Paragraph('• '+inline(item[2:].replace('\n',' ')), styles['BodyText']))
    elif re.match(r'^\d+\. ', block):
        for item in re.split(r'\n(?=\d+\. )', block):
            story.append(Paragraph(inline(item.replace('\n',' ')), styles['BodyText']))
    else:
        story.append(Paragraph(inline(block.replace('\n',' ')), styles['BodyText']))
story.extend([PageBreak(), Paragraph('Exact model identities', styles['Heading1'])])
for row in rows:
    story.append(Paragraph(f"{row['label']} · {row['role']}", styles['Heading2']))
    story.append(XPreformatted('\n'.join(textwrap.wrap(row['icv1'], width=105, break_long_words=True, break_on_hyphens=False)), mono))
    story.append(Spacer(1,8))
story.append(Paragraph('Exact model, generator and subgroup parameters are frozen in inputs.tsv; source and output hashes accompany this supplement.', styles['BodyText']))

def footer(canvas, doc):
    canvas.saveState(); canvas.setFont('DVS', 7)
    canvas.setFillColor(colors.HexColor('#687A87'))
    canvas.drawString(38, 24, 'Selected / skipped hypotheses · exploratory · 21 October 2026')
    canvas.drawRightString(557, 24, str(doc.page)); canvas.restoreState()

doc = SimpleDocTemplate(str(ROOT/'HYPOTHESES.pdf'), pagesize=(595.276,841.89),
                        leftMargin=38, rightMargin=38, topMargin=38, bottomMargin=40,
                        title='Selected versus skipped NIST candidates: two open hypotheses',
                        author='Curve assessment')
doc.build(story, onFirstPage=footer, onLaterPages=footer)
