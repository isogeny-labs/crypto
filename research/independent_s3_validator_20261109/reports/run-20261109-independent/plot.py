"""Render already-recorded counts; no arithmetic experiment is performed."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
study = here.parent.parent
rust = [json.loads(line) for line in (study / 'evidence/local/actual.jsonl').read_text().splitlines()]
reference = [json.loads(line) for line in (study.parent / 'theorem_census_20261109/evidence/semaev-native.jsonl').read_text().splitlines()]
cpp = {r['q']: r for r in reference if r['m'] == 3 and r['q'] in [5, 7, 11, 13, 17, 19]}
fig, ax = plt.subplots(figsize=(8, 2.7), layout='constrained')
ax.plot([r['q'] for r in rust], [r['zeros'] for r in rust], 'o-', label='Rust: completed local run', linewidth=2)
ax.plot([r['q'] for r in rust], [cpp[r['q']]['zeros'] for r in rust], 'x', label='C++: retained known answer', markersize=9, color='black')
ax.set(xlabel='Prime field p', ylabel='S3 zero tuples (exact count)', xticks=[r['q'] for r in rust])
ax.ticklabel_format(axis='y', style='plain')
ax.legend(loc='upper left', frameon=False)
ax.grid(alpha=.2)
for suffix in ['svg', 'pdf']:
    fig.savefig(here / ('validation.' + suffix))
