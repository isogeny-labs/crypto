"""Plot existing native values and typeset reports; no scientific computation."""
import html
import json
import os
import posixpath
import re
import textwrap
from pathlib import Path

os.environ.setdefault('MPLCONFIGDIR', '/tmp/nist-work-mpl')
os.environ.setdefault('XDG_CACHE_HOME', '/tmp/nist-work-document-cache')
import fitz
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import SimpleDocTemplate, Paragraph, Image, Table, TableStyle, Spacer, Preformatted

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / 'reports'
INK, BLUE, GOLD, GRAY = '#17384B', '#2F6F8F', '#A36B32', '#B8C3CB'
URL = 'https://github.com/isogeny-labs/crypto/blob/codex/nist-period-ecpp-20261026/'
RP = 'research/nist_skip_criterion_20261025/period_work_20261027'
LABELS = ['Root catalog records', 'Peak catalog records', 'Total catalog records',
          'Root returned forms', 'Total returned forms', 'Chain levels', 'Elliptic levels',
          'Factor prints', 'Factoring-parameter prints', 'Forced retries']
SHORT = ['Root\ncatalog', 'Peak\ncatalog', 'Total\ncatalog', 'Root\nforms', 'Total\nforms',
         'Chain\nlevels', 'Elliptic\nlevels', 'Factor\nprints', 'Parameter\nprints', 'Forced\nretries']
plt.rcParams.update({'font.family': 'DejaVu Sans', 'font.size': 9, 'svg.fonttype': 'none'})


def save(fig, path):
    fig.savefig(path.with_suffix('.svg'), bbox_inches='tight')
    fig.savefig(path.with_suffix('.png'), dpi=150, bbox_inches='tight')
    plt.close(fig)
    svg = path.with_suffix('.svg')
    svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines()) + '\n')


def envelopes(path, title):
    data = json.loads((ROOT / 'results/summary.json').read_text())
    values = [next(e['skipped_kept'] for e in row['envelopes']
                   if e['scope'] == 'per_field' and e['orientation'] == 'upper')
              for row in data['features']]
    fig, ax = plt.subplots(figsize=(8, 4.5), layout='constrained')
    ax.barh(LABELS, values, color=GOLD, height=.65)
    for i, value in enumerate(values):
        ax.text(value + .1, i, str(value), va='center', fontsize=9)
    ax.invert_yaxis()
    ax.set_xlim(0, 12.4)
    ax.set_xticks([0, 2, 4, 6, 8, 10, 11])
    ax.set_xlabel('Skipped orders still accepted (count out of 11)')
    ax.set_title(title, loc='left', weight='bold', fontsize=11, pad=12)
    ax.spines[['top', 'right']].set_visible(False)
    fig.get_layout_engine().set(rect=(0, .09, 1, 1))
    fig.text(.02, .015, 'All ten published controls retained by every bound · 21 rows · exact finite counts · labels visible', fontsize=7)
    save(fig, path)


def count_table(path):
    data = json.loads((ROOT / 'lean/count-data.json').read_text())
    rows = data['records']
    fig, ax = plt.subplots(figsize=(8.4, 2.75), layout='constrained')
    ax.axis('off')
    names = [r['label'] + ('\nomitted' if r['definition'].startswith('skipped') else '\npublished') for r in rows]
    table = ax.table(cellText=[r['values'] for r in rows], rowLabels=names,
                     colLabels=SHORT, cellLoc='center', rowLoc='left', loc='center',
                     colWidths=[.078] * 10)
    table.auto_set_font_size(False)
    table.set_fontsize(8)
    table.scale(1, 2.0)
    for (row, col), cell in table.get_celld().items():
        cell.set_edgecolor('#D4DDE3')
        if row == 0:
            cell.set_facecolor('#E9EFF3')
            cell.set_text_props(fontsize=7, color=INK)
        elif col == -1:
            cell.set_text_props(color=GOLD if rows[row-1]['definition'].startswith('skipped') else BLUE)
    ax.set_title('Exact count vectors: each omission ≤ each published control', loc='left', weight='bold', fontsize=11)
    fig.text(.02, .015, 'Native table → literal data bridge → Lean constants · no scores or normalized values', fontsize=7)
    save(fig, path)


def box(ax, x, y, text, color=BLUE, width=.34, height=.18):
    ax.add_patch(FancyBboxPatch((x, y), width, height, boxstyle='round,pad=.008',
                              facecolor='#F1F5F7', edgecolor=color))
    ax.text(x + width / 2, y + height / 2, text, ha='center', va='center', fontsize=9, color=INK)


