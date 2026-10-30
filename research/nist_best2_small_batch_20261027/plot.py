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

# GPU A/B: rate ratio and derived GPU time per solve per (curve, W)
gpu = [json.loads(l) for l in (HERE / "gpu-ab-v2.summary.jsonl").read_text().splitlines() if l.strip()]
fig, (a1, a2) = plt.subplots(1, 2, figsize=(7.4, 2.8), sharex=True)
for curve, col, off in (("P-256", "#2f6db5", -0.08), ("P-384", "#d68910", 0.08)):
    rows_c = sorted((r for r in gpu if r["curve"] == curve), key=lambda r: r["W"])
    xs = [i + off for i in range(len(rows_c))]
    a1.errorbar(xs, [r["rate_ratio_median"] for r in rows_c],
                yerr=[[r["rate_ratio_median"] - r["rate_ratio_min"] for r in rows_c],
                      [r["rate_ratio_max"] - r["rate_ratio_median"] for r in rows_c]],
                fmt="o", color=col, ms=4, capsize=2, label=curve)
    t = [r["gpu_time_per_solve"] for r in rows_c]
    lo = [r["gpu_time_per_solve_range"][0] for r in rows_c]
    hi = [r["gpu_time_per_solve_range"][1] for r in rows_c]
    a2.errorbar(xs, t, yerr=[[a - b for a, b in zip(t, lo)], [b - a for a, b in zip(t, hi)]],
                fmt="o", color=col, ms=4, capsize=2, label=curve)
for a in (a1, a2):
    a.set_xticks(range(3), ["W = 16", "W = 32", "W = 64"])
a1.axhline(1 / 1.1428, color="#c0392b", lw=1, ls=":")
a1.text(0.75, 1 / 1.1428 + 0.002, "break-even 1/1.143", color="#c0392b", fontsize=7)
a1.set_ylabel("rate, best of P±M / plain")
a1.set_title("Rate ratio (median, min–max of 3 reps)", loc="left", fontsize=8.5)
a2.axhline(1.0, color="#888888", lw=0.8)
a2.set_ylabel("GPU time per solve / plain")
a2.set_title("Derived: 1 / (1.143 × rate ratio)", loc="left", fontsize=8.5)
a2.legend(fontsize=7, frameon=False)
fig.text(0.5, 0.01, "RTX PRO 6000, PTX build, global table, 256 threads, 2048 iterations per cell; "
         "interval = iteration CI × rate min–max", ha="center", fontsize=6.5)
fig.tight_layout(rect=(0, 0.04, 1, 1))
for ext in ("svg", "png"):
    fig.savefig(HERE / "reports" / f"gpu.{ext}", dpi=160)
print("wrote reports/gpu.{svg,png}")
