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
    for arm,label,color in [("binary","Binary loop","#355c7d"),("cached","Signed cache","#b5612a"),("direct","Direct signs","#2b8055")]:
        rows=data[arm][role]
        ax.plot([r["n"] for r in rows],[r["adds"] for r in rows],marker="o",markersize=3,label=label,color=color)
    ax.set_title(role+" / all ten separate targets")
    ax.set_xlabel("Public source target nP")
    ax.grid(alpha=.2)
axes[0].set_ylabel("Cold point-add requests")
axes[0].legend(frameon=False,fontsize=8)
fig.suptitle("F43 prefix bases / setup + one target + source check",fontsize=11)
fig.text(.5,.01,"direct-sign-20261025; cache validation included; component requests only, no overall cost ranking",ha="center",fontsize=8)
fig.tight_layout(rect=(0,.05,1,.95))
fig.savefig(here/"signs.pdf")
p=here/"signs.svg";fig.savefig(p)
p.write_text("\n".join(line.rstrip() for line in p.read_text().splitlines())+"\n")
plt.close(fig)

# Proposed-route layout only; no arithmetic or mathematical verification.
route = json.loads((here/"route-data.json").read_text())
fig, ax = plt.subplots(figsize=(9.5,1.45))
ax.set_axis_off()
positions=[0.14,0.50,0.86]
for x,node in zip(positions,route["nodes"]):
    ax.text(x,.43,node["label"],ha="center",va="center",fontsize=8,
            transform=ax.transAxes,bbox={"boxstyle":"round,pad=0.4","fc":"white","ec":"#333333"})
for i,edge in enumerate(route["edges"]):
    ax.annotate("",xy=(positions[i+1]-.11,.43),xytext=(positions[i]+.11,.43),
                xycoords="axes fraction",arrowprops={"arrowstyle":"->","linestyle":"dashed","color":"#333333"})
    ax.text((positions[i]+positions[i+1])/2,.78,edge["label"],ha="center",va="center",
            fontsize=7,transform=ax.transAxes)
fig.suptitle(route["title"],fontsize=9)
fig.tight_layout(rect=(0,0,1,.85))
fig.savefig(here/"route.pdf")
p=here/"route.svg";fig.savefig(p)
p.write_text("\n".join(line.rstrip() for line in p.read_text().splitlines())+"\n")
plt.close(fig)