def dominance(path, title):
    data = json.loads((ROOT / 'results/verification.json').read_text())
    positions = {'p239-1562': (.03, .62), 'p239-2682': (.03, .22),
                 'p239-2185': (.62, .62), 'p239-3048': (.62, .22)}
    fig, ax = plt.subplots(figsize=(8, 3.4), layout='constrained')
    ax.set_xlim(0, 1)
    ax.set_ylim(0, 1)
    ax.axis('off')
    for name, (x, y) in positions.items():
        box(ax, x, y, name + ('\nomitted' if x < .5 else '\npublished'), GOLD if x < .5 else BLUE)
    for pair in data['same_field_dominance_pairs']:
        x, y = positions[pair['skipped']]
        u, v = positions[pair['published']]
        ax.annotate('', xy=(u-.015, v+.09), xytext=(x+.355, y+.09),
                    arrowprops={'arrowstyle': '->', 'color': GRAY, 'lw': 1.3})
    ax.text(.49, .92, '≤ on every one of ten recorded counts', ha='center', fontsize=9)
    ax.text(.5, .05, 'Keeping prime239v1 under any monotone upper budget also keeps both omissions', ha='center', fontsize=8)
    ax.set_title(title, loc='left', weight='bold', fontsize=11)
    save(fig, path)


def diagram(path, spec):
    fig, ax = plt.subplots(figsize=(8, 2.5), layout='constrained')
    ax.set_xlim(0, 1)
    ax.set_ylim(0, 1)
    ax.axis('off')
    for i, step in enumerate(spec['steps']):
        x = .015 + i * .335
        box(ax, x, .39, step, width=.285, height=.35)
        if i < 2:
            ax.annotate('', xy=(x+.32, .56), xytext=(x+.295, .56), arrowprops={'arrowstyle': '->', 'color': GRAY})
    ax.set_title(spec['title'], loc='left', weight='bold', fontsize=11)
    ax.text(.02, .08, spec['caption'], fontsize=8)
    save(fig, path)


def proof_history(path, spec):
    data = json.loads((ROOT / 'proof/attempts.json').read_text())
    fig, ax = plt.subplots(figsize=(8, 4.2), layout='constrained')
    ax.axis('off')
    cells = [[r['id'], 'build ' + str(r.get('build_status', '—')) + ' / audit ' + str(r.get('audit_status', '—')),
              '\n'.join(textwrap.wrap(r['reason'], 70))] for r in data['attempts']]
    table = ax.table(cellText=cells, colLabels=['Attempt', 'Exit status', 'Recorded outcome'],
                     cellLoc='left', colWidths=[.09, .23, .68], loc='center')
    table.auto_set_font_size(False)
    table.set_fontsize(7)
    table.scale(1, 2.6)
    for (row, col), cell in table.get_celld().items():
        cell.set_edgecolor('#D4DDE3')
        if row == 0:
            cell.set_facecolor('#E9EFF3')
    ax.set_title(spec['title'], loc='left', weight='bold', fontsize=11)
    fig.text(.02, .01, 'Statuses only; final eight kernel proofs and unchanged axiom auditor pass · no measured clock', fontsize=7)
    save(fig, path)


envelopes(OUT / 'work-limits', 'Per-field upper work limits retain omissions')
count_table(OUT / 'dominance-table')
dominance(OUT / 'dominance', 'A conditional obstruction to monotone work budgets')
for vf in sorted(OUT.glob('*/visual.json')):
    spec = json.loads(vf.read_text())
    path = vf.parent / 'visual'
    if spec['kind'] == 'diagram':
        diagram(path, spec)
    elif spec['kind'] == 'envelopes':
        envelopes(path, spec['title'])
    elif spec['kind'] == 'dominance':
        dominance(path, spec['title'])
    else:
        proof_history(path, spec)
(OUT / 'graph-source.json').write_text(json.dumps({
    'kind': 'native-recorded-values-rendering/v1',
    'envelopes': '../results/summary.json', 'vectors': '../lean/count-data.json',
    'dominance': '../results/verification.json', 'proof_history': '../proof/attempts.json',
    'population': '21 reused saved packets;10 published,11 skipped;210 known counts',
    'uncertainty': 'Exact finite counts; labels visible; no sampled interval or held-out test',
    'python_role': 'Layout and literal value selection only; no scientific arithmetic'
}, indent=2) + '\n')

font = Path('/usr/share/fonts/truetype/dejavu')
for name, f in [('DVS', 'DejaVuSans.ttf'), ('DVS-Bold', 'DejaVuSans-Bold.ttf'), ('DVM', 'DejaVuSansMono.ttf')]:
    pdfmetrics.registerFont(TTFont(name, str(font / f)))
pdfmetrics.registerFontFamily('DVS', normal='DVS', bold='DVS-Bold', italic='DVS', boldItalic='DVS-Bold')
styles = getSampleStyleSheet()
for key in ['Title', 'BodyText', 'Heading1']:
    styles[key].fontName = 'DVS'
    styles[key].textColor = colors.HexColor(INK)
styles['Title'].fontSize = 20
styles['Title'].leading = 25
styles['Title'].spaceAfter = 12
styles['BodyText'].fontSize = 9
styles['BodyText'].leading = 12
styles['BodyText'].spaceAfter = 7
styles['BodyText'].allowWidows = 0
styles['BodyText'].allowOrphans = 0
styles['Heading1'].fontName = 'DVS-Bold'
styles['Heading1'].fontSize = 12
styles['Heading1'].leading = 15
styles['Heading1'].spaceBefore = 10
cell_style = ParagraphStyle('Cell', parent=styles['BodyText'], fontSize=7.6, leading=10, spaceAfter=0)
mono_style = ParagraphStyle('Mono', parent=styles['BodyText'], fontName='DVM', fontSize=7, leading=10)


