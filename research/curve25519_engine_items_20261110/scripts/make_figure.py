#!/usr/bin/env python3
"""figures/engine_items.{svg,png} from data/pooled_measured.json: (left) pooled ratios for item A
(twcj-cv0 / twcj) and item B (rho-neg-chained / rho-neg) at w = 0, 0.1, 1 with 95 % CI, both band
readings, against the frozen model points; (right two) phase breakdown per arm and size."""
import json, os
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
HERE = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')
d = json.load(open(os.path.join(HERE, 'data/pooled_measured.json')))
model = json.load(open(os.path.join(HERE, 'data/model.json')))
fig, axes = plt.subplots(1, 3, figsize=(13.5, 4.5), gridspec_kw=dict(width_ratios=[1.3, 1, 1]))
ax = axes[0]
items = [('A: cv0 / twcj', 'twcj-cv0/twcj', 'A'), ('B: chained / rho-neg', 'rho-neg-chained/rho-neg', 'B')]
col = {'k20': '#08519c', 'k24': '#6baed6'}
x = 0
ticks, labels = [], []
for lab, key, it in items:
    for k in ('k20', 'k24'):
        for rd, mk in (('priced', 'o'), ('unpriced', 's')):
            for j, w in enumerate(('0', '0.1', '1')):
                r = d[k]['ratios'][rd][key][w]
                xx = x + j * 0.22 + (0.08 if rd == 'unpriced' else 0)
                ax.errorbar([xx], [r['ratio']], yerr=[[r['ratio'] - r['ci95'][0]], [r['ci95'][1] - r['ratio']]],
                            fmt=mk, color=col[k], ms=4, capsize=2, mfc='white' if rd == 'unpriced' else col[k])
        m = model['sizes'][k][it]['ratio']['0']
        ax.scatter([x + 0.04], [m], marker='x', color='#d62728', s=45, zorder=5)
        ticks.append(x + 0.26); labels.append(f'{it} k={k[1:]}')
        x += 1
ax.axhline(1, color='k', lw=0.8)
ax.axhspan(0.91, 0.95, xmin=0.5, xmax=0.75, color='#fdd0a2', alpha=0.5, label='B1 band [0.91, 0.95] (k=20)')
ax.set_xticks(ticks); ax.set_xticklabels(labels, fontsize=8)
ax.set_ylabel('pooled ratio, 95% CI (w = 0, 0.1, 1 left to right)')
ax.set_title('Engine items: filled = table work priced (default), open = unpriced;\nred x = frozen model at w = 0', fontsize=8.5)
ax.legend(fontsize=7, loc='lower right')
cmap = {'fixed': '#bdbdbd', 'setup': '#fdae6b', 'walk': '#3182bd', 'verify': '#74c476', 'probes': '#9e9ac8', 'check': '#bcbddc'}
for ax, k in zip(axes[1:], ('k20', 'k24')):
    names = ['rho-neg', 'rho-neg-chained', 'twcj', 'twcj-cv0']
    for i, a in enumerate(names):
        ph = d[k]['phases'][a]
        if a.startswith('rho'):
            parts = [('setup', ph.get('setup', 0)), ('walk', ph.get('search', 0)), ('check', ph.get('internal_verification', 0))]
        else:
            parts = [('fixed', ph['fixed (base, LA, [d]G check)']), ('setup', ph['setup']), ('walk', ph['walk']),
                     ('verify', ph['collision verify']), ('probes', ph['probes'])]
        b = 0
        for n, v in parts:
            ax.bar(i, v, 0.6, bottom=b, color=cmap[n], edgecolor='white', lw=0.5, label=n if (i in (0, 2) and k == 'k20' and n not in ('setup', 'walk') or (i == 2 and k == 'k20')) else None)
            if v > 0.12:
                ax.text(i, b + v / 2, f'{v:.2f}', ha='center', va='center', fontsize=6.5)
            b += v
        ax.text(i, b + 0.03, f'{b:.3f}', ha='center', va='bottom', fontsize=7.5, weight='bold')
    ax.set_xticks(range(len(names))); ax.set_xticklabels(['rho-neg', 'rho-neg\nchained', 'twcj', 'twcj\ncv=0'], fontsize=8)
    ax.set_title(f'k = {k[1:]}: phases, mean over measured runs (sqrt(r) units, w = 0)', fontsize=8.5)
    ax.set_ylim(0, 2.6)
h, l = axes[1].get_legend_handles_labels()
seen = {}
for hh, ll in zip(h, l):
    seen.setdefault(ll, hh)
axes[1].legend(seen.values(), seen.keys(), fontsize=7, loc='upper left')
axes[1].set_ylabel('S at w = 0')
fig.tight_layout()
for ext in ('svg', 'png'):
    fig.savefig(os.path.join(HERE, f'figures/engine_items.{ext}'), dpi=150)
print('wrote figures/engine_items.svg,png')
