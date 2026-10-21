"""Render the committed N83 K128 index-stage receipts as visual reports."""

from pathlib import Path
import json
import math

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from reportlab.lib import colors
from reportlab.lib.pagesizes import letter
from reportlab.lib.styles import getSampleStyleSheet
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, Image, PageBreak


ROOT = Path(__file__).resolve().parent
OUT = ROOT
OUT.mkdir(exist_ok=True)
NAMES = ('k128_a', 'k128_b', 'k128_shared_a', 'k128_shared_b')
RECEIPTS = [json.loads((ROOT / 'runs' / name / 'result.json').read_text()) for name in NAMES]

for receipt in RECEIPTS:
    assert receipt['status'] == 'PASS_INDEX_STAGE_CONTROL'
    arms = receipt['arms']
    assert len(arms) == 2 and all(arm['exit_code'] == 0 for arm in arms)
    assert len({arm['stage']['base_hash'] for arm in arms}) == 1
    assert len({arm['stage']['state_digest_blake3'] for arm in arms}) == 1
    assert all(arm['stage']['regular_states'] == 680000 and
               arm['stage']['root_table_entries'] == 1359488 for arm in arms)

fig, ax = plt.subplots(figsize=(9.5, 4.5))
groups = range(4)
mono = []
split = []
for receipt in RECEIPTS:
    by_name = {arm['arm']: arm for arm in receipt['arms']}
    mono.append(by_name['monolithic']['stage']['index_build_ms'] / 1000)
    split.append(by_name['split']['stage']['index_build_ms'] / 1000)
ax.bar([x-0.19 for x in groups], mono, width=0.37, label='Monolithic', color='#425e86')
ax.bar([x+0.19 for x in groups], split, width=0.37, label='Split', color='#d68b3c')
ax.set_xticks(list(groups), ['dual A\nmono first', 'dual B\nsplit first',
                              'shared A\nmono first', 'shared B\nsplit first'])
ax.set_ylabel('Index construction (seconds)')
ax.set_ylim(0, max(mono + split) * 1.2)
ax.legend(frameon=False, ncol=2)
ax.set_title('N83 K128 root-index stage control: order reverses the timing direction')
ax.grid(axis='y', alpha=.25)
ax.set_axisbelow(True)
fig.tight_layout()
fig.savefig(OUT / 'index_build.svg')
fig.savefig(OUT / 'index_build.png', dpi=150)
plt.close(fig)

def states(k):
    return 83 * k * (k - 1) // 2 + 42 * k

def request(n):
    return (5 * n + 3) // 4

def next_power_of_two(n):
    return 1 << (n - 1).bit_length()

def table_bytes(k, split=None):
    parts = (states(k),) if split is None else (states(split), states(k) - states(split))
    return 5 * sum(next_power_of_two(2 * request(part)) for part in parts)

def bloom_bytes(k):
    return next_power_of_two(16 * request(states(k))) // 8

configs = [('K4405 mono', 4405, None), ('K5086 mono', 5086, None),
           ('K5086 split4300', 5086, 4300), ('K5086 split4549', 5086, 4549)]
assert table_bytes(5086, 4549) == 13421772800
assert bloom_bytes(5086) == 4294967296
assert 11 * states(5086) == 11808494247
fig, ax = plt.subplots(figsize=(9.5, 4.5))
xpos = list(range(len(configs)))
state_gib = [11 * states(k) / 2**30 for _, k, _ in configs]
table_gib = [table_bytes(k, split) / 2**30 for _, k, split in configs]
bloom_gib = [bloom_bytes(k) / 2**30 for _, k, _ in configs]
ax.bar(xpos, state_gib, color='#647d9c', label='11-byte states')
ax.bar(xpos, table_gib, bottom=state_gib, color='#dc9750', label='Exact table')
ax.bar(xpos, bloom_gib, bottom=[a+b for a,b in zip(state_gib,table_gib)],
       color='#77a888', label='wide3 Bloom')
ax.set_xticks(xpos, [name.replace(' ', '\n') for name, _, _ in configs])
ax.set_ylabel('Calculated allocation (GiB)')
ax.set_ylim(0, 42)
ax.legend(frameon=False, ncol=3, loc='upper left')
ax.set_title('N83 root-index capacity calculation; excludes other memory')
ax.grid(axis='y', alpha=.25)
ax.set_axisbelow(True)
fig.tight_layout()
fig.savefig(OUT / 'capacity.svg')
fig.savefig(OUT / 'capacity.png', dpi=150)
plt.close(fig)

styles = getSampleStyleSheet()
small = styles['BodyText']
small.fontSize = 9
small.leading = 13

