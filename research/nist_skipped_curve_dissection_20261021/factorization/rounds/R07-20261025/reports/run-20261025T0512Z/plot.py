"""Render recorded filter-stage counts, without cryptographic computation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
rows = json.loads((here / "stages.json").read_text())["stages"]
plt.rcParams["svg.fonttype"] = "none"
fig, ax = plt.subplots(figsize=(8.5, 2.6), layout="constrained")
ax.barh([r["label"] for r in rows], [r["relations"] for r in rows],
        color=["#2457a6", "#a34d30"])
ax.set_xscale("log")
ax.set_xlim(1, 1e8)
for i, row in enumerate(rows):
    ax.annotate(f'{row["relations"]:,}', (row["relations"], i),
                xytext=(6, 0), textcoords="offset points", va="center")
ax.set(xlabel="Native relation count (logarithmic scale)",
       title="R07: singleton removal leaves 92 relations; no matrix")
ax.grid(axis="x", alpha=.2)
fig.savefig(here / "stages.svg")
p = here / "stages.svg"
p.write_text("\n".join(s.rstrip() for s in p.read_text().splitlines()) + "\n")
