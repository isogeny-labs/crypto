"""Plot recorded endpoint ranks only; no solving or timing analysis."""
from pathlib import Path
import csv
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
root=Path(__file__).resolve().parent
rows=list(csv.reader((root/'evidence/larger_full_output_v1/graph_data.csv').open()))
fig,ax=plt.subplots(figsize=(7.5,2.5))
for depth,label,color,shift in [('4','Degree 16 (30 draws each)','#839fb5',-.18),('5','Degree 32 (8 / 30 / 30 draws)','#247c68',.18)]:
    chosen=[r for r in rows if r[0]==depth]
    bars=ax.bar([i+shift for i in range(3)],[int(r[6]) for r in chosen],width=.34,label=label,color=color)
    ax.bar_label(bars,padding=3,fontsize=10)
ax.set_xticks(range(3),['Direct','Refined (narrower chart)','Cofactor + msolve'],fontsize=9)
ax.set_ylabel('Final consumer rank')
ax.set_ylim(0,19)
ax.set_yticks([0,3,6,9,12,15,18])
ax.spines[['top','right']].set_visible(False)
ax.legend(frameon=False,loc='upper right',fontsize=8)
ax.set_title('F8191 larger_full_output_v1: one fixed target and seed per arm',fontsize=11)
fig.text(.5,.01,'Only degree-32 Direct recovered; ranks have unequal stopping points and are not a cost ratio.',ha='center',fontsize=8)
fig.tight_layout(rect=(0,.04,1,1))
fig.savefig(root/'larger-full-output-counts.svg')
fig.savefig(root/'larger-full-output-counts.pdf')
svg=root/'larger-full-output-counts.svg'
svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
