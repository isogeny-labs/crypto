"""Render recorded native checkpoint counts; no arithmetic research code."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
data = json.loads((here / "plot-data.json").read_text())
rows = data["rows"]
fig, ax = plt.subplots(figsize=(7.8, 3.0))
bars = ax.bar([r["label"] for r in rows], [r["records"] for r in rows],
              color=["#9ba9bb", "#7aa7ad", "#227b78"])
ax.bar_label(bars, padding=4)
ax.set_ylabel("Validated saved candidate records")
ax.set_title("R04 · checkpoint records, including repeated candidates")
ax.set_ylim(0, data["axis_max"])
ax.spines[["top", "right"]].set_visible(False)
ax.grid(axis="y", alpha=0.15)
ax.set_axisbelow(True)
fig.tight_layout()
fig.savefig(here / "checkpoint-records.svg")
fig.savefig(here / "checkpoint-records.png", dpi=150)
svg = here / "checkpoint-records.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
