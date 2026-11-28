"""Layout only: render recorded ranks; no research arithmetic or data execution."""
from pathlib import Path
import json
import csv
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

base = Path(__file__).resolve().parent
cases = json.loads((base / "RESULTS.json").read_text())["cases"]
with (base / "rank-data.csv").open("w", newline="") as handle:
    writer = csv.writer(handle)
    writer.writerow(["orientation", "rows", "columns", "rank", "quotient_columns", "quotient_rank"])
    for case in cases:
        writer.writerow(["opposite" if case["negative"] else "canonical", *[case[k] for k in ("rows", "columns", "rank", "quotient_columns", "quotient_rank")]])
plt.rcParams.update({"svg.fonttype": "none", "font.size": 10})
fig, ax = plt.subplots(figsize=(8, 2.35))
labels, dimensions, ranks = [], [], []
for case in cases:
    sign = "Opposite" if case["negative"] else "Canonical"
    labels.extend([sign + " ambient", sign + " complement"])
    dimensions.extend([case["columns"], case["quotient_columns"]])
    ranks.extend([case["rank"], case["quotient_rank"]])
x = list(range(len(labels)))
ax.bar(x, dimensions, width=0.62, color="#c9d6e4", label="Columns")
ax.bar(x, ranks, width=0.37, color="#38658d", label="Recorded rank")
for i, (dimension, rank) in enumerate(zip(dimensions, ranks)):
    ax.text(i, dimension + 1, f"{rank}/{dimension}", ha="center", fontsize=10)
ax.set_xticks(x, labels)
ax.set_ylabel("Rank / columns (count)")
ax.set_ylim(0, 65)
ax.set_title("Fixed received-ideal-v1 box · 108 rows per orientation")
ax.legend(frameon=False, ncol=2, loc="lower right")
ax.spines[["top", "right"]].set_visible(False)
fig.tight_layout()
fig.savefig(base / "ranks.svg")
fig.savefig(base / "ranks.pdf")
