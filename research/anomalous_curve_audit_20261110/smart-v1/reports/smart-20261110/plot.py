"""Document rendering only; plot the preserved exact logical-operation counts."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
r=Path(__file__).resolve().parent
d=json.loads((r/'steps.json').read_text())
fig,ax=plt.subplots(figsize=(7.1,2.7))
x=d['lifts']; w=.34
for label,key,shift,color in [('F5 control','small',-w/2,'#748ba3'),('160-bit fixture','large',w/2,'#28634b')]:
 bars=ax.bar([i+shift for i in x],d[key],width=w,label=label,color=color)
 ax.bar_label(bars,padding=3,fontsize=9)
ax.set_xticks(x,[str(i) for i in x]);ax.set_xlabel('Fixed coefficient lift')
ax.set_ylabel('Logical operations (G + Q)');ax.set_ylim(0,490)
ax.legend(frameon=False,loc='upper center',bbox_to_anchor=(.5,1.2),ncol=2);ax.spines[['top','right']].set_visible(False)
fig.tight_layout();fig.savefig(r/'steps.svg');fig.savefig(r/'steps.pdf');plt.close(fig)

# Normalize generated SVG whitespace; no geometry, data, or PDF change.
p=r/"steps.svg"
p.write_text("\n".join(line.rstrip() for line in p.read_text().splitlines())+"\n")
