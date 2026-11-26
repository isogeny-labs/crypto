"""Render recorded catalog counts and witness ranks; no field computations."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
rows = json.loads((root / 'evidence/full_cutoff_v1/summary.json').read_text())
plt.rcParams.update({'svg.fonttype': 'none', 'font.size': 9})
fig, (left, right) = plt.subplots(1, 2, figsize=(9, 2.7))
for values, shift, label, color in [
    ([r['regular_charts'] for r in rows], -.19, 'Prior regular catalog', '#8a9aaa'),
    ([r['full_charts'] for r in rows], .19, 'Full multiset catalog', '#267e68'),
]:
    bars = left.bar([i + shift for i in range(3)], values, .36, label=label, color=color)
    left.bar_label(bars, padding=2)
left.set_xticks(range(3), ['F127\ndegree 16', 'F8191\ndegree 16', 'F8191\ndegree 32'])
left.set_ylabel('Number of systems in catalog')
left.set_ylim(0, 145)
left.legend(frameon=False, fontsize=8, loc='upper left')
left.set_title('Complete catalog adds branches')
patterns = rows[2]['patterns']
r2 = [r['ranks'][2] for r in patterns]
r3 = [r['ranks'][3] for r in patterns]
b2 = right.bar(range(3), r2, .55, color='#bc7041', label='Effective rank 2')
b3 = right.bar(range(3), r3, .55, bottom=r2, color='#267e68', label='Effective rank 3')
right.bar_label(b2, labels=[str(v) if v else '' for v in r2], padding=2)
right.bar_label(b3, labels=[str(v) if v else '' for v in r3], padding=2)
right.set_xticks(range(3), [str(r['distinct_images']) for r in patterns])
right.set_xlabel('Distinct fine images in a four-leaf tuple')
right.set_ylabel('Checked signed RR5 bindings')
right.set_ylim(0, 11200)
right.legend(frameon=False, fontsize=8, loc='upper left')
right.set_title('F8191 degree 32: singular subfamily')
for ax in (left, right):
    ax.spines[['top', 'right']].set_visible(False)
fig.text(.5, .01, 'full_cutoff_v1: exact finite counts, unsigned; no timing or solver-output comparison.',
         ha='center', fontsize=8)
fig.tight_layout(rect=(0, .045, 1, 1))
fig.savefig(root / 'full-cutoff-counts.svg')
fig.savefig(root / 'full-cutoff-counts.pdf')
svg = root / 'full-cutoff-counts.svg'
svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines()) + '\n')
