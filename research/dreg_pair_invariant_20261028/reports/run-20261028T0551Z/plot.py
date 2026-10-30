"""Layout only.  Draws plot-data.json, which the native scorer produces from
the committed records:

    target/release/examples/dreg_invariant_score --plot-data \
        research/dreg_pair_invariant_20261028/runs > plot-data.json

Nothing is classified or counted here: every number on the figure is a
field of plot-data.json, computed in Rust."""
import json
import os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
data = json.load(open(os.path.join(here, "plot-data.json")))
rows = data["rows"]
colour = {"B": "#8a8a8a", "Q": "#d1495b", "R": "#2e86ab"}
label = {"B": "B plain chain", "Q": "Q invariant p = x1·x2", "R": "R control r = x1·e1"}
cells = [("22-6", "(n, ℓ) = (22, 6)"), ("21-7", "(n, ℓ) = (21, 7)")]

fig, axes = plt.subplots(1, 2, figsize=(11, 4.6))

# Left: degree-3 closure rank over degree-3 columns, one point per draw.
ax = axes[0]
for ci, (cell, name) in enumerate(cells):
    for ai, arm in enumerate(["B", "Q", "R"]):
        pts = [r for r in rows if r["cell"] == cell and r["arm"] == arm]
        x = ci * 4 + ai
        ys = [r["rank3"] / r["cols3"] for r in pts]
        ax.scatter([x] * len(ys), ys, color=colour[arm], edgecolor="black",
                   linewidth=0.6, s=40, zorder=3,
                   label=label[arm] if ci == 0 else None)
        degs = sorted({r["degree"] for r in pts})
        ax.text(x, 0.47, "D " + "/".join(degs), ha="center", fontsize=8)
ax.set_xticks([1, 5])
ax.set_xticklabels([c[1] for c in cells], fontsize=9)
ax.set_ylim(0, 0.5)
ax.set_ylabel("degree-3 closure rank / degree-3 columns", fontsize=9)
ax.set_title("Degree 3: no arm refutes (rank 1 would be a refutation)\n"
             "D = measured closure degree per draw", fontsize=9)
ax.legend(fontsize=8, loc="lower left")

# Right: columns of the degree-4 Macaulay matrix per arm (log scale).
ax = axes[1]
for ci, (cell, name) in enumerate(cells):
    for ai, arm in enumerate(["B", "Q", "R"]):
        r = next(r for r in rows if r["cell"] == cell and r["arm"] == arm)
        cols = r["cols4"] if arm == "B" else r["cols4_needed"]
        x = ci * 4 + ai
        ax.bar(x, cols, color=colour[arm], edgecolor="black", linewidth=0.6)
        ax.text(x, cols * 1.08, f"{cols:,}", ha="center", fontsize=7, rotation=90,
                va="bottom")
ax.set_yscale("log")
ax.set_ylim(5e4, 5e6)
ax.set_xticks([1, 5])
ax.set_xticklabels([c[1] for c in cells], fontsize=9)
ax.set_ylabel("columns of the degree-4 matrix, C(N, ≤4)", fontsize=9)
ax.set_title("Degree 4: the matrix Q and R would need, against B's\n"
             "(draw 0 shown; B refutes at 4 only at (22, 6))", fontsize=9)

fig.suptitle("Summand-swap invariant on the m = 3 chained S3 descent — "
             "research/dreg_pair_invariant_20261028, runs 2026-10-28", fontsize=10)
fig.tight_layout()
fig.savefig(os.path.join(here, "invariant.svg"))
fig.savefig(os.path.join(here, "invariant.png"), dpi=150)
