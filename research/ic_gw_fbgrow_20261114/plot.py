#!/usr/bin/env python3
"""Plots from analysis.json: DL counted ratio G/REF per size (per-seed dots, geometric mean line) and CPU ratio."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
HERE = os.path.dirname(os.path.abspath(__file__))
A = json.load(open(os.path.join(HERE, "analysis.json")))
QS = [262111, 524287, 1048507, 2097091, 4194271]
KS = [18, 19, 20, 21, 22]
col = {0.0: "#1b7837", 0.5: "#2166ac", 1.0: "#b2182b"}
fig, axs = plt.subplots(1, 3, figsize=(13, 4.2), sharey=False)
for ax, g in zip(axs[:2], ("G160", "G155")):
    for w in (0.0, 0.5, 1.0):
        gm = [A["ratios"][f"{g}/DL/{q}/w{w}/h0.1"]["geomean"] for q in QS]
        ax.plot(KS, gm, "-o", color=col[w], label=f"additions {w} M-eq (hash 0.1)")
        for k, q in zip(KS, QS):
            ax.scatter([k] * 3, A["ratios"][f"{g}/DL/{q}/w{w}/h0.1"]["per_seed"], s=8, color=col[w], alpha=.45)
    ax.axhline(1, color="k", lw=.8)
    ax.set_title(f"{g}: DL counted cost / REF")
    ax.set_xlabel("k = log2 q"); ax.set_xticks(KS); ax.grid(alpha=.3)
axs[0].set_ylabel("ratio (below 1 = growth cheaper)")
axs[0].legend(fontsize=7)
ax = axs[2]
for g, m in (("G160", "s"), ("G155", "^")):
    ax.plot(KS, [A["cells"][f"{g}/DL/{q}"]["cpu_geomean"] for q in QS], "-" + m, label=g)
    for k, q in zip(KS, QS):
        ax.scatter([k] * 3, A["cells"][f"{g}/DL/{q}"]["cpu_ratio_per_seed"], s=8, alpha=.4, color="gray")
ax.axhline(1, color="k", lw=.8); ax.set_title("total CPU ratio G / REF (L0 diagnostic)")
ax.set_xlabel("k = log2 q"); ax.set_xticks(KS); ax.grid(alpha=.3); ax.legend()
fig.suptitle(f"Incremental FB growth vs charged restart: verdict {A['headline'].upper()} [pcc:unverified]", fontsize=10)
fig.tight_layout()
fig.savefig(os.path.join(HERE, "fbgrow.svg")); fig.savefig(os.path.join(HERE, "fbgrow.png"), dpi=130)
