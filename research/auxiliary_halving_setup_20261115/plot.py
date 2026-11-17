"""Render recorded qualification values only; no research computation."""
from pathlib import Path
import csv
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

study = Path(__file__).resolve().parent
with (study / "setup-counts.tsv").open() as handle:
    rows = list(csv.DictReader(handle, delimiter="\t"))
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10})
plt.rcParams["svg.hashsalt"] = "aux-halving-qualification001"
fig, ax = plt.subplots(figsize=(7.8, 3.5), layout="constrained")
bars = ax.bar([row["p"] for row in rows],
              [int(row["curve_trials"]) for row in rows], color="#3f7899")
ax.bar_label(bars, padding=3)
ax.set_ylim(0, 22)
ax.set_xlabel("Prime field p")
ax.set_ylabel("Curve trials (count)")
ax.set_title("Order-eight subgroup and trace-good coset\none fixed-seed control per field")
ax.spines[["top", "right"]].set_visible(False)
ax.text(0, -0.25, "qualification001; seed 1234; unfiltered setup; n=1 per field\n"
        "Functional controls; no statistical interval or scaling fit.",
        transform=ax.transAxes, fontsize=8)
svg = study / "setup-counts.svg"
fig.savefig(svg, metadata={"Date": "2026-11-15"})
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
