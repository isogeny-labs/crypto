"""Part B figure: mean S per arm at w = 0 / 0.1 / 1 (ecbench band JSON), p24 and p28, log scale."""
import json
import matplotlib; matplotlib.use('Agg'); import matplotlib.pyplot as plt
fig, axes = plt.subplots(1, 2, figsize=(12, 4.6))
for ax, t in zip(axes, ['p24', 'p28']):
    b = json.load(open(f'analysis/partB_band_{t}.json'))
    data = {r['arm']: r['mean_s'] for r in b['rows']}
    arms = sorted(data, key=lambda a: data[a][0])
    w = 0.27
    for i, (lab, col) in enumerate([('w = 0', '#1f77b4'), ('w = 0.1', '#ff7f0e'), ('w = 1', '#d62728')]):
        ax.bar([k + (i - 1) * w for k in range(len(arms))], [data[a][i] for a in arms], w, label=lab, color=col)
    ax.set_xticks(range(len(arms))); ax.set_xticklabels(arms, rotation=55, ha='right', fontsize=7)
    ax.set_yscale('log'); ax.set_ylabel('mean S (GAE / sqrt r)'); ax.set_title(f'{t}: 24 measured runs per arm, all verified'); ax.grid(axis='y', alpha=.3)
axes[0].legend(fontsize=8)
fig.suptitle('Part B: every full-rank arm reads the same relations; the target-pinned incumbent and rho are far below', fontsize=10)
fig.tight_layout()
for ext in ('svg', 'png'): fig.savefig(f'figures/partB_band.{ext}', dpi=150)
print('ok')
