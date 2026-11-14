#!/usr/bin/env python3
"""Aggregate the native closure results (runs/native/*.jsonl, computed by closure/pc_closure) into
results.csv, results_table.md and figures/d_vs_n.svg (no plotting library), and cross-check them against
the Python instrument's runs (runs/*.jsonl) cell by cell.  Usage: aggregate.py  (from the research dir)"""
import csv, glob, json, os, re
from collections import defaultdict

ARMS = {'K': 'K: Kummer y²', 'S': 'S: G_m Möbius σ', 'T': 'T: T₂ Möbius τ',
        'M': 'M: Kummer + Möbius link', 'C': 'C: T₂ circle + link'}
def load(f):
    return [json.loads(l) for l in open(f) if l.strip()]
# Python cross-check runs: tag -> (certified D or None, per-D ranks)
pyruns = {}
for f in glob.glob('runs/*.jsonl'):
    recs = load(f)
    if recs and 'header' in recs[0]:
        pyruns[os.path.basename(f)[:-6]] = (next((r['certified_D'] for r in recs if 'certified_D' in r), None),
                                            {r['D']: r['rank'] for r in recs if 'D' in r})
rows = []
for f in sorted(glob.glob('runs/native/*.jsonl')):
    tag = os.path.basename(f)[:-6]
    recs = load(f)
    sysf = f'runs/systems/{tag}.json'
    if not recs or not os.path.exists(sysf): continue
    h = json.load(open(sysf))['header']
    cert = next((r['certified_D'] for r in recs if 'certified_D' in r), None)
    cens = next((r['censored_D_ge'] for r in recs if 'censored_D_ge' in r), None)
    done = [r for r in recs if 'D' in r]
    lastD = max((r['D'] for r in done), default=0)
    # a run killed by the timeout: D >= lastD+1 (1 not in W_lastD), reported as censored
    if cert is None and cens is None:
        cens = lastD + 1 if done else None
        status = 'timeout'
    else:
        status = 'certified' if cert else 'censored'
    py = pyruns.get(tag)
    if py is None: xc = 'none'
    else:
        pyD, pyr = py
        ranks_eq = all(pyr.get(r['D']) == r['rank'] for r in done if r['D'] in pyr)
        shared = [r['D'] for r in done if r['D'] in pyr]
        if pyD is not None and cert is not None: xc = ('agree' if pyD == cert and ranks_eq else 'DISAGREE')
        else: xc = (f'partial D<={max(shared)}' if shared and ranks_eq else ('DISAGREE' if shared else 'none'))
    rows.append(dict(arm=h['arm'], p=h['p'], m=h['m'], t=h['t'], N=h['N'], n_vars=h['n_vars'], seed=h['seed'],
                     target=h['target'], consistent=h['consistent_by_enum'], D=cert, D_ge=cens, status=status,
                     cols_at_D=next((r['cols'] for r in done if r['D'] == (cert or lastD)), None),
                     seconds=round(sum(r['seconds'] for r in done), 2), python_crosscheck=xc, file=tag))
rows.sort(key=lambda r: (r['target'], r['m'], r['t'], r['arm'], r['p'], r['seed']))
with open('results.csv', 'w', newline='') as fh:
    w = csv.DictWriter(fh, fieldnames=list(rows[0].keys())); w.writeheader(); w.writerows(rows)

def cell(r):
    if r['status'] == 'certified': return str(r['D'])
    return f"≥{r['D_ge']}" if r['D_ge'] else '—'

# Markdown table: random targets, per (m,t): arm -> seeds
tab = defaultdict(dict)
for r in rows:
    if r['target'] != 'random': continue
    key = (r['m'], r['t'])
    tab[key].setdefault((r['arm'], r['p']), []).append((r['seed'], cell(r), r['seconds']))
arm_cols = sorted({k for v in tab.values() for k in v}, key=lambda k: ('KSTMC'.index(k[0]), k[1]))
lines = ['| m | t | N | ' + ' | '.join(f"{a} (p={p})" for a, p in arm_cols) + ' |',
         '|---|---|---|' + '---|' * len(arm_cols)]
for (m, t) in sorted(tab):
    vals = []
    for k in arm_cols:
        e = sorted(tab[(m, t)].get(k, []))
        vals.append(', '.join(f"{d}" for s, d, sec in e) if e else '')
    lines.append(f"| {m} | {t} | {m*t} | " + ' | '.join(vals) + ' |')
xs = [r['python_crosscheck'] for r in rows]
lines += ['', f"Python cross-check (same dumped system): {xs.count('agree')} cells agree on D and every per-degree rank, "
          f"{sum(x.startswith('partial') for x in xs)} agree on the degrees Python finished, {xs.count('DISAGREE')} disagree, "
          f"{xs.count('none')} not run in Python."]
