# Layout only; values are exact counters emitted by the native auditor.
from pathlib import Path
import os,json
os.environ.setdefault("MPLCONFIGDIR","/tmp/qt-cm837-mpl")
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
p=Path(__file__).resolve().parent;d=json.loads((p/"plot-data.json").read_text())
fig,axes=plt.subplots(1,2,figsize=(8,3.5),sharey=True)
for a,mode in zip(axes,["original","reduced"]):
 rows=[x for x in d if x["mode"]==mode]
 for role,color,style,mark in [("main","#1769aa","-","o"),("twist","#bd263b","--","s"),("matched","#9b7400",":","^"),("pullback","#15805b","-.","x")]:
  a.plot([x["cap"] for x in rows],[x[role]["verified_targets"] for x in rows],label=role,color=color,linestyle=style,marker=mark,markersize=4)
 a.set_title(mode);a.set_xlabel("Expanded-witness budget");a.set_xticks(range(9));a.grid(alpha=.2)
axes[0].set_ylabel("Verified source-target runs")
h,l=axes[0].get_legend_handles_labels();fig.legend(h,l,loc="lower center",ncol=4,frameon=False)
fig.suptitle("F37 / r19: full size-two policy frontier",fontsize=12);fig.tight_layout(rect=(0,.1,1,.92))
fig.savefig(p/"frontier.svg");fig.savefig(p/"frontier.pdf");plt.close(fig)
q=p/"frontier.svg";q.write_text("\n".join(x.rstrip() for x in q.read_text().splitlines())+"\n")
