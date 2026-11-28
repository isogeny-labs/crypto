"""Layout only: read native-recorded ranks; no field or scientific aggregation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
data = json.loads((root / "../../section-operator-v1/native-v1/NAIVE_COUNTEREXAMPLES.json").read_text())
cases = data["cases"]
labels = [f"{c['field']}\n{c['kind']}" for c in cases]
fig, ax = plt.subplots(figsize=(10, 3.2))
positions = list(range(len(cases)))
left = [i - 0.18 for i in positions]
right = [i + 0.18 for i in positions]
ax.bar(left, [c["naive_rank"] for c in cases], width=0.34, label="Naive rows", color="#c76e55")
ax.bar(right, [c["corrected_rank"] for c in cases], width=0.34, label="Local confluent rows", color="#3d779b")
ax.set_xticks(positions, labels, fontsize=8)
ax.set_yticks(range(7))
ax.set_ylim(0, 7.2)
ax.set_ylabel("Native matrix rank (maximum 6)")
ax.set_title("Four fixed false-positive controls: each group sum differs from target")
ax.legend(loc="upper center", ncol=2, frameon=False)
ax.spines[["top", "right"]].set_visible(False)
ax.grid(axis="y", alpha=0.15)
ax.set_axisbelow(True)
fig.tight_layout()
for suffix in ["svg", "pdf"]:
    fig.savefig(root / f"counterexample-ranks.{suffix}")
# Normalize generated SVG whitespace before freezing/hashing; no input data edits.
svg = root / "counterexample-ranks.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
