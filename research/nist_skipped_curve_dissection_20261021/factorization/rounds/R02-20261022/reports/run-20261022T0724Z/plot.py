"""Layout only: graph recorded native verifier fields; perform no arithmetic."""
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
data = json.loads((here / "plot-data.json").read_text())
fig, ax = plt.subplots(figsize=(8.0, 3.4))
rows = data["rows"]
positions = [0, 1, 2]
ax.barh([p + 0.18 for p in positions], [r["before_bits"] for r in rows],
        height=0.32, label="R01 residual", color="#9ba9bb")
ax.barh([p - 0.18 for p in positions], [r["after_bits"] for r in rows],
        height=0.32, label="R02 residual", color="#227b78")
for p, row in zip(positions, rows):
    ax.text(row["before_bits"] + 5, p + 0.18, str(row["before_bits"]), va="center")
    ax.text(row["after_bits"] + 5, p - 0.18,
            str(row["after_bits"]) if row["after_bits"] else "complete", va="center")
ax.set_yticks(positions, [r["label"] for r in rows])
ax.set_xlim(0, 490)
ax.set_xlabel("Largest explicitly composite residual (bits)")
ax.set_title("R02 · three fixed degree-2 extension-twist orders", pad=36)
ax.legend(loc="upper left", bbox_to_anchor=(0, 1.14), ncol=2, frameon=False)
ax.spines[["top", "right"]].set_visible(False)
ax.grid(axis="x", alpha=0.15)
ax.set_axisbelow(True)
fig.tight_layout()
fig.savefig(here / "residual-bits.svg")
fig.savefig(here / "residual-bits.png", dpi=150)
svg = here / "residual-bits.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
