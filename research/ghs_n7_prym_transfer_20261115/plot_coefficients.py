"""Layout only: plot recorded/derived counts from figure-data.json."""
import json
from pathlib import Path

import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
data = json.loads((here / "figure-data.json").read_text())
counts = [data["classical_cubic_slice"], data["other_published_W1"],
          data["outside_published_W1"]]
labels = ["Classical genus-7 slice", "Other published W1", "Outside W1"]
colors = ["#276e8e", "#e4a33d", "#c8ced4"]
fig, ax = plt.subplots(figsize=(9, 3.2), constrained_layout=True)
left = 0
for count, label, color in zip(counts, labels, colors):
    ax.barh([0], [count], left=left, color=color, label=f"{label}: {count}")
    ax.text(left + count / 2, 0, str(count), ha="center", va="center",
            color="white" if color != "#c8ced4" else "#263238", weight="bold")
    left += count
ax.set_xlim(0, data["nonzero_b"])
ax.set_yticks([])
ax.set_xlabel("Nonzero coefficient values b in F128 (exact count)")
ax.set_title("Degree-7 GHS coefficient census at q=2")
ax.legend(ncol=3, loc="upper center", bbox_to_anchor=(0.5, -0.27), frameon=False)
fig.savefig(here / "coefficient-census.svg")
svg = here / "coefficient-census.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
