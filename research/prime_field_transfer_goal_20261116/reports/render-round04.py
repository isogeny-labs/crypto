"""Document plotting only: read existing qualification counts, never run crypto."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

base = Path(__file__).resolve().parent
receipts = base.parent / 'receipts' / 'round-04'
datasets = []
fig, axes = plt.subplots(1, 2, figsize=(10, 2.6), constrained_layout=True)
plt.rcParams.update({'svg.fonttype': 'none', 'font.family': 'DejaVu Sans'})
for ax, u, total in zip(axes, [3, 5], [18, 4]):
    for version, label, color in [('02', 'Constant denominator', '#748c96'),
                                  ('04', 'Rational Padé', '#127c65')]:
        files = [f'qualify-{version}-u3-all.jsonl'] if u == 3 else [
            f'qualify-{version}-u5-public-{i}.jsonl' for i in range(4)]
        counts = {}
        for name in files:
            for line in (receipts / name).read_text().splitlines():
                row = json.loads(line)
                if row['kind'] == 'precision_summary':
                    n = row['precision']
                    old = counts.setdefault(n, {'inputs': 0, 'accepted': 0})
                    old['inputs'] += row['inputs']
                    old['accepted'] += row['accepted']
        ns = sorted(counts)
        values = [counts[n]['accepted'] for n in ns]
        datasets.append({'u': u, 'version': version, 'source_files': files,
                         'counts': [{'precision': n, **counts[n]} for n in ns]})
        ax.plot(ns, values, '-o', label=label, color=color, linewidth=2.1, markersize=5)
    ax.set_xscale('log', base=2)
    ax.set_xticks([1, 2, 4, 8, 16, 32], labels=['1', '2', '4', '8', '16', '32'])
    ax.set_yticks([0, 6, 12, 18] if u == 3 else [0, 1, 2, 3, 4])
    ax.set_ylim(-.15, total + .6)
    ax.set_title('Order19: all18 nonidentity inputs' if u == 3 else 'Order61: four fixed public inputs', fontsize=10)
    ax.set_xlabel('Source precision N (p-adic digits)', fontsize=9)
    ax.set_ylabel('Accepted inputs (count)', fontsize=9)
    ax.grid(alpha=.2)
    ax.spines[['top', 'right']].set_visible(False)
axes[0].legend(loc='center right', fontsize=8, frameon=False)
fig.savefig(base / 'round-04-coverage.svg')
(base / 'round-04-coverage-data.json').write_text(json.dumps({
    'scope': 'Recorded fixed-bin counts only; no timing, probability estimate or error bars',
    'datasets': datasets}, indent=2) + '\n')