def inline(s, src):
    s = html.escape(s)
    def link(match):
        url = html.unescape(match[2])
        if not url.startswith(('https://', 'http://')):
            url = URL + posixpath.normpath(RP + '/' + src.parent.relative_to(ROOT).as_posix() + '/' + url)
        return '<link href="' + html.escape(url, quote=True) + '" color="#235E80">' + match[1] + '</link>'
    s = re.sub(r'\[([^\]]+)\]\(([^)]+)\)', link, s)
    s = re.sub(r'`([^`]+)`', r'<font name="DVM" size="7.5">\1</font>', s)
    return re.sub(r'\*\*([^*]+)\*\*', r'<b>\1</b>', s)


def build(src, attachments):
    story = []
    for block in src.read_text().split('\n\n'):
        block = block.strip()
        if not block:
            continue
        if block.startswith('!['):
            path = (src.parent / re.search(r'\]\(([^)]+)\)', block)[1]).resolve()
            pix = fitz.Pixmap(str(path))
            story.extend([Image(str(path), width=510, height=510 * pix.height / pix.width), Spacer(1, 8)])
        elif block.startswith('```'):
            story.append(Preformatted('\n'.join(block.splitlines()[1:-1]), mono_style))
        elif block.startswith('|'):
            lines = [x for x in block.splitlines() if not re.match(r'^\|[\s:|\-]+\|$', x)]
            data = [[Paragraph(inline(c.strip(), src), cell_style) for c in x.strip('|').split('|')] for x in lines]
            table = Table(data, colWidths=[170, 340], repeatRows=1, hAlign='LEFT')
            table.setStyle(TableStyle([
                ('BACKGROUND', (0, 0), (-1, 0), colors.HexColor('#E9EFF3')),
                ('VALIGN', (0, 0), (-1, -1), 'TOP'), ('LEFTPADDING', (0, 0), (-1, -1), 5),
                ('RIGHTPADDING', (0, 0), (-1, -1), 5), ('TOPPADDING', (0, 0), (-1, -1), 5),
                ('BOTTOMPADDING', (0, 0), (-1, -1), 5), ('LINEBELOW', (0, 0), (-1, 0), .5, colors.HexColor(GRAY))]))
            story.extend([table, Spacer(1, 8)])
        elif block.startswith('#'):
            style = 'Title' if block.startswith('# ') else 'Heading1'
            story.append(Paragraph(inline(block.lstrip('#').strip(), src), styles[style]))
        else:
            story.append(Paragraph(inline(block.replace('\n', ' '), src), styles['BodyText']))
    def footer(canvas, doc):
        canvas.setFont('DVS', 7)
        canvas.setFillColor(colors.HexColor('#607581'))
        canvas.drawString(38, 22, 'Saved ECPP work · 27 October 2026 · conditional count-based exclusion; historical criterion unresolved')
        canvas.drawRightString(557, 22, str(doc.page))
    raw = src.with_suffix('.rendering.pdf')
    out = src.with_suffix('.pdf')
    title = src.read_text().splitlines()[0][2:]
    SimpleDocTemplate(str(raw), pagesize=(595.276, 841.89), leftMargin=38, rightMargin=38,
                      topMargin=35, bottomMargin=39, title=title).build(story, onFirstPage=footer, onLaterPages=footer)
    doc = fitz.open(raw)
    for name, path in attachments:
        doc.embfile_add(name, path.read_bytes(), filename=name, desc=str(path.relative_to(ROOT)))
    doc.set_metadata({'title': title, 'subject': '21 saved packets;210 checked counts;40 cutoffs fail; conditional monotone-budget proof'})
    doc.save(out, garbage=4, deflate=True)
    doc.close()
    raw.unlink()
    return out


run_pdfs = []
for src in sorted(OUT.glob('*/REPORT.md')):
    run_pdfs.append(build(src, [('visual-source.json', src.with_name('visual.json'))]))
files = ['PROTOCOL.md', 'plan.json', 'VALIDATION_PROTOCOL.md', 'inputs/historical.tsv',
         'inputs/admission.tsv', 'results/profile.tsv', 'results/summary.json',
         'results/verification.json', 'results/witnesses.tsv', 'export.json',
         'evidence/export-admission.json', 'evidence/preparation-delivery.json',
         'evidence/validator-delivery.json', 'evidence/final-delivery.json',
         'evidence/preparation.tar.xz', 'runs.tar.xz', 'profiler.cpp', 'verify.cpp', 'export.cpp',
         'lean/WorkLimits.lean', 'lean/count-data.json', 'lean/lean-toolchain',
         'proof/PROOF_MAP.json', 'proof/attempts.json', 'proof/commands.json',
         'proof/kernel-build.txt', 'proof/axiom-audit.txt', 'proof/REPRODUCE.md', 'REPRODUCE.md']
attachments = [(name.replace('/', '__'), ROOT / name) for name in files]
attachments.extend((p.parent.name + '.pdf', p) for p in run_pdfs)
build(ROOT / 'REPORT.md', attachments)
