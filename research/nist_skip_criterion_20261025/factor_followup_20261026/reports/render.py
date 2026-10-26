"""Plot recorded native values and typeset Markdown. No research computation."""
import csv
import html
import json
import posixpath
import re
import sys
from pathlib import Path

import fitz
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import numpy as np
from matplotlib.patches import FancyBboxPatch
from reportlab.lib import colors
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import Image, KeepTogether, PageBreak, Paragraph, SimpleDocTemplate, Spacer, Table, TableStyle

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / 'reports'
REPO_PATH = 'research/nist_skip_criterion_20261025/factor_followup_20261026'
REPO_URL = 'https://github.com/isogeny-labs/crypto/blob/codex/nist-skip-criterion-20261025/'
INK = '#17384B'
GOLD = '#B27C3C'
plt.rcParams.update({'font.family': 'DejaVu Sans', 'font.size': 10})


def save_plot(fig, target):
    fig.savefig(target.with_suffix('.svg'))
    fig.savefig(target.with_suffix('.png'), dpi=150)
    plt.close(fig)
    p = target.with_suffix('.svg')
    p.write_text('\n'.join(line.rstrip() for line in p.read_text().splitlines()) + '\n')


if "--documents-only" not in sys.argv:
    coverage = json.loads((ROOT / 'results/factor-summary.json').read_text())
    families = ['minus', 'plus', 'twist', 'disc']
    fig, ax = plt.subplots(figsize=(8, 2.6), layout='constrained')
    values = [coverage['complete_by_family'][name] for name in families]
    ax.barh(['n-1', 'n+1', 'Twist', 'Discriminant'], values, color=[INK, GOLD, INK, INK], height=.6)
    for y, value in enumerate(values):
        ax.text(value + .2, y, str(value) + ' / 21', va='center')
    ax.invert_yaxis()
    ax.set_xlim(0, 24)
    ax.set_xticks([0, 7, 14, 21])
    ax.set_xlabel('Proved complete factorizations (21 curves per family)')
    ax.set_title('Native coverage: ' + str(coverage['complete_and_proven']) + ' of 84 targets proved complete',
                 loc='left', weight='bold')
    ax.spines[['top', 'right', 'left']].set_visible(False)
    save_plot(fig, OUT / 'factor-coverage')

    rows = list(csv.DictReader((ROOT / 'results/cohort-factor-bits.tsv').open(), delimiter='\t'))
    labels = list(dict.fromkeys(row['label'] for row in rows))
    by_key = {(row['label'], row['target']): row for row in rows}
    matrix = [[float(by_key[label, kind]['largest_per_field_permille'])
               if by_key[label, kind]['complete'] == 'true' else float('nan')
               for kind in families] for label in labels]
    fig, ax = plt.subplots(figsize=(8, 6.8), layout='constrained')
    cmap = plt.colormaps['Blues'].copy()
    cmap.set_bad('#E8E8E8')
    im = ax.imshow(np.ma.masked_invalid(matrix), cmap=cmap, vmin=0, vmax=1050, aspect='auto')
    ax.set_xticks(range(4), ['n-1', 'n+1', 'Twist', '|t²-4p|'])
    ax.set_yticks(range(len(labels)), [label + ('  P' if by_key[label, 'minus']['role'] == 'published' else '  S')
                                     for label in labels], fontsize=9)
    for i, label in enumerate(labels):
        for j, kind in enumerate(families):
            row = by_key[label, kind]
            text = row['largest_prime_bits']
            color = 'white' if row['complete'] == 'true' and int(row['largest_per_field_permille']) > 650 else INK
            ax.text(j, i, text, ha='center', va='center', fontsize=10, color=color)
    ax.set_title('Largest proved prime factor in each integer (bits)', loc='left', weight='bold', pad=12)
    ax.tick_params(length=0)
    ax.spines[:].set_visible(False)
    bar = fig.colorbar(im, ax=ax, fraction=.035, pad=.025)
    bar.set_label('Native recorded size / field bits (permille, rounded down)')
    fig.get_layout_engine().set(rect=(0, .045, 1, .955))
    fig.text(.5, .012, 'P = published; S = skipped. All 21 rows retained; all 84 factorizations proved complete.',
             ha='center', fontsize=8)
    save_plot(fig, OUT / 'cohort-factor-bits')

    baseline = json.loads((ROOT.parent / 'results/extended-generation.json').read_text())
    updated = json.loads((ROOT / 'results/full21-generation-summary.json').read_text())
    calibration_values = [baseline['short_rule_search_as_good_count'], updated['short_rule_search_as_good_count']]
    fig, ax = plt.subplots(figsize=(8, 2.5), layout='constrained')
    ax.barh([f"Initial {baseline['features_per_row']:,} features", f"Updated {updated['features_per_row']:,} features"], calibration_values,
            color=['#798B96', INK], height=.55)
    for i, value in enumerate(calibration_values):
        ax.text(value + 35, i, str(value) + ' / 4,480', va='center')
    ax.invert_yaxis()
    ax.set_xlim(0, 4480)
    ax.set_xlabel('Enumerated grouped label assignments admitting an exact 1–2-test fit')
    ax.set_title('Generation-family calibration on all 21 curves', loc='left', weight='bold')
    ax.spines[['top', 'right', 'left']].set_visible(False)
    save_plot(fig, OUT / 'generation-calibration')
    (OUT / 'graph-source.json').write_text(json.dumps({
        'kind': 'recorded-native-values-only/v1',
        'factor_coverage_source': '../results/factor-summary.json',
        'factor_bits_source': '../results/cohort-factor-bits.tsv',
        'generation_calibration_sources': ['../../results/extended-generation.json', '../results/full21-generation-summary.json'],
        'generation_calibration_recorded_counts': calibration_values,
        'population': 'All 21 curves; all 4,480 grouped label assignments',
        'uncertainty': 'Exact finite counts; exchangeability is a model assumption, not evidence of historical use'
    }, indent=2) + '\n')


