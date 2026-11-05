from pathlib import Path
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
p=Path(__file__).resolve().parent
rows=json.loads((p/"plot-data.json").read_text())
fig,ax=plt.subplots(figsize=(8.2,3.1))
for arm,offset,color in [("baseline",-.18,"#1b6c8e"),("spike40",.18,"#b66c21")]:
 vals=[x["last_fall_degree"] for x in rows if x["arm"]==arm]
 bars=ax.bar([i+offset for i in range(4)],vals,.34,label=arm,color=color)
 ax.bar_label(bars,padding=3)
ax.set_xticks(range(4),["Koblitz\nplanted","Koblitz\nnatural","Binary\nplanted","Binary\nnatural"])
ax.set_ylabel("Certified last fall degree")
ax.set_ylim(0,3.8);ax.set_yticks(range(4));ax.legend(loc="upper right",frameon=False,ncol=2)
ax.spines[["top","right"]].set_visible(False)
ax.set_title("Same target and candidate-domain size; retained m=2 systems")
fig.tight_layout();fig.savefig(p/"lfd-comparison.svg");plt.close(fig)

svg = p / "lfd-comparison.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
