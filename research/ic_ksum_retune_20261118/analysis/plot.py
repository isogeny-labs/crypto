#!/usr/bin/env python3
"""Figures from results.json and pilot/pilot_summary.json (layout only). cwd = study dir -> ratios.svg, model.svg"""
import json, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
plt.rcParams.update({"font.size": 10, "axes.spines.top": False, "axes.spines.right": False, "svg.fonttype": "none"})
C1, C2, INK, GRID = "#2a78d6", "#eb6834", "#333333", "#dddddd"
d = json.load(open("results.json")); W = ["0", "0.1", "1"]
lab = {"P24": "P24  k5 N11 -> k6 N10", "B18": "B18  k4 N7 -> k5 N7", "B24": "B24  k5 N9 -> k6 N9"}
fig, ax = plt.subplots(figsize=(9, 4.6))
for i, row in enumerate(["P24", "B18", "B24"]):
    for j, w in enumerate(W):
        x = i * 4 + j; p = d["rows"][row]["weights"][w]["pooled"]
        ax.plot([x, x], [p["lower"], p["upper"]], color=C1, lw=2, solid_capstyle="round")
        ax.plot(x, p["ratio"], "o", color=C1, ms=8, mec="white", mew=2)
        for k, q in enumerate(d["rows"][row]["weights"][w]["per_replicate"]):
            ax.plot(x + 0.25, q["ratio"], "o", color=C2, ms=4, alpha=0.9)
        ax.text(x, 0.855, f"w={w}", ha="center", color=INK, fontsize=8)
    ax.text(i * 4 + 1, 1.115, lab[row] + ("  (holds)" if d["rows"][row]["holds"] else "  (fails registered rule)"), ha="center", color=INK, fontsize=9)
ax.axhline(1, color=INK, lw=1)
ax.set_xlim(-0.7, 10.7); ax.set_ylim(0.84, 1.13); ax.set_xticks([]); ax.grid(axis="y", color=GRID)
ax.set_ylabel("cost(retuned) / cost(incumbent)")
ax.set_title("Rule R1 against the producer-sized -psr arm, counted cost, 432 paired runs per row [pcc:unverified]", fontsize=10, loc="left")
ax.plot([], [], "o", color=C1, label="pooled ratio, 95% CI (lin-t and bootstrap envelope)"); ax.plot([], [], "o", color=C2, ms=4, label="per replicate (3 sessions)")
ax.legend(loc="upper center", bbox_to_anchor=(0.5, -0.04), ncol=2, frameon=False, fontsize=8)
fig.tight_layout(); fig.savefig("ratios.svg")

s = json.load(open("pilot/pilot_summary.json"))
fig, ax = plt.subplots(figsize=(6.5, 4.6))
for row, v in s.items():
    inc_s = next(a["0"]["s_inc"] for n, a in v["arms"].items() if n != v["incumbent"])
    xs, ys = [], []
    for n, a in v["arms"].items():
        xs.append(a["model_c0"]); ys.append(inc_s if n == v["incumbent"] else a["0"]["s"])
    col = C1 if row.startswith("P") else C2
    ax.plot(xs, ys, "o", color=col, ms=5, alpha=0.8, mec="white", mew=0.8)
ax.plot([3, 26], [3, 26], color=INK, lw=1); ax.set_xscale("log"); ax.set_yscale("log")
from matplotlib.ticker import FixedLocator, NullLocator, ScalarFormatter
for a in (ax.xaxis, ax.yaxis):
    a.set_major_locator(FixedLocator([3, 4, 5, 6, 8, 10, 15, 20, 25])); a.set_minor_locator(NullLocator()); a.set_major_formatter(ScalarFormatter())
ax.set_xlabel("model C0(N, k; r) / sqrt(r)  (rule.py; excludes fixed walk set-up)"); ax.set_ylabel("observed S, w = 0 (pilot, 64 targets)")
ax.plot([], [], "o", color=C1, label="prime rows P20-P24"); ax.plot([], [], "o", color=C2, label="binary rows B15-B27"); ax.legend(frameon=False, fontsize=8)
ax.set_title("Pilot: 92 (k, N) arms, model against observation [pcc:unverified]", fontsize=10, loc="left"); ax.grid(color=GRID, lw=0.5)
fig.tight_layout(); fig.savefig("model.svg")
