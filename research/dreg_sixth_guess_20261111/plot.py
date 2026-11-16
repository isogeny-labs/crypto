#!/usr/bin/env python3
"""Guesses needed to hold the degree-4 closure, against l (m = 3): recorded bounds from the parent
studies (kstar_recorded.json of dreg_guess_scaling_20261106) plus this study's draw-1 result at l = 9."""
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
fig, ax = plt.subplots(figsize=(6.4, 4.2))
pts = [(6, 0, "0 on 8/8"), (7, 2, "1-2 on 10/10"), (8, 4, "<= 4 on 4/4"), (9, 5, "<= 5 on 3/4 (S5)")]
for l, k, lab in pts:
    ax.plot(l, k, "v", color="#2471a3", ms=9)
    ax.annotate(lab, (l, k), xytext=(6, -12), textcoords="offset points", fontsize=8)
ax.plot(9, 6, "v", color="#c0392b", ms=11, label="this study: <= 6 on draw 1 (both sixth guesses)")
ls = [6, 7, 8, 9, 10]
ax.plot(ls, [3 * (l - 7) + 1 for l in ls], "--", color="#7f8c8d", label="K_marginal: k = 3(l-7)+1")
ax.plot(ls, [3 * (l - 6) for l in ls], ":", color="#7f8c8d", label="K_excess: k = 3(l-6)")
ax.plot(ls, ls, "-", color="#d5d8dc", lw=1, label="k = l (whole x1: the pair-table exponent)")
ax.set_xlabel("l (factor-base subspace dimension), m = 3"); ax.set_ylabel("guesses k* (upper bound, ▼)")
ax.set_xlim(5.6, 10.2); ax.set_ylim(-0.5, 12); ax.grid(alpha=0.3); ax.legend(fontsize=7.5, loc="upper left")
ax.set_title("Guesses holding the MutantXL closure at degree 4", fontsize=10)
fig.tight_layout(); fig.savefig("kstar.svg"); fig.savefig("kstar.png", dpi=200)
