"""Lay out recorded values only; arithmetic and checks live in Rust/Lean."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst

p = Path(__file__).resolve().parent
subprocess.run(["dot", "-Tsvg", str(p / "transfers.dot"), "-o", str(p / "transfers.svg")], check=True)
data = json.loads((p / "counts.json").read_text())
plt.rcParams.update({"svg.fonttype": "none", "font.family": "DejaVu Sans", "font.size": 9})
fig, ax = plt.subplots(figsize=(7.8, 1.65), layout="constrained")
rows = data["rows"]
ax.barh([r["model"] for r in rows], [r["finite_points"] for r in rows], color="#4f8b6b")
for r in rows:
    ax.text(r["finite_points"] + 2, r["model"], "120 finite; 0 at infinity", va="center")
ax.set_xlim(0, 165)
ax.set_xticks([0, 40, 80, 120])
ax.set_xlabel("F101 rational curve points · exact native full02 / 925b02")
ax.spines[["top", "right"]].set_visible(False)
fig.savefig(p / "counts.svg")
plt.close(fig)
for f in p.glob("*.svg"):
    f.write_text("\n".join(line.rstrip() for line in f.read_text().splitlines()).rstrip() + "\n")
typst.compile(str(p / "report.typ"), output=str(p / "report.pdf"))
