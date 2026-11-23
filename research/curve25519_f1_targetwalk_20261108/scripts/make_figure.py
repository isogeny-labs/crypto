#!/usr/bin/env python3
"""Figures from data/pooled_measured.json (frozen band) and data/sensitivity_inserts.json (post hoc):
figures/ratio_to_rho.{svg,png} and figures/phases.{svg,png}."""
import json, os
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
HERE = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')
d = json.load(open(os.path.join(HERE, 'data/pooled_measured.json')))
sens = json.load(open(os.path.join(HERE, 'data/sensitivity_inserts.json')))
arms = [('f1-x07', 'F=1 table-then-probe (x=0.7)', '#9e9e9e'),
        ('tw-x15', 'F=1 target-in-walk (tw=1)', '#6baed6'),
        ('twcj-x15', 'F=1 target-in-walk + chained jumps (primary)', '#08519c')]
ws = ['0', '0.1', '1']
fig, axes = plt.subplots(1, 2, figsize=(10.5, 4.4), sharey=True)
for ax, k in zip(axes, ('k20', 'k24')):
    for i, (a, lab, c) in enumerate(arms):
        xs = [j + (i - 1) * 0.26 for j in range(3)]
        rr = [d[k]['ratios'][f'{a}/rho-neg'][w] for w in ws]
        y = [r['ratio'] for r in rr]
        lo = [r['ratio'] - r['ci95'][0] for r in rr]; hi = [r['ci95'][1] - r['ratio'] for r in rr]
        ax.bar(xs, y, 0.24, color=c, label=lab, yerr=[lo, hi], capsize=3)
        sy = [sens[k][f'{a}/rho-neg'][w]['ratio'] for w in ws]
        ax.scatter(xs, sy, marker='_', s=260, color='#d62728', zorder=5,
                   label='post hoc: walk inserts also priced at w' if (i == 0 and k == 'k20') else None)
    ax.axhline(1.0, color='k', lw=1); ax.text(2.5, 1.01, 'rho-neg parity', ha='right', va='bottom', fontsize=8)
    ax.axhline(1.5, color='#ff7f0e', lw=0.8, ls='--',
               label='F=1 two-collision floor / rho model = 1.5x (derived)' if k == 'k20' else None)
    ax.set_xticks(range(3)); ax.set_xticklabels([f'w={w}' for w in ws])
    ax.set_title(f'k = {k[1:]}: 8 single-target sessions x 64 measured rounds', fontsize=9)
    ax.set_ylim(0, 2.25)
axes[0].set_ylabel('pooled S(arm) / S(rho-neg), 95% CI')
axes[0].legend(fontsize=7.5, loc='lower left')
fig.tight_layout()
for ext in ('svg', 'png'):
    fig.savefig(os.path.join(HERE, f'figures/ratio_to_rho.{ext}'), dpi=150)

# Phase breakdown (w = 0), units of sqrt(r).
fig, axes = plt.subplots(1, 2, figsize=(10.5, 4.2), sharey=True)
cmap = {'fixed': '#bdbdbd', 'setup': '#fdae6b', 'walk': '#3182bd', 'verify': '#74c476', 'probes': '#9e9ac8'}
for ax, k in zip(axes, ('k20', 'k24')):
    names = ['rho-neg', 'f1-x07', 'tw-x15', 'twcj-x15']
    for i, a in enumerate(names):
        ph = d[k]['phases'][a]
        if a == 'rho-neg':
            parts = [('setup', ph['setup']), ('walk', ph['search']), ('verify', ph['internal_verification'])]
        elif a == 'f1-x07':
            parts = [('fixed', ph['fixed (base, LA, [d]G check)']), ('setup', ph['setup + collision verify']),
                     ('walk', ph['walk']), ('probes', ph['probes'])]
        else:
            parts = [('fixed', ph['fixed (base, LA, [d]G check)']), ('setup', ph['setup']),
                     ('walk', ph['walk']), ('verify', ph['collision verify']), ('probes', ph['probes'])]
        b = 0
        for n, v in parts:
            ax.bar(i, v, 0.6, bottom=b, color=cmap[n], edgecolor='white', lw=0.5)
            if v > 0.12:
                ax.text(i, b + v / 2, f'{v:.2f}', ha='center', va='center', fontsize=7)
            b += v
        ax.text(i, b + 0.03, f'{b:.2f}', ha='center', va='bottom', fontsize=8, weight='bold')
    ax.axhline(1.329, color='#d62728', lw=0.8, ls=':')
    ax.text(-0.45, 1.36, 'F=1 search floor 1.33 (derived)', ha='left', fontsize=7, color='#d62728',
            bbox=dict(fc='white', ec='none', alpha=0.8))
    ax.set_ylim(0, 3.1)
    ax.set_xticks(range(4)); ax.set_xticklabels(['rho-neg', 'f1-x07', 'tw', 'tw+cj'], fontsize=8)
    ax.set_title(f'k = {k[1:]}: mean cost per phase, w = 0', fontsize=9)
axes[0].set_ylabel('mean GAE / sqrt(r)')
from matplotlib.patches import Patch
axes[1].legend([Patch(color=c) for c in cmap.values()],
               ['fixed (base, LA, [d]G check)', 'set-up (jumps, chains; f1: + collision verify)',
                'walk / rho search', 'collision verify (rho: internal check)', 'target probes'],
               fontsize=7, loc='upper right')
fig.tight_layout()
for ext in ('svg', 'png'):
    fig.savefig(os.path.join(HERE, f'figures/phases.{ext}'), dpi=150)
print('figures written')
