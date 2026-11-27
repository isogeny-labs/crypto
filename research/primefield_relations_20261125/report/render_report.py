#!/usr/bin/env python3
"""Typeset fixed, already-recorded evidence. No research result is computed here.

Run: python3 render_report.py
Requires ReportLab and DejaVu Sans. Writes REPORT.pdf and editable SVG figures.
The tiny SVG loader reads this script's rect/line/polygon/text vocabulary only.
Research implementations and arithmetic certificates remain Rust.
"""

from pathlib import Path
import xml.etree.ElementTree as ET

from reportlab.pdfgen import canvas
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.lib.colors import HexColor, Color
from reportlab.lib.styles import ParagraphStyle
from reportlab.platypus import Paragraph
from reportlab.graphics.shapes import Drawing, Rect, Line, Polygon, String
from reportlab.graphics import renderPDF

HERE = Path(__file__).resolve().parent
FONT_DIR = Path('/usr/share/fonts/truetype/dejavu')
for name, filename in [
    ('DejaVu', 'DejaVuSans.ttf'),
    ('DejaVu-Bold', 'DejaVuSans-Bold.ttf'),
]:
    pdfmetrics.registerFont(TTFont(name, str(FONT_DIR / filename)))
pdfmetrics.registerFontFamily('DejaVu', normal='DejaVu', bold='DejaVu-Bold')

NAVY = '#102C42'
TEAL = '#087E8B'
INK = '#243746'
MUTED = '#526574'
PALE = '#F0F5F7'
LINE = '#D8E2E8'
AMBER = '#8B5B18'
AMBER_BG = '#FFF4DF'
PAGE_W, PAGE_H = 612, 792
LEFT, RIGHT = 46, 566
WIDTH = RIGHT - LEFT

MEMBERSHIP_SVG = '''<svg xmlns="http://www.w3.org/2000/svg" width="1080" height="282" viewBox="0 0 1080 282">
<title>Fixed M31 membership and archived signed-replay gate</title>
<desc>Attempt 001: 29 selector roots become 17 rational abscissae and 34 signed lifts. Only three original labeled columns enter the archived signed replay. No new relation was discovered.</desc>
<rect x="0" y="0" width="1080" height="282" fill="#FFFFFF"/>
<text x="0" y="26" font-family="DejaVu Sans" font-size="22" font-weight="bold" fill="#102C42">Attempt 001: four distinct admission stages</text>
<rect x="0" y="49" width="237" height="160" rx="10" fill="#F0F5F7" stroke="#D8E2E8"/>
<rect x="281" y="49" width="237" height="160" rx="10" fill="#F0F5F7" stroke="#D8E2E8"/>
<rect x="562" y="49" width="237" height="160" rx="10" fill="#E8F4F4" stroke="#A6D5D8"/>
<rect x="843" y="49" width="237" height="160" rx="10" fill="#F0F5F7" stroke="#D8E2E8"/>
<text x="18" y="79" font-family="DejaVu Sans" font-size="17" font-weight="bold" fill="#102C42">Selector membership</text>
<text x="18" y="135" font-family="DejaVu Sans" font-size="40" font-weight="bold" fill="#087E8B">29</text>
<text x="18" y="167" font-family="DejaVu Sans" font-size="19" fill="#243746">distinct F_p roots</text>
<text x="18" y="192" font-family="DejaVu Sans" font-size="15" fill="#526574">complete support checked</text>
<text x="299" y="79" font-family="DejaVu Sans" font-size="19" font-weight="bold" fill="#102C42">Curve lift</text>
<text x="299" y="135" font-family="DejaVu Sans" font-size="40" font-weight="bold" fill="#087E8B">17 / 34</text>
<text x="299" y="167" font-family="DejaVu Sans" font-size="17" fill="#243746">abscissae / signed lifts</text>
<text x="299" y="192" font-family="DejaVu Sans" font-size="17" fill="#526574">on the canonical curve</text>
<text x="580" y="79" font-family="DejaVu Sans" font-size="19" font-weight="bold" fill="#102C42">Original labeled</text>
<text x="580" y="103" font-family="DejaVu Sans" font-size="19" font-weight="bold" fill="#102C42">columns</text>
<text x="580" y="151" font-family="DejaVu Sans" font-size="40" font-weight="bold" fill="#087E8B">3</text>
<text x="580" y="184" font-family="DejaVu Sans" font-size="18" fill="#243746">identity + public tags</text>
<text x="861" y="79" font-family="DejaVu Sans" font-size="19" font-weight="bold" fill="#102C42">Signed replay</text>
<text x="861" y="124" font-family="DejaVu Sans" font-size="20" font-weight="bold" fill="#087E8B">Archived relation</text>
<text x="861" y="151" font-family="DejaVu Sans" font-size="19" fill="#243746">beta != 0</text>
<text x="861" y="176" font-family="DejaVu Sans" font-size="17" fill="#243746">scalar verified</text>
<text x="861" y="200" font-family="DejaVu Sans" font-size="16" fill="#526574">zero new search calls</text>
<line x1="243" y1="129" x2="268" y2="129" stroke="#526574" stroke-width="3"/>
<polygon points="268,122 278,129 268,136" fill="#526574"/>
<line x1="524" y1="129" x2="549" y2="129" stroke="#526574" stroke-width="3"/>
<polygon points="549,122 559,129 549,136" fill="#526574"/>
<line x1="805" y1="129" x2="830" y2="129" stroke="#526574" stroke-width="3"/>
<polygon points="830,122 840,129 830,136" fill="#526574"/>
<text x="0" y="246" font-family="DejaVu Sans" font-size="19" fill="#526574">Fixed fixture; terminal values were conditioned on an archived relation.</text>
<text x="0" y="273" font-family="DejaVu Sans" font-size="18" fill="#526574">Evidence: shared-terminal-gate/results/attempt-001/result.json</text>
</svg>
'''

