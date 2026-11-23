#!/usr/bin/env python3
"""Plot the recorded last fall degrees of the width ladder (devcheck rows, transcribed in
README section 5) against the fitted law of Conjecture G. Plots recorded values only."""
import os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
# (t, d_F, censored?) from devcheck/README.md, m = 2, lower median of 2 draws
data = {
    1: [(2,3,0),(3,3,0),(4,4,0),(5,4,0),(6,4,0),(7,4,0),(8,5,0),(9,5,0),(10,5,0),(11,5,0)],
    2: [(2,4,0),(3,4,0),(4,4,0),(5,4,0),(6,5,0),(7,5,0),(8,5,0),(9,5,0),(10,6,0),(11,6,0),(12,6,0)],
    3: [(2,5,0),(3,5,0),(4,5,0),(5,5,0),(6,5,0),(7,5,0),(8,5,0),(9,6,0),(10,6,1)],
    4: [(2,5,0),(3,5,0),(4,5,0),(5,5,0),(6,5,0),(7,5,0),(8,5,0),(9,6,1),(10,6,1)],
}
ffd = {1: 3, 2: 4, 3: 5, 4: 5}
s = {1: 0, 2: 2, 3: 5, 4: 5}
colors = {1: "#2e8b57", 2: "#1f5fbf", 3: "#c0392b", 4: "#8e44ad"}
off = {1: -0.09, 2: -0.03, 3: 0.03, 4: 0.09}
fig, ax = plt.subplots(figsize=(8.4, 5.0))
for W, rows in data.items():
    tx = [r[0] for r in rows if not r[2]]; ty = [r[1] + off[W] for r in rows if not r[2]]
    cx = [r[0] for r in rows if r[2]]; cy = [r[1] + off[W] for r in rows if r[2]]
    ax.plot(tx, ty, "o", color=colors[W], ms=6, label=f"W = {W} measured" + (" (= tower S3)" if W == 2 else ""))
    if cx:
        ax.plot(cx, cy, "^", color=colors[W], mfc="none", ms=8, label=f"W = {W} censored lower bound")
    ts = list(range(2, 15))
    fit = [ffd[W] + max(0, (t - s[W]) // 4) + off[W] for t in ts]
    ax.step(ts, fit, where="post", color=colors[W], lw=1, ls="--", alpha=0.8)
for (t, d, W) in [(12, 6, 1), (13, 7, 3), (14, 7, 2)]:
    ax.plot([t], [d + off[W]], marker="*", color=colors[W], ms=13, mfc="white")
ax.text(11.6, 7.35, "stars: falsifying predictions\n(not yet run)", fontsize=8)
ax.set_xlabel("tower height t (m = 2, N = 2t one-bit unknowns)")
ax.set_ylabel("last fall degree d_F")
ax.set_title("Width ladder: recorded d_F and the fitted law d_F = ffd(W) + max(0, floor((t - s_W)/4))\n"
             "(rows: devcheck/*.jsonl, development checks [pcc:unverified]; dashed: Conjecture G; ffd(W) proved, Thm 4)",
             fontsize=9)
ax.set_xlim(1.5, 14.5); ax.set_ylim(2.5, 8)
ax.grid(alpha=0.3); ax.legend(fontsize=7.5, loc="upper left", ncol=2)
fig.tight_layout()
fig.savefig(os.path.join(root, "fig_growth.svg")); fig.savefig(os.path.join(root, "fig_growth.png"), dpi=150)
print("wrote fig_growth.svg/png")
