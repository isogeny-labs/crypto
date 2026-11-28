"""Layout only: plot recorded Rust representation counts; no research computation."""
import csv
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
with (root / "counts.csv").open() as stream:
    rows = list(csv.DictReader(stream))
fig, ax = plt.subplots(figsize=(6.5, 2.8), layout="constrained")
values = [int(row["variables"]) for row in rows]
bars = ax.bar([row["k"] for row in rows], values, color=["#829daf", "#245a78"], width=0.5)
ax.set_yscale("log")
ax.set_ylim(10000, 20000000)
ax.set_ylabel("Program variables (log scale)")
ax.set_xlabel("k; original q = 213,156,431 held fixed")
ax.set_title("Recorded representation sizes; no exponent fit", fontsize=11)
for bar, value in zip(bars, values):
    ax.annotate(f"{value:,}", (bar.get_x() + bar.get_width() / 2, value),
                xytext=(0, 5), textcoords="offset points", ha="center", fontsize=10)
ax.spines[["top", "right"]].set_visible(False)
fig.savefig(root / "counts.svg")
fig.savefig(root / "counts.pdf")
plt.close(fig)
# Normalize renderer whitespace before the report's first hash/claim.
svg = root / "counts.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
