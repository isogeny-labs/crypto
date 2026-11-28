"""Figures for Part A v3: median row_ops and rows read by |F|, per arm, m = 2 and m = 3."""
import json, statistics as S, collections
import matplotlib; matplotlib.use('Agg'); import matplotlib.pyplot as plt
R = [json.loads(l) for l in open('partA/stream-v3.jsonl')]
g = collections.defaultdict(list)
for r in R: g[(r['m'], r['arm'])].append(r)
FB = [32, 64, 128, 256, 512]
style = {
 'incremental-gauss': ('#888888', '--', 'incremental-gauss (target-pinned, incumbent)'),
 'incremental-gauss-echelon': ('#bbbbbb', ':', 'incremental-gauss-echelon (target-pinned)'),
 'incremental-gauss-full-rank': ('#1f77b4', '-', 'incremental-gauss-full-rank'),
 'filter+wiedemann': ('#2ca02c', '-', 'filter+wiedemann'),
 'filter+lanczos': ('#17becf', '-', 'filter+lanczos'),
 'filter+wiedemann+match': ('#2ca02c', '--', 'filter+wiedemann+match (gate)'),
 'mixed-peel': ('#d62728', '-', 'mixed-peel'),
 'mixed-peel-lazy': ('#ff7f0e', '-', 'mixed-peel-lazy'),
 'mixed-peel+bloom': ('#9467bd', ':', 'mixed-peel+bloom'),
}
for metric, key, fname, ylabel in [('row_ops', 'work', 'row_ops', 'median row_ops (log)'),
                                    ('rows', 'rows_read', 'rows_read', 'median rows read (log)')]:
    fig, axes = plt.subplots(1, 2, figsize=(11, 4.2), sharey=False)
    for ax, m in zip(axes, [2, 3]):
        for arm, (col, ls, lab) in style.items():
            ys = [S.median(x[key] for x in g[(m, arm)] if x['fb'] == fb) for fb in FB]
            ax.plot(FB, ys, color=col, ls=ls, marker='o', ms=3, label=lab)
        ax.set_xscale('log', base=2); ax.set_yscale('log'); ax.set_xlabel('|F| (factor-base columns)')
        ax.set_title(f'm = {m} factor-base non-zeros + target column'); ax.set_ylabel(ylabel); ax.grid(alpha=.3)
    axes[1].legend(fontsize=7, loc='upper left')
    fig.suptitle(f'Part A v3: {metric}, 16 planted streams per cell (all answers correct)', fontsize=10)
    fig.tight_layout()
    for ext in ('svg', 'png'): fig.savefig(f'figures/partA_{fname}.{ext}', dpi=150)
print('ok')
