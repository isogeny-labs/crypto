"""Render recorded synthetic controls and a conditional dependency diagram."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch

root = Path(__file__).resolve().parent
record = json.loads((root / "evidence/local-v1/result.json").read_text())
plt.rcParams["svg.fonttype"] = "none"
fig, (logic, ax) = plt.subplots(2, 1, figsize=(8.5, 4.3), gridspec_kw={"height_ratios": [1, 1.7]}, layout="constrained")
logic.set(xlim=(0, 10), ylim=(0, 2))
logic.axis("off")
for x, label in [(0.1,"One anchor\nResidual supports disjoint"), (3.5,"After projection: test\nfor private new columns"), (6.9,"IF all added rows peel:\nno old-base rank gain")]:
    logic.add_patch(FancyBboxPatch((x,0.35),2.9,1.2,boxstyle="round,pad=0.03",facecolor="#e7edf3",edgecolor="#355b75"))
    logic.text(x+1.45,0.95,label,ha="center",va="center",fontsize=9)
for x in (3.05,6.45):
    logic.annotate("",xy=(x+0.4,0.95),xytext=(x,0.95),arrowprops={"arrowstyle":"->"})
logic.set_title("Conditional obstruction; actual P-192 projection collisions remain untested",fontsize=10)
labels = ["Private", "Shared alias", "Signed alias", "Unsigned cycle", "Signed cycle", "Zero new part"]
bars = ax.bar(labels, [c["closed_rank_gain"] for c in record["cases"]],color="#28658c")
ax.bar_label(bars,padding=3)
ax.set_ylim(0,1.4)
ax.set_yticks([0,1])
ax.set_ylabel("Additional closed rank")
ax.tick_params(axis="x",labelrotation=18)
ax.set_title("Six synthetic coefficient-matrix controls; one fixed qualification",fontsize=10)
ax.spines[["top","right"]].set_visible(False)
fig.savefig(root / "closure.svg")
fig.savefig(root / "closure.png",dpi=100)
