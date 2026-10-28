"""Render recorded projective-law branch counts; no analysis here."""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent
rows = [json.loads(line) for line in (ROOT / "results/complete-law-branches.jsonl").read_text().splitlines()]
assert len(rows) == 21 and all(r["cases"] == 648 for r in rows)
fig, (ax, ax2) = plt.subplots(1, 2, figsize=(13, 7.3), sharey=True)
y = list(range(len(rows)))
left = [0] * len(rows)
for name, color in (("inverse", "#4c78a8"), ("doubling", "#f58518"), ("distinct", "#54a24b")):
    values = [r[name] for r in rows]
    ax.barh(y, values, left=left, label=name, color=color, height=0.76)
    left = [a + b for a, b in zip(left, values)]
ax.set_yticks(y, [str(r["p"]) for r in rows])
ax.invert_yaxis()
ax.set_xlim(0, 680)
ax.set_xlabel("Cases matching Cantor per curve (18 × 18 pairs × 2 scales)")
ax.set_ylabel("Base-field prime p")
ax.set_title("1+1: 13,608 / 13,608 match Cantor")
ax.legend(ncol=3, loc="upper center", bbox_to_anchor=(0.5, -0.11),
          fontsize=8, frameon=False)
ax.spines[["top", "right"]].set_visible(False)
ax2.barh(y, [r["generic_1plus2_checked"] for r in rows],
         label="generic disjoint checked", color="#54a24b", height=0.76)
ax2.barh(y, [r["cancellation_1plus2_checked"] for r in rows],
         left=[r["generic_1plus2_checked"] for r in rows],
         label="inverse cancellation checked", color="#4c78a8", height=0.76)
ax2.barh(y, [r["coincident_1plus2_checked"] for r in rows],
         left=[r["generic_1plus2_checked"] + r["cancellation_1plus2_checked"] for r in rows],
         label="2P plus other checked", color="#f58518", height=0.76)
ax2.barh(y, [r["tripling_1plus2_checked"] for r in rows],
         left=[r["generic_1plus2_checked"] + r["cancellation_1plus2_checked"] +
               r["coincident_1plus2_checked"] for r in rows],
         label="tripling checked", color="#b279a2", height=0.76)
ax2.set_xlim(0, 610)
ax2.set_xlabel("Cases per curve (18 × 16 × 2 scales)")
ax2.set_title("1+2: 12,096 / 12,096 cases match Cantor")
ax2.legend(ncol=2, loc="upper center", bbox_to_anchor=(0.5, -0.11),
           fontsize=8, frameon=False)
ax2.spines[["top", "right"]].set_visible(False)
fig.tight_layout(pad=1.5)
for ext in ("svg", "png"):
    fig.savefig(ROOT / f"complete-law-gate.{ext}", dpi=180, bbox_inches="tight")

fig2, ax3 = plt.subplots(figsize=(7.5, 7.3))
ax3.barh(y, [r["generic_2plus2_checked"] for r in rows],
         color="#54a24b", height=0.76, label="distinct generic")
ax3.barh(y, [r["doubling_2plus2_checked"] for r in rows],
         left=[r["generic_2plus2_checked"] for r in rows],
         color="#f58518", height=0.76, label="doubling")
targeted = [sum(r[key] for key in (
    "degenerate_2plus2_checked", "inverse_2plus2_checked",
    "shared_negative_2plus2_checked", "shared_positive_2plus2_checked",
    "same_u_mixed_2plus2_checked", "special_doubling_2plus2_checked",
    "degree_drop_doubling_2plus2_checked")) for r in rows]
ax3.barh(y, targeted,
         left=[r["generic_2plus2_checked"] + r["doubling_2plus2_checked"] for r in rows],
         color="#4c78a8", height=0.76, label="targeted exceptional")
ax3.set_yticks(y, [str(r["p"]) for r in rows])
ax3.invert_yaxis()
ax3.set_xlim(0, 330)
ax3.set_xlabel("Checks matching Cantor per curve (256 ordered pairs + targeted cases)")
ax3.set_ylabel("Base-field prime p")
ax3.set_title("2+2: 6,551 checked additions match Cantor")
ax3.legend(ncol=3, loc="upper center", bbox_to_anchor=(0.5, -0.11), frameon=False)
ax3.spines[["top", "right"]].set_visible(False)
fig2.tight_layout(pad=1.5)
for ext in ("svg", "png"):
    fig2.savefig(ROOT / f"complete-law-2plus2.{ext}", dpi=180, bbox_inches="tight")
