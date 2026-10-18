#!/usr/bin/env python3
"""s_vs_dimension.svg: S = gae/√r of every arm against the factor-base
dimension, at n = 23 (sessions/n23) and n = 31 (sessions/n31), from the
measured (non-warm-up) verified records. m = 2 pair-table arms, the m = 3
one-link inherited-F4 arms, and the rho reference as a horizontal band.
Values in s_vs_dimension.diagram.json. Run from the repository root with the
research-visuals venv.
"""
import json
import math
import os
import re
import statistics as st

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
INK, INK2, MUTED, GRID = "#0b0b0b", "#52514e", "#8a8984", "#e6e5e1"
COL = {("m2", 23): "#2a78d6", ("m2", 31): "#1b4f99", ("m3", 23): "#eb6834", ("rho", 23): "#1baf7a", ("rho", 31): "#117a52"}


def load(n):
    rows = {}
    r = None
    for line in open(os.path.join(HERE, "sessions", f"n{n}", "records.jsonl")):
        x = json.loads(line)
        if x["warmup"] or x["outcome"]["status"] != "verified":
            continue
        r = x["workload"]["curve"]["r"]
        rows.setdefault(x["arm"], []).append(x["cost"]["total_gae"] / math.sqrt(r))
    return r, {a: (st.mean(v), min(v), max(v), len(v)) for a, v in rows.items()}


data = {}
fig, ax = plt.subplots(figsize=(9, 5.4))
for n in (23, 31):
    r, arms = load(n)
    data[n] = {"r": r, "arms": {a: dict(mean=m, min=lo, max=hi, runs=k) for a, (m, lo, hi, k) in arms.items()}}
    for fam, pat, label in (("m2", r"ic-m2-d(\d+)$", "m = 2 pair table (chain-free)"), ("m3", r"ic-m3-d(\d+)-f4$", "m = 3 one-link chain, inherited F4")):
        pts = sorted((int(mm.group(1)), arms[a]) for a in arms if (mm := re.match(pat, a)))
        if not pts:
            continue
        xs = [d for d, _ in pts]; ys = [v[0] for _, v in pts]
        lo = [v[0] - v[1] for _, v in pts]; hi = [v[2] - v[0] for _, v in pts]
        ax.errorbar(xs, ys, yerr=[lo, hi], fmt="o-" if fam == "m2" else "s--", color=COL[(fam, n)], lw=1.6, ms=5,
                    capsize=3, label=f"n = {n}: {label}", zorder=3)
    rho = arms.get("rho-strong")
    if rho:
        ax.axhspan(rho[1], rho[2], color=COL[("rho", n)], alpha=0.18, zorder=1)
        ax.axhline(rho[0], color=COL[("rho", n)], lw=1.2, zorder=2)
        ax.text(14.3, rho[0] * (1.9 if n == 31 else 0.55), f"rho-strong, n = {n}: S = {rho[0]:.2f}", va="center",
                fontsize=8, color=COL[("rho", n)])
# the n/2 marks
for n, c in ((23, COL[("m2", 23)]), (31, COL[("m2", 31)])):
    ax.axvline(n / 2, color=c, lw=0.8, ls=":", zorder=1)
    ax.text(n / 2, 1.2e5, f"n/2 = {n / 2:g} (n = {n})", rotation=90, va="top", ha="right", fontsize=7.5, color=c)
ax.set_yscale("log")
ax.set_ylim(0.3, 2e5)
ax.set_xlim(7.5, 17.5)
ax.set_xticks(range(8, 17, 2))
ax.set_xlabel("Frobenius-invariant factor-base dimension (|F| = 2^dim signed points)", fontsize=9, color=INK2)
ax.set_ylabel("S = gae / √r, every phase charged (log scale)", fontsize=9, color=INK2)
ax.grid(axis="y", color=GRID, lw=0.8)
for sp in ("top", "right"):
    ax.spines[sp].set_visible(False)
ax.tick_params(colors=INK2, labelsize=8.5)
ax.legend(loc="upper left", fontsize=7.8, frameon=False)
fig.text(0.02, 0.985, "Chain-free (m = 2) against one-link (m = 3) Koblitz index calculus by base dimension (lead L2, 2026-10-18)",
         ha="left", va="top", fontsize=11, color=INK, fontweight="bold")
fig.text(0.02, 0.945, "icv1-f2m23-t5197-69e76b73 and icv1-f2m31-tm90707-c95f16f5; mean of the verified measured runs, bars = min…max; "
         "bands = the rho reference's range", ha="left", va="top", fontsize=8, color=INK2)
fig.text(0.01, 0.005, "Counted group operations (ecbench, L0). The m = 2 cost is the folded pair table, "
         "×14–16 per +2 in dimension. Neither curve is on the IC leaderboard.", fontsize=7.2, color=MUTED, va="bottom")
fig.tight_layout(rect=(0, 0.03, 1, 0.92))
fig.savefig(os.path.join(HERE, "s_vs_dimension.svg"), format="svg")
fig.savefig(os.path.join(HERE, "s_vs_dimension.png"), dpi=150)
json.dump({"schema": "s_vs_dimension/v1", "sessions": data}, open(os.path.join(HERE, "s_vs_dimension.diagram.json"), "w"), indent=1)
print("wrote s_vs_dimension.svg/.png/.diagram.json")
