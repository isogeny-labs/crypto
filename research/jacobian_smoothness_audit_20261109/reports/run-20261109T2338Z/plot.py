"""Render recorded witness-export counters only; no arithmetic research."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
here = Path(__file__).resolve().parent
rows = json.loads((here/'../../evidence/stoll-witness-v1/export.json').read_text())['runs']
fig, ax = plt.subplots(figsize=(6.7, 3.1))
for offset, key, label, color in [(-.25,'witness_rows','Candidate rows','#285d8e'), (0,'high_reductions','High reductions','#3d9774'), (.25,'nontrivial_normalization_units','Non-one units','#b27936')]:
    values = [r['measurements']['counters'][key] for r in rows]
    bars = ax.bar([i+offset for i in range(len(rows))],values,.23,label=label,color=color)
    ax.bar_label(bars,padding=2,fontsize=9)
ax.set_xticks([0,1],['Known cycle (25 pairs)','Complete model (100 pairs)'])
ax.set_ylabel('Recorded candidate certificates')
ax.set_ylim(0,123)
ax.legend(frameon=False,fontsize=8)
ax.spines[['top','right']].set_visible(False)
fig.tight_layout()
for ext in ['svg','pdf','png']:
    fig.savefig(here/('counts.'+ext),dpi=150)
p = here/'counts.svg'
p.write_text('\n'.join(s.rstrip() for s in p.read_text().splitlines())+'\n')
