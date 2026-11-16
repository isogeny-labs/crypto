#!/usr/bin/env python3
"""amendment1/figures/engine_items_validated.{svg,png}: (left) (A) twcj-cv0 / twcj and (middle)
twcj-cv0 / rho-neg at w = 0, 0.1, 1 with 95 % CI under both band readings, first 16 sessions
(measured code with the uncounted sign map, erratum E1) against the Addendum 1 sessions (fixed
code); (right) phase breakdown of the Addendum 1 sessions at both sizes."""
import json, os
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
HERE = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')
first = json.load(open(os.path.join(HERE, '..', 'data/pooled_measured.json')))
amend = json.load(open(os.path.join(HERE, 'data/pooled_measured.json')))
fig, axes = plt.subplots(1, 3, figsize=(14, 4.6), gridspec_kw=dict(width_ratios=[1.15, 1.15, 1]))
col = {'first': '#9e9e9e', 'amend': '#08519c'}
for ax, key, title in ((axes[0], 'twcj-cv0/twcj', '(A) twcj-cv0 / twcj'),
                       (axes[1], 'twcj-cv0/rho-neg', 'distance: twcj-cv0 / rho-neg')):
    x = 0
    ticks, labels = [], []
    for k in ('k20', 'k24'):
        for rd in ('priced', 'unpriced'):
            for j, w in enumerate(('0', '0.1', '1')):
                for src, dd, off in (('first', first, -0.07), ('amend', amend, 0.07)):
                    r = dd[k]['ratios'][rd][key][w]
                    xx = x + j * 0.32 + off
                    ax.errorbar([xx], [r['ratio']], yerr=[[r['ratio'] - r['ci95'][0]], [r['ci95'][1] - r['ratio']]],
                                fmt='o' if rd == 'priced' else 's', color=col[src], ms=4, capsize=2,
                                mfc=col[src] if rd == 'priced' else 'white',
                                label=('first 16 sessions (E1: sign map uncounted)' if src == 'first' else 'Addendum 1 sessions (fixed code)')
                                if (x == 0 and j == 0) else None)
            ticks.append(x + 0.32); labels.append(f'k={k[1:]}\n{rd}')
            x += 1.2
    ax.axhline(1, color='k', lw=0.8)
    ax.set_xticks(ticks); ax.set_xticklabels(labels, fontsize=7.5)
    ax.set_title(f'{title}; w = 0, 0.1, 1 left to right in each group\nfilled = table work priced (default), open = unpriced', fontsize=8.5)
    ax.set_ylabel('pooled ratio, 95% CI (8 sessions)')
axes[0].legend(fontsize=7, loc='upper left')
cmap = {'fixed': '#bdbdbd', 'setup': '#fdae6b', 'walk': '#3182bd', 'verify': '#74c476', 'check': '#bcbddc'}
ax = axes[2]
names = ['rho-neg', 'twcj', 'twcj-cv0']
for g, k in enumerate(('k20', 'k24')):
    for i, a in enumerate(names):
        ph = amend[k]['phases'][a]
        if a.startswith('rho'):
            parts = [('setup', ph.get('setup', 0)), ('walk', ph.get('search', 0)), ('check', ph.get('internal_verification', 0))]
        else:
            parts = [('fixed', ph['fixed (base, LA, [d]G check)']), ('setup', ph['setup']), ('walk', ph['walk']),
                     ('verify', ph['collision verify'])]
        xi = g * 3.6 + i
        b = 0
        for n, v in parts:
            ax.bar(xi, v, 0.7, bottom=b, color=cmap[n], edgecolor='white', lw=0.5, label=n if (g == 0 and (i == 1 or (i == 0 and n == 'check'))) else None)
            b += v
        ax.text(xi, b + 0.03, f'{b:.3f}', ha='center', va='bottom', fontsize=7)
ax.set_xticks([0, 1, 2, 3.6, 4.6, 5.6])
ax.set_xticklabels(['rho-neg', 'twcj', 'cv=0', 'rho-neg', 'twcj', 'cv=0'], fontsize=7.5)
ax.text(1, -0.33, 'k = 20', ha='center', fontsize=8); ax.text(4.6, -0.33, 'k = 24', ha='center', fontsize=8)
ax.set_ylim(0, 2.6)
ax.set_title('Addendum 1 sessions: phases, mean over measured runs\n(sqrt(r) units, w = 0)', fontsize=8.5)
ax.set_ylabel('S at w = 0')
ax.legend(fontsize=7, loc='upper right')
fig.tight_layout()
for ext in ('svg', 'png'):
    fig.savefig(os.path.join(HERE, f'figures/engine_items_validated.{ext}'), dpi=150)
print('wrote amendment1/figures/engine_items_validated.svg,png')
