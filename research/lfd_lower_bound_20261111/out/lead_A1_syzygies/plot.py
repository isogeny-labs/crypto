#!/usr/bin/env python3
"""Figure: kernel dimension against the trivial family, per cell (grid.jsonl). Run in this dir."""
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
EMIN = {15: 3, 17: 3, 19: 4, 21: 4, 23: 4, 25: 4, 27: 5}
rows = [json.loads(l) for l in open('grid.jsonl')]
fig, ax = plt.subplots(figsize=(7, 4))
for rel, mk, lab in ((-1, 'o', 'e = e_min - 1'), (0, 's', 'e = e_min')):
    xs, ys, ts = [], [], []
    for d in rows:
        r = d['r']
        if r['e'] - EMIN[r['n']] != rel: continue
        xs.append(r['n'] + (0.15 if rel == 0 else -0.15)); ys.append(r['kernel_dim']); ts.append(r['trivial_rank'])
    ax.scatter(xs, ys, marker=mk, label=f'kernel dim, {lab}', zorder=3)
    ax.scatter(xs, ts, marker='_', s=200, color='k', zorder=4, label=f'M(k-1,e-1), {lab}' if rel == 0 else None)
ax.set_yscale('log'); ax.set_xlabel('n (k = (n+1)/2)'); ax.set_ylabel('dimension')
ax.set_title('Homogeneous selector kernel vs trivial family [pcc:unverified]')
ax.legend(fontsize=8); ax.grid(alpha=0.3)
fig.tight_layout(); fig.savefig('kernel_vs_trivial.svg'); fig.savefig('kernel_vs_trivial.png', dpi=130)
