"""Layout only: plot exact counts from the native fixed-control record."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

record = json.loads(Path('../../root-reference-v1/native-v1/controls.json').read_text())
fig, axes = plt.subplots(1, 2, figsize=(8.3, 3.2), layout='constrained')
for ax, (label, data) in zip(axes, [
    ('Fp101 fixed fixture (13 sum keys)', record['toy']['setup']),
    ('Actual six-column fixture (160 keys)', record['actual_six_column']['setup']),
]):
    values = [data['signed_half_tuples'], data['retained_half_witnesses']]
    bars = ax.bar(['Enumerated halves', 'Retained witnesses'], values, color=['#4267A8', '#D68A31'])
    ax.bar_label(bars, padding=3, labels=[str(x) for x in values])
    ax.set_ylim(0, max(values) * 1.20)
    ax.set_title(label, fontsize=10)
    ax.set_ylabel('Original signed half-tuple count')
    ax.tick_params(axis='x', labelsize=9)
    ax.spines[['top', 'right']].set_visible(False)
fig.suptitle('Exact enumeration and retained witness counts — fixed controls', fontsize=12)
fig.savefig('counts.svg')
fig.savefig('counts.pdf')
