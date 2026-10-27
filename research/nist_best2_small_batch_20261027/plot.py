"""Lay out the recorded iteration summaries as SVG/PNG. Reads summary.jsonl
(reference: no negation map) and summary-vs-neg_canon.jsonl from iterations-v1;
places those numbers and computes nothing.

    python plot.py
"""
import json
import pathlib

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = pathlib.Path(__file__).resolve().parent
RUN = HERE / "iterations-v1"
load = lambda f: [json.loads(l) for l in (RUN / f).read_text().splitlines() if l.strip()]
vs_plain = {r["arm"]: r for r in load("summary.jsonl")}
vs_canon = {r["arm"]: r for r in load("summary-vs-neg_canon.jsonl")}
plt.rcParams.update({"font.size": 8.5, "svg.fonttype": "none"})
rows = [("negation map (kernel walk), vs no negation", vs_plain["neg_canon"]),
        ("negation map + best of P±M, vs no negation", vs_plain["neg_pm_score"]),
        ("best of P±M, vs the kernel walk", vs_canon["neg_pm_score"])]
fig, ax = plt.subplots(figsize=(7.2, 2.2))
for y, (lab, r) in enumerate(rows):
    lo, hi = r["ci95"]; v = r["speedup_ref_over_arm"]
    ax.errorbar(v, y, xerr=[[v - lo], [hi - v]], fmt="o", color="#2f6db5", ms=4, capsize=2)
    if "predicted" in r:
        p = r["predicted"] if y < 2 else 1.1547
        ax.plot(p, y, marker="|", color="#c0392b", ms=12, mew=2)
ax.set_yticks(range(len(rows)), [f"{l}  n={r['paired_n']}" for l, r in rows])
ax.set_ylim(-0.6, len(rows) - 0.4)
ax.axvline(1.0, color="#888888", lw=0.8)
ax.set_xlabel("steps speedup = reference mean / arm mean, paired, 95% bootstrap; red: prediction\n"
              "r = 256, 64 walks, DP 2^-8, N ≈ 2^36", fontsize=7)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(HERE / "reports" / f"iterations.{ext}", dpi=160)
print("wrote reports/iterations.{svg,png}")
