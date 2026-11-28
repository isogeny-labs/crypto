#!/usr/bin/env python3
"""P3 figure: best-arm cost bracket per model vs matched negation rho. [pcc:unverified]"""
import json, glob, math
from collections import defaultdict
import matplotlib; matplotlib.use('Agg')
import matplotlib.pyplot as plt
STEP = {3: 290.3, 4: 479.8, 6: 1164.9}
best = defaultdict(list); by = defaultdict(list)
for f in glob.glob('raw/*.jsonl'):
    for l in open(f):
        r = json.loads(l); by[(r['d'], r['m'], r['g'], r['p'], r['curve_index'])].append(r)
for k, rs in by.items():
    b = min((r for r in rs if not r['exhausted']), key=lambda r: r['fpm_total_low'])
    best[k[:4]].append((b['fpm_total_low'], b['fpm_total']))
gm = lambda xs: math.exp(sum(map(math.log, xs)) / len(xs))
fig, ax = plt.subplots(figsize=(7, 4.6))
cols = {(4, 0, 3): '#1f77b4', (5, 2, 4): '#d62728', (5, 0, 6): '#2ca02c'}
names = {(4, 0, 3): 'g=3 smooth quartic (control, e=2)', (5, 2, 4): 'g=4 two-node quintic (e=3)',
         (5, 0, 6): 'g=6 smooth quintic (e=3)'}
for m, c in cols.items():
    ps = sorted(p for (d, mm, g, p) in best if (d, mm, g) == m)
    lo = [gm([a for a, _ in best[m + (p,)]]) for p in ps]; hi = [gm([b for _, b in best[m + (p,)]]) for p in ps]
    ax.fill_between(ps, lo, hi, color=c, alpha=0.18)
    ax.plot(ps, lo, 'o-', color=c, label=names[m] + ': IC, LA bracket')
    g = m[2]
    ax.plot(ps, [math.sqrt(math.pi * p ** g / 4) * STEP[g] for p in ps], '--', color=c, lw=1,
            label=f'g={g}: negation rho, hyperelliptic step')
ax.set_xscale('log'); ax.set_yscale('log'); ax.set_xlabel('p'); ax.set_ylabel('counted cost (FPM)')
ax.set_title('P3: plane-model relation phase + LA bracket against matched rho [pcc:unverified]', fontsize=9)
ax.legend(fontsize=6.5, loc='upper left'); ax.grid(True, which='both', alpha=0.3)
fig.tight_layout(); fig.savefig('p3_cost_vs_rho.svg')
