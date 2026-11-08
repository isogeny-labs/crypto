#!/usr/bin/env python3
"""figures/ladder.{svg,png}: every measured arm's distance to matched rho-neg at k = 20 (w = 0 and
w = 0.1, committed band), from data/ladder.json (values copied from the named source files)."""
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
    ax.plot([r['w0'], r['w01']], [yi, yi], color=col[r['track']], lw=1, alpha=0.6)
    ax.scatter([r['w0']], [yi], color=col[r['track']], s=46, zorder=3)
    ax.scatter([r['w01']], [yi], color=col[r['track']], s=46, zorder=3, marker='s', facecolors='white')
for f, v in d['floors'].items():
    ax.axvline(v, color='#ff7f0e', ls='--', lw=0.8)
    ax.text(v, len(rows) - 0.4, f'{f} floor {v}', rotation=90, va='top', ha='right', fontsize=7, color='#ff7f0e')
ax.axvline(1, color='k', lw=1); ax.text(1, -0.9, 'rho-neg parity', ha='center', fontsize=7.5)
ax.set_xscale('log'); ax.set_xlim(0.8, 3e6)
ax.set_yticks(y); ax.set_yticklabels([r['arm'] for r in rows], fontsize=8)
ax.set_xlabel('S(arm) / S(matched rho-neg) at k = 20   (filled: w = 0; open square: w = 0.1, committed band)')
from matplotlib.lines import Line2D
h = [Line2D([], [], color=c, marker='o', ls='', label=t) for t, c in col.items()]
ax.legend(handles=h, fontsize=8, loc='lower right', title='track', title_fontsize=8)
ax.set_title('Curve25519-family index calculus, k = 20: distance remaining to matched rho per arm (sources in data/ladder.json)', fontsize=9)
ax.grid(axis='x', ls=':', lw=0.5)
fig.tight_layout()
for ext in ('svg', 'png'):
    fig.savefig(os.path.join(HERE, f'figures/ladder.{ext}'), dpi=150)
print('wrote figures/ladder.svg,png')
