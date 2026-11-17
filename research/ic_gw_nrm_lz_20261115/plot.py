#!/usr/bin/env python3
"""Plots analysis.json -> nrmlz.svg / nrmlz.png. usage: plot.py"""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__))
a = json.load(open(os.path.join(H, "analysis.json")))
QS = [262111, 524287, 1048507, 2097091, 4194271]
ks = [18, 19, 20, 21, 22]
col = {0.0: "#1f77b4", 0.5: "#7570b3", 1.0: "#d95f02"}
fig, ax = plt.subplots(1, 3, figsize=(15, 4))
for j, y in enumerate(("DL", "DL1", "D")):
    for w in (0.0, 0.5, 1.0):
        for i, q in enumerate(QS):
            ps = a["ratios"][f"BOTH/{y}/{q}/{w}/0.0"]["per_seed"]
            ax[j].plot([ks[i]] * len(ps), ps, ".", color=col[w], alpha=.5)
        ax[j].plot(ks, [a["ratios"][f"BOTH/{y}/{q}/{w}/0.0"]["geomean"] for q in QS], "o-", color=col[w], label=f"BOTH w_add = {w}")
    for arm, ls in (("NRM", ":"), ("LZ", "--")):
        ax[j].plot(ks, [a["ratios"][f"{arm}/{y}/{q}/0.0/0.0"]["geomean"] for q in QS], ls, color="#555", label=f"{arm} w_add = 0")
    ax[j].axhline(1, color="k", lw=.7)
    ax[j].set_title(f"{y} cold M-eq, arm / REF (dots: seeds 1-6; lines: geomean)", fontsize=8)
    ax[j].set_xlabel("k = log2 q"); ax[j].set_xticks(ks); ax[j].legend(fontsize=7); ax[j].grid(alpha=.3)
fig.suptitle("Plane-quartic IC: NRM (normalised points) and LZ_REUSE (scalar Lanczos dots) vs BLR1, counted cost [pcc:unverified, L0]", fontsize=10)
fig.tight_layout()
for e in ("svg", "png"):
    fig.savefig(os.path.join(H, f"nrmlz.{e}"), dpi=130)
