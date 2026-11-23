"""Plot recorded counts and render reports; no scientific execution or arithmetic."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
import typst

ROOT = Path(__file__).resolve().parent
data = json.loads((ROOT / "actual/PLOT_INPUT_TRANSCRIPTION.json").read_text())
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10})
fig, ax = plt.subplots(figsize=(9.2, 3.6), layout="constrained")
x = np.arange(4)
for i, (label, color) in enumerate(zip(
        ["Good (4,4)", "Mixed (4,2,2)", "Dual (2,2,2,2)"],
        ["#246a9b", "#c45d27", "#8596a2"])):
    bars = ax.bar(x + (i-1)*0.24, [r["stable"][i] for r in data["rows"]],
                  width=0.23, color=color, label=label)
    ax.bar_label(bars, padding=3, fontsize=9)
ax.set_xticks(x, [r["cycle"] for r in data["rows"]])
ax.set_ylabel("Stable route / lift incidences (count)")
ax.set_ylim(0, 38000)
ax.spines[["top", "right"]].set_visible(False)
ax.legend(loc="upper center", bbox_to_anchor=(0.5, 1.16), ncol=3, frameon=False)
ax.set_axisbelow(True)
ax.grid(axis="y", alpha=0.2)
fig.savefig(ROOT / "actual/stable-incidences.svg", bbox_inches="tight")
svg = ROOT / "actual/stable-incidences.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
fig.savefig(ROOT / "actual/stable-incidences.png", dpi=150, bbox_inches="tight")
plt.close(fig)

for p in sorted(ROOT.glob("*/flow.dot")):
    subprocess.run(["dot", "-Tsvg", str(p), "-o", str(p.with_suffix(".svg"))], check=True)
for p in sorted(ROOT.glob("*/report.typ")):
    if p.parent.name != "prospective":
        typst.compile(str(p), output=str(p.parent / "REPORT.pdf"))
