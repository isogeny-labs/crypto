"""Render already recorded counts; no curve arithmetic, solves or new measurements."""
from pathlib import Path
import csv
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
root=Path(__file__).resolve().parent
rows=list(csv.reader((root/'evidence/cold_accounting_v1/graph_data.csv').open()))
fig,ax=plt.subplots(figsize=(7.5,2.7))
labels=['Direct','Refined (narrower chart)','Cofactor + msolve']
for column,label,color,shift in [(1,'Oracle queries','#325b82',-0.24),(2,'Selected rows','#49a28e',0),(3,'Incomplete queries','#ce8250',0.24)]:
    values=[int(row[column]) for row in rows]
    bars=ax.bar([i+shift for i in range(len(rows))],values,width=.23,label=label,color=color)
    ax.bar_label(bars,padding=2,fontsize=9)
ax.set_xticks(range(len(rows)),labels,fontsize=9)
ax.set_ylabel('Recorded count')
ax.set_ylim(0,15)
ax.set_yticks([0,3,6,9,12,15])
ax.spines[['top','right']].set_visible(False)
ax.legend(frameon=False,ncol=3,loc='upper left',fontsize=8)
ax.set_title('F127 cold_accounting_v1: one fixed target and seed per arm',fontsize=11)
fig.text(.5,.01,'Counts only; no timing comparison or sampling uncertainty estimated.',ha='center',fontsize=8)
fig.tight_layout(rect=(0,.04,1,1))
fig.savefig(root/'cold-accounting-counts.svg')
fig.savefig(root/'cold-accounting-counts.pdf')
