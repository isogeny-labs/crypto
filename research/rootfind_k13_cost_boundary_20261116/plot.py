#!/usr/bin/env python3
"""Draw the archived K13 solve versus outside-solve clock boundary."""

import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

plt.rcParams["svg.hashsalt"] = "rootfind-k13-cost-boundary-v1"

here = Path(__file__).resolve().parent
data = json.loads((here / "SUMMARY.json").read_text())
order = [
    "rho-strong", "rootfind-m3", "batch-m3", "batch-m3-aa",
    "f6-ic", "inherited-f4", "matrix-f5", "matrix-f4",
]
solve = [data["arms"][a]["solve_wall_ns_sum"] / 1e6 for a in order]
outside = [data["arms"][a]["outside_solve_ns_sum"] / 1e6 for a in order]
y = np.arange(len(order))

fig, ax = plt.subplots(figsize=(10.2, 5.8))
fig.patch.set_facecolor("white")
ax.barh(y - 0.18, solve, height=0.34, color="#2067a0", label="Solve clock")
ax.barh(y + 0.18, outside, height=0.34, color="#d08535", label="Process minus solve")
ax.set_yticks(y, order)
ax.invert_yaxis()
ax.set_xscale("log")
ax.set_xlim(0.05, 100000)
ax.set_xlabel("Sum across four measured rows (ms, log scale)")
ax.set_title("K13 L0 archived clock boundary", loc="left", weight="bold")
ax.grid(axis="x", alpha=0.18, which="both")
ax.set_axisbelow(True)
ax.legend(loc="upper center", ncol=2, frameon=False)
for i, (s, o) in enumerate(zip(solve, outside)):
    ax.text(s * 1.08, i - 0.18, f"{s:.3g}", va="center", fontsize=8, color="#154b76")
    ax.text(o * 1.08, i + 0.18, f"{o:.3g}", va="center", fontsize=8, color="#86500e")
fig.text(0.10, 0.005, "ECBS1hc322a24b6610 · 2 targets × 2 measured rounds · producer L0 clocks · [pcc:unverified]", fontsize=8)
fig.tight_layout(rect=(0, 0.035, 1, 0.98))
svg_path = here / "clock_boundary.svg"
fig.savefig(svg_path, metadata={"Date": None})
svg_path.write_text("\n".join(line.rstrip() for line in svg_path.read_text().splitlines()) + "\n")
fig.savefig(here / "clock_boundary.png", dpi=150)
