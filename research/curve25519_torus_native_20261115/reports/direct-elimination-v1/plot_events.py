"""Layout only: plot recorded counters; no new research computation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
r = json.loads((here.parent.parent / "direct-elimination-v1/native-v1/controls.json").read_text())
c = r["actual_q1_k2"]["report"]["counts"]
fields = ["visits_reserved", "field_mul", "field_add_sub", "field_inverse", "branch_nodes", "field_sqrt"]
labels = ["Structural visits reserved", "Field multiplications", "Field additions/subtractions", "Inversion calls", "Branch nodes", "Square-root calls"]
values = [c[f] for f in fields]
fig, ax = plt.subplots(figsize=(7.2, 2.7), layout="constrained")
bars = ax.barh(labels[::-1], values[::-1], color="#315c82")
ax.set_xscale("log")
ax.set_xlim(1, 700000)
ax.bar_label(bars, fmt="%.0f", padding=4, fontsize=9)
ax.set_xlabel("Recorded category counts (log axis; different modeled events)")
ax.set_title("One q1/k2 public-root control — not a timing, RAM or cost comparison", fontsize=10)
ax.spines[["top", "right"]].set_visible(False)
for ext in ("svg", "pdf"):
    fig.savefig(here / f"events.{ext}")
p = here / "events.svg"
p.write_text("\n".join(line.rstrip() for line in p.read_text().splitlines()) + "\n")
