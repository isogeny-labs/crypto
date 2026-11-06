#!/usr/bin/env python3
"""Layout of the four recorded online medians; no benchmark computation."""
from math import log10
from pathlib import Path

# Exact medians transcribed from each n*/control-comparison.json/session record.
rows = [(20, 0.849690, 1.183083), (24, 23.555580, 1.646459),
        (28, 435.896655, 3.453848), (32, 9520.577446, 7.591005)]
left, right, top, bottom = 100, 790, 75, 420
xs = {bits: left + i * (right-left) / 3 for i, (bits, _, _) in enumerate(rows)}
def y(ms):
    return bottom - (log10(ms) + 1) / 5 * (bottom-top)
def line(column, color):
    pts = ' '.join(f'{xs[b]:.1f},{y(r[column]):.1f}' for r in rows for b in [r[0]])
    return f'<polyline points="{pts}" fill="none" stroke="{color}" stroke-width="3"/>'

parts = ['<svg xmlns="http://www.w3.org/2000/svg" width="900" height="530" viewBox="0 0 900 530">',
         '<rect width="900" height="530" fill="white"/>',
         '<style>text{font-family:DejaVu Sans,sans-serif;fill:#17202a}.title{font-size:20px;font-weight:bold}.lab{font-size:13px}.small{font-size:11px}</style>',
         '<text class="title" x="100" y="31">IC curve-size panel on EKS CPU Spot</text>',
         '<text class="lab" x="100" y="53">Recorded online median, 5 measured rounds per size; log scale</text>']
for exp in range(-1,5):
    yy = y(10**exp)
    parts += [f'<line x1="{left}" y1="{yy:.1f}" x2="{right}" y2="{yy:.1f}" stroke="#dce3e8"/>',
              f'<text class="lab" x="{left-12}" y="{yy+5:.1f}" text-anchor="end">10^{exp}</text>']
parts += [f'<line x1="{left}" y1="{top}" x2="{left}" y2="{bottom}" stroke="#263238"/>',
          f'<line x1="{left}" y1="{bottom}" x2="{right}" y2="{bottom}" stroke="#263238"/>',
          f'<text class="lab" x="{(left+right)/2}" y="465" text-anchor="middle">curve size (bits)</text>',
          '<text class="lab" x="18" y="244" transform="rotate(-90 18 244)" text-anchor="middle">online wall time (ms)</text>']
for bits, ic, rho in rows:
    x = xs[bits]
    parts += [f'<text class="lab" x="{x:.1f}" y="441" text-anchor="middle">{bits}</text>',
              f'<circle cx="{x:.1f}" cy="{y(ic):.1f}" r="5" fill="#bd3f2e"/>',
              f'<circle cx="{x:.1f}" cy="{y(rho):.1f}" r="5" fill="#1e68a3"/>']
parts += [line(1, '#bd3f2e'), line(2, '#1e68a3'),
          '<rect x="105" y="480" width="12" height="12" fill="#bd3f2e"/><text class="lab" x="123" y="491">IC pipeline</text>',
          '<rect x="255" y="480" width="12" height="12" fill="#1e68a3"/><text class="lab" x="273" y="491">rho.negation</text>',
          '<text class="small" x="105" y="513">Online interval excludes setup. Source: n20/n24/n28/n32 session/records.jsonl and control-comparison.json.</text>',
          '</svg>']
Path(__file__).with_name('online-median.svg').write_text('\n'.join(parts) + '\n')
