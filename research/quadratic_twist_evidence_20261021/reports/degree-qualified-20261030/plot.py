# Presentation only: plot exact counters already emitted by the native auditor.
from pathlib import Path
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
p=Path(__file__).resolve().parent
d=json.loads((p/"plot-data.json").read_text())
fig,axes=plt.subplots(2,2,figsize=(8,5.7),sharex=True,sharey=True)
for col,variant in enumerate(["flat","retained"]):
 for row,mode in enumerate(["original","reduced"]):
  a=axes[row,col];records=[v for v in d[variant] if v["mode"]==mode]
  for role,color,style,marker in [("main","#1769aa","-","o"),("twist","#bd263b","--","s"),("matched","#9b7400",":","^"),("pullback","#15805b","-.","x")]:
   a.plot([v["cap"] for v in records],[v[role]["verified_targets"] for v in records],label=role,color=color,linestyle=style,marker=marker,markersize=4)
  a.set_title(variant+" / "+mode);a.set_ylim(-1,25);a.set_xticks(range(9));a.grid(alpha=.2)
  if row==1:a.set_xlabel("Expanded-witness cap")
  if col==0:a.set_ylabel("Verified source target-runs")
handles,labels=axes[0,0].get_legend_handles_labels();fig.legend(handles,labels,loc="lower center",ncol=4,frameon=False)
fig.suptitle("Actual F13 source recovery under certified degree budgets",fontsize=12)
fig.tight_layout(rect=(0,.06,1,.94));fig.savefig(p/"recovery.svg");fig.savefig(p/"recovery.pdf");plt.close(fig)

# SVG presentation whitespace only; coordinate values are unchanged.
pth=p/"recovery.svg"
pth.write_text("\n".join(line.rstrip() for line in pth.read_text().splitlines())+"\n")