def run_visual(spec_path):
    spec = json.loads(spec_path.read_text())
    if spec['kind'] == 'bars':
        fig, ax = plt.subplots(figsize=(8, 2.7), layout='constrained')
        ax.barh(spec['labels'], spec['values'], color=INK, height=.58)
        for i, value in enumerate(spec['values']):
            ax.text(value, i, '  ' + str(value), va='center')
        ax.invert_yaxis()
        ax.set_xlabel(spec['xlabel'])
        ax.set_xlim(left=0, right=max(spec['values']) * 1.18 if max(spec['values']) else 1)
        ax.spines[['top', 'right', 'left']].set_visible(False)
        ax.set_title(spec['title'], loc='left', weight='bold', fontsize=12)
    else:
        fig, ax = plt.subplots(figsize=(8, 2.6), layout='constrained')
        ax.set_xlim(0, 1)
        ax.set_ylim(0, 1)
        ax.axis('off')
        for i, text in enumerate(spec['steps']):
            x = .015 + i * .335
            ax.add_patch(FancyBboxPatch((x, .32), .29, .36,
                         boxstyle='round,pad=.012', facecolor='#EDF2F5', edgecolor=INK))
            ax.text(x + .145, .5, text, ha='center', va='center', fontsize=9)
            if i < 2:
                ax.annotate('', xy=(x + .325, .5), xytext=(x + .295, .5),
                            arrowprops={'arrowstyle': '->', 'color': INK})
        ax.set_title(spec['title'], loc='left', weight='bold', fontsize=12)
    fig.get_layout_engine().set(rect=(0, .12, 1, .88))
    fig.text(.5, .018, spec['note'], ha='center', fontsize=8)
    save_plot(fig, spec_path.parent / 'visual')


for spec in sorted(OUT.glob('run-*/visual.json')):
    if not spec.with_name('visual.svg').exists() or not spec.with_name('visual.png').exists():
        run_visual(spec)

