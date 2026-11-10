#!/usr/bin/env python3
"""Summarize an M4GB A/B results.tsv (base vs new, interleaved repeats) into a
Markdown table and a grouped-bar SVG of median wall seconds. Standard library only."""
import csv, statistics, sys, json
from collections import defaultdict
tsv, md_out, svg_out, json_out = sys.argv[1:5]
logdir = sys.argv[5] if len(sys.argv) > 5 else None
import os
rows = list(csv.DictReader(open(tsv), delimiter='\t'))
cells = defaultdict(lambda: defaultdict(list))   # (input,config) -> arm -> [(wall, hash, deg, mem)]
for r in rows:
    try: w = float(r['wall_s'])
    except ValueError: continue
    h = r['output_hash']
    if h.startswith('unit=') and logdir:
        lg = os.path.join(logdir, f"{r['input']}.{r['config']}.{r['arm']}.r{r['rep']}.log")
        try: txt = open(lg).read()
        except OSError: txt = ''
        h = 'unit-ideal' if 'contradiction:' in txt else 'NO-OUTPUT'
    cells[(r['input'], r['config'])][r['arm']].append((w, h, r['maxdeg'], r['peak_mib']))
order = [k for k in dict.fromkeys((r['input'], r['config']) for r in rows) if k in cells]
summary = []
for key in order:
    arms = cells[key]
    if 'base' not in arms or 'new' not in arms: continue
    b = [x[0] for x in arms['base']]; n = [x[0] for x in arms['new']]
    hb = {x[1] for x in arms['base']}; hn = {x[1] for x in arms['new']}
    summary.append({
        'input': key[0], 'config': key[1], 'n': min(len(b), len(n)),
        'base_median': statistics.median(b), 'base_min': min(b), 'base_max': max(b),
        'new_median': statistics.median(n), 'new_min': min(n), 'new_max': max(n),
        'speedup_median': statistics.median(b) / statistics.median(n) if statistics.median(n) > 0 else float('nan'),
        'speedup_minmin': min(b) / min(n) if min(n) > 0 else float('nan'),
        'same_output': hb == hn and len(hb) == 1,
        'base_hash': sorted(hb), 'new_hash': sorted(hn),
        'maxdeg_base': sorted({x[2] for x in arms['base']}), 'maxdeg_new': sorted({x[2] for x in arms['new']}),
        'peak_mib_base': sorted({x[3] for x in arms['base']}), 'peak_mib_new': sorted({x[3] for x in arms['new']}),
    })
json.dump(summary, open(json_out, 'w'), indent=1)
with open(md_out, 'w') as f:
    f.write('| input | binary | n | base median s (min–max) | patched median s (min–max) | speedup (median) | speedup (min/min) | same output |\n|:--|:--|--:|--:|--:|--:|--:|:--|\n')
    for s in summary:
        f.write(f"| `{s['input']}` | `{s['config']}` | {s['n']} | {s['base_median']:.3f} ({s['base_min']:.3f}–{s['base_max']:.3f}) | {s['new_median']:.3f} ({s['new_min']:.3f}–{s['new_max']:.3f}) | {s['speedup_median']:.2f}× | {s['speedup_minmin']:.2f}× | {'yes' if s['same_output'] else 'NO: ' + str(s['base_hash']) + ' vs ' + str(s['new_hash'])} |\n")
# SVG: grouped bars of median wall, log scale
import math
W, H, L, Bm, T = 1100, 420, 70, 150, 40
items = summary; n = len(items); gw = (W - L - 20) / max(n, 1)
vals = [v for s in items for v in (s['base_median'], s['new_median']) if v > 0]
lo, hi = min(vals) / 1.5, max(vals) * 1.5
def y(v): return T + (H - T - Bm) * (1 - (math.log10(v) - math.log10(lo)) / (math.log10(hi) - math.log10(lo)))
out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="Helvetica, Arial, sans-serif" font-size="11">',
       f'<rect width="{W}" height="{H}" fill="white"/>',
       f'<text x="{L}" y="22" font-size="15" font-weight="bold">M4GB upstream (base) vs patched (arithmetic kernels + declared-variable field equations): median wall seconds, 1 thread, log scale</text>']
e = math.floor(math.log10(lo));
while 10**e <= hi:
    if 10**e >= lo:
        out.append(f'<line x1="{L}" x2="{W-20}" y1="{y(10**e):.1f}" y2="{y(10**e):.1f}" stroke="#ddd"/>')
        out.append(f'<text x="{L-6}" y="{y(10**e)+4:.1f}" text-anchor="end">{10**e:g} s</text>')
    e += 1
for i, s in enumerate(items):
    x0 = L + i * gw + gw * 0.12; bw = gw * 0.34
    for j, (arm, col) in enumerate((('base_median', '#8c8c8c'), ('new_median', '#1f77b4'))):
        v = s[arm]; x = x0 + j * (bw + gw * 0.08)
        out.append(f'<rect x="{x:.1f}" y="{y(v):.1f}" width="{bw:.1f}" height="{H-Bm-y(v):.1f}" fill="{col}"/>')
        out.append(f'<text x="{x+bw/2:.1f}" y="{y(v)-3:.1f}" text-anchor="middle" font-size="9">{v:.2f}</text>')
    out.append(f'<text x="{x0+bw+gw*0.04:.1f}" y="{H-Bm+12}" text-anchor="middle" font-size="10" font-weight="bold">{s["speedup_median"]:.2f}×</text>')
    out.append(f'<text transform="translate({x0+bw+gw*0.04:.1f},{H-Bm+22}) rotate(40)" font-size="9">{s["input"]} / {s["config"]}</text>')
out.append(f'<rect x="{W-260}" y="30" width="12" height="12" fill="#8c8c8c"/><text x="{W-244}" y="40">upstream 01860a52 (patches off)</text>')
out.append(f'<rect x="{W-260}" y="48" width="12" height="12" fill="#1f77b4"/><text x="{W-244}" y="58">patched (0001 + 0002)</text>')
out.append('</svg>')
open(svg_out, 'w').write('\n'.join(out))
print(open(md_out).read())
