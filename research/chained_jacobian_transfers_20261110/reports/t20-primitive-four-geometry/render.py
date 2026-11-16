"""Render recorded branch orbits and the report; no research arithmetic."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst
p = Path(__file__).resolve().parent
subprocess.run(["dot", "-Tsvg", str(p / "quotient.dot"), "-o", str(p / "quotient.svg")], check=True)
rows = json.loads((p / "counts.json").read_text())["rows"]
plt.rcParams.update({"svg.fonttype": "none", "font.family": "DejaVu Sans", "font.size": 9})
fig, ax = plt.subplots(figsize=(7.7, 1.8), layout="constrained")
labels = [r["name"] for r in rows]
ax.barh(labels, [r["first_orbit"] for r in rows], color="#4f8b6b", label="First orbit")
ax.barh(labels, [r["second_orbit"] for r in rows], left=[r["first_orbit"] for r in rows], color="#c18c42", label="Second orbit")
ax.set_xlim(0, 6)
ax.set_xticks([0, 1, 2, 3, 4, 5, 6])
ax.set_xlabel("Branch points per Frobenius orbit · full39 /84454e · complete6-root populations")
ax.spines[["top", "right"]].set_visible(False)
ax.legend(frameon=False, ncol=2, loc="upper center", bbox_to_anchor=(0.5, 1.3))
fig.savefig(p / "counts.svg")
plt.close(fig)
for file in p.glob("*.svg"):
    file.write_text("\n".join(line.rstrip() for line in file.read_text().splitlines()).rstrip() + "\n")
typst.compile(str(p / "report.typ"), output=str(p / "report.pdf"))