font_dir = Path('/usr/share/fonts/truetype/dejavu')
for name, file in [('DVS', 'DejaVuSans.ttf'), ('DVS-Bold', 'DejaVuSans-Bold.ttf'),
                   ('DVS-Italic', 'DejaVuSans-Oblique.ttf'), ('DVM', 'DejaVuSansMono.ttf')]:
    pdfmetrics.registerFont(TTFont(name, str(font_dir / file)))
pdfmetrics.registerFontFamily('DVS', normal='DVS', bold='DVS-Bold', italic='DVS-Italic', boldItalic='DVS-Bold')
styles = getSampleStyleSheet()
for key in ['BodyText', 'Title', 'Heading1', 'Heading2']:
    styles[key].fontName = 'DVS'
    styles[key].textColor = colors.HexColor(INK)
styles['BodyText'].fontSize = 9
styles['BodyText'].leading = 11.5
styles['BodyText'].spaceAfter = 5
styles['Title'].fontSize = 22
styles['Title'].leading = 27
styles['Title'].spaceAfter = 12
styles['Heading1'].fontName = 'DVS-Bold'
styles['Heading1'].fontSize = 13
styles['Heading1'].spaceBefore = 9
styles['Heading1'].keepWithNext = True
styles['Heading2'].fontName = 'DVS-Bold'
styles['Heading2'].fontSize = 10.5
styles['Heading2'].keepWithNext = True
cell = ParagraphStyle('Cell', parent=styles['BodyText'], fontSize=8, leading=10, spaceAfter=0)


def inline(text, source):
    text = html.escape(text)
    def link(match):
        label, url = match[1], html.unescape(match[2])
        if not url.startswith(('https://', 'http://')):
            relative = str(source.parent.relative_to(ROOT))
            url = REPO_URL + posixpath.normpath(REPO_PATH + '/' + relative + '/' + url)
        return '<link href="' + html.escape(url) + '" color="#235E80">' + label + '</link>'
    text = re.sub(r'\[([^\]]+)\]\(([^)]+)\)', link, text)
    text = re.sub(r'`([^`]+)`', r'<font name="DVM" size="8">\1</font>', text)
    return re.sub(r'\*\*([^*]+)\*\*', r'<b>\1</b>', text)


def story_from(source, content=None):
    story = []
    for block in (source.read_text() if content is None else content).split('\n\n'):
        block = block.strip()
        if not block:
            continue
        if block.startswith('!['):
            match = re.search(r'\]\(([^)]+)\)', block)
            file = source.parent / match[1]
            if file.suffix == '.svg':
                file = file.with_suffix('.png')
            with fitz.open(file) as img:
                bounds = img[0].rect
            story.append(Image(str(file), width=506, height=506 * bounds.height / bounds.width))
            story.append(Spacer(1, 8))
        elif block.startswith('|'):
            lines = [line for line in block.splitlines() if not re.match(r'^\|[\s:|-]+\|$', line)]
            data = [[Paragraph(inline(c.strip(), source), cell) for c in line.strip('|').split('|')] for line in lines]
            widths = {2: [200, 306], 3: [190, 190, 126], 4: [180, 96, 115, 115]}[len(data[0])]
            table = Table(data, colWidths=widths, repeatRows=1, hAlign='LEFT')
            table.setStyle(TableStyle([
                ('BACKGROUND', (0, 0), (-1, 0), colors.HexColor('#E8EFF3')),
                ('VALIGN', (0, 0), (-1, -1), 'TOP'),
                ('LEFTPADDING', (0, 0), (-1, -1), 5), ('RIGHTPADDING', (0, 0), (-1, -1), 5),
                ('TOPPADDING', (0, 0), (-1, -1), 5), ('BOTTOMPADDING', (0, 0), (-1, -1), 5),
                ('LINEBELOW', (0, 0), (-1, 0), .6, colors.HexColor('#B7C5CF'))]))
            story.extend([KeepTogether([table]), Spacer(1, 8)])
        elif block.startswith('#'):
            level = len(block) - len(block.lstrip('#'))
            key = 'Title' if level == 1 else 'Heading1' if level == 2 else 'Heading2'
            story.append(Paragraph(inline(block.lstrip('#').strip(), source), styles[key]))
        elif block.startswith('- '):
            for item in re.split(r'\n(?=- )', block):
                story.append(Paragraph('• ' + inline(item[2:].replace('\n', ' '), source), styles['BodyText']))
        else:
            story.append(Paragraph(inline(block.replace('\n', ' '), source), styles['BodyText']))
    return story


