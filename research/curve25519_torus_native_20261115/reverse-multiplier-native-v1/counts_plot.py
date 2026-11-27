"""Lay out recorded metadata only; no field/group/solver computation."""
import csv
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
queries = json.loads((root / "RESULTS.json").read_text())["actual_queries"]
with (root / "counts.csv").open("w", newline="") as handle:
    writer = csv.writer(handle)
    writer.writerow(["fixed_query", "multiplier_lifts", "geometric_rows"])
    for query in queries:
        writer.writerow([query["name"], query["ordered_multiplier_lifts"], query["geometric_rows"]])
plt.rcParams.update({"svg.fonttype": "none", "font.size": 10})
fig, ax = plt.subplots(figsize=(7.1, 2.35), layout="constrained")
positions = [0, 1, 2]
ax.bar([x - 0.16 for x in positions], [q["ordered_multiplier_lifts"] for q in queries], 0.30, label="Multiplier lifts", color="#28658d")
ax.bar([x + 0.16 for x in positions], [q["geometric_rows"] for q in queries], 0.30, label="Geometric signed rows", color="#e89947")
ax.set_xticks(positions, ["Public u9", "Public −u9", "Known-consistent R"])
ax.set_yticks([0, 1, 2, 3, 4])
ax.set_ylim(0, 4.9)
ax.set_ylabel("Recorded count per query")
ax.legend(loc="upper right", frameon=False, ncols=2, fontsize=9)
ax.spines[["top", "right"]].set_visible(False)
ax.set_title("Four multiplier lifts encode one row in each fixed control", fontsize=11)
fig.savefig(root / "counts.pdf")
fig.savefig(root / "counts.svg")
fig.savefig(root / "counts.png", dpi=150)
# Normalize freshly generated SVG layout before any attestation is issued.
svg = root / "counts.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
