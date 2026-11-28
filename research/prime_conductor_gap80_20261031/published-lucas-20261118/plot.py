"""Render only recorded integer sizes and certificate status; no research arithmetic."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
summary = json.loads((root / "summary.json").read_text())
previous = json.loads((root.parent / "record-search-20261113/arithmetic.json").read_text())
status = "certificate replayed" if summary["certificate_replayed"] else "certificate incomplete"
fig, ax = plt.subplots(figsize=(8.2, 3.1), constrained_layout=True)
labels = ["Frozen target prime", "Published K-571 factor", "Fixed n = 7351 conductor"]
values = [80, previous["large_prime_bits"], summary["conductor_bits"]]
bars = ax.barh(labels, values, color=["#49779a", "#49779a", "#388a75"])
for bar, value in zip(bars, values):
    ax.text(value + 35, bar.get_y() + bar.get_height()/2, str(value), va="center")
ax.set_xlim(0, max(values) * 1.13)
ax.set_xlabel("Recorded bit length; no runtime or evaluated-crossing comparison")
ax.set_title("Conductor arithmetic only · fixed larger case: " + status)
ax.spines[["top", "right"]].set_visible(False)
ax.invert_yaxis()
reports = root / "reports"
reports.mkdir(exist_ok=True)
for ext in ("svg", "png"):
    path = reports / ("sizes." + ext)
    fig.savefig(path, dpi=170)
    if ext == "svg":
        path.write_text("\n".join(line.rstrip() for line in path.read_text().splitlines()) + "\n")
