"""Presentation only: all statistics are supplied by the frozen Rust analyzer."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
data = json.loads((here / "results/summary.json").read_text())
labels = ["Graph IC\n6 variables", "Graph IC\n5 variables", "Native rho\nplain, batch16"]
colors = ["#4768a8", "#238b8b", "#c47d2d"]
fig, ax = plt.subplots(figsize=(8.4, 3.25), constrained_layout=True)
for i, arm in enumerate(data["arms"]):
    ax.bar(i, arm["median_seconds"], color=colors[i], width=0.62, alpha=0.85)
    ax.scatter([i - .12, i - .06, i, i + .06, i + .12], arm["walls"],
               color="#1e293b", s=17, zorder=3)
    ax.annotate(f'{arm["median_seconds"]:.2f} s', (i, arm["median_seconds"]),
                xytext=(0, 12), textcoords="offset points", ha="center", fontsize=10)
ax.set_xticks([0, 1, 2], labels)
ax.set_ylabel("Complete worker-process wall time (seconds)")
ax.set_ylim(bottom=0)
ax.set_title("Fixed p1009 instance: median bars and five individual cold-process calls", fontsize=11)
ax.grid(axis="y", alpha=.18)
ax.set_axisbelow(True)
ax.spines[["top", "right"]].set_visible(False)
fig.savefig(here / "wall.svg")
svg = here / "wall.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
fig.savefig(here / "wall.pdf")
plt.close(fig)

# Administrative document rendering and lossless PDF compression only.
import hashlib
import typst
import pymupdf
typst.compile(str(here / "report.typ"), output=str(here / "report.pdf"))
document = pymupdf.open(here / "report.pdf")
before = [hashlib.sha256(p.get_pixmap().samples).digest() for p in document]
compressed = document.tobytes(garbage=4, deflate=True, use_objstms=1)
document.close()
(here / "report.pdf").write_bytes(compressed)
document = pymupdf.open(here / "report.pdf")
after = [hashlib.sha256(p.get_pixmap().samples).digest() for p in document]
assert before == after
