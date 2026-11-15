"""Report layout only: plot the two recorded finite-census counts."""
import csv
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
plt.rcParams.update({"font.size": 11, "svg.hashsalt": "critical-pair-20261115"})
for name in ("qualification001", "qualification002"):
    folder = root / "reports" / name
    with (folder / "counts.csv").open() as source:
        rows = list(csv.DictReader(source))
    fig, ax = plt.subplots(figsize=(6, 2.6), layout="constrained")
    values = [int(row["targets"]) for row in rows]
    bars = ax.bar([row["status"] for row in rows], values, color=["#21765c", "#a14c39"])
    ax.bar_label(bars, padding=3)
    ax.set_ylim(0, 14)
    ax.set_yticks(range(0, 13, 2))
    ax.set_ylabel("Targets (full 12-target census)")
    ax.set_title(name + "; fixed F19 auxiliary control")
    ax.spines[["top", "right"]].set_visible(False)
    path = folder / "completion.svg"
    fig.savefig(path, metadata={"Date": "2026-11-15"})
    path.write_text("\n".join(line.rstrip() for line in path.read_text().splitlines()) + "\n")
    plt.close(fig)
