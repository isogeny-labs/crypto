#!/usr/bin/env python3
"""figures/scaling.{svg,png}: distance to rho-neg across k (this study at k = 28, 32; the k = 20, 24
points are read from the sibling studies' pooled_measured.json files) and the arms' S against
their walk floors."""
import json, os
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
HERE = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')
d = json.load(open(os.path.join(HERE, 'data/pooled_measured.json')))
jumps = json.load(open(os.path.join(HERE, '../curve25519_tw_jumps_20261108/data/pooled_measured.json')))
tw = json.load(open(os.path.join(HERE, '../curve25519_f1_targetwalk_20261108/data/pooled_measured.json')))
# (k, ratio record, source)
series = {
  'twcj (jumps=16) / rho-neg': [(20, jumps['k20']['ratios']['twcj-j16/rho-neg']['0']), (24, jumps['k24']['ratios']['twcj-j16/rho-neg']['0']),
                                 (28, d['k28']['ratios']['twcj-j16/rho-neg']['0']), (32, d['k32']['ratios']['twcj-j16/rho-neg']['0'])],
  'f1-x07 (table then probes) / rho-neg': [(20, tw['k20']['ratios']['f1-x07/rho-neg']['0']), (24, tw['k24']['ratios']['f1-x07/rho-neg']['0']),
                                            (28, d['k28']['ratios']['f1-x07/rho-neg']['0']), (32, d['k32']['ratios']['f1-x07/rho-neg']['0'])],
}
fig, axes = plt.subplots(1, 2, figsize=(11, 4.3))
ax = axes[0]
for (lab, pts), c in zip(series.items(), ('#08519c', '#9e9e9e')):
    ks = [p[0] for p in pts]; y = [p[1]['ratio'] for p in pts]
    lo = [p[1]['ratio'] - p[1]['ci95'][0] for p in pts]; hi = [p[1]['ci95'][1] - p[1]['ratio'] for p in pts]
    ax.errorbar(ks, y, yerr=[lo, hi], fmt='o-', color=c, capsize=3, label=lab)
ax.axhline(1.5, color='#ff7f0e', ls='--', lw=0.8, label='F = 1 floor / rho model = 1.5 (derived)')
ax.axhline(1.0, color='k', lw=1)
ax.set_xticks([20, 24, 28, 32]); ax.set_xlabel('k (bits of p)'); ax.set_ylabel('pooled S / S(rho-neg), w = 0, 95% CI')
ax.set_ylim(0.8, 1.9); ax.legend(fontsize=7.5, loc='lower left')
ax.set_title('Distance to rho-neg across sizes (k = 20, 24 from the sibling studies)', fontsize=8.5)
ax = axes[1]
S_tw = {20: jumps['k20']['S']['twcj-j16']['0']['mean'], 24: jumps['k24']['S']['twcj-j16']['0']['mean'],
        28: d['k28']['S']['twcj-j16']['0']['mean'], 32: d['k32']['S']['twcj-j16']['0']['mean']}
W_tw = {20: jumps['k20']['phases']['twcj-j16']['walk'], 24: jumps['k24']['phases']['twcj-j16']['walk'],
        28: d['k28']['phases']['twcj-j16']['walk'], 32: d['k32']['phases']['twcj-j16']['walk']}
S_rho = {20: jumps['k20']['S']['rho-neg']['0']['mean'], 24: jumps['k24']['S']['rho-neg']['0']['mean'],
         28: d['k28']['S']['rho-neg']['0']['mean'], 32: d['k32']['S']['rho-neg']['0']['mean']}
W_rho = {20: jumps['k20']['phases']['rho-neg']['search'], 24: jumps['k24']['phases']['rho-neg']['search'],
         28: d['k28']['phases']['rho-neg']['search'], 32: d['k32']['phases']['rho-neg']['search']}
ks = [20, 24, 28, 32]
ax.plot(ks, [S_tw[k] for k in ks], 'o-', color='#08519c', label='twcj total S')
ax.plot(ks, [W_tw[k] for k in ks], 's--', color='#08519c', mfc='white', label='twcj walk only')
ax.plot(ks, [S_rho[k] for k in ks], 'o-', color='#2ca02c', label='rho-neg total S')
ax.plot(ks, [W_rho[k] for k in ks], 's--', color='#2ca02c', mfc='white', label='rho-neg search only')
ax.axhline(1.329, color='#08519c', lw=0.8, ls=':', label='second-collision floor 1.329')
ax.axhline(0.886, color='#2ca02c', lw=0.8, ls=':', label='first-collision floor 0.886')
ax.set_xticks(ks); ax.set_xlabel('k'); ax.set_ylabel('mean S at w = 0 (sqrt(r) units)'); ax.set_ylim(0.7, 2.3)
ax.legend(fontsize=7, loc='upper right', ncol=2); ax.set_title('Both arms approach their collision floors as k grows', fontsize=8.5)
fig.tight_layout()
for ext in ('svg', 'png'):
    fig.savefig(os.path.join(HERE, f'figures/scaling.{ext}'), dpi=150)
print('wrote figures/scaling.svg,png')