# All exponents below are copied from capacity-checker/evidence/bounds.tsv.
# Scaling bars is layout only; this program does not evaluate any probability.
RECORDED_BOUNDS = [(4, 405), (8, 207), (12, 66), (14, 5)]

def make_bounds_svg():
    parts = [
        '<svg xmlns="http://www.w3.org/2000/svg" width="1080" height="320" viewBox="0 0 1080 320">',
        '<title>Recorded exact probability-bound exponents</title>',
        '<desc>Uniform distinct tag model; depth at most 10, pool 32768, capture at least 128 classes. Recorded bounds for 4, 8, 12, and 14 selected fibers are at most 2^-405, 2^-207, 2^-66, and 2^-5.</desc>',
        '<rect x="0" y="0" width="1080" height="320" fill="#FFFFFF"/>',
        '<text x="0" y="26" font-family="DejaVu Sans" font-size="22" font-weight="bold" fill="#102C42">Certified exponent k in probability bound P &lt;= 2^-k</text>',
        '<text x="0" y="56" font-family="DejaVu Sans" font-size="18" fill="#526574">Derived bounds, not measured failure rates. Larger k is a stronger upper bound.</text>',
    ]
    start, axis_w = 130, 710
    for tick in [0, 100, 200, 300, 400]:
        x = start + axis_w * tick / 405
        parts.append(f'<line x1="{x}" y1="76" x2="{x}" y2="264" stroke="#E2E9EE" stroke-width="1"/>')
        parts.append(f'<text x="{x}" y="286" text-anchor="middle" font-family="DejaVu Sans" font-size="18" fill="#526574">{tick}</text>')
    for row, (fibers, exponent) in enumerate(RECORDED_BOUNDS):
        y = 86 + row * 45
        bar_w = axis_w * exponent / 405
        parts.append(f'<text x="0" y="{y+22}" font-family="DejaVu Sans" font-size="21" fill="#243746">t = {fibers}</text>')
        parts.append(f'<rect x="{start}" y="{y}" width="{bar_w}" height="29" rx="3" fill="#087E8B"/>')
        parts.append(f'<text x="1080" y="{y+22}" text-anchor="end" font-family="DejaVu Sans" font-size="22" font-weight="bold" fill="#102C42">P &lt;= 2^-{exponent}</text>')
    parts.append('<text x="0" y="317" font-family="DejaVu Sans" font-size="18" fill="#526574">Recorded data: capacity-checker/evidence/bounds.tsv; exact integer rounding.</text>')
    parts.append('</svg>')
    return '\n'.join(parts) + '\n'

def svg_drawing(path):
    root = ET.parse(path).getroot()
    w, h = float(root.attrib['width']), float(root.attrib['height'])
    drawing = Drawing(w, h)
    def col(value):
        return None if value in (None, 'none') else HexColor(value)
    for item in root:
        tag = item.tag.rsplit('}', 1)[-1]
        a = item.attrib
        fill, stroke = col(a.get('fill')), col(a.get('stroke'))
        sw = float(a.get('stroke-width', 1))
        if tag == 'rect':
            drawing.add(Rect(float(a['x']), h-float(a['y'])-float(a['height']),
                             float(a['width']), float(a['height']),
                             rx=float(a.get('rx', 0)), ry=float(a.get('rx', 0)),
                             fillColor=fill, strokeColor=stroke, strokeWidth=sw))
        elif tag == 'line':
            drawing.add(Line(float(a['x1']), h-float(a['y1']),
                             float(a['x2']), h-float(a['y2']),
                             strokeColor=stroke, strokeWidth=sw))
        elif tag == 'polygon':
            points = []
            for xy in a['points'].split():
                x, y = map(float, xy.split(','))
                points.extend([x, h-y])
            drawing.add(Polygon(points, fillColor=fill, strokeColor=stroke))
        elif tag == 'text':
            drawing.add(String(float(a['x']), h-float(a['y']), ''.join(item.itertext()),
                               fontName='DejaVu-Bold' if a.get('font-weight') == 'bold' else 'DejaVu',
                               fontSize=float(a['font-size']), fillColor=fill,
                               textAnchor=a.get('text-anchor', 'start')))
    return drawing

