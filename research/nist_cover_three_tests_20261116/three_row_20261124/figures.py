"""Layout recorded JSON values only; no curve, field, rank or query arithmetic."""
from pathlib import Path
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
records = [json.loads((root / "evidence/qualification-v1" / (arm + ".json")).read_text()) for arm in ["u", "v", "w"]]
plt.rcParams.update({"svg.fonttype": "none", "font.size": 11})
fig, ax = plt.subplots(figsize=(7.5, 3.0), constrained_layout=True)
labels = [r["arm"] for r in records]
bars = ax.bar(labels, [r["query_count"] for r in records], color=["#4C78A8", "#59A14F", "#B07AA1"], width=0.55)
for bar, record in zip(bars, records):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height(), f'{record["query_count"]:,}\n{record["witness_count"]} witnesses', ha="center", va="bottom")
ax.set_ylim(0, 2600000)
ax.set_ylabel("Complete necessary-identity queries")
ax.set_title("Frozen P-192 three-row arms: qualification-v1")
ax.ticklabel_format(style="plain", axis="y")
ax.spines[["top", "right"]].set_visible(False)
fig.savefig(root / "queries.svg")
fig.savefig(root / "queries.pdf")
