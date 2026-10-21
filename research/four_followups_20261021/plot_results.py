"""Render the native analyzer's frozen values; do no cryptographic arithmetic."""
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

BASE = Path("research/four_followups_20261021")
summary = json.loads((BASE / "results/matched/summary.json").read_text())
assert summary["baseline_replayed"] == 1260
rows = summary["summary"]
bits = [16, 20, 24, 28, 32, 36, 40]
arms = [
    ("fb1-j16", "full base · 16 jumps"),
    ("fb1-j8", "full base · 8 jumps"),
    ("fb1-j32", "full base · 32 jumps"),
    ("fb2-j16", "half base · 16 jumps"),
    ("fb4-j16", "quarter base · 16 jumps"),
]
fig, (ax, bx) = plt.subplots(1, 2, figsize=(12, 4.8))
for arm, label in arms:
    selected = [next(r for r in rows if r["arm"] == arm and r["slice"] == str(b)) for b in bits]
    assert all(r["targets"] == 180 and r["verified"] == 180 for r in selected)
    ax.plot(bits, [r["representative"]["candidate_over_rho"] for r in selected], marker="o", label=label)
    bx.plot(bits, [r["processed_divisors"] for r in selected], marker="o", label=label)
ax.axhline(1.0, color="black", linestyle="--", linewidth=1, label="matched rho")
ax.set(xlabel="Source subgroup order (bits)", ylabel="Cold operation-count cost / matched rho", title="Complete one-target cost")
bx.set(xlabel="Source subgroup order (bits)", ylabel="Processed divisors (count)", title="Relation-search volume")
bx.set_yscale("log")
for axis in (ax, bx):
    axis.set_xticks(bits)
    axis.grid(alpha=0.25)
handles, labels = ax.get_legend_handles_labels()
fig.legend(handles, labels, loc="lower center", bbox_to_anchor=(0.5, 0.095), ncol=3, fontsize=9)
fig.suptitle("Frozen factor-base / jump sweep · 21 curves × 60 targets per arm")
fig.text(0.5, 0.015, "Matched run 20261021T1546Z · M + 0.5S + 10I; 0.1 GAE per native logical operation · 180 paired targets at each bit size", ha="center", fontsize=8)
fig.tight_layout(rect=[0, 0.24, 1, 0.94])
for suffix in ("svg", "png"):
    fig.savefig(BASE / f"factor-base-grid.{suffix}", dpi=150, bbox_inches="tight")
plt.close(fig)
svg = BASE / "factor-base-grid.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")

# A diagram of the exact Kummer ambiguity; each of the 21 raw witnesses is
# retained in results/kummer-ambiguity.jsonl rather than recomputed here.
gate = [json.loads(line) for line in (BASE / "results/kummer-ambiguity.jsonl").read_text().splitlines()]
assert len(gate) == 21
assert all(r["order_check"] and r["arithmetic_check"] and r["same_kummer_inputs"] and r["different_kummer_outputs"] for r in gate)
fig, ax = plt.subplots(figsize=(9.2, 3.8))
ax.axis("off")
boxes = [
    (0.12, 0.67, "K(P), K(2P)"),
    (0.50, 0.67, "K(P), K(−2P)"),
    (0.12, 0.20, "K(3P)"),
    (0.50, 0.20, "K(P)"),
]
for x, y, label in boxes:
    ax.text(x, y, label, transform=ax.transAxes, ha="center", va="center",
            fontsize=15, bbox=dict(boxstyle="round,pad=0.4", facecolor="#eef4fa", edgecolor="#375a7f"))
ax.annotate("add", xy=(0.12, 0.29), xytext=(0.12, 0.56), xycoords="axes fraction",
            textcoords="axes fraction", ha="center", arrowprops=dict(arrowstyle="->"))
ax.annotate("add", xy=(0.50, 0.29), xytext=(0.50, 0.56), xycoords="axes fraction",
            textcoords="axes fraction", ha="center", arrowprops=dict(arrowstyle="->"))
ax.text(0.80, 0.67, "same unsigned inputs", transform=ax.transAxes, ha="center", fontsize=11)
ax.text(0.80, 0.20, "different outputs", transform=ax.transAxes, ha="center", fontsize=11)
ax.set_title("Kummer ordinary-addition ambiguity · exact witnesses on 21/21 transferred subgroups")
fig.text(0.5, 0.02, "Run 20261021T1534Z · r > 4; all source-order and Jacobian arithmetic checks passed", ha="center", fontsize=9)
fig.tight_layout(rect=[0, 0.07, 1, 0.94])
for suffix in ("svg", "png"):
    fig.savefig(BASE / f"kummer-ambiguity.{suffix}", dpi=150, bbox_inches="tight")
plt.close(fig)
svg = BASE / "kummer-ambiguity.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
