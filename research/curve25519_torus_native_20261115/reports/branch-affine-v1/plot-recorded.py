"""Layout only: copy totals and rows from the native post-call reader."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "none"
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
source = here.parent.parent / "branch-affine-v1/audit-native-v1/RESULT.json"
raw = source.read_bytes()
(here / "native-counts.json").write_bytes(raw)
data = json.loads(raw)
fig, axes = plt.subplots(1, 3, figsize=(10.8, 2.75), gridspec_kw={"width_ratios": [2.4, 4.2, 2]})
categories = ("Root", "Unsupported", "Cap")
colors = ("#2f8a62", "#8498ad", "#d87d2c")
bottom = [0, 0]
for category, color in zip(categories, colors):
    counts = [data["baseline_curve_outcomes"].get(category, 0), data["enabled_curve_outcomes"].get(category, 0)]
    axes[0].bar([0, 1], counts, bottom=bottom, color=color, label=category)
    for i, value in enumerate(counts):
        if value:
            axes[0].text(i, bottom[i] + value / 2, str(value), ha="center", va="center", color="white", fontsize=9)
    bottom = [a + b for a, b in zip(bottom, counts)]
axes[0].set_xticks([0, 1], ["Baseline", "Enabled"])
axes[0].set_ylabel("Fixed curve requests")
axes[0].set_ylim(0, 34)
axes[0].set_title("All 28 outcomes retained", fontsize=10)
axes[0].legend(fontsize=7, loc="upper right")
rows = [r for r in data["rows"] if r["after"] == "Root"]
xs = list(range(len(rows)))
axes[1].plot(xs, [r["before_nodes"] for r in rows], "o-", color="#1c5f94", label="Baseline", markersize=5)
axes[1].plot(xs, [r["after_nodes"] for r in rows], "x--", color="#d87d2c", label="Enabled", markersize=5)
axes[1].set_title("13 completed curve roots: nodes unchanged", fontsize=10)
axes[1].set_xlabel("Completed input index (data maps ids)")
axes[1].set_ylabel("Recorded search nodes")
axes[1].grid(alpha=.2)
axes[1].legend(fontsize=8)
small = next(r for r in data["small"] if r["id"] == "conditional-root")
values = [small["before_nodes"], small["after_nodes"]]
axes[2].bar([0, 1], values, color=["#1c5f94", "#d87d2c"])
for i, value in enumerate(values):
    axes[2].text(i, value + .08, str(value), ha="center", fontsize=10)
axes[2].set_xticks([0, 1], ["Baseline", "Enabled"], fontsize=8)
axes[2].set_ylabel("Recorded search nodes")
axes[2].set_ylim(0, 4)
axes[2].set_title("Synthetic Fp29 control", fontsize=10)
fig.suptitle("Branch-affine v1: cumulative-model caps are incomplete traces, never gains", fontsize=11)
fig.tight_layout()
for ext in ("svg", "pdf", "png"):
    fig.savefig(here / ("outcomes." + ext), dpi=150)
svg = here / "outcomes.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
