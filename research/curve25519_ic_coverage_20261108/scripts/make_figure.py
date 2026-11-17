#!/usr/bin/env python3
"""figures/ladder.{svg,png}: every measured arm's distance to matched rho-neg at k = 20 (w = 0 and
w = 0.1; F = 1 rows also with table inserts priced), from data/ladder.json (values copied from the named source files)."""
import json, os
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
HERE = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')
d = json.load(open(os.path.join(HERE, 'data/ladder.json')))
rows = d['rows']
col = {'algebraic': '#d62728', 'generic': '#9e9e9e', 'generic (BSGS boundary)': '#6baed6', 'generic (F = 1)': '#08519c'}
fig, ax = plt.subplots(figsize=(11, 5.6))
y = list(range(len(rows)))[::-1]
for yi, r in zip(y, rows):
    xs = [r['w0']] + [v for v in (r.get('w01'), r.get('w01_priced')) if v is not None]
    ax.plot([min(xs), max(xs)], [yi, yi], color=col[r['track']], lw=1, alpha=0.6)
    ax.scatter([r['w0']], [yi], color=col[r['track']], s=46, zorder=3)
    if r.get('w01') is not None:
        ax.scatter([r['w01']], [yi], color=col[r['track']], s=46, zorder=3, marker='s', facecolors='white')
    if r.get('w01_priced') is not None:
        ax.scatter([r['w01_priced']], [yi], color=col[r['track']], s=60, zorder=4, marker='D', facecolors='none', linewidths=1.5)
for f, v in d['floors'].items():
    ax.axvline(v, color='#ff7f0e', ls='--', lw=0.8)
    ax.text(v, len(rows) - 0.4, f'{f} floor {v}', rotation=90, va='top', ha='right', fontsize=7, color='#ff7f0e')
ax.axvline(1, color='k', lw=1); ax.text(1, -0.9, 'rho-neg parity', ha='center', fontsize=7.5)
ax.set_xscale('log'); ax.set_xlim(0.8, 3e6)
ax.set_yticks(y); ax.set_yticklabels([r['arm'] for r in rows], fontsize=8)
ax.set_xlabel('S(arm) / S(matched rho-neg) at k = 20, log scale (markers keyed in the legend)')
from matplotlib.lines import Line2D
h = [Line2D([], [], color=c, marker='o', ls='', label=t) for t, c in col.items()]
h += [Line2D([], [], color='k', marker='s', mfc='white', ls='', label='w = 0.1, pre-2026-11-08 reading (inserts unpriced)'),
      Line2D([], [], color='k', marker='D', mfc='none', ls='', label='w = 0.1, inserts priced (F = 1 rows; ruling 2026-11-08)')]
ax.legend(handles=h, fontsize=8, loc='lower right', title='track', title_fontsize=8)
ax.set_title('Curve25519-family index calculus, k = 20: distance remaining to matched rho per arm (sources in data/ladder.json)', fontsize=9)
ax.grid(axis='x', ls=':', lw=0.5)
fig.tight_layout()
for ext in ('svg', 'png'):
    fig.savefig(os.path.join(HERE, f'figures/ladder.{ext}'), dpi=150)
print('wrote figures/ladder.svg,png')
