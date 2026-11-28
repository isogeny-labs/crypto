"""Plot recorded native counts and lay out the report; no research arithmetic."""
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
fig, ax = plt.subplots(figsize=(7.8, 2.1), layout="constrained")
labels = [r["target"] for r in rows]
ax.barh(labels, [r["equal"] for r in rows], color="#c18c42", label="Equal")
ax.barh(labels, [r["different"] for r in rows], left=[r["equal"] for r in rows], color="#4f8b6b", label="Different")
for r in rows:
    ax.text(12500, r["target"], "12208 differ /109 equal", va="center", fontsize=8)
ax.set_xlim(0, 17000)
ax.set_xticks([0, 4000, 8000, 12317])
ax.set_xlabel("Source rational classes · wrong uniform norm · full37 /3c1cb7")
ax.spines[["top", "right"]].set_visible(False)
ax.legend(frameon=False, ncol=2, loc="upper center", bbox_to_anchor=(0.5, 1.2))
fig.savefig(p / "counts.svg")
plt.close(fig)
for f in p.glob("*.svg"):
    f.write_text("\n".join(line.rstrip() for line in f.read_text().splitlines()).rstrip() + "\n")
typst.compile(str(p / "report.typ"), output=str(p / "report.pdf"))
