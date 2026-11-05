"""Plot recorded request counts only; no research arithmetic or fitting."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
matplotlib.rcParams["pdf.use14corefonts"] = True
here=Path(__file__).resolve().parent
data=json.loads((here/"coverage-data.json").read_text())["arms"]
fig,axes=plt.subplots(1,2,figsize=(9.5,3.0),sharey=True)
for ax,role in zip(axes,["main","twist"]):
    for arm,label,color in [("reference","Binary sign loop","#355c7d"),("signed_cache","Charged signed cache","#b5612a")]:
        rows=data[arm][role]
        ax.plot([r["n"] for r in rows],[r["adds"] for r in rows],marker="o",markersize=3,label=label,color=color)
    ax.set_title(role+" / all ten separate targets")
    ax.set_xlabel("Public source target nP")
    ax.grid(alpha=.2)
axes[0].set_ylabel("Cold point-add requests")
axes[0].legend(frameon=False,fontsize=8)
fig.suptitle("F43 prefix bases / setup + one target + source check",fontsize=11)
fig.text(.5,.01,"signed-cache-20261025; cache validation included; reads/curve checks separately counted, not priced",ha="center",fontsize=8)
fig.tight_layout(rect=(0,.05,1,.95))
fig.savefig(here/"lifting.pdf")
p=here/"lifting.svg";fig.savefig(p)
p.write_text("\n".join(line.rstrip() for line in p.read_text().splitlines())+"\n")
plt.close(fig)
