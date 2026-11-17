#!/usr/bin/env python3
"""Summarize ab_corpus.py output into RESULTS.md, results.json and results.svg.

    summarize.py corpus_ab.json RESULTS.md results.json results.svg [BASE NEW AA]

Per system: the median wall over repeats per arm.  Per cell: the median over
systems of those medians, the median over systems of the paired ratio NEW/BASE
(with its min and max), the same for the A/A control AA/BASE, and the number of
systems on which NEW is below BASE.  Systems where the arms disagree are
counted and listed.  Standard library only."""
import json, math, statistics, sys
src, md_out, json_out, svg_out = sys.argv[1:5]
base, new, aa = (sys.argv[5:8] + ['old', 'new', 'oldAA'][len(sys.argv[5:8]):])[:3]
d = json.load(open(src))
cells = {}
disagree = []
for s in d['systems']:
    med = {a: statistics.median(r['wall_ns'] for r in runs) / 1e6 for a, runs in s['runs'].items()}
    c = cells.setdefault(s['cell'], {'n_vars': s['n_vars'], 'sys': []})
    c['sys'].append(med)
    if not s['agree']:
        disagree.append((s['cell'], s['system'], s['outcome']))
def order(cell):
    fam, n, np_, m = cell.split('-')
    return (int(m), int(n), fam)
rows = []
for cell in sorted(cells, key=order):
    c = cells[cell]
    r_new = [x[new] / x[base] for x in c['sys']]
    r_aa = [x[aa] / x[base] for x in c['sys']] if aa in c['sys'][0] else []
    rows.append({'cell': cell, 'n_vars': c['n_vars'], 'systems': len(c['sys']),
                 'base_ms': statistics.median(x[base] for x in c['sys']),
                 'new_ms': statistics.median(x[new] for x in c['sys']),
                 'ratio_median': statistics.median(r_new), 'ratio_min': min(r_new), 'ratio_max': max(r_new),
                 'new_below_base': sum(1 for x in r_new if x < 1),
                 'aa_ratio_median': statistics.median(r_aa) if r_aa else None,
                 'aa_ratio_min': min(r_aa) if r_aa else None, 'aa_ratio_max': max(r_aa) if r_aa else None})
json.dump({'arms': d['arms'], 'repeats': d['repeats'], 'cells': rows,
           'disagreements': disagree}, open(json_out, 'w'), indent=1)
with open(md_out, 'w') as f:
    f.write(f"Paired, per captured system: median wall of {d['repeats']} requests per arm through `--serve`, one thread. "
            f"`{base}` = patches 0001–0003, `{new}` = 0001–0004, `{aa}` = a second server of the `{base}` build (A/A noise control). "
            "Cell columns: median over the cell's systems; ratios are the median (min–max) over systems of the per-system ratio.\n\n")
    f.write(f'| cell | vars | systems | `{base}` ms | `{new}` ms | `{new}`/`{base}` | systems `{new}` below | A/A `{aa}`/`{base}` |\n')
    f.write('|:--|--:|--:|--:|--:|--:|--:|--:|\n')
    for r in rows:
        aa_s = f"{r['aa_ratio_median']:.2f} ({r['aa_ratio_min']:.2f}–{r['aa_ratio_max']:.2f})" if r['aa_ratio_median'] else 'n/a'
        f.write(f"| {r['cell']} | {r['n_vars']} | {r['systems']} | {r['base_ms']:.2f} | {r['new_ms']:.2f} | "
                f"{r['ratio_median']:.2f} ({r['ratio_min']:.2f}–{r['ratio_max']:.2f}) | {r['new_below_base']}/{r['systems']} | {aa_s} |\n")
    f.write(f"\nDisagreements between arms (reply word or output bytes): {len(disagree)}.\n")
    for x in disagree:
        f.write(f'- {x}\n')
# SVG: per cell, NEW/BASE ratio (median with min-max bar) and the A/A band
W, H, L, B, T = 980, 420, 60, 120, 44
n = len(rows); gw = (W - L - 20) / max(n, 1)
lo, hi = 0.3, 1.4
def y(v): return T + (H - T - B) * (hi - min(max(v, lo), hi)) / (hi - lo)
o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="Helvetica, Arial, sans-serif" font-size="11">',
     f'<rect width="{W}" height="{H}" fill="white"/>',
     f'<text x="{L}" y="20" font-size="14" font-weight="bold">M4GB patch 0004 (divisibility signatures): per-system wall ratio to patches 0001–0003, through --serve</text>',
     f'<text x="{L}" y="36" fill="#555">median over the cell\'s systems with min–max bar; grey: A/A control (second server of the old build); one thread</text>']
for v in (0.4, 0.6, 0.8, 1.0, 1.2):
    o.append(f'<line x1="{L}" x2="{W-20}" y1="{y(v):.1f}" y2="{y(v):.1f}" stroke="{"#999" if v == 1.0 else "#e5e5e5"}"/>')
    o.append(f'<text x="{L-6}" y="{y(v)+4:.1f}" text-anchor="end">{v:.1f}×</text>')
for i, r in enumerate(rows):
    x = L + i * gw + gw / 2
    if r['aa_ratio_median']:
        o.append(f'<line x1="{x-6:.1f}" x2="{x-6:.1f}" y1="{y(r["aa_ratio_min"]):.1f}" y2="{y(r["aa_ratio_max"]):.1f}" stroke="#bbb" stroke-width="3"/>')
        o.append(f'<circle cx="{x-6:.1f}" cy="{y(r["aa_ratio_median"]):.1f}" r="3" fill="#999"/>')
    o.append(f'<line x1="{x+6:.1f}" x2="{x+6:.1f}" y1="{y(r["ratio_min"]):.1f}" y2="{y(r["ratio_max"]):.1f}" stroke="#d62728" stroke-width="3" opacity="0.5"/>')
    o.append(f'<circle cx="{x+6:.1f}" cy="{y(r["ratio_median"]):.1f}" r="4" fill="#d62728"/>')
    o.append(f'<text transform="translate({x+3:.1f},{H-B+14}) rotate(50)" font-size="10">{r["cell"]} ({r["n_vars"]} vars)</text>')
o.append('</svg>')
open(svg_out, 'w').write('\n'.join(o))
print(open(md_out).read())
