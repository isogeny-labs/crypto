"""Render recorded Lean proof counts; this performs no research calculation."""
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
record = json.loads((here.parents[1] / "verification.json").read_text())
counts = record["counts"]
values = [counts["split_u_pairs_including_identity"], counts["nonsplit_u_pairs"]]
labels = ["Split u (identity included)", "Nonsplit u"]
fig, ax = plt.subplots(figsize=(6.4, 1.35), layout="constrained")
ax.barh([1, 0], values, color=["#477b9a", "#bb8742"], height=0.55)
ax.set_yticks([1, 0], labels, fontsize=8)
ax.set_xlim(0, counts["valid_reduced_polynomial_pairs"])
ax.set_xticks(range(counts["valid_reduced_polynomial_pairs"] + 1))
ax.tick_params(axis="x", labelsize=7)
ax.set_xlabel("Representations; complete reduced-pair model n=8", fontsize=8)
ax.set_title("F3, y²=x⁵−x: kernel-derived counts", fontsize=9)
for y, value in zip([1, 0], values):
    ax.text(value + 0.05, y, str(value), va="center", fontsize=8)
ax.spines[["top", "right"]].set_visible(False)
fig.savefig(here / "counts.svg")
fig.savefig(here / "counts.pdf")
fig.savefig("/workspace/scratch/mumford-count-bars.png", dpi=150)
