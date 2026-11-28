#!/usr/bin/env python3
"""Typeset the native M31 operation census; do not compute research results.

The input must be the actual Rust-produced initial.json. Scientific aggregates,
phase sums, count comparisons and the exact source-share fraction are read from
its presentation_summary. This script only formats recorded values, lays out
plots and hashes the input bytes for document provenance. It never runs the
measurement or derives a cost, ratio, yield, uncertainty or timing result.

Example (after the frozen run and its receipts exist):
  python3 render_report.py --measurement-commit COMMIT --executable-sha256 SHA256

Writes REPORT.pdf, phase-work.svg and membership-work.svg beside this source.
Requires ReportLab, DejaVu Sans and the actual input; there is no mock-data mode.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re
from xml.sax.saxutils import escape
import xml.etree.ElementTree as ET

from reportlab.graphics import renderPDF
from reportlab.graphics.shapes import Drawing, Line, Rect, String
from reportlab.lib.colors import HexColor
from reportlab.lib.styles import ParagraphStyle
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.pdfgen import canvas
from reportlab.platypus import Paragraph


HERE = Path(__file__).resolve().parent
MEASUREMENT = HERE.parent
STUDY = MEASUREMENT.parent
REPO = STUDY.parent.parent
FONT_DIR = Path('/usr/share/fonts/truetype/dejavu')
PAGE_W, PAGE_H = 612, 792
LEFT, RIGHT, WIDTH = 46, 566, 520
NAVY, TEAL, INK = '#102C42', '#087E8B', '#243746'
MUTED, PALE, LINE = '#526574', '#F0F5F7', '#D8E2E8'
AMBER, AMBER_BG, BLUE = '#8B5B18', '#FFF4DF', '#526C94'
FP = 'fp_modular_products_including_squares'
FR = 'fr_modular_products_including_squares'
SOURCE_CORPUS = 'ordered_archived_4608_x_stream'
POSITIVE_CORPUS = 'all_29_positive_selector_roots'

PHASE_LABELS = {
    'curve_admission': 'Curve and target admission',
    'archived_source_generation': 'Archived source generation',
    'canonical_full_pool_roster': 'Full-pool roster preparation',
    'historical_columns_and_public_tags': 'Historical public-tag checks',
    'terminal_fixture_construction': 'Terminal fixture construction',
    'recursive_root_recovery': 'Inverse-fiber root recovery',
    'exact_support_certificate': 'Exact support certificate',
    'rational_curve_lifts': 'Rational curve lifts',
    'full_4608_pool_label_admission': 'Full-pool label admission',
    'historical_three_column_admission': 'Historical label admission',
    'archived_relation_replay': 'Archived relation replay',
}


def number(value: int) -> str:
    """Decimal formatting only; counts are supplied by Rust."""
    if type(value) is not int or value < 0:
        raise ValueError(f'Expected a nonnegative native integer, got {value!r}')
    return f'{value:,}'


def register_fonts() -> None:
    for name, filename in [('DejaVu', 'DejaVuSans.ttf'), ('DejaVu-Bold', 'DejaVuSans-Bold.ttf')]:
        pdfmetrics.registerFont(TTFont(name, str(FONT_DIR / filename)))
    pdfmetrics.registerFontFamily('DejaVu', normal='DejaVu', bold='DejaVu-Bold')


def svg_text(x, y, value, size=18, color=INK, bold=False, anchor='start') -> str:
    weight = ' font-weight="bold"' if bold else ''
    return (f'<text x="{x}" y="{y}" font-family="DejaVu Sans" font-size="{size}" '
            f'fill="{color}" text-anchor="{anchor}"'
            f'{weight}>{escape(str(value))}</text>')


def svg_start(title: str, desc: str, height: int) -> list[str]:
    return [f'<svg xmlns="http://www.w3.org/2000/svg" width="1080" height="{height}" '
            f'viewBox="0 0 1080 {height}">', f'<title>{escape(title)}</title>',
            f'<desc>{escape(desc)}</desc>',
            f'<rect x="0" y="0" width="1080" height="{height}" fill="#FFFFFF"/>']


def phase_svg(summary: dict) -> str:
    rows = summary['phases']
    row_h, first_y = 29, 75
    total_y = first_y + len(rows) * row_h + 12
    height = total_y + 58
    out = svg_start('Typed work by charged phase',
                    'Fp products have linear bars; Fr products, group-add calls and '
                    'point-scalar calls are separate count columns, never one cost.', height)
    out += [svg_text(0, 22, 'Charged phase', 19, NAVY, True),
            svg_text(625, 22, 'Fp products', 18, NAVY, True, 'end'),
            svg_text(770, 22, 'Fr products', 17, NAVY, True, 'end'),
            svg_text(925, 22, 'Group-add calls', 15, NAVY, True, 'end'),
            svg_text(1080, 22, 'Point-scalar calls', 14, NAVY, True, 'end'),
            svg_text(0, 46, 'Products include squares. Bars show only Fp products; numeric columns keep their own units.', 16, MUTED)]
    # A maximum and conversion to pixels are plotting geometry, not a result.
    scale_max = max([row['counts'][FP] for row in rows] + [1])
    for i, row in enumerate(rows):
        y, counts = first_y + i * row_h, row['counts']
        if i % 2 == 0:
            out.append(f'<rect x="0" y="{y-19}" width="1080" height="{row_h}" fill="{PALE}"/>')
        label = PHASE_LABELS[row['phase']]
        out.append(svg_text(8, y, label, 16, INK, row['phase'] == 'archived_source_generation'))
        bar_width = 196 * counts[FP] / scale_max
        if counts[FP]:
            out.append(f'<rect x="278" y="{y-15}" width="{bar_width:.6f}" height="18" fill="{TEAL}"/>')
        for x, key in [(625, FP), (770, FR), (925, 'group_add_calls'), (1080, 'point_scalar_calls')]:
            out.append(svg_text(x-8, y, number(counts[key]), 17, NAVY, False, 'end'))
    out.append(f'<line x1="0" y1="{total_y-23}" x2="1080" y2="{total_y-23}" stroke="{LINE}" stroke-width="2"/>')
    out.append(svg_text(8, total_y, 'Common counted path', 17, NAVY, True))
    common = summary['common_counted_path']
    for x, key in [(625, FP), (770, FR), (925, 'group_add_calls'), (1080, 'point_scalar_calls')]:
        out.append(svg_text(x-8, total_y, number(common[key]), 17, NAVY, True, 'end'))
    out.append(svg_text(0, height-31, 'Other counter families remain in the raw ledger; zero displayed products do not make a phase free.', 15, MUTED))
    out.append(svg_text(0, height-12, 'Repeated membership controls and the all-positive diagnostic corpus are outside these phase totals.', 15, MUTED))
    out.append('</svg>')
    return '\n'.join(out) + '\n'


def membership_svg(prepared: dict) -> str:
    out = svg_start('Prepared membership: three separate units',
                    'A is the D8 chain followed by six-terminal lookup. B searches '
                    'the certified explicit root set. Each panel has an independent '
                    'linear count axis; key payload excludes container overhead.', 292)
    specs = [('Fp products', 'products per full query stream', lambda arm: arm['counts'][FP]),
             ('Key comparisons', 'three-way comparator calls', lambda arm: arm['three_way_key_comparisons']),
             ('Key payload', 'bytes of stored u64 keys', lambda arm: arm['key_payload_bytes'])]
    for panel, (title, subtitle, field) in enumerate(specs):
        x, w = panel * 368, 344
        out.append(f'<rect x="{x}" y="0" width="{w}" height="252" rx="8" fill="{PALE}" stroke="{LINE}"/>')
        out.append(svg_text(x+17, 30, title, 22, NAVY, True))
        out.append(svg_text(x+17, 55, subtitle, 16, MUTED))
        # Independent axes prevent comparisons across unlike units.
        panel_max = max(field(prepared['A']), field(prepared['B']), 1)
        for row, arm in enumerate(['A', 'B']):
            value, y = field(prepared[arm]), 100 + row * 75
            color = TEAL if arm == 'A' else BLUE
            out.append(svg_text(x+17, y+6, arm, 20, color, True))
            if value:
                out.append(f'<rect x="{x+48}" y="{y-15}" width="{185*value/panel_max:.3f}" height="24" rx="2" fill="{color}"/>')
            out.append(svg_text(x+w-16, y+6, number(value), 21, NAVY, True, 'end'))
        out.append(svg_text(x+48, 227, '0', 15, MUTED))
        out.append(svg_text(x+233, 227, number(panel_max), 15, MUTED, False, 'end'))
        out.append(f'<line x1="{x+48}" y1="207" x2="{x+233}" y2="207" stroke="{LINE}" stroke-width="1"/>')
    out += [svg_text(0, 282, 'A  D8 chain + terminal lookup', 18, TEAL, True),
            svg_text(556, 282, 'B  Explicit certified-root lookup', 18, BLUE, True), '</svg>']
    return '\n'.join(out) + '\n'


def svg_drawing(path: Path) -> Drawing:
    root = ET.parse(path).getroot()
    w, h = float(root.attrib['width']), float(root.attrib['height'])
    drawing = Drawing(w, h)
    def color(value):
        return None if value in (None, 'none') else HexColor(value)
    for item in root:
        tag, a = item.tag.rsplit('}', 1)[-1], item.attrib
        fill, stroke = color(a.get('fill')), color(a.get('stroke'))
        if tag == 'rect':
            drawing.add(Rect(float(a['x']), h-float(a['y'])-float(a['height']),
                             float(a['width']), float(a['height']),
                             rx=float(a.get('rx', 0)), ry=float(a.get('rx', 0)),
                             fillColor=fill, strokeColor=stroke, strokeWidth=float(a.get('stroke-width', 1))))
        elif tag == 'line':
            drawing.add(Line(float(a['x1']), h-float(a['y1']), float(a['x2']), h-float(a['y2']),
                             strokeColor=stroke, strokeWidth=float(a.get('stroke-width', 1))))
        elif tag == 'text':
            drawing.add(String(float(a['x']), h-float(a['y']), ''.join(item.itertext()),
                               fontName='DejaVu-Bold' if a.get('font-weight') == 'bold' else 'DejaVu',
                               fontSize=float(a['font-size']), fillColor=fill,
                               textAnchor=a.get('text-anchor', 'start')))
    return drawing


def text(c, x, y, value, size=10, color=INK, bold=False, mono=False):
    c.setFillColor(HexColor(color))
    c.setFont('Courier' if mono else ('DejaVu-Bold' if bold else 'DejaVu'), size)
    c.drawString(x, y, str(value))


def paragraph(c, x, top, width, value, size=9.2, leading=12.5, color=INK):
    style = ParagraphStyle('body', fontName='DejaVu', fontSize=size, leading=leading,
                           textColor=HexColor(color), allowWidows=0, allowOrphans=0)
    p = Paragraph(value, style)
    _, height = p.wrap(width, 1000)
    p.drawOn(c, x, top-height)
    return top-height


def box(c, x, y, width, height, fill=PALE, stroke=LINE, radius=7):
    c.setFillColor(HexColor(fill))
    c.setStrokeColor(HexColor(stroke))
    c.setLineWidth(.6)
    c.roundRect(x, y, width, height, radius, fill=1, stroke=1)


def figure(c, path, x, y, width):
    d = svg_drawing(path)
    scale = width/d.width
    d.scale(scale, scale)
    renderPDF.draw(d, c, x, y)


def heading(c, y, section, title):
    text(c, LEFT, y, section, 8.5, TEAL, True)
    text(c, LEFT+28, y-1, title, 12, NAVY, True)


def chrome(c, page):
    text(c, LEFT, 757, 'ISOGENY LABS  /  MEASUREMENT EVIDENCE', 8.3, TEAL, True)
    text(c, LEFT, 739, 'M31 SHARED SELECTOR  |  ATTEMPT 001', 8, MUTED)
    box(c, 453, 742, 113, 25, AMBER_BG, '#E6D2AD', 5)
    text(c, 462, 750, '[pcc:unverified]', 8.5, AMBER, True)
    c.setStrokeColor(HexColor(LINE))
    c.line(LEFT, 48, RIGHT, 48)
    text(c, LEFT, 31, 'Fixed archived fixture  |  native operation counts  |  PR #5217', 7.4, MUTED)
    text(c, 533, 31, f'{page} / 2', 8, MUTED)


def path_label(path: Path) -> str:
    return path.resolve().relative_to(REPO).as_posix()


def require_hex(value: str, length: int, label: str) -> str:
    if not re.fullmatch(f'[0-9a-f]{{{length}}}', value):
        raise ValueError(f'{label} must be a full lowercase {length}-hex identity')
    return value


def build(args):
    raw_bytes = args.input.read_bytes()
    data = json.loads(raw_bytes)
    if data['schema'] != 'isogeny.m31-shared-selector-operation-census/v1' or data['status'] != 'pass':
        raise ValueError('This report template requires the successful native census schema')
    summary = data['presentation_summary']
    corpora = {x['corpus']: x for x in summary['prepared_query_corpora']}
    prepared, positive = corpora[SOURCE_CORPUS], corpora[POSITIVE_CORPUS]
    recorded_corpora = {x['corpus']: x for x in data['membership_corpora']}
    source_pass = recorded_corpora[SOURCE_CORPUS]['passes'][0]
    admission, selector, relation = data['label_admission'], data['selector'], data['historical_relation']
    share, provenance = summary['source_phase_share_of_common_fp_products'], data['source_provenance']
    raw_sha = hashlib.sha256(raw_bytes).hexdigest()
    source_commit = require_hex(args.measurement_commit, 40, 'Measurement source commit')
    binary_sha = require_hex(args.executable_sha256, 64, 'Executable SHA-256')
    replay_receipt_path = args.replay_receipt or args.input.with_name('replay-comparison.json')
    publication_receipt_path = args.source_publication_receipt or args.input.with_name('source-publication-receipt.json')
    replay_receipt = json.loads(replay_receipt_path.read_text())
    publication_receipt = json.loads(publication_receipt_path.read_text())
    # Bind supplied producer receipts to document provenance. These metadata
    # checks do not rerun a test, compare scientific outputs, or confer PCC standing.
    if (replay_receipt['schema'] != 'isogeny.m31-counts-replay-comparison/v1'
            or replay_receipt['source_commit'] != source_commit
            or replay_receipt['executable_sha256'] != binary_sha
            or replay_receipt['initial']['sha256'] != raw_sha
            or replay_receipt['replay']['sha256'] != raw_sha
            or replay_receipt['initial']['exit_code'] != 0
            or replay_receipt['replay']['exit_code'] != 0
            or replay_receipt['byte_identical'] is not True
            or replay_receipt['tests']['exit_code'] != 0):
        raise ValueError('The producer replay receipt does not support this successful run report')
    if (publication_receipt['commit'] != source_commit
            or publication_receipt['published_before_outcomes'] is not True):
        raise ValueError('Source-publication receipt does not match the measured source')
    recorded_tests = replay_receipt['tests']
    for manifest in [args.pre_call_manifest, args.post_call_manifest]:
        if manifest is not None and not manifest.is_file():
            raise FileNotFoundError(manifest)

    register_fonts()
    phase_path, member_path = HERE/'phase-work.svg', HERE/'membership-work.svg'
    phase_path.write_text(phase_svg(summary), encoding='utf-8')
    member_path.write_text(membership_svg(prepared), encoding='utf-8')
    c = canvas.Canvas(str(HERE/'REPORT.pdf'), pagesize=(PAGE_W, PAGE_H), invariant=1)
    c.setTitle('M31 shared-selector work - deterministic native count measurement')
    c.setAuthor('Isogeny Labs')
    c.setSubject('Fixed archived-fixture counts; no admitted clock, rho or discovery claim')

    chrome(c, 1)
    text(c, LEFT, 698, 'M31 shared-selector work', 25, NAVY, True)
    text(c, LEFT, 675, 'A deterministic census of construction and replay', 12.8, MUTED)
    paragraph(c, LEFT, 654, WIDTH,
              'The measured path reconstructs a public archived source, certifies the selector '
              'support, admits original labels and replays the known useful relation. '
              '<b>The target, row and terminal fixture were fixed before this run.</b>', 9.7, 13.3)
    box(c, LEFT, 563, WIDTH, 45)
    text(c, LEFT+12, 592, 'FIELD PRIME p', 7.6, MUTED, True)
    text(c, LEFT+12, 576, number(data['p']), 11.5, NAVY, True)
    text(c, LEFT+185, 592, 'GROUP ORDER r', 7.6, MUTED, True)
    text(c, LEFT+185, 576, number(data['r']), 11.5, NAVY, True)
    text(c, LEFT+360, 592, 'PUBLIC SOURCE', 7.6, MUTED, True)
    text(c, LEFT+360, 576, number(data['source_pool']['records'])+' columns', 11.5, NAVY, True)

    heading(c, 540, '01', 'Where the measured arithmetic is spent')
    figure(c, phase_path, LEFT, 292, WIDTH)
    text(c, LEFT, 278, 'Exact source-generation share of common Fp products', 8.4, MUTED, True)
    text(c, LEFT, 262, number(share['numerator'])+' / '+number(share['denominator']), 11, NAVY, True)
    text(c, LEFT+330, 263, 'Native-recorded unreduced fraction', 7.6, MUTED)

    heading(c, 235, '02', 'Complete support and full-pool coverage')
    coverage = [('FIELD ROOTS', positive['query_count']),
                ('RATIONAL x VALUES', admission['rational_abscissae_tested']),
                ('SIGNED CURVE LIFTS', selector['signed_curve_lifts']),
                ('ADMITTED POOL COLUMNS', admission['full_pool_admitted_count'])]
    for i, (label, value) in enumerate(coverage):
        x = LEFT+i*132
        box(c, x, 163, 124, 54)
        text(c, x+9, 200, label, 6.5, MUTED, True)
        text(c, x+9, 177, number(value), 20, TEAL, True)
    paragraph(c, LEFT, 150, WIDTH,
              f'The support is joined against all <b>{number(admission["full_pool_columns"])}</b> original columns; '
              f'<b>{number(admission["full_pool_rejected_rational_abscissae"])}</b> rational abscissae have no admitted '
              f'column in that pool. The separate historical roster has '
              f'<b>{number(admission["historical_roster_columns"])}</b> columns. Full-pool admission is a fixture-coverage '
              'count, not a new-relation yield estimate.', 8.5, 11.8)
    paragraph(c, LEFT, 91, WIDTH,
              '<b>Counter boundary:</b> field products include exponentiation squares. Inversion/square-root '
              'calls overlap that work. Group and point-scalar calls are structural counts, not GAE. '
              'Hash/tree probes, allocator/copying work and direct small-constant arithmetic are unpriced.', 7.7, 10.5, MUTED)
    c.showPage()

    chrome(c, 2)
    text(c, LEFT, 699, 'Prepared membership and custody', 23, NAVY, True)
    paragraph(c, LEFT, 677, WIDTH,
              f'Both arms receive the identical ordered <b>{number(prepared["query_count"])}</b>-query public-source '
              f'stream and accept <b>{number(source_pass["hits"])}</b> entries. A evaluates the depth-three D8 chain '
              'then searches six terminals; B searches the same certified explicit root set.', 9.5, 13.2)
    figure(c, member_path, LEFT, 475, WIDTH)
    paragraph(c, LEFT, 461, WIDTH,
              '<b>Prepared lookup only:</b> one full query pass per arm. Key payload is stored u64 keys; '
              'headers, spare capacity and shared modulus/depth are excluded. A zero product count '
              'does not remove comparison or storage work, and no field-product ratio is defined.', 8.4, 11.6)

    heading(c, 407, '03', 'Setup stays charged in the complete construction path')
    charged = summary['charged_common_plus_one_full_stream_pass']
    box(c, LEFT, 334, WIDTH, 54)
    text(c, LEFT+12, 371, 'COMMON PHASES + ONE SOURCE-QUERY PASS', 7.2, MUTED, True)
    text(c, LEFT+12, 349, 'A', 10, TEAL, True)
    text(c, LEFT+33, 349, number(charged['A'][FP])+' Fp products', 9.2, NAVY, True)
    text(c, LEFT+275, 349, 'B', 10, BLUE, True)
    text(c, LEFT+296, 349, number(charged['B'][FP])+' Fp products', 9.2, NAVY, True)
    paragraph(c, LEFT, 324, WIDTH,
              'These totals are assembled by Rust from the measured common phases and one membership pass. '
              'The two A/A controls and five ordered A/B pairs are repeatability controls; they are '
              'not independent instances. The all-positive root corpus is a separate diagnostic.', 8.5, 11.8)

    heading(c, 269, '04', 'Known-answer replay and evidence identities')
    paragraph(c, LEFT, 251, WIDTH,
              f'Archived row: alpha = <b>{relation["alpha"]}</b>, beta = <b>{relation["beta"]}</b>; '
              f'replayed scalar <b>{relation["replayed_scalar"]}</b>. Native output records '
              f'<b>{number(data["new_relation_search_calls"])}</b> new relation-search calls. '
              'The producer receipt records <b>two successful invocations with byte-identical JSON</b>; '
              f'native tests: <b>{number(recorded_tests["passed"])} passed, '
              f'{number(recorded_tests["failed"])} failed</b>. Standing remains '
              '<b>[pcc:unverified]</b>. No wall-time, rho comparison or '
              'new-relation discovery claim is made.', 8.3, 11.6)
    identity_rows = [
        ('Measured source commit', source_commit),
        ('Native arithmetic commit', provenance['native_commit']),
        ('Archived source commit', provenance['archive_commit']),
        ('Executable SHA-256', binary_sha),
        ('Raw JSON SHA-256', raw_sha),
        ('Frozen source seed', provenance['archive_seed_hex']),
    ]
    y = 178
    for label, value in identity_rows:
        text(c, LEFT, y, label, 6.8, MUTED, True)
        text(c, LEFT+126, y, value, 6.6, INK, mono=True)
        y -= 10.5
    evidence_paths = [path_label(args.input), path_label(args.pre_call_manifest)]
    if args.post_call_manifest is not None:
        evidence_paths.append(path_label(args.post_call_manifest))
    evidence_paths.extend([path_label(replay_receipt_path), path_label(publication_receipt_path)])
    paragraph(c, LEFT, 103, WIDTH,
              '<b>Repository-relative evidence paths:</b><br/>'
              + '<br/>'.join(escape(p) for p in evidence_paths), 6.6, 8.2, MUTED)
    c.save()
    print(HERE/'REPORT.pdf')
    print(phase_path)
    print(member_path)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--input', type=Path, default=MEASUREMENT/'results/attempt-001/initial.json')
    parser.add_argument('--measurement-commit', required=True)
    parser.add_argument('--executable-sha256', required=True)
    parser.add_argument('--pre-call-manifest', type=Path, required=True)
    parser.add_argument('--post-call-manifest', type=Path)
    parser.add_argument('--replay-receipt', type=Path,
                        help='Defaults to replay-comparison.json beside the native input')
    parser.add_argument('--source-publication-receipt', type=Path,
                        help='Defaults to source-publication-receipt.json beside the native input')
    args = parser.parse_args()
    build(args)


if __name__ == '__main__':
    main()
