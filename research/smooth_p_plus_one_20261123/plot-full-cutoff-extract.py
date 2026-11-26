"""Plot recorded completion counts; no cryptographic computation or timing fit."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
rows = json.loads((root / 'evidence/full_cutoff_extract_v1/summary.json').read_text())
plt.rcParams.update({'svg.fonttype': 'none', 'font.size': 9})
fig, (left, right) = plt.subplots(1, 2, figsize=(9, 2.7))
keys = [(127, 4), (8191, 4), (8191, 5)]
for name, shift, color, label in [
    ('FullCutoff', -.19, '#267e68', 'Full cutoff'),
    ('ProductionCofactorMsolve', .19, '#6685aa', 'Cofactor + msolve'),
]:
    groups = [[r for r in rows if (r['prime'], r['depth']) == k and r['mode'] == name] for k in keys]
    complete = [sum(r['status'] == 'exact' for r in g) for g in groups]
    planned = [len(g) for g in groups]
    xs = [i + shift for i in range(3)]
    left.bar(xs, planned, .35, color='#e8e8e8')
    bars = left.bar(xs, complete, .35, color=color, label=label)
    left.bar_label(bars, labels=[f'{a}/{b}' for a, b in zip(complete, planned)], padding=2)
left.set_xticks(range(3), ['F127\ndegree 16', 'F8191\ndegree 16', 'F8191\ndegree 32'])
left.set_ylim(0, 6.6)
left.set_yticks(range(6))
left.set_ylabel('Complete queries / planned queries')
left.legend(frameon=False, fontsize=8, loc='upper left')
left.set_title('Full-set completion under the fixed limit')
large = [r for r in rows if r['mode'] == 'FullCutoff' and r['depth'] == 5]
bars = right.bar(range(5), [r['charts_completed'] for r in large], .6, color='#bc7041')
right.bar_label(bars, padding=2)
right.axhline(119, color='#666666', ls='--', lw=1)
right.text(2, 122, '119 charts required for each answer', ha='center', fontsize=8)
right.set_xticks(range(5), ['17G', '43G', '89G', '152G', 'Planted'])
right.set_ylim(0, 136)
right.set_ylabel('Completed full-cutoff charts')
right.set_title('F8191 degree 32: retained incomplete prefixes')
for ax in (left, right):
    ax.spines[['top', 'right']].set_visible(False)
fig.text(.5, .01, 'full_cutoff_extract_v1: one frozen call per arm/target; unsigned output qualification, no timing ratio.', ha='center', fontsize=8)
fig.tight_layout(rect=(0, .045, 1, 1))
fig.savefig(root / 'full-cutoff-extract-counts.svg')
fig.savefig(root / 'full-cutoff-extract-counts.pdf')
svg = root / 'full-cutoff-extract-counts.svg'
svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines()) + '\n')