def text(c, x, y, value, size=10, color=INK, bold=False, mono=False):
    c.setFillColor(HexColor(color))
    # PDF's standard Courier face avoids embedding a third font subset.
    # Code labels here are fixed ASCII; research content is unchanged.
    c.setFont('Courier' if mono else ('DejaVu-Bold' if bold else 'DejaVu'), size)
    c.drawString(x, y, value)

def paragraph(c, x, top, width, value, size=10, leading=14, color=INK):
    style = ParagraphStyle('body', fontName='DejaVu', fontSize=size, leading=leading,
                           textColor=HexColor(color), allowWidows=0, allowOrphans=0)
    p = Paragraph(value, style)
    _, height = p.wrap(width, 1000)
    p.drawOn(c, x, top-height)
    return top-height

def box(c, x, y, w, h, fill=PALE, stroke=LINE, radius=8):
    c.setFillColor(HexColor(fill))
    c.setStrokeColor(HexColor(stroke))
    c.setLineWidth(.6)
    c.roundRect(x, y, w, h, radius, fill=1, stroke=1)

def heading(c, y, number, title):
    text(c, LEFT, y, number, 9, TEAL, True)
    text(c, LEFT+29, y-1, title, 12, NAVY, True)

def figure(c, path, x, y, width):
    d = svg_drawing(path)
    scale = width / d.width
    d.scale(scale, scale)
    renderPDF.draw(d, c, x, y)

def chrome(c, page):
    text(c, LEFT, 757, 'ISOGENY LABS  /  RESEARCH EVIDENCE', 8.4, TEAL, True)
    text(c, LEFT, 738, '25 NOVEMBER 2026  |  FIXED-CONTENT BRIEF', 8, MUTED)
    box(c, 453, 742, 113, 25, AMBER_BG, '#E6D2AD', 5)
    text(c, 462, 750, '[pcc:unverified]', 8.5, AMBER, True)
    c.setStrokeColor(HexColor(LINE))
    c.line(LEFT, 48, RIGHT, 48)
    text(c, LEFT, 31, 'primefield_relations_20261125  |  correctness and formula evaluation', 7.5, MUTED)
    text(c, 533, 31, f'{page} / 2', 8, MUTED)

