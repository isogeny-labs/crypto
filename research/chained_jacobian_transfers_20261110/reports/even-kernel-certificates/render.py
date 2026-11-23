"""Render recorded geometric counts and the report; no research arithmetic."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst

p = Path(__file__).resolve().parent
subprocess.run(["dot", "-Tsvg", str(p / "maps.dot"), "-o", str(p / "maps.svg")], check=True)
rows = json.loads((p / "counts.json").read_text())["rows"]
plt.rcParams.update({"svg.fonttype": "none", "font.family": "DejaVu Sans", "font.size": 9})
fig, ax = plt.subplots(figsize=(7.8, 1.9), layout="constrained")
labels = [r["name"] for r in rows]
ax.barh(labels, [r["equal"] for r in rows], color="#4f8b6b", label="Equal")
ax.barh(labels, [r["different"] for r in rows], left=[r["equal"] for r in rows], color="#c18c42", label="Different")
ax.set_xlim(0, 49)
ax.set_xticks([0, 7, 21, 42, 49])
ax.set_xlabel("Geometric E0[7] source points · full38 /9f7c80 · complete49-input population")
ax.spines[["top", "right"]].set_visible(False)
ax.legend(frameon=False, ncol=2, loc="upper center", bbox_to_anchor=(0.5, 1.28))
fig.savefig(p / "counts.svg")
plt.close(fig)
for file in p.glob("*.svg"):
    file.write_text("\n".join(line.rstrip() for line in file.read_text().splitlines()).rstrip() + "\n")
typst.compile(str(p / "report.typ"), output=str(p / "report.pdf"))