def footer(canvas, doc):
    canvas.saveState()
    canvas.setFont('DVS', 7)
    canvas.setFillColor(colors.HexColor('#657986'))
    canvas.drawString(38, 24, 'Native factorization follow-up · 26 October 2026 · run status in report')
    canvas.drawRightString(557, 24, str(doc.page))
    canvas.restoreState()


def render(source, attachments=()):
    output = source.with_suffix('.pdf')
    doc = SimpleDocTemplate(str(output), pagesize=(595.276, 841.89), leftMargin=38, rightMargin=38,
                            topMargin=38, bottomMargin=40, title=source.read_text().splitlines()[0].lstrip('# '),
                            author='Native curve study')
    story = story_from(source)
    if source == ROOT / 'REPORT.md':
        factors = ROOT / 'results/completed-targets-hex.md'
        sections = factors.read_text().split('\n## ')
        story.extend(story_from(factors, '\n## '.join(sections[:3])))
        story.extend([PageBreak(), *story_from(factors, '## ' + '\n## '.join(sections[3:]))])
    doc.build(story, onFirstPage=footer, onLaterPages=footer)
    if attachments:
        pdf = fitz.open(output)
        for file in attachments:
            pdf.embfile_add(file.name, file.read_bytes(), filename=file.name, desc=posixpath.relpath(file, ROOT))
        temp = output.with_name('REPORT-attached.pdf')
        pdf.save(temp, deflate=True)
        pdf.close()
        temp.replace(output)


render(ROOT / 'REPORT.md', [ROOT / 'evidence/closed-runs.tar.xz', ROOT / 'results/cohort-factor-bits.tsv',
                            ROOT / 'evidence/backend-manifest-001.json',
                            ROOT / 'evidence/backend-progress-001-delivery.json',
                            ROOT / 'results/completed-targets-hex.md', ROOT / 'results/curve-13-plus.json',
                            ROOT / 'results/curve-19-disc.json', ROOT / 'results/curve-20-minus.json',
                            ROOT / 'results/curve-20-plus.json',
                            ROOT / 'evidence/backend-manifest-002.json',
                            ROOT / 'evidence/backend-progress-002-delivery.json',
                            ROOT / 'evidence/final-native-003.tar.zst',
                            ROOT / 'evidence/final-manifest-003.json',
                            ROOT / 'evidence/final-native-003-delivery.json',
                            ROOT / 'evidence/success-cleanup-audit.json',
                            ROOT.parent / 'sea_replay_20261026/results/coverage.tsv',
                            ROOT.parent / 'sea_replay_20261026/evidence/replay-001.tar.xz'])
for source in sorted(OUT.glob('run-*/REPORT.md')):
    if not source.with_suffix('.pdf').exists():
        render(source)

inspection = []
for file in [ROOT / 'REPORT.pdf', *sorted(OUT.glob('run-*/REPORT.pdf'))]:
    with fitz.open(file) as pdf:
        for number, page in enumerate(pdf):
            outside = []
            for block in page.get_text('blocks'):
                if block[0] < 0 or block[1] < 0 or block[2] > page.rect.width + .1 or block[3] > page.rect.height + .1:
                    outside.append(list(block[:4]))
            inspection.append({'file': str(file.relative_to(ROOT)), 'page': number + 1,
                               'text_bounds_outside_page': outside, 'text_present': bool(page.get_text().strip())})
            if outside:
                raise ValueError('text outside page ' + str(file))
(OUT / 'pdf-layout-checks.json').write_text(json.dumps(inspection, indent=2) + '\n')
print('Rendered report and run PDFs; page bounds checked. Visual page inspection is separate.')
