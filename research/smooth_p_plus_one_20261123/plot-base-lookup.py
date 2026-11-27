"""Plot recorded query counts only; no arithmetic research, solving or timing analysis."""
from pathlib import Path
import csv
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
root=Path(__file__).resolve().parent
rows=list(csv.reader((root/'evidence/base_lookup_v1/graph_data.csv').open()))
fig,ax=plt.subplots(figsize=(7.5,2.7))
for policy,label,color,shift in [('original','Original first-row policy','#325b82',-.18),('lookup','Same policy + common lookup','#49a28e',.18)]:
    selected=[r for r in rows if r[0]==policy]
    bars=ax.bar([i+shift for i in range(3)],[int(r[4]) for r in selected],width=.34,label=label,color=color)
    ax.bar_label(bars,padding=3,fontsize=10)
ax.set_xticks(range(3),['Direct','Refined (narrower oracle)','Cofactor + msolve'],fontsize=9)
ax.set_ylabel('Four-leaf oracle calls')
ax.set_ylim(0,15)
ax.set_yticks([0,3,6,9,12,15])
ax.spines[['top','right']].set_visible(False)
ax.legend(frameon=False,loc='upper left',fontsize=8)
ax.set_title('F127 base_lookup_v1: one fixed target and seed per arm/policy',fontsize=11)
fig.text(.5,.01,'Counts include incomplete queries; no timing comparison or sampling uncertainty estimated.',ha='center',fontsize=8)
fig.tight_layout(rect=(0,.04,1,1))
fig.savefig(root/'base-lookup-counts.svg')
fig.savefig(root/'base-lookup-counts.pdf')
svg=root/'base-lookup-counts.svg'
svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
