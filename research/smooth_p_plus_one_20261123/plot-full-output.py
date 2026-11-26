"""Plot recorded count fields only; no solving, research arithmetic or timing analysis."""
from pathlib import Path
import csv
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
root=Path(__file__).resolve().parent
rows=list(csv.reader((root/'evidence/full_output_v1/graph_data.csv').open()))
fig,ax=plt.subplots(figsize=(7.5,2.7))
policies=[('original','First row','#325b82'),('lookup','First + lookup','#839fb5'),('full','Full output','#247c68'),('full_lookup','Full + lookup','#84c7b6')]
for j,(policy,label,color) in enumerate(policies):
    chosen=[r for r in rows if r[0]==policy]
    bars=ax.bar([i+(j-1.5)*.19 for i in range(3)],[int(r[3]) for r in chosen],width=.18,label=label,color=color)
    ax.bar_label(bars,padding=2,fontsize=9)
ax.set_xticks(range(3),['Direct','Refined (narrower oracle)','Cofactor + msolve'],fontsize=9)
ax.set_ylabel('Four-leaf oracle calls')
ax.set_ylim(0,15)
ax.set_yticks([0,3,6,9,12,15])
ax.spines[['top','right']].set_visible(False)
ax.legend(frameon=False,loc='upper left',fontsize=8,ncols=2)
ax.set_title('F127 full_output_v1: one fixed target and seed per arm/policy',fontsize=11)
fig.text(.5,.01,'Includes incomplete calls; no timing comparison or sampling uncertainty estimated.',ha='center',fontsize=8)
fig.tight_layout(rect=(0,.04,1,1))
fig.savefig(root/'full-output-counts.svg')
fig.savefig(root/'full-output-counts.pdf')
svg=root/'full-output-counts.svg'
svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
