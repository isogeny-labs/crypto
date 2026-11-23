#!/usr/bin/env python3
"""Lay out recorded coupling excess E = D_T - D_S (rounds 1-3) as excess.svg. Values are transcribed
from the logs cited in README.md tables; no computation."""
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

# (m, E, complete?) at the smallest measured t per m (t=4; m=6 at t=5)
pts = [(2, 0, True), (3, 2, True), (4, 3, True), (5, 5, False), (6, 6, False)]
fig, ax = plt.subplots(figsize=(6, 4))
ax.plot([p[0] for p in pts], [p[1] for p in pts], "-", color="#2e75b6", alpha=0.5)
for m, e, done in pts:
    ax.plot(m, e, "o" if done else "^", color="#2e75b6", mfc="#2e75b6" if done else "none", ms=9)
ax.plot([4, 5, 6], [3, 5, 6], ls=":", color="#c0504d", lw=1)
ax.annotate("pre-registered limits P2", (5.05, 5.1), color="#c0504d", fontsize=8)
ax.set_xlabel("arity m (summands per relation)")
ax.set_ylabel("coupling excess E = D_T - D_S (D_S = 4)")
ax.set_xticks([2, 3, 4, 5, 6])
ax.set_title("Tree encoding: excess degree from the separators\n(open ^ = lower bound: cut off by time or 13.5 GiB memory)", fontsize=9)
fig.tight_layout()
fig.savefig("excess.svg")
