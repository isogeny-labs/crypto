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
# Warmup lines in run order S1, P1, S2, P2 (record paths name the process).
procs = {}
for w in a["warmups"]:
    procs.setdefault((w.get("parallel_build", False)), []).append(w)
fig, axes = plt.subplots(1, 2, figsize=(12, 4.4))
for ax, depth in zip(axes, (5, 4)):
    labels, values, colors = [], [], []
    seq = [w["seconds"] for w in a["warmups"] if w["meet_depth"] == depth and not w.get("parallel_build")]
    par = [w["seconds"] for w in a["warmups"] if w["meet_depth"] == depth and w.get("parallel_build")]
    for k, v in enumerate(seq):
        labels.append(f"S{k + 1}\n1 thread"); values.append(v); colors.append("#888888")
    for k, v in enumerate(par):
        labels.append(f"P{k + 1}\n4 threads"); values.append(v); colors.append("#1f77b4")
    bars = ax.bar(labels, values, color=colors)
    for b, v in zip(bars, values):
        ax.text(b.get_x() + b.get_width() / 2, v, f"{v:.1f} s", ha="center", va="bottom", fontsize=8)
    n = next(w["front_fingerprints"] for w in a["warmups"] if w["meet_depth"] == depth)
    ax.set_title(f"depth-{depth} set build ({n:,} fingerprints), wall s", fontsize=9)
    ax.set_ylabel("seconds (L0 4-vCPU host)")
    ax.grid(alpha=0.3, axis="y")
fig.suptitle("Chained-S3 on N83 (icv1-f2m83-t6151469093347-cdcc5432), m = 9: parallel fingerprint-set build — run n83_chain_sat_parbuild_20261101", fontsize=10)
fig.tight_layout()
fig.savefig(out / "n83_chain_sat_parbuild_m9.svg")
fig.savefig(out / "n83_chain_sat_parbuild_m9.png", dpi=150)
print("ok")
