"""Render only values already recorded in gls_symmetry_summary_v2.csv.

No research computation or solver logic belongs here. The native Rust
summarizer generated the CSV from the frozen NDJSON record.
"""
import csv
import sys
from pathlib import Path

import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
run = sys.argv[1] if len(sys.argv) > 1 else "v2"
assert run in {"v1", "v2", "v2_aa"}
rows = list(csv.DictReader((HERE / f"gls_symmetry_summary_{run}.csv").open()))
fold = [r for r in rows if r["arm"] == "fold"]
assert len(fold) == 9
labels = [f"p={r['p']}\nB={r['b']}" for r in fold]
heights = [float(r["flat_over_arm_w0"]) for r in fold]

fig, ax = plt.subplots(figsize=(10.2, 4.5))
colors = ["#24649a" if h >= 1 else "#bf5f43" for h in heights]
ax.bar(range(9), heights, color=colors, width=0.7)
ax.axhline(1, color="#333333", linewidth=1)
ax.set_xticks(range(9), labels)
ax.set_ylim(0, max(2.0, max(heights) * 1.17))
ax.set_ylabel("flat / folded Fp² multiplication count (ratio)")
ax.set_title(f"Cold one-target GLS fold diagnostic · {run} run · three targets per cell")
for x, y in enumerate(heights):
    ax.text(x, y + 0.035, f"{y:.3f}", ha="center", fontsize=8)
fig.text(
    0.5, 0.005,
    "Exact deterministic counts; each cell repeats the same setup for three target scalars. "
    "Other operations remain unpriced.",
    ha="center", fontsize=8,
)
fig.tight_layout(rect=(0, 0.045, 1, 1))
fig.savefig(HERE / f"gls_fold_cost_{run}.svg")
fig.savefig(HERE / f"gls_fold_cost_{run}.png", dpi=150)
plt.close(fig)
