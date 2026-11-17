#!/usr/bin/env python3
"""Rendering only: plot frozen Rust analyzer medians/intervals, no new statistics.

Run after the timed supervisor finishes. All data are read, never modified.
"""
import json
from pathlib import Path
import sys
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
panel = sys.argv[1] if len(sys.argv)>1 else 'main'
assert panel in ('main', 'preflight')
prefix = '' if panel=='main' else 'calibration-'
summary_path = 'run/summary.json' if panel=='main' else 'run/preflight-summary.json'
summary = json.loads((root / summary_path).read_text())
retained_n = 6 if panel=='main' else 1
configs = [
 ('coset:word62:m8192', 'coset word62 / 8192'),
 ('coset:word62:m16384', 'coset word62 / 16384'),
 ('coset:p224:m2048', 'coset P224 / 2048'),
 ('rational:word62:m2048', 'rational word62 / 2048'),
 ('rational:p224:m128', 'rational P224 / 128'),
 ('rational:p224:m512', 'rational P224 / 512'),
]
groups = {g['group']: g for g in summary['groups']}
populations = [('seeded-random-abscissa', 'Random abscissa'), ('conditioned-sum', 'Conditioned sum')]
styles = [('school', 'Schoolbook / Half', '#2764a0', 'o'), ('newton', 'Euclid-Newton / Half', '#bd5e16', 's'), ('lookup', 'Lookup / Half', '#40824e', '^')]
plt.rcParams.update({'font.size': 8, 'svg.fonttype': 'none'})
for stem, phases, title in [
 ('counts', [('query_muls', 'Query only'), ('cold_muls', 'Full construction + query')], 'Count ratio: reference / half-GCD'),
 ('timing', [('query_ns', 'Query only'), ('cold_ns', 'Full construction + query')], 'Local elapsed ratio: reference / half-GCD · below L2'),
]:
 fig, axes = plt.subplots(2, 2, figsize=(9.2, 6.3), sharey=True)
 plotted = []
 for row, (pop, poplabel) in enumerate(populations):
  for col, (phase, phaselabel) in enumerate(phases):
   ax = axes[row, col]
   for j, (arm, label, color, marker) in enumerate(styles):
    for i, (key, cfglabel) in enumerate(configs):
     group = groups.get(f'{key}:{pop}:finite')
     if group is None:
      continue
     value = group['paired_ratios'][f'{arm}/half:{phase}']
     if value['n'] == 0:
      continue
     # Median and endpoints were computed by the preregistered Rust analyzer.
     lo, hi = value['bootstrap_median95']
     med = value['median']
     y = i + (j-1)*0.17
     ax.plot([lo, hi], [y, y], color=color, lw=1.25)
     ax.plot([med], [y], color=color, marker=marker, ms=4, ls='none', label=label if i==0 else None)
     plotted.append({'group':group['group'], 'phase':phase, 'reference':arm, 'native_ratio_summary':value})
   ax.axvline(1, color='#666', ls='--', lw=.8)
   ax.set_xscale('log')
   ax.set_yticks(range(len(configs)), [v for _, v in configs])
   ax.grid(axis='x', alpha=.18)
   ax.set_title(f'{poplabel} · {phaselabel}')
   ax.set_xlabel('Paired ratio (log scale); >1 favors half-GCD')
   if row==0 and col==0:
    ax.legend(fontsize=7, loc='best', framealpha=.8)
 axes[0,0].invert_yaxis()
 fig.suptitle(title)
 fig.text(.5, .013, f'VIETA-SPEED-{panel.upper()}-20261117 · up to n={retained_n} per population/configuration · descriptive bootstrap median intervals\nFrozen native summaries; no pooled population inference; local elapsed clocks are diagnostic only.', ha='center', fontsize=7)
 fig.tight_layout(rect=(0,.058,1,.96))
 for ext, kwargs in [('svg', {}), ('png', {'dpi':150})]:
  output = root / f'{prefix}{stem}.{ext}'
  if output.exists():
   sys.exit(f'refuse replacement: {output}')
  fig.savefig(output, **kwargs)
 plt.close(fig)
 data = root / f'{prefix}{stem}.plot-data.json'
 with data.open('x') as f:
  json.dump({'schema':'vieta-recorded-plot/v1', 'source':summary_path, 'rendering_only':True, 'values':plotted}, f, indent=2)
