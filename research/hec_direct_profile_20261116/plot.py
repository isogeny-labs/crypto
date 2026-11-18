"""Layout only: render Rust-produced per-case statistics; never compute measurements."""
import csv
import sys
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
root = Path(sys.argv[1])
with (root / "PLOT.tsv").open() as stream:
    rows = list(csv.DictReader(stream, delimiter="\t"))
fig, ax = plt.subplots(figsize=(10.8, 5.7))
for name, offset, color in [("Direct", -0.22, "#057f8c"), ("Gaussian same family", 0.0, "#b96827"), ("Best existing", 0.22, "#424f66")]:
    selected = [row for row in rows if row["series"] == name]
    x = [int(row["case"]) + offset for row in selected]
    med = [float(row["median_wall_s"]) for row in selected]
    low = [float(row["min_wall_s"]) for row in selected]
    high = [float(row["max_wall_s"]) for row in selected]
    ax.vlines(x, low, high, color=color, linewidth=2)
    ax.scatter(x, med, color=color, s=42, label=name, zorder=3)
labels = [row["label"].replace(" / t", "\ntarget ") for row in rows if row["series"] == "Direct"]
ax.set_xticks(range(6), labels)
ax.set_yscale("log")
ax.set_ylabel("Wall seconds per solver call (log scale)")
ax.set_title("Complete algorithm-cold single-target IC cost", loc="left", fontsize=16, pad=18)
ax.grid(axis="y", which="major", color="#d8e0e7")
ax.set_axisbelow(True)
ax.spines[["top", "right"]].set_visible(False)
ax.legend(loc="upper center", bbox_to_anchor=(0.5, -0.14), ncol=3, frameon=False)
record = (root / "RUN-ID.txt").read_text().strip()
fig.text(0.02, 0.025, "Median and min–max of 5 batches × 16 fresh setups; shared ARM L0.\n" + record + "; all six inputs shown; full 17-arm table retained.", fontsize=8)
fig.tight_layout(rect=(0, 0.12, 1, 1))
plt.rcParams["svg.fonttype"] = "none"
fig.savefig(root / "COST.svg")
fig.savefig(root / "COST.png", dpi=150)
