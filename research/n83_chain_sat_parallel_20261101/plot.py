"""Lay out recorded values from ANALYSIS.json (native n83_chain_sat_analyze).
Plotting only. usage: plot.py <ANALYSIS.json> <out_dir>"""
import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

a = json.loads(Path(sys.argv[1]).read_text())
out = Path(sys.argv[2])
out.mkdir(parents=True, exist_ok=True)
MS = sorted({c["m"] for c in a["cells"]})
cell = {(c["m"], c["arm"], c["kind"]): c for c in a["cells"]}
base = {m: next(c["arm"] for c in a["cells"] if c["m"] == m and not c["arm"].endswith("+parallel")) for m in MS}
arm = {"control": lambda m: base[m], "parallel": lambda m: base[m] + "+parallel"}
LABEL = {"control": "A6 indexed (merged, 1 thread)", "parallel": "A7 + parallel back enumeration (4 threads)"}
COLOR = {"control": "#888888", "parallel": "#1f77b4"}
ticks = [f"m={m}\nd={base[m].split('meet')[1].split('+')[0]}" for m in MS]

fig, axes = plt.subplots(1, 2, figsize=(12, 4.6))
w = 0.35
for i, key in enumerate(["control", "parallel"]):
    ys = [cell.get((m, arm[key](m), "public"), {}).get("refuted", 0) + cell.get((m, arm[key](m), "planted"), {}).get("verified", 0) for m in MS]
    axes[0].bar([x + (i - 0.5) * w for x in MS], ys, w, label=LABEL[key], color=COLOR[key])
axes[0].set_xticks(MS, ticks)
axes[0].set_ylim(0, 10.5)
axes[0].set_title("decided queries of 8 (3 public refuted + 5 planted verified)", fontsize=9)
axes[0].legend(fontsize=7, loc="upper center", ncol=1)
axes[0].grid(alpha=0.3, axis="y")
ax = axes[1]
for key in ("control", "parallel"):
    for kind, ls, mk in (("planted", "-", "^"), ("public", ":", "o")):
        ys = [cell.get((m, arm[key](m), kind), {}).get("median_seconds_all") for m in MS]
        ax.plot(MS, ys, ls=ls, marker=mk, color=COLOR[key], label=f"{LABEL[key]} · {kind}")
ax.axhline(120, color="k", ls=":", lw=1)
ax.text(MS[0], 122, "120 s per-query cap", fontsize=7)
ax.set_xticks(MS, ticks)
ax.set_ylim(0, 135)
ax.set_ylabel("median seconds per query (L0 host, capped)")
ax.set_title("query cost", fontsize=9)
ax.legend(fontsize=6.5, loc="center right")
ax.grid(alpha=0.3)
fig.suptitle("Chained-S3 on N83 (icv1-f2m83-t6151469093347-cdcc5432), m = 8, 9: parallel back enumeration — run n83_chain_sat_parallel_20261101", fontsize=10)
fig.tight_layout()
fig.savefig(out / "n83_chain_sat_parallel_m8_9.svg")
fig.savefig(out / "n83_chain_sat_parallel_m8_9.png", dpi=150)
print("ok")
