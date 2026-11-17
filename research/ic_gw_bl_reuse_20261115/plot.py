#!/usr/bin/env python3
"""Plots analysis.json -> blreuse.svg / blreuse.png. usage: plot.py"""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__))
a = json.load(open(os.path.join(H, "analysis.json")))
QS = [262111, 524287, 1048507, 2097091, 4194271]
ks = [18, 19, 20, 21, 22]
col = {0.0: "#1f77b4", 0.5: "#7570b3", 1.0: "#d95f02"}
fig, ax = plt.subplots(1, 2, figsize=(11, 4))
for w in (0.0, 0.5, 1.0):
    for i, q in enumerate(QS):
        ps = a["ratios"][f"DL/{q}/{w}/0.0"]["per_seed"]
        ax[0].plot([ks[i]] * len(ps), ps, ".", color=col[w], alpha=.5)
    ax[0].plot(ks, [a["ratios"][f"DL/{q}/{w}/0.0"]["geomean"] for q in QS], "o-", color=col[w], label=f"w_add = {w}")
ax[0].axhline(1, color="k", lw=.7)
ax[0].set_title("DL cold M-eq, BLR / REF (dots: seeds 1-6; line: geomean)", fontsize=9)
ax[0].set_xlabel("k = log2 q"); ax[0].set_xticks(ks); ax[0].legend(fontsize=8); ax[0].grid(alpha=.3)
ax[1].plot(ks, [a["la_lmul_ratio"][str(q)]["geomean"] for q in QS], "s-", color="#1b9e77", label="linear_algebra.lmul")
for y, c in (("DL1", "#666"), ("D", "#aaa")):
    ax[1].plot(ks, [a["ratios"][f"{y}/{q}/0.0/0.0"]["geomean"] for q in QS], "o--", color=c, label=f"{y} M-eq")
ax[1].axhline(1, color="k", lw=.7)
ax[1].set_title("Component ratios BLR / REF (geomean)", fontsize=9)
ax[1].set_xlabel("k = log2 q"); ax[1].set_xticks(ks); ax[1].legend(fontsize=8); ax[1].grid(alpha=.3)
fig.suptitle("Plane-quartic IC: exact block-Lanczos gram reuse, counted cost [pcc:unverified, L0]", fontsize=10)
fig.tight_layout()
for e in ("svg", "png"):
    fig.savefig(os.path.join(H, f"blreuse.{e}"), dpi=130)
