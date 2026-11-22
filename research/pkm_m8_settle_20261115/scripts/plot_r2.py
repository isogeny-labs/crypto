#!/usr/bin/env python3
"""Plot D_PC intervals [L, U] and msolve D (reading A) vs N for m = 8, 9, 10. Usage: plot_r2.py analysis.json OUTPREFIX"""
import json
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
import matplotlib.ticker  # noqa: E402

d = json.load(open(sys.argv[1]))
fig, axes = plt.subplots(1, 3, figsize=(12, 3.8), sharey=True)
for ax, m in zip(axes, (8, 9, 10)):
    cs = sorted((v for v in d["cells"].values() if v["m"] == m), key=lambda v: v["N"])
    for v in cs:
        N, L, U, A = v["N"], v["L"], v["U"], v["D_A"]
        if L is not None and U is not None and L == U:
            ax.plot([N], [L], marker="s", ms=13, color="#1f5f99", alpha=0.8)
        elif L is not None and U is not None:
            ax.plot([N, N], [L, U], color="#1f5f99", lw=6, solid_capstyle="butt", alpha=0.8)
        elif L is not None:
            ax.annotate("", xy=(N, L + 0.8), xytext=(N, L), arrowprops=dict(arrowstyle="->", color="#1f5f99", lw=2))
            ax.plot([N], [L], marker="_", ms=14, mew=3, color="#1f5f99")
        elif U is not None:
            ax.plot([N], [U], marker="_", ms=14, mew=3, color="#999999")
        if A is not None:
            ax.plot([N], [A], marker="o", ms=5, color="#c0392b", zorder=5)
    rb = d["readingB"][str(m)]
    ra = d["readingA"][str(m)]
    ax.set_title(f"m = {m}: A {ra['verdict']}, B {rb['verdict']}", fontsize=10)
    ax.set_xlabel("N (tower bits)")
    ax.grid(alpha=0.3)
    ax.xaxis.set_major_locator(matplotlib.ticker.MaxNLocator(integer=True))
axes[0].set_ylabel("degree")
axes[0].plot([], [], color="#1f5f99", lw=6, label="D_PC: square = value L=U, arrow = L only")
axes[0].plot([], [], "o", color="#c0392b", label="msolve D, both draws exact")
axes[0].legend(fontsize=7, loc="upper left")
axes[0].set_ylim(5.5, 9.5)
fig.suptitle("PKM Kummer chain, p = 2013265921: certified PC-degree bounds (r7/m8 round 2) [pcc:unverified]", fontsize=10)
fig.tight_layout()
for ext in ("svg", "png", "pdf"):
    fig.savefig(f"{sys.argv[2]}.{ext}", dpi=130)
print("ok")
