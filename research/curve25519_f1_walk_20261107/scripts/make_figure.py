#!/usr/bin/env python3
"""Figure: pooled S ratios to rho-neg per arm, k = 20 and 24, w = 0, 0.1, 1 (data/pooled.json)."""
import json, os
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
HERE = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')
d = json.load(open(os.path.join(HERE, 'data/pooled.json')))
arms = [('f2-inc', 'F=2 incumbent x=0.9'), ('f1-x05', 'F=1 x=0.5'), ('f1-x07', 'F=1 x=0.7 (primary)'), ('f1-x09', 'F=1 x=0.9')]
cols = ['#8c8c8c', '#9ecae1', '#2171b5', '#6baed6']
fig, axes = plt.subplots(1, 2, figsize=(10, 4.2), sharey=True)
for ax, k in zip(axes, ('k20', 'k24')):
    ws = ['0', '0.1', '1']
    for i, ((a, lab), c) in enumerate(zip(arms, cols)):
        xs = [j + (i - 1.5) * 0.18 for j in range(3)]
        rr = [d[k]['ratios'][f'{a}/rho-neg'][w] for w in ws]
        y = [r['ratio'] for r in rr]
        lo = [r['ratio'] - r['ci95'][0] for r in rr]; hi = [r['ci95'][1] - r['ratio'] for r in rr]
        ax.bar(xs, y, 0.17, color=c, label=lab, yerr=[lo, hi], capsize=3)
    ax.axhline(1.0, color='k', lw=1); ax.text(2.45, 1.01, 'rho-neg parity', ha='right', va='bottom', fontsize=8)
    ax.axhline(1.5, color='#d62728', lw=0.8, ls='--'); ax.text(-0.45, 1.51, 'F=1 model floor 1.5x (derived)', fontsize=8, color='#d62728', bbox=dict(fc='white', ec='none', alpha=0.8))
    ax.axhline(1.875, color='#ff7f0e', lw=0.8, ls=':'); ax.text(-0.45, 1.885, 'F=2 model floor 1.875x (derived)', fontsize=8, color='#ff7f0e', bbox=dict(fc='white', ec='none', alpha=0.8))
    ax.set_xticks(range(3)); ax.set_xticklabels([f'w={w}' for w in ws])
    ax.set_title(f'{k[1:]}-bit p, 8 single-target sessions x 64 measured rounds', fontsize=9)
    ax.set_ylim(0, 2.1)
axes[0].set_ylabel('pooled S(arm) / S(rho-neg), 95% CI')
axes[0].legend(fontsize=8, loc='lower left')
fig.tight_layout()
for ext in ('svg', 'png'):
    fig.savefig(os.path.join(HERE, f'figures/ratio_to_rho.{ext}'), dpi=150)
