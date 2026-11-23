"""Render recorded development counts only; no cryptographic computation."""
from pathlib import Path
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst

root = Path(__file__).parent
rows = json.loads((root / "plot-data.json").read_text())["cases"]
fig, ax = plt.subplots(figsize=(8, 2.5))
x = list(range(len(rows)))
for offset, key, label, color in [(-.18, "rank_v4", "V4", "#78909c"), (.18, "rank_v5", "V5", "#1565c0")]:
    bars = ax.bar([i+offset for i in x], [r[key] for r in rows], width=.36, label=label, color=color)
    ax.bar_label(bars, fontsize=8)
ax.set_xticks(x, [f'p={r["field"]}\nt={r["target"]}' for r in rows])
ax.set_ylabel("Dimension in selector quotient")
ax.set_ylim(0, 220)
ax.legend(frameon=False)
ax.set_title("Small synthetic chain: recorded filtered-closure dimensions")
fig.tight_layout()
fig.savefig(root / "dimensions.svg")
typst.compile(str(root / "report.typ"), output=str(root / "report.pdf"))
