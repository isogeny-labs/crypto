#!/usr/bin/env python3
"""l1_ratios.svg / l1_ratios.png from analysis/summary.json: per-cell MC ratios with 95 % CIs."""
import json, os
import matplotlib; matplotlib.use('Agg'); import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__)); s = json.load(open(f'{H}/analysis/summary.json'))
comps = [('hash/rand', '#1f77b4'), ('hash/bits_rand', '#2ca02c'), ('rand/rand2 (A/A)', '#7f7f7f'), ('hash/hash2 (A/A)', '#9467bd'), ('plant3/rand', '#d62728')]
fig, ax = plt.subplots(figsize=(7, 3.4)); x = range(len(s['cells']))
for i, (c, col) in enumerate(comps):
    off = (i - 2) * 0.12
    ys = [cell['ratios'][c]['mc'] for cell in s['cells']]
    lo = [cell['ratios'][c]['mc'] - cell['ratios'][c]['mc_ci'][0] for cell in s['cells']]
    hi = [cell['ratios'][c]['mc_ci'][1] - cell['ratios'][c]['mc'] for cell in s['cells']]
    ax.errorbar([j + off for j in x], ys, yerr=[lo, hi], fmt='o', ms=3, lw=1, color=col, label=c)
ax.axhline(1, color='k', lw=.6); ax.axhspan(0.97, 1.03, color='k', alpha=.06)
ax.set_xticks(list(x)); ax.set_xticklabels([f"{c['k']}\nN={c['N']}" for c in s['cells']], fontsize=6)
ax.set_ylabel('verified-hit-rate ratio', fontsize=8); ax.tick_params(axis='y', labelsize=7)
ax.set_title('L1: hash base vs random/interval base, 95% CI (band = +-3%)  [pcc:unverified]', fontsize=8)
ax.legend(fontsize=6, ncol=3, loc='upper center'); ax.set_ylim(0.95, 1.1)
fig.tight_layout(); fig.savefig(f'{H}/l1_ratios.svg'); fig.savefig(f'{H}/l1_ratios.png', dpi=80)
