"""Render only recorded exact Lean moment controls."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
here=Path(__file__).resolve().parent
rows=json.loads((here/'proof-values.json').read_text())['rows']
fig,ax=plt.subplots(figsize=(5.9,2.8))
for offset,key,label,color in [(-.16,'mean','Mean','#285d8e'),(.16,'variance','Variance','#3d9774')]:
    bars=ax.bar([i+offset for i in range(2)],[r[key] for r in rows],.3,label=label,color=color)
    ax.bar_label(bars,labels=['1' if key=='mean' else r['variance_exact'] for r in rows],padding=3)
ax.set_xticks(range(2),[r['label'] for r in rows])
ax.set_ylim(0,1.22)
ax.set_ylabel('In-degree moments at vertex 0')
ax.legend(frameon=False,fontsize=9)
ax.spines[['top','right']].set_visible(False)
fig.tight_layout()
for ext in ['svg','pdf','png']:
    fig.savefig(here/('moments.'+ext),dpi=150)
p=here/'moments.svg'
p.write_text('\n'.join(s.rstrip() for s in p.read_text().splitlines())+'\n')
