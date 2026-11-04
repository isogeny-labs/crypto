#!/usr/bin/env python3
"""Lay out recorded coupling excess D-4 per cell (round 7 README tables) as round7.svg; no computation."""
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
cells = ["m3 t4", "m3 t5", "m4 t4", "m4 t5", "m5 t4"]
# (value, complete?) per arm; None = not run
data = {
    "T (cherry tree)": [(2, 1), (2, 1), (3, 1), (3, 0), (5, 0)],
    "E1 (Edwards v)": [(2, 1), (2, 1), (3, 1), (3, 0), None],
    "E1Q (2-torsion quotient)": [(1, 1), (1, 1), (1, 1), (2, 1), None],
    "RR (Riemann-Roch)": [(0, 1), (1, 1), None, None, (3, 0)],
}
colors = {"T (cherry tree)": "#2e75b6", "E1 (Edwards v)": "#7f7f7f",
          "E1Q (2-torsion quotient)": "#4f9a4f", "RR (Riemann-Roch)": "#c0504d"}
fig, ax = plt.subplots(figsize=(8, 4))
w = 0.2
for j, (arm, vals) in enumerate(data.items()):
    for i, v in enumerate(vals):
        if v is None: continue
        e, done = v
        ax.bar(i + (j - 1.5) * w, e, width=w, color=colors[arm], fill=bool(done),
               edgecolor=colors[arm], hatch="" if done else "//", label=arm if i == 0 or (arm.startswith("RR") and i == 0) else None)
for arm in data:
    ax.bar([], [], color=colors[arm], label=None)
handles = [plt.Rectangle((0, 0), 1, 1, color=colors[a]) for a in data]
ax.legend(handles, list(data), fontsize=8, loc="upper left")
ax.set_xticks(range(len(cells))); ax.set_xticklabels(cells)
ax.set_ylabel("coupling excess D - 4 (hatched = lower bound)")
ax.set_title("Round 7: the 2-torsion quotient in the Edwards v-coordinate lowers the excess (msolve, p = 786433)", fontsize=9)
fig.tight_layout(); fig.savefig("round7.svg")
