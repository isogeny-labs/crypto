"""Plot recorded calibration values only; no curve arithmetic or new results."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
HERE = Path(__file__).resolve().parent
export = json.loads((HERE / '../../evidence/calibration/export-v2.json').read_text())
rows = export['runs']
x = list(range(len(rows)))
fig, ax = plt.subplots(figsize=(6.2, 2.6))
for offset, key, label, color in [(-.19,'control_pairs','All reduced pairs','#285d8e'),(.19,'control_smooth','Split u','#3d9774')]:
    bars = ax.bar([v+offset for v in x], [r['measurements']['counted'][key] for r in rows], width=.36, label=label, color=color)
    ax.bar_label(bars, padding=2)
ax.set_xticks(x, ['F3 positive', 'F5 all-smooth control'])
ax.set_ylabel('Representations (identity included)')
ax.set_ylim(0, 19)
ax.legend(loc='upper left', frameon=False)
ax.set_title('Recorded exact calibration counts; one execution per field')
ax.spines[['top', 'right']].set_visible(False)
fig.tight_layout()
for ext in ['svg','pdf','png']:
    fig.savefig(HERE / ('counts.'+ext), dpi=150)