def make_pdf(path, title, paragraphs, table_rows, capacity=False):
    doc = SimpleDocTemplate(str(path), pagesize=letter, leftMargin=47,
                            rightMargin=47, topMargin=45, bottomMargin=45)
    story = [Paragraph(title, styles['Title']), Spacer(1, 12)]
    for text in paragraphs:
        story.extend([Paragraph(text, small), Spacer(1, 8)])
    table = Table(table_rows, hAlign='LEFT', repeatRows=1)
    table.setStyle(TableStyle([
        ('BACKGROUND', (0, 0), (-1, 0), colors.HexColor('#e8edf5')),
        ('GRID', (0, 0), (-1, -1), 0.3, colors.HexColor('#b8c1cc')),
        ('FONT', (0, 0), (-1, 0), 'Helvetica-Bold'),
        ('FONTSIZE', (0, 0), (-1, -1), 8),
        ('TOPPADDING', (0, 0), (-1, -1), 5),
        ('BOTTOMPADDING', (0, 0), (-1, -1), 5),
    ]))
    story.extend([table, Spacer(1, 14)])
    image = Image(str(OUT / 'index_build.png'), width=500, height=237)
    story.extend([image, Spacer(1, 8)])
    story.append(Paragraph('Source: research/n83_two_tier_root_20261021/runs/'
                           '{k128_a,k128_b,k128_shared_a,k128_shared_b}/result.json; '
                           'same study directory contains inputs, stderr, stdout, and runner copies.', small))
    if capacity:
        story.extend([
            PageBreak(), Paragraph('Calculated K5086 split capacity', styles['Title']),
            Spacer(1, 12),
            Paragraph('The 30-bit state-ID arithmetic admits K5086 and excludes '
                      'K5087 and above. With primary-first lookup, K4549 is the '
                      'unique split point meeting the 2^31 primary and 2^29 '
                      'extension slot capacities. Five-byte table storage is '
                      '12.5 GiB, versus 15 GiB at split K4300 and 20 GiB '
                      'monolithic. The K5086 point-count formula projects '
                      '844,276 points; enumeration and subgroup checks remain '
                      'to be run.', small), Spacer(1, 12),
            Image(str(OUT / 'capacity.png'), width=500, height=237),
            Spacer(1, 10),
            Paragraph('Calculated arrays only. The 11-byte states, five-byte '
                      'table, and wide3 union Bloom total 27.50 GiB at '
                      'K5086/K4549. Other memory and peak RSS are excluded. '
                      'No K5086 timing or target result is inferred.', small),
            Spacer(1, 8),
            Paragraph('Source and proof map: research/n83_two_tier_root_20261021/'
                      'CAPACITY.md and Capacity.lean; Rust source: examples/'
                      'koblitz_orbit_dlp_fast.rs.', small),
        ])
    doc.build(story)

rows = [['Order', 'Mode', 'Mono (s)', 'Split (s)', 'Faster stage']]
for name, rec, m, s in zip(NAMES, RECEIPTS, mono, split):
    rows.append([name, 'shared' if 'shared' in name else 'dual',
                 f'{m:.3f}', f'{s:.3f}', 'mono' if m < s else 'split'])

make_pdf(OUT / 'RUN.pdf', 'N83 K128 split root index: run record', [
    'Four paired index-stage controls built the same 21,248-point N83 factor base '
    'and 680,000 symmetric states. Each pair used one executable in alternate '
    'monolithic/split order. The public target file was fixed but target work '
    'was never started in these stage controls.',
    'All four process receipts passed. Each arm inserted 1,359,488 exact root '
    'entries and retained the same base hash and state digest. The split point '
    'was K64. The dual variant used per-table Bloom filters; the shared variant '
    'used one union filter. Local shared macOS timing is a stage diagnostic.',
    'Source revision and executable hashes are frozen in each input.json. '
    'Raw results are in the study directory, and independent exact-root unit '
    'controls passed in the example test suite (22 tests).'
], rows)

make_pdf(OUT / 'RESULT.pdf', 'N83 K128 split root index: result', [
    'The split exact-root construction preserved the K128 base, state set, '
    'and root-entry count. Index-build timing changed direction with run order: '
    'the paired local timings do not establish a stage speedup.',
    'At K128 the monolithic five-byte table allocated 10,485,760 bytes and '
    'the split tables allocated 13,107,200 bytes. Shared Bloom allocated '
    '2,097,152 bytes, matching monolithic and saving 524,288 bytes compared '
    'with separate split-table filters. The frozen K4405 layout formula saves '
    '128 MiB of Bloom space against separate filters and adds 671,088,640 '
    'table bytes against monolithic.',
    'Status: STAGE_DIAGNOSTIC_ONLY. Candidate ID and single-target online '
    'speedup remain unset. A full-size K4405 same-target scalar replay and '
    'Tier 1 paired measurement are the next qualification steps.'
], rows, capacity=True)

print(json.dumps({'pdfs': ['RUN.pdf','RESULT.pdf'], 'svg': 'index_build.svg',
                  'png': 'index_build.png', 'capacity_svg': 'capacity.svg',
                  'capacity_png': 'capacity.png', 'mono_s': mono, 'split_s': split}))
