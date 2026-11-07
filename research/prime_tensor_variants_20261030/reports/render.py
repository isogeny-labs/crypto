#!/usr/bin/env python3
"""Layout only: plot recorded native counts and render a source-linked report.

No curve arithmetic, research computations, experiment harness or verifier.
"""
from pathlib import Path
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst

here = Path(__file__).resolve().parent
data = json.loads((here.parent / "packet-unit/mode-records/summary.json").read_text())
plt.rcParams.update({"svg.hashsalt": "prime-ternary-20261030", "font.size": 11})
fig, ax = plt.subplots(figsize=(8.5, 3.1), layout="constrained")
ax.bar([0, 1, 2], data["wrong_unsplit_mismatches_by_precision"], width=.5,
       color="#c14338", label="Faulty unsplit factorization")
ax.scatter([0, 1, 2], [0, 0, 0], s=65, marker="o", color="#16705a",
           label="Correct target-specific factorization", zorder=3)
ax.set(xticks=[0, 1, 2], xticklabels=["mod 3", "mod 9", "mod 27"],
       ylabel="Mismatching fixed cases (out of 64)", ylim=(-1, 18),
       title="A structural defect appears only at higher precision")
ax.legend(loc="upper left", fontsize=9, frameon=False)
ax.set_yticks([0, 4, 8, 12, 16])
ax.grid(axis="y", alpha=.2)
ax.spines[["top", "right"]].set_visible(False)
fig.savefig(here / "mode-mismatches.svg", metadata={"Date": None})
svg = here / "mode-mismatches.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
fig.savefig(here / "mode-mismatches.png", dpi=150)
plt.close(fig)
typst.compile(str(here / "report.typ"), output=str(here / "report.pdf"), root=str(here.parent))
