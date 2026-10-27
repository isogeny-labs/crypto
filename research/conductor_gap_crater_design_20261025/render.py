#!/usr/bin/env python3
"""Lay out recorded native analysis values only; no research computation."""
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.ticker import PercentFormatter

ROOT = Path(__file__).resolve().parent
data = json.loads((ROOT / "design.json").read_text())
frozen = json.loads((ROOT.parent / "conductor_gap_replication_20261025/analysis/panel-A.json").read_text())["levels"][0]
exact = data["enumerated_whole_class_bootstrap"]
plt.rcParams.update({"font.size": 10, "axes.spines.top": False, "axes.spines.right": False})

def save(fig, name):
    svg = ROOT / "figures" / f"{name}.svg"
    fig.savefig(svg, bbox_inches="tight")
    svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
    fig.savefig(ROOT / "figures" / f"{name}.pdf", bbox_inches="tight")
    fig.savefig(ROOT / "figures" / f"{name}.png", dpi=150, bbox_inches="tight")
    plt.close(fig)

fig, ax = plt.subplots(figsize=(7.2, 2.7))
ax.axvspan(.95, 1.05, color="#dff0e8", label="Predeclared ±5% band")
ax.axvline(1, color="#667085", lw=1)
for y, row, key, color in [(1, frozen, "ci98", "#65758b"), (0, exact, "ci98", "#174c80")]:
    ax.plot(row[key], [y, y], color=color, lw=3)
    ax.plot([row["ratio"]], [y], "o", color=color)
ax.set_yticks([1, 0], ["Frozen Monte Carlo", "Full resampling distribution"])
ax.set_xlabel("Normalized counted-work ratio S2/S1 (98% intervals)")
ax.set_xlim(.94, 1.07)
ax.set_ylim(-.65, 1.65)
ax.legend(loc="upper left", frameon=False, fontsize=8)
fig.suptitle("Crater equivalence remains inconclusive", x=.53)
fig.tight_layout()
save(fig, "crater-ci")

selected = [(8, 100, 1), (8, 100, 5), (8, 1000, 1), (8, 1000, 5), (16, 100, 5), (16, 1000, 1), (16, 1000, 2)]
lookup = {(s["classes"], s["targets_per_crater"], s["seeds_per_target"]): s for s in data["power_scenarios"]}
rows = [lookup[k] for k in selected]
labels = [f"{k} classes\n{m:,} targets, {r} seeds" for k, m, r in selected]
values = [s["approx_power_at_observed_ratio_with_estimated_heterogeneity"] for s in rows]
fig, ax = plt.subplots(figsize=(8.5, 4.8))
ax.barh(labels, values, color=["#8195ad"] * 4 + ["#22655a"] * 3)
ax.invert_yaxis()
ax.axvline(.9, color="#9a5418", ls="--", lw=1, label="Illustrative 90% planning target")
ax.set_xlim(0, 1.04)
ax.xaxis.set_major_formatter(PercentFormatter(1))
ax.set_xlabel("Projected probability of establishing equivalence")
ax.tick_params(axis="y", labelsize=9)
ax.legend(frameon=False, fontsize=8, loc="lower right")
fig.suptitle("Exploratory repetition model — projections, not measurements")
fig.text(.5, .01, "Assumes ratio 1.0111 and estimated class heterogeneity; 98% interval rule. Model uncertainty is unquantified.", ha="center", fontsize=8)
fig.tight_layout(rect=(0, .07, 1, .97))
save(fig, "repetition-power")
