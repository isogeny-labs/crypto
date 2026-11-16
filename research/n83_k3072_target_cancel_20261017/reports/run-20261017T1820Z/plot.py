import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
p=Path(__file__).resolve().parent
r=json.loads((p/"raw/run_receipt.json").read_text())
fig,ax=plt.subplots(figsize=(8.8,2.6))
fig.patch.set_facecolor("#f5f7fa");ax.set_facecolor("#f5f7fa")
ax.hlines(1,0,r["outer_wall_seconds"],color="#ad6b15",linewidth=5)
ax.plot([r["outer_wall_seconds"]],[1],marker=">",markersize=12,color="#ad6b15")
ax.text(610,1.16,"Setup reached the 1,200 s cap",ha="center",fontsize=11,color="#70440b")
ax.text(610,0.12,"Not run after serial timeout",ha="center",fontsize=11,color="#536171")
ax.set_yticks([1,0],["Serial", "Cancellation"])
ax.set_ylim(-.35,1.6);ax.set_xlim(0,1350);ax.set_xlabel("Recorded outer wall (seconds); no completed target interval")
ax.set_xticks([0,300,600,900,1200]);ax.spines[["top","right","left"]].set_visible(False)
ax.grid(axis="x",alpha=.2);fig.tight_layout()
for ext in ("svg","png"):fig.savefig(p/("outcomes."+ext),dpi=180,bbox_inches="tight")

svg=p/"outcomes.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines())+"\n")
