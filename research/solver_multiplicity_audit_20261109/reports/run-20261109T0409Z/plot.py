"""Plot recorded exact counts only; performs no research computation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.ticker import MaxNLocator

here=Path(__file__).resolve().parent
rows=[json.loads(s) for s in (here/'../../evidence/analysis.jsonl').read_text().splitlines()]
ids=['double','split','simple','degree3']
first=[next(r for r in rows if r['fixture']==name and r['replicate']==0) for name in ids]
fig,ax=plt.subplots(figsize=(7.2,2.8),layout='constrained')
series=[('Distinct Fp points','points','#34688C'),('Quotient length','msolve_qdim','#D28A37'),('Eliminant degree','parameter_degree','#43866A')]
for offset,(label,key,color) in zip([-0.24,0,0.24],series):
    bars=ax.bar([i+offset for i in range(4)],[r[key] for r in first],width=0.23,label=label,color=color)
    ax.bar_label(bars,padding=2,fontsize=9)
ax.set_xticks(range(4),['Double point','Split points','Simple point','F2 degree control'])
ax.set_ylim(0,2.9)
ax.set_ylabel('Exact count')
ax.yaxis.set_major_locator(MaxNLocator(integer=True))
ax.legend(loc='upper center',ncol=3,fontsize=8,frameon=False)
ax.spines[['top','right']].set_visible(False)
ax.set_axisbelow(True)
ax.grid(axis='y',alpha=0.15)
fig.savefig(here/'support-vs-length.svg')
fig.savefig(here/'support-vs-length.pdf')
