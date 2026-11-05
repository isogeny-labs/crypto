"""Render existing source-review prose and diagram; no research computation."""
import html
import os
import re
import subprocess
from pathlib import Path

os.environ.setdefault('XDG_CACHE_HOME', '/tmp/nist-source-render-cache')
Path(os.environ['XDG_CACHE_HOME']).mkdir(parents=True, exist_ok=True)

import fitz
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import SimpleDocTemplate, Paragraph, Image, Spacer

ROOT = Path(__file__).resolve().parent.parent
diagram = ROOT / 'reports/decision-gap'
for suffix, fmt in [('.svg', 'svg'), ('.png', 'png')]:
    subprocess.run(['dot', '-T' + fmt, '-Gdpi=150', str(diagram.with_suffix('.dot')),
                    '-o', str(diagram.with_suffix(suffix))], check=True)
font = Path('/usr/share/fonts/truetype/dejavu')
for name, file in [('DVS', 'DejaVuSans.ttf'), ('DVS-Bold', 'DejaVuSans-Bold.ttf')]:
    pdfmetrics.registerFont(TTFont(name, str(font / file)))
pdfmetrics.registerFontFamily('DVS', normal='DVS', bold='DVS-Bold', italic='DVS', boldItalic='DVS-Bold')
styles = getSampleStyleSheet()
for name in ['Title', 'BodyText', 'Heading1']:
    styles[name].fontName = 'DVS'
    styles[name].textColor = colors.HexColor('#17384B')
styles['Title'].fontSize = 18
styles['Title'].leading = 22
styles['Title'].spaceAfter = 10
styles['BodyText'].fontSize = 9
styles['BodyText'].leading = 12
styles['BodyText'].spaceAfter = 7
styles['BodyText'].allowWidows = 0
styles['BodyText'].allowOrphans = 0
styles['Heading1'].fontName = 'DVS-Bold'
styles['Heading1'].fontSize = 11
styles['Heading1'].leading = 14
styles['Heading1'].spaceBefore = 7
prefix = 'https://github.com/isogeny-labs/crypto/blob/codex/nist-period-ecpp-20261026/research/nist_skip_criterion_20261025/source_check_20261027/'

def inline(text):
    text = html.escape(text)
    def link(match):
        url = html.unescape(match[2])
        if not url.startswith('https://'):
            url = prefix + url
        return '<link href="' + html.escape(url, quote=True) + '" color="#235E80">' + match[1] + '</link>'
    text = re.sub(r'\[([^\]]+)\]\(([^)]+)\)', link, text)
    return re.sub(r'\*\*([^*]+)\*\*', r'<b>\1</b>', text)

story = []
for block in (ROOT / 'REPORT.md').read_text().split('\n\n'):
    block = block.strip()
    if not block:
        continue
    if block.startswith('!['):
        pix = fitz.Pixmap(str(diagram.with_suffix('.png')))
        story.extend([Image(str(diagram.with_suffix('.png')), width=510, height=510 * pix.height / pix.width), Spacer(1, 8)])
    elif block.startswith('#'):
        story.append(Paragraph(inline(block.lstrip('#').strip()), styles['Title' if block.startswith('# ') else 'Heading1']))
    else:
        story.append(Paragraph(inline(block.replace('\n', ' ')), styles['BodyText']))

def footer(canvas, doc):
    canvas.setFont('DVS', 7)
    canvas.setFillColor(colors.HexColor('#607581'))
    canvas.drawString(38, 22, 'Source check · 27 October 2026 · no historical selector established')
    canvas.drawRightString(557, 22, str(doc.page))

temp = ROOT / 'REPORT.rendering.pdf'
SimpleDocTemplate(str(temp), pagesize=(595.276, 841.89), leftMargin=38, rightMargin=38,
                  topMargin=35, bottomMargin=39).build(story, onFirstPage=footer, onLaterPages=footer)
doc = fitz.open(temp)
for name in ['source-check.json', 'sources/original-draft-report.md', 'sources/acceptance-report.md',
             'reports/decision-gap.dot', 'reports/render.py']:
    doc.embfile_add(name.replace('/', '__'), (ROOT / name).read_bytes(),
                    filename=name.replace('/', '__'), desc=name)
doc.set_metadata({'title': 'Historical skip criterion: source check',
                  'subject': 'Original decision evidence unavailable in the checked scope; criterion unresolved'})
doc.save(ROOT / 'REPORT.pdf', garbage=4, deflate=True)
doc.close()
temp.unlink()
