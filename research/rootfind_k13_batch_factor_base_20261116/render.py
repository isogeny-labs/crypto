"""Render the source-derived inversion-call counts; do not infer timings."""

import csv
from pathlib import Path

import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
with (here / "inversion-counts.csv").open(newline="") as handle:
    rows = list(csv.DictReader(handle))

labels = [row["curve"] for row in rows]
legacy = [int(row["legacy_inversions"]) for row in rows]
batched = [int(row["batch_inversions"]) for row in rows]
x = list(range(len(rows)))
fig, ax = plt.subplots(figsize=(8, 3.8))
ax.bar([v - 0.19 for v in x], legacy, 0.38, label="Individual inversions")
ax.bar([v + 0.19 for v in x], batched, 0.38, label="Batch inversions")
ax.set_xticks(x, labels)
ax.set_yscale("log")
ax.set_ylim(0.8, 400)
ax.set_ylabel("Field inversion calls (source-derived, log scale)")
ax.set_title("Standard-subspace base lift, one build per curve")
ax.legend(frameon=False, loc="upper left")
ax.grid(axis="y", alpha=0.2)
fig.tight_layout()
fig.savefig(here / "inversion-counts.svg")
