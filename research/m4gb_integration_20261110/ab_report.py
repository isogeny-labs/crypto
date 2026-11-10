#!/usr/bin/env python3
"""Summarize M4GB A/B results (base vs new, interleaved repeats) into a Markdown
table, a JSON summary and a grouped-bar SVG of median wall seconds.

    ab_report.py results.tsv RESULTS.md results.svg results.json logs/ [results_rerun.tsv]

`logs/` holds `<input>.<config>.<arm>.r<rep>.log` and, for runs that produced
a basis, `<input>.<config>.<arm>.r<rep>.out`.  A run's output class is
`unit-ideal` (log says `contradiction:`), `censored` (no output and no
contradiction: the process died, e.g. `std::bad_alloc` under the ulimit) or
the SHA-256 prefix of its basis with upstream's `^2 + ` lines for undeclared
variables removed and the lines sorted.  A cell with a censored run in either
arm is reported as censored, with no speedup.  If a re-run file is given, its
cells replace the censored cells of the first file.  Standard library only."""
import csv, hashlib, json, math, os, statistics, sys
from collections import defaultdict

tsv, md_out, svg_out, json_out, logdir = sys.argv[1:6]
rerun = sys.argv[6] if len(sys.argv) > 6 else None

def classify(r):
    stem = os.path.join(logdir, f"{r['input']}.{r['config']}.{r['arm']}.r{r['rep']}")
    try:
        txt = open(stem + '.log', errors='replace').read()
    except OSError:
        txt = ''
    if os.path.exists(stem + '.out'):
        lines = sorted(l.strip() for l in open(stem + '.out', errors='replace') if l.strip() and not l.startswith('^'))
        return hashlib.sha256('\n'.join(lines).encode()).hexdigest()[:12]
    if 'contradiction:' in txt:
        return 'unit-ideal'
    return 'censored'

def load(path):
    cells = defaultdict(lambda: defaultdict(list))
    order = []
    for r in csv.DictReader(open(path), delimiter='\t'):
        try:
            w = float(r['wall_s'])
        except ValueError:
            continue
        key = (r['input'], r['config'])
        if key not in order:
            order.append(key)
        cells[key][r['arm']].append((w, classify(r), r['maxdeg'], r['peak_mib']))
    return cells, order

cells, order = load(tsv)
replaced = []
if rerun and os.path.exists(rerun):
    rcells, rorder = load(rerun)
    for key in rorder:
        if key in cells and any(x[1] == 'censored' for arm in cells[key].values() for x in arm):
            cells[key] = rcells[key]
            replaced.append(key)
        elif key not in cells:
            cells[key] = rcells[key]; order.append(key)

summary = []
for key in order:
    arms = cells[key]
    if 'base' not in arms or 'new' not in arms:
        continue
    b = [x[0] for x in arms['base']]; n = [x[0] for x in arms['new']]
    cb = {x[1] for x in arms['base']}; cn = {x[1] for x in arms['new']}
    censored = 'censored' in cb or 'censored' in cn
    s = {
        'input': key[0], 'config': key[1], 'n': min(len(b), len(n)),
        'base_median': statistics.median(b), 'base_min': min(b), 'base_max': max(b),
        'new_median': statistics.median(n), 'new_min': min(n), 'new_max': max(n),
        'censored': censored,
        'speedup_median': None if censored else statistics.median(b) / statistics.median(n),
        'speedup_minmin': None if censored else min(b) / min(n),
        'same_output': (not censored) and cb == cn and len(cb) == 1,
        'base_output': sorted(cb), 'new_output': sorted(cn),
        'maxdeg_base': sorted({x[2] for x in arms['base']}), 'maxdeg_new': sorted({x[2] for x in arms['new']}),
        'peak_mib_base': sorted({x[3] for x in arms['base']}), 'peak_mib_new': sorted({x[3] for x in arms['new']}),
        'rerun': key in replaced,
    }
    summary.append(s)
json.dump(summary, open(json_out, 'w'), indent=1)

with open(md_out, 'w') as f:
    f.write('| input | binary | n | base median s (min–max) | patched median s (min–max) | speedup (median) | speedup (min/min) | output |\n|:--|:--|--:|--:|--:|--:|--:|:--|\n')
    for s in summary:
        if s['censored']:
            out = 'censored: a run died (`std::bad_alloc` under the virtual-memory cap); times are time-to-failure'
            sp = sm = 'n/a'
        else:
            out = ('same basis' if s['same_output'] else 'DIFFERENT: ' + str(s['base_output']) + ' vs ' + str(s['new_output']))
            if s['base_output'] == ['unit-ideal']:
                out = 'same verdict: unit ideal (no solution)'
            sp = f"{s['speedup_median']:.2f}×"; sm = f"{s['speedup_minmin']:.2f}×"
        tag = ' (re-run, 14 GB cap)' if s['rerun'] else ''
        f.write(f"| `{s['input']}`{tag} | `{s['config']}` | {s['n']} | {s['base_median']:.3f} ({s['base_min']:.3f}–{s['base_max']:.3f}) | {s['new_median']:.3f} ({s['new_min']:.3f}–{s['new_max']:.3f}) | {sp} | {sm} | {out} |\n")

items = [s for s in summary if not s['censored']]
W, H, L, Bm, T = 1100, 430, 70, 160, 40
n = len(items); gw = (W - L - 20) / max(n, 1)
vals = [v for s in items for v in (s['base_median'], s['new_median']) if v > 0]
lo, hi = min(vals) / 1.5, max(vals) * 1.6
def y(v): return T + (H - T - Bm) * (1 - (math.log10(v) - math.log10(lo)) / (math.log10(hi) - math.log10(lo)))
out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="Helvetica, Arial, sans-serif" font-size="11">',
       f'<rect width="{W}" height="{H}" fill="white"/>',
       f'<text x="{L}" y="22" font-size="14" font-weight="bold">M4GB upstream 01860a52 (grey) vs patched 0001+0002 (blue): median wall seconds, one thread, log scale; label = speedup</text>']
e = math.floor(math.log10(lo))
while 10 ** e <= hi:
    if 10 ** e >= lo:
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
out.append('</svg>')
open(svg_out, 'w').write('\n'.join(out))
print(open(md_out).read())
