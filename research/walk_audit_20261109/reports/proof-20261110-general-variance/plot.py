"""Render only recorded exact Lean controls; no research computation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
here=Path(__file__).resolve().parent
rows=json.loads((here/'proof-values.json').read_text())['rows']
fig,ax=plt.subplots(figsize=(7,2.9))
for offset,key,label,color in [(-.17,'mean','Mean','#285d8e'),(.17,'variance','Variance','#3d9774')]:
    bars=ax.bar([i+offset for i in range(len(rows))],[r[key] for r in rows],.31,label=label,color=color)
    ax.bar_label(bars,labels=['1' if key=='mean' else r['variance_exact'] for r in rows],padding=3)
ax.set_xticks(range(len(rows)),[r['label'] for r in rows],fontsize=8)
ax.set_ylim(0,1.23)
ax.set_ylabel('Actual in-degree moments at 0')
ax.legend(frameon=False,fontsize=9)
ax.spines[['top','right']].set_visible(False)
fig.tight_layout()
for ext in ['svg','pdf','png']:
    fig.savefig(here/('moments.'+ext),dpi=150)
p=here/'moments.svg'
p.write_text('\n'.join(s.rstrip() for s in p.read_text().splitlines())+'\n')
