#!/usr/bin/env python3
"""Figure for EXP-PKM-IC-001 from results.json."""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(HERE, "results.json")))
cells = R["cells"]
labels = [f'{c["bits"]}-bit\nM={c["M"]}' for c in cells]
arms = [("small-x", "#555555"), ("kummer", "#7a3e9d"), ("random-x", "#c8553d")]
fig, (ax, bx) = plt.subplots(1, 2, figsize=(11, 4))
w = 0.26
for i, (arm, col) in enumerate(arms):
    xs = [j + (i - 1) * w for j in range(len(cells))]
    ax.bar(xs, [c[arm]["ops_median"] for c in cells], w, color=col, label=arm)
    bx.bar(xs, [c[arm]["over_rho_median"] for c in cells], w, color=col, label=arm)
for a in (ax, bx):
    a.set_xticks(range(len(cells)))
    a.set_xticklabels(labels, fontsize=7)
    a.set_yscale("log")
    a.grid(alpha=0.3, axis="y", which="both")
ax.set_ylabel("charged group operations per solve (median of 6)")
ax.set_title("whole solve: factor base shape does not matter")
bx.set_ylabel("÷ expected rho, √(πn/2)")
bx.set_title("distance to Pollard rho (theoretical reference)")
ax.legend(fontsize=7)
fig.suptitle("EXP-PKM-IC-001: Kummer-coset vs small-x vs random factor bases, prime-field pair-sum IC, m = 2",
             fontsize=9)
for ext in ("svg", "png"):
    fig.savefig(os.path.join(HERE, "figures", f"cost.{ext}"), dpi=160, bbox_inches="tight")
print("ok")