def build():
    HERE.mkdir(parents=True, exist_ok=True)
    (HERE / 'membership-replay.svg').write_text(MEMBERSHIP_SVG)
    (HERE / 'capacity-bounds.svg').write_text(make_bounds_svg())
    (HERE / 'capacity-bounds.csv').write_text('selected_fibers_t,recorded_exponent_k_for_P_le_2_minus_k\n4,405\n8,207\n12,66\n14,5\n')

    c = canvas.Canvas(str(HERE / 'REPORT.pdf'), pagesize=(PAGE_W, PAGE_H), invariant=1)
    c.setTitle('Prime-field relation finding - shared selectors and public tags')
    c.setAuthor('Isogeny Labs')
    c.setSubject('Fixed M31 evidence brief; producer-run correctness; no performance claim')
    chrome(c, 1)
    text(c, LEFT, 699, 'Prime-field relation finding', 25, NAVY, True)
    text(c, LEFT, 676, 'Shared selectors and public tags', 14, MUTED)
    paragraph(c, LEFT, 655, WIDTH,
              'A scoped capacity bound and a native M31 replay now connect selector membership '
              'to original public labels. The evidence consists of exact arithmetic evaluation '
              'and deterministic correctness checks.', 10.2, 14.5)
    box(c, LEFT, 554, WIDTH, 51)
    text(c, LEFT+14, 587, 'FIELD PRIME p', 8, MUTED, True)
    text(c, LEFT+14, 568, '2147483647 = 2^31 - 1', 12, NAVY, True)
    text(c, LEFT+287, 587, 'CANONICAL GROUP ORDER r', 8, MUTED, True)
    text(c, LEFT+287, 568, '2147446831', 12, NAVY, True)

    heading(c, 529, '01', 'Adaptive uniform-tag capacity')
    paragraph(c, LEFT, 511, WIDTH,
              '<b>Model:</b> N = 32768 distinct nonidentity negation classes sampled uniformly '
              'without replacement from M = 1073723415. Chains contain at most 10 reduced '
              'quadratic rational maps; each fiber has capacity at most 1024. The event is '
              'capture of at least 128 sampled classes in at most t fibers. Chain fitting '
              'and terminal selection are both adaptive. [1]', 9.5, 13.4)
    figure(c, HERE / 'capacity-bounds.svg', LEFT, 274, WIDTH)
    text(c, LEFT, 257, 'CHECKED: 4 Rust tests passed; 62 full exact rational certificates retained.', 9, TEAL, True)
    paragraph(c, LEFT, 244, WIDTH,
              'Scope is the stated uniform-tag model and map family. These are conditional '
              'mathematical bounds, not empirical failure rates or universal ECDLP lower bounds.', 8.8, 12, MUTED)

    heading(c, 197, '02', 'Membership to original-column replay')
    figure(c, HERE / 'membership-replay.svg', LEFT, 58, WIDTH)
    c.showPage()

    chrome(c, 2)
    text(c, LEFT, 699, 'Evidence and trust boundary', 24, NAVY, True)
    paragraph(c, LEFT, 676, WIDTH,
              '<b>Attempt 001:</b> the shared-terminal gate passed 2 Rust tests and all 47 '
              'recorded assertion entries, covering 31 distinct check names. The fixture uses '
              'a depth-three Dickson chain with six selected terminal values. [2]', 10.1, 14.3)

    box(c, LEFT, 542, WIDTH, 78, '#E8F4F4', '#A6D5D8')
    text(c, LEFT+14, 599, 'ARCHIVED USEFUL RELATION - REPLAYED', 9, TEAL, True)
    text(c, LEFT+14, 579, 'alpha = 1144570523    beta = 2011873483 != 0 mod r', 9.4, NAVY, mono=True)
    text(c, LEFT+14, 560, 'd = 1038579171; [d]G = Q verified', 10, NAVY, True)

    paragraph(c, LEFT, 526, WIDTH,
              'The program reconstructs the historical public source and uses its archived '
              'indices and signs. It makes <b>zero relation-search or pair-table calls</b>. '
              'The known scalar is expected verifier data. This is an archived correctness '
              'replay, not a blinded discovery run. [2]', 9.6, 13.3)

    heading(c, 458, '03', 'Tagged reflection: constructive control')
    text(c, LEFT, 434, 'h_A(R) = x(R - [1/2]A),     fiber = {R, A-R}', 10, NAVY, mono=True)
    paragraph(c, LEFT, 421, WIDTH,
              'Both points receive public tags by transport on the curve. The constructed '
              'identity R + (A-R) - A has <b>alpha = beta = 0</b> and is rejected as useless. '
              'This verifies complete labeling for one curve-map fiber while preserving '
              'the nonzero-target-coefficient requirement. [2, 3]', 9.5, 13.1)

    heading(c, 351, '04', 'Standing and remaining dependency')
    paragraph(c, LEFT, 333, WIDTH,
              '<b>[pcc:unverified]</b> Producer-run correctness evidence. The mathematical '
              'arguments are reviewed hand proofs, <b>not Lean-checked</b>; exact-integer '
              'certificates validate formula evaluation. No new relation search, timing '
              'comparison, or speedup claim is made.', 9.4, 13.0)
    paragraph(c, LEFT, 273, WIDTH,
              '<b>Next dependency:</b> jointly construct a public-tag source, selector, and '
              'extractor that skips candidate classes and returns a new relation with '
              'nonzero aggregate target coefficient. <b>Impact: 3/10</b> - a concrete '
              'capacity result and reusable admission checks, with extraction still open.', 9.3, 12.8)

    heading(c, 205, '05', 'Evidence paths')
    text(c, LEFT, 185, 'Relative to research/primefield_relations_20261125/', 8, MUTED)
    sources = [
        '[1] capacity-checker/evidence/bounds.tsv',
        '    capacity-checker/evidence/certificates.json',
        '    capacity-checker/evidence/tests.txt',
        '[2] shared-terminal-gate/results/attempt-001/result.json',
        '    shared-terminal-gate/results/attempt-001/tests.stdout',
        '[3] notes/adaptive-chain-proof.md',
        '    notes/selector-tag-bridge.md',
    ]
    for index, value in enumerate(sources):
        text(c, LEFT, 169-index*13, value, 8.1, INK, mono=True)
    c.save()
    print(HERE / 'REPORT.pdf')

if __name__ == '__main__':
    build()
