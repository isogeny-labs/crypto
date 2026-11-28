"""Plot already-exported degree data; no field arithmetic or solver work."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
exact = json.loads((root/'evidence/exact-m2/exact-unplanted-m2.json').read_text())
rows = json.loads((root/'evidence/bound-assembly/derived-full-bounds.json').read_text())['cells']
rows = [r for r in rows if r['kind'] == 'unplanted']
fig, axes = plt.subplots(1, 2, figsize=(10, 3.9))
values = [exact['dense_full_ideal_regularity'], exact['chain_full_ideal_regularity']]
axes[0].bar(['Dense', 'Chain'], values, color=['#596579', '#00867b'])
for i, value in enumerate(values):
    axes[0].text(i, value+4, str(value), ha='center', fontsize=12)
axes[0].set_ylim(0, 225)
axes[0].set_ylabel('Ideal-module regularity D')
axes[0].set_title('Exact full system: fixed unplanted m=2')
m = [r['m'] for r in rows]
axes[1].plot(m, [r['dense_D_lower'] for r in rows], 'o-', color='#596579', label='Dense lower bound')
axes[1].plot(m, [r['chain_D_upper'] for r in rows], 's-', color='#00867b', label='Chain upper bound')
axes[1].set_xlabel('Relation arity m')
axes[1].set_title('General proof applications; review pending')
axes[1].legend(frameon=False, fontsize=9)
axes[1].set_xticks(m)
for ax in axes:
    ax.grid(axis='y', alpha=.2)
    ax.set_axisbelow(True)
    ax.spines[['top', 'right']].set_visible(False)
fig.text(.5, .015, 'Unsigned certificates. Bounds are not exact degrees or F4/runtime measurements.', ha='center', fontsize=9)
fig.tight_layout(rect=(0, .055, 1, 1))
for suffix in ('pdf', 'svg'):
    fig.savefig(root/f'complete-comparison.{suffix}')
