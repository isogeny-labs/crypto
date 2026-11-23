"""Render recorded values; Rust/Lean supplies all arithmetic and checks."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst

p = Path(__file__).resolve().parent
subprocess.run(["dot", "-Tsvg", str(p / "transfers.dot"), "-o", str(p / "transfers.svg")], check=True)
rows = json.loads((p / "counts.json").read_text())["rows"]
plt.rcParams.update({"svg.fonttype": "none", "font.family": "DejaVu Sans", "font.size": 9})
fig, ax = plt.subplots(figsize=(7.8, 1.6), layout="constrained")
labels = [r["model"] for r in rows]
ax.barh(labels, [r["finite"] for r in rows], color="#4f8b6b", label="Finite")
ax.barh(labels, [r["infinity"] for r in rows], left=[r["finite"] for r in rows], color="#ba8840", label="Infinity")
for r in rows:
    ax.text(122, r["model"], f'{r["finite"]} + {r["infinity"]}', va="center")
ax.set_xlim(0, 140)
ax.set_xticks([0, 40, 80, 120])
ax.set_xlabel("F101 rational curve points · exact full02 / f04008")
ax.spines[["top", "right"]].set_visible(False)
ax.legend(frameon=False, ncol=2, loc="upper center", bbox_to_anchor=(0.5, 1.28))
fig.savefig(p / "counts.svg")
plt.close(fig)
for f in p.glob("*.svg"):
    f.write_text("\n".join(line.rstrip() for line in f.read_text().splitlines()).rstrip() + "\n")
typst.compile(str(p / "report.typ"), output=str(p / "report.pdf"))
