#!/usr/bin/env python3
"""Render ranks.svg from result.json (no plotting library needed)."""
import json
r = json.load(open('result.json'))
bars = [("full rank", r['CD_full_rank']), ("new-column rank", r['CD_new_rank']),
        ("closed rank", r['CD_closed_rank']), ("baseline rank", r['CD_baseline_rank']),
        ("gain (recorded run)", r['CD_gain']), ("gain (planted control)", r['F1_planted_gain'])]
W, H, left, top, bh, gap = 720, 330, 190, 50, 30, 12
scale = (W - left - 80) / max(v for _, v in bars)
out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif">',
       f'<rect width="{W}" height="{H}" fill="#ffffff"/>',
       f'<text x="20" y="28" font-size="15" font-weight="bold" fill="#1a1a1a">P-192 split-tangent local run: ranks mod n (independent check)</text>']
for i, (name, v) in enumerate(bars):
    y = top + i * (bh + gap)
    col = '#c2410c' if 'planted' in name else ('#2563eb' if 'gain' not in name else '#0f766e')
    out.append(f'<text x="{left-10}" y="{y+bh*0.68}" font-size="13" text-anchor="end" fill="#333">{name}</text>')
    out.append(f'<rect x="{left}" y="{y}" width="{max(v*scale,2)}" height="{bh}" fill="{col}"/>')
    out.append(f'<text x="{left+max(v*scale,2)+8}" y="{y+bh*0.68}" font-size="13" fill="#111">{v}</text>')
out.append(f'<text x="20" y="{H-14}" font-size="11" fill="#555">108 rows x 429 columns (225 old, 204 new); each new column is used by exactly one row. Source: result.json</text>')
out.append('</svg>')
open('ranks.svg', 'w').write('\n'.join(out))
