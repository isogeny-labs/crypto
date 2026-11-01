#!/usr/bin/env python3
"""Presentation only: plot already recorded values; never run research code."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
data = json.loads((HERE.parent/'historical/results.json').read_text())
rows = data['exhaustive']['per_prime']
cases = data['pairings']
figure, axes = plt.subplots(1, 2, figsize=(10, 3.5), constrained_layout=True)
axes[0].bar([str(row['p']) for row in rows], [row['nonsingular_models'] for row in rows], color='#176d68')
axes[0].set(xlabel='Prime field p', ylabel='Models checked (exact count)', title='Historical model census')
axes[1].bar([str(row['r']) for row in cases], [row['all_logs_recovered'] for row in cases], color='#31699a')
axes[1].set(xlabel='Subgroup order r', ylabel='Complete recoveries (exact count)', title='Historical Python IC recoveries')
for axis in axes:
    axis.spines[['top', 'right']].set_visible(False)
figure.suptitle('Recorded 2026-10-21 historical correctness run; no timing or scaling claim', fontsize=11)
for extension in ['svg', 'png', 'pdf']:
    figure.savefig(HERE/f'coverage.{extension}', dpi=150)
plt.close(figure)
(HERE/'coverage-data.json').write_text(json.dumps({'per_prime': rows,
    'pairings': [{'p': row['p'], 'r': row['r'], 'all_logs_recovered': row['all_logs_recovered']} for row in cases],
    'source': '../historical/results.json', 'uncertainty': 'not applicable: exact finite counts'}, indent=2)+'\n')

figure, axis = plt.subplots(figsize=(10, 2.3), constrained_layout=True)
axis.set(xlim=(0,10), ylim=(0,3)); axis.axis('off')
boxes = [(1.5, 1.9, 'Frozen Python snapshot\n12 hash-bound files'),
         (5, 1.9, 'Native C++ audit\nno Python execution'),
         (8.5, 1.9, 'Verified certificates\n3 IC / 81 scalar / 3219 bilinear')]
for x, y, label in boxes:
    axis.text(x,y,label,ha='center',va='center',fontsize=9,
        bbox={'boxstyle':'round,pad=0.6','facecolor':'#e6f0f2','edgecolor':'#285d75'})
axis.annotate('',xy=(3.6,1.9),xytext=(2.8,1.9),arrowprops={'arrowstyle':'->'})
axis.annotate('',xy=(7.1,1.9),xytext=(6.3,1.9),arrowprops={'arrowstyle':'->'})
axis.text(5,0.55,'Not rerun natively: full model/S3/S4 census, trace-zero/subfield enumeration, relation collector',
          ha='center',fontsize=9,color='#7b3a35')
for extension in ['svg','png','pdf']:
    figure.savefig(HERE/f'audit-scope.{extension}',dpi=150)
plt.close(figure)
