#!/usr/bin/env python3
"""D vs N per arity from out/analysis.json (exact = filled, censored lower bound = open triangle). [pcc:unverified]"""
import json
import os
import sys

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

out = sys.argv[1]
a = json.load(open(os.path.join(out, "analysis.json")))
cols = {"6": "#1b6ca8", "7": "#c0392b", "8": "#27ae60", "9": "#8e44ad", "10": "#d68910"}
fig, ax = plt.subplots(figsize=(8, 4.8))
for i, (m, L) in enumerate(a["ladders"].items()):
    off = (i - 2) * 0.06
    ex = [(r["N"], r["D_cell"]) for r in L["rungs"] if r["status"] == "exact"]
    ce = [(r["N"], r["D_lb"]) for r in L["rungs"] if r["status"] == "censored" and r["D_lb"] is not None]
    ax.plot([x for x, _ in ex], [y + off for _, y in ex], "-o", color=cols[m],
            label=f"m={m}: {L['verdict']} (sigma_hat={L['sigma_hat']})")
    if ce:
        ax.plot([x for x, _ in ce], [y + off for _, y in ce], "^", mfc="none", color=cols[m], ms=9)
ax.set_xlabel("N = total tower bits (sum of per-summand depths)")
ax.set_ylabel("msolve F4 max degree D (min over 2 draws)")
ax.set_title("PKM Kummer chain, P1 = 2013265921: D vs N (open triangle = censored, D >= value)")
ax.set_yticks([5, 6, 7, 8, 9])
ax.grid(alpha=0.3)
ax.legend(fontsize=8, loc="lower right")
fig.tight_layout()
for ext in ("svg", "png", "pdf"):
    fig.savefig(os.path.join(out, f"D_vs_N.{ext}"), dpi=130)
print("ok")
