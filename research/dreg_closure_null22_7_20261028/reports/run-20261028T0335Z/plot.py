"""Layout only.  Draws the cells in plot-data.json, which the native scorer
produces from the committed records:

    target/release/examples/dreg_score null-plot-data > plot-data.json

Nothing is classified or counted here: each bar is one cell's
`refuting` out of `of` draws, as computed in Rust."""
import json
import os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
cells = json.load(open(os.path.join(here, "plot-data.json")))

colour = {"null": "#8a8a8a", "this": "#d1495b", "g3": "#e0a030", "pos": "#2e86ab"}
fig, ax = plt.subplots(figsize=(9.5, 4.6))
for i, c in enumerate(cells):
    frac = c["refuting"] / c["of"] if c["of"] else 0
    ax.barh(i, frac, color=colour[c["kind"]], edgecolor="black", linewidth=0.8)
    ax.text(frac + 0.015, i, f'{c["refuting"]} of {c["of"]}', va="center", fontsize=9)
ax.set_yticks(range(len(cells)))
ax.set_yticklabels([c["label"] for c in cells], fontsize=8)
ax.invert_yaxis()
ax.set_xlim(0, 1.15)
ax.set_xlabel("fraction of draws on which the degree-4 closure refutes", fontsize=9)
ax.axvline(0.5, color="black", lw=0.5, ls=":")
ax.set_title("Draws on which the degree-4 closure refutes, by cell\n"
             "research/dreg_closure_null22_7_20261028, run 2026-10-28T03:35Z", fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(here, "null.svg"))
fig.savefig(os.path.join(here, "null.png"), dpi=150)
