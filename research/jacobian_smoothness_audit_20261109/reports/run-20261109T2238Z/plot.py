"""Plot only recorded export counters; no curve arithmetic or new results."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
HERE = Path(__file__).resolve().parent
export = json.loads((HERE / '../../evidence/census-v1/export.json').read_text())
rows = [r for r in export['runs'] if r['replicate'] == 0]
fig, axes = plt.subplots(1, 2, figsize=(7.2, 3.2))
for ax, columns, title, unit in [
    (axes[0], [('models_attempted','All models','#7b8993'),('valid_models','Squarefree','#285d8e')], 'Complete monic-quintic models', 'Models'),
    (axes[1], [('proper_nontrivial_unequal','All proper nontrivial','#285d8e'),('proper_odd_prime_noncharacteristic_unequal','Odd prime, r != p','#3d9774')], 'Unequal cyclic fractions', 'Subgroups across models')]:
    for offset, (key, label, color) in zip([-.19,.19], columns):
        values = [r['measurements']['counters'][key] for r in rows]
        bars = ax.bar([i+offset for i in range(len(rows))], values, width=.36, label=label, color=color)
        ax.bar_label(bars, padding=2, fontsize=8)
    ax.set_xticks(range(len(rows)), ['F3','F5'])
    ax.set_ylabel(unit)
    ax.set_title(title, fontsize=10)
    ax.margins(y=.27)
    ax.legend(loc='upper left', frameon=False, fontsize=7)
    ax.spines[['top','right']].set_visible(False)
fig.tight_layout()
for ext in ['svg','pdf','png']:
    fig.savefig(HERE / ('counts.'+ext), dpi=150)

# Normalize derived SVG formatting; preserve all recorded data and raw logs.
svg_path = HERE / 'counts.svg'
svg_path.write_text('\n'.join(line.rstrip() for line in svg_path.read_text().splitlines()) + '\n')
