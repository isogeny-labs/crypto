#!/usr/bin/env python3
"""Presentation only: render recorded counters; never execute research code."""
import json
import os
import tempfile
from pathlib import Path
os.environ.setdefault('MPLCONFIGDIR', str(Path(tempfile.gettempdir()) / 'twist-matplotlib'))
os.environ.setdefault('XDG_CACHE_HOME', str(Path(tempfile.gettempdir()) / 'twist-render-cache'))
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
study = here.parent.parent
historical = json.loads((study / 'historical/results.json').read_text())
native = json.loads((study / 'native/runs/run-20261021T150125Z/results.json').read_text())
run_ids = ['run-20261021T144307Z', 'run-20261021T145210Z', 'run-20261021T150125Z']
series = [json.loads((study / 'native/runs' / run_id / 'results.json').read_text()) for run_id in run_ids]
parity = json.loads((study / 'native/runs/run-20261021T150125Z/parity.json').read_text())
data = {
    'run': 'run-20261021T150125Z',
    'historical_census': historical['exhaustive']['per_prime'],
    'native_census': native['exhaustive']['per_prime'],
    'historical_pairings': [{'r': x['r'], 'all_logs_recovered': x['all_logs_recovered']} for x in historical['pairings']],
    'native_pairings': [{'r': x['r'], 'all_logs_recovered': x['all_logs_recovered']} for x in native['pairings']],
    'parity': parity,
    'successful_native_series': [
        {'run': run_id, 'census': record['exhaustive']['per_prime'],
         'pairings': [{'r': x['r'], 'all_logs_recovered': x['all_logs_recovered']} for x in record['pairings']]}
        for run_id, record in zip(run_ids, series)
    ],
}
(here / 'graph-data.json').write_text(json.dumps(data, indent=2) + '\n')
fig, axes = plt.subplots(1, 2, figsize=(10, 3.4), constrained_layout=True)
for axis, original, replay, key, label, title in [
    (axes[0], data['historical_census'], data['native_census'], 'nonsingular_models', 'p', 'Full curve-model census'),
    (axes[1], data['historical_pairings'], data['native_pairings'], 'all_logs_recovered', 'r', 'Complete toy IC recoveries'),
]:
    xs = list(range(len(original)))
    axis.bar([x-.30 for x in xs], [row[key] for row in original], .20, label='Historical Python', color='#8a9aaa')
    for index, (record, color, stamp) in enumerate(zip(data['successful_native_series'], ['#a2ccc7', '#57a198', '#176d68'], ['14:43', '14:52', '15:01'])):
        rows = record['census' if label == 'p' else 'pairings']
        axis.bar([x-.10+.20*index for x in xs], [row[key] for row in rows], .20, label=f'Native {stamp} UTC', color=color)
    axis.set_xticks(xs, [str(row[label]) for row in original])
    axis.set(xlabel='Prime field p' if label == 'p' else 'Subgroup order r', ylabel='Exact count', title=title)
    axis.spines[['top', 'right']].set_visible(False)
    axis.legend(frameon=False, fontsize=7)
fig.suptitle('Recorded finite coverage: successful native replay series, 21 October 2026', fontsize=11)
fig.savefig(here / 'coverage.svg')
svg_path = here / 'coverage.svg'
svg_path.write_text('\n'.join(line.rstrip() for line in svg_path.read_text().splitlines()) + '\n')
fig.savefig(here / 'coverage.pdf')
fig.savefig(here / 'coverage.png', dpi=150)