ctrl = [r for r in rows if r['target'] == 'planted']
lines += ['', 'Planted (consistent) controls, `1 ∈ W_D` must stay false:', '']
lines += ['| arm | p | m | t | result |', '|---|---|---|---|---|']
for r in ctrl:
    lines.append(f"| {r['arm']} | {r['p']} | {r['m']} | {r['t']} | {'FALSE POSITIVE D=' + str(r['D']) if r['D'] else 'no 1 up to D=' + str((r['D_ge'] or 1) - 1)} |")
open('results_table.md', 'w').write('\n'.join(lines) + '\n')
print('\n'.join(lines))

# SVG: D vs N per arm for m=2 (left) and m=3 (right), seed-1 values, censored as open markers with arrow
os.makedirs('figures', exist_ok=True)
COL = {'K': '#1f3b73', 'S': '#c0392b', 'T': '#d35400', 'M': '#2e8b57', 'C': '#7d3c98'}
OFF = {'K': -0.18, 'S': -0.09, 'T': 0.0, 'M': 0.09, 'C': 0.18}
W, H, PAD = 900, 380, 50
def panel(m, x0, width):
    pts = defaultdict(list)
    for r in rows:
        if r['target'] != 'random' or r['m'] != m or r['seed'] != 1: continue
        if r['arm'] == 'T' and r['p'] != 8191: continue
        pts[r['arm']].append((r['N'], r['D'] if r['D'] else r['D_ge'], r['status'] == 'certified'))
    Ns = sorted({n for v in pts.values() for n, _, _ in v}) or [m]
    Ds = [d for v in pts.values() for _, d, _ in v if d] or [3]
    nmin, nmax = min(Ns) - 1, max(Ns) + 1
    dmin, dmax = min(Ds) - 1, max(Ds) + 1
    sx = lambda n: x0 + PAD + (n - nmin) / (nmax - nmin) * (width - 2 * PAD)
    sy = lambda d: H - PAD - (d - dmin) / (dmax - dmin) * (H - 2 * PAD)
    s = [f'<text x="{x0 + width/2:.0f}" y="22" text-anchor="middle" font-size="15" font-weight="bold">m = {m}</text>']
    s.append(f'<line x1="{sx(nmin)}" y1="{sy(dmin)}" x2="{sx(nmax)}" y2="{sy(dmin)}" stroke="#333"/>')
    s.append(f'<line x1="{sx(nmin)}" y1="{sy(dmin)}" x2="{sx(nmin)}" y2="{sy(dmax)}" stroke="#333"/>')
    for n in range(nmin + 1, nmax):
        s.append(f'<text x="{sx(n):.1f}" y="{H - PAD + 16}" text-anchor="middle" font-size="11">{n}</text>')
    for d in range(dmin, dmax + 1):
        s.append(f'<text x="{sx(nmin) - 8:.1f}" y="{sy(d) + 4:.1f}" text-anchor="end" font-size="11">{d}</text>')
        s.append(f'<line x1="{sx(nmin)}" y1="{sy(d):.1f}" x2="{sx(nmax)}" y2="{sy(d):.1f}" stroke="#eee"/>')
    s.append(f'<text x="{x0 + width/2:.0f}" y="{H - 10}" text-anchor="middle" font-size="12">N = m·t tower bits</text>')
    for arm in 'KSTMC':
        v = sorted(pts.get(arm, []))
        if not v: continue
        cert = [(sx(n) + OFF[arm] * 40, sy(d)) for n, d, c in v if c and d]
        if len(cert) > 1:
            s.append('<polyline fill="none" stroke="%s" stroke-width="1.5" points="%s"/>' % (COL[arm], ' '.join(f'{x:.1f},{y:.1f}' for x, y in cert)))
        for n, d, c in v:
            if not d: continue
            x, y = sx(n) + OFF[arm] * 40, sy(d)
            if c: s.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="4.5" fill="{COL[arm]}"/>')
            else: s.append(f'<path d="M{x-5:.1f},{y+4:.1f} L{x+5:.1f},{y+4:.1f} L{x:.1f},{y-6:.1f} Z" fill="none" stroke="{COL[arm]}" stroke-width="1.6"/>')
    return s
svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H + 40}" font-family="Helvetica, Arial, sans-serif" style="background:#fff">']
svg += panel(2, 0, W // 2); svg += panel(3, W // 2, W // 2)
y = H + 12; x = 20
svg.append(f'<text x="{x}" y="{y}" font-size="11" fill="#333">raw PC solving degree D (filled: certified; open triangle: censored lower bound D ≥). </text>')
y += 18
for arm in 'KSTMC':
    svg.append(f'<rect x="{x}" y="{y - 9}" width="10" height="10" fill="{COL[arm]}"/><text x="{x + 14}" y="{y}" font-size="11">{ARMS[arm]}</text>')
    x += 185
svg.append('</svg>')
open('figures/d_vs_n.svg', 'w').write('\n'.join(svg))
print('wrote results.csv, results_table.md, figures/d_vs_n.svg')
