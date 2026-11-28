"""Render recorded control counts and conceptual flow; no research arithmetic."""
import json
from pathlib import Path
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
data = json.loads((root.parent / "summary.json").read_text())
fig, ax = plt.subplots(figsize=(8, 3.6))
labels = ["p=6666679, degree 37", "p=223, degree 5", "p=229, degree 5"]
rows = [r["matching_rows"] for r in data["plan_row_checks"]]
ax.barh(labels, rows, color=["#326c98", "#649e87", "#649e87"])
ax.invert_yaxis()
ax.set_xlim(0, 110)
ax.set_xlabel("Exact scalar/coordinate rows matched (three curve models)")
ax.set_title("Small public input/plan compatibility controls")
for i, value in enumerate(rows):
    ax.text(value + 1, i, str(value), va="center")
ax.spines[["top", "right"]].set_visible(False)
fig.tight_layout()
fig.savefig(root / "counts.svg")
fig.savefig(root / "counts.png", dpi=150)
plt.close(fig)

fig, ax = plt.subplots(figsize=(8, 2.6))
ax.axis("off")
nodes = [(0.14, 0.72, "E / Fp\nG, sG"),
         (0.81, 0.72, "E′ / Fp\nH, sH"),
         (0.14, 0.14, "Coordinate control\nu=2: (4x, 8y)"),
         (0.81, 0.14, "ecbench plan\nexecution pending")]
for x, y, label in nodes:
    ax.text(x, y, label, ha="center", va="center",
            bbox={"boxstyle": "round,pad=0.6", "fc": "#edf3f7", "ec": "#326c98"})
for start, end in [((0.29, 0.72), (0.66, 0.72)),
                   ((0.14, 0.54), (0.14, 0.34)),
                   ((0.81, 0.54), (0.81, 0.34))]:
    ax.annotate("", xy=end, xytext=start,
                arrowprops={"arrowstyle": "->", "color": "#326c98"})
ax.text(0.47, 0.78, "explicit φ", ha="center")
fig.tight_layout()
fig.savefig(root / "flow.svg")
fig.savefig(root / "flow.png", dpi=150)
plt.close(fig)
for name in ("counts.svg", "flow.svg"):
    path = root / name
    path.write_text("\n".join(line.rstrip() for line in path.read_text().splitlines()) + "\n")
