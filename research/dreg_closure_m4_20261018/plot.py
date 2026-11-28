#!/usr/bin/env python3
"""closure_m4.svg: closure and XL ranks per degree on the (n, l, m) = (13, 5, 4)
draws, and the (13, 6, 4) draws that stopped at the column cap. Reads
runs/*.jsonl only; values also in closure_m4.diagram.json."""
import glob
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
INK, INK2, MUTED, GRID = "#0b0b0b", "#52514e", "#8a8984", "#e6e5e1"
recs = []
for f in sorted(glob.glob(os.path.join(HERE, "runs", "m4-*.jsonl"))):
    x = json.loads(open(f).read().strip().splitlines()[-1])
    peak = None
    for line in open(f[:-6] + ".log"):
        if line.startswith("peak VmHWM"):
            peak = int(line.split()[-2]) / 1e6
    recs.append(dict(cell=x["cell"], draw=x["draw"], ell=x["ell"], closure=x["closure_degree"],
                     measured_to=x["measured_to"], peak_gb=peak,
                     scan=[dict(degree=s["degree"], columns=s["columns"], xl_rank=s["xl_rank"],
                                closure_rank=s["closure_rank"], xl_refuted=s["xl_refuted"],
                                closure_refuted=s["closure_refuted"], secs=s["secs"]) for s in x["scan"]]))
fig, axes = plt.subplots(1, 2, figsize=(11, 4.6))
ax = axes[0]
for i, r in enumerate(recs):
    for s in r["scan"]:
        xo = s["degree"] + (i - 3.5) * 0.06
        c = "#2a78d6" if r["ell"] == 5 else "#eb6834"
        ax.scatter(xo, s["closure_rank"], color=c, marker="o", s=26, zorder=3)
        ax.scatter(xo, s["xl_rank"], color=c, marker="x", s=26, zorder=3)
ax.set_yscale("log")
ax.set_xticks([3, 4])
ax.set_xlabel("degree", fontsize=9, color=INK2)
ax.set_ylabel("rank (closure ●, XL ×); closure rank 1 = refuted", fontsize=9, color=INK2)
ax.axhline(1, color=MUTED, lw=0.8, ls=":")
ax.text(3.55, 1.25, "refutation (rank 1)", fontsize=7.5, color=MUTED)
ax.scatter([], [], color="#2a78d6", label="(13, 5, 4): 4 unsatisfiable draws")
ax.scatter([], [], color="#eb6834", label="(13, 6, 4): 4 draws, stopped at degree 3 (cap)")
ax.legend(fontsize=7.6, frameon=False, loc="center left")
ax.grid(axis="y", color=GRID, lw=0.8)
ax2 = axes[1]
l5 = [r for r in recs if r["ell"] == 5]
ax2.bar([str(r["draw"]) for r in l5], [r["peak_gb"] for r in l5], color="#2a78d6")
for j, r in enumerate(l5):
    t = r["scan"][-1]["secs"]
    ax2.text(j, r["peak_gb"] + 0.15, f"{t / 60:.0f} min", ha="center", fontsize=8, color=INK2)
ax2.axhline(13, color="#eb6834", lw=1, ls="--")
ax2.text(-0.4, 13.2, "13 GB cap; (13, 6) degree 4 needs ≈ 14 GB (251 176 columns)", fontsize=7.5, color="#eb6834")
ax2.set_ylim(0, 16)
ax2.set_xlabel("(13, 5, 4) draw", fontsize=9, color=INK2)
ax2.set_ylabel("peak resident GB at degree 4", fontsize=9, color=INK2)
for a in axes:
    for sp in ("top", "right"):
        a.spines[sp].set_visible(False)
    a.tick_params(colors=INK2, labelsize=8.5)
fig.text(0.02, 0.985, "Closure degree at m = 4 (lead L1, 2026-10-18): 4 on every (13, 5) draw; XL never refutes",
         ha="left", va="top", fontsize=11, color=INK, fontweight="bold")
fig.text(0.01, 0.005, "Dense MutantXL closure (examples/dreg_closure --to 4), draws from examples/dreg_ladder seed 20261016; "
         "15.7 GB host, one process at a time. Degree observations only, no cost row.", fontsize=7.2, color=MUTED, va="bottom")
fig.tight_layout(rect=(0, 0.03, 1, 0.93))
fig.savefig(os.path.join(HERE, "closure_m4.svg"), format="svg")
fig.savefig(os.path.join(HERE, "closure_m4.png"), dpi=150)
json.dump({"schema": "closure_m4/v1", "records": recs}, open(os.path.join(HERE, "closure_m4.diagram.json"), "w"), indent=1)
print(f"wrote closure_m4.svg/.png/.diagram.json from {len(recs)} records")
