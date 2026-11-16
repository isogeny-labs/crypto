#!/usr/bin/env python3
"""figures/jumps.{svg,png} from data/pooled_measured.json: pooled ratio to the 16-jump arm per
jump count (w = 0, 95 % CI, committed band) and the phase breakdown per arm (units of sqrt(r))."""
import json, os
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
HERE = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')
d = json.load(open(os.path.join(HERE, 'data/pooled_measured.json')))
model = json.load(open(os.path.join(HERE, 'data/model.json')))
J = [2, 4, 8, 16, 32]
fig, axes = plt.subplots(1, 3, figsize=(13, 4.3), gridspec_kw=dict(width_ratios=[1.1, 1, 1]))
ax = axes[0]
for k, c, off in (('k20', '#08519c', -0.12), ('k24', '#6baed6', 0.12)):
    xs, ys, lo, hi, ms = [], [], [], [], []
    for i, j in enumerate(J):
        if j == 16:
            continue
        r = d[k]['ratios'][f'twcj-j{j}/twcj-j16']['0']
        xs.append(i + off); ys.append(r['ratio']); lo.append(r['ratio'] - r['ci95'][0]); hi.append(r['ci95'][1] - r['ratio'])
        ms.append(model[k]['arms'][f'j{j}']['ratio_to_j16'])
    ax.errorbar(xs, ys, yerr=[lo, hi], fmt='o', color=c, capsize=3, label=f'{k[1:]}-bit curve, measured')
    ax.scatter(xs, ms, marker='x', color=c, s=40, label=f'{k[1:]}-bit, frozen model' )
ax.axhline(1, color='k', lw=1)
ax.set_xticks(range(len(J))); ax.set_xticklabels([f'jumps={j}' for j in J])
ax.set_yscale('log'); ax.set_ylabel('pooled S(arm) / S(jumps=16), w = 0, 95% CI')
ax.set_title('Jump count of the target-in-walk arm', fontsize=9); ax.legend(fontsize=7)
cmap = {'fixed': '#bdbdbd', 'setup': '#fdae6b', 'walk': '#3182bd', 'verify': '#74c476', 'probes': '#9e9ac8'}
for ax, k in zip(axes[1:], ('k20', 'k24')):
    names = ['rho-neg'] + [f'twcj-j{j}' for j in J]
    for i, a in enumerate(names):
        ph = d[k]['phases'][a]
        if a == 'rho-neg':
            parts = [('setup', ph['setup']), ('walk', ph['search']), ('verify', ph['internal_verification'])]
        else:
            parts = [('fixed', ph['fixed (base, LA, [d]G check)']), ('setup', ph['setup']), ('walk', ph['walk']),
                     ('verify', ph['collision verify']), ('probes', ph['probes'])]
        b = 0
        for n, v in parts:
            ax.bar(i, v, 0.6, bottom=b, color=cmap[n], edgecolor='white', lw=0.5, label=n if (i == 1 and k == 'k20') else None)
            if v > 0.2:
                ax.text(i, b + v / 2, f'{v:.2f}', ha='center', va='center', fontsize=6.5)
            b += v
        ax.text(i, b + 0.05, f'{b:.2f}', ha='center', va='bottom', fontsize=7.5, weight='bold')
    ax.axhline(1.329, color='#d62728', lw=0.8, ls=':', label='two-collision floor 1.329 (derived)' if k == 'k20' else None)
    ax.set_xticks(range(len(names))); ax.set_xticklabels(['rho-neg'] + [f'j{j}' for j in J], fontsize=8)
    ax.set_title(f'k = {k[1:]}: phases, mean over 512 measured runs (sqrt(r) units)', fontsize=8.5)
    ax.set_ylim(0, 8)
axes[1].set_ylabel('S at w = 0'); axes[1].legend(fontsize=7, loc='upper right')
fig.tight_layout()
for ext in ('svg', 'png'):
    fig.savefig(os.path.join(HERE, f'figures/jumps.{ext}'), dpi=150)
print('wrote figures/jumps.svg,png')
