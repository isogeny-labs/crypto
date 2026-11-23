"""Document layout only: plot already-recorded fixed comparison counts."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
record = json.loads((here.parent.parent / "trace-lift-v1/native-v1/controls.json").read_text())
labels = [f"p{r['p']}, q{r['q']}, {r['labels']} labels" for r in record["finite"]]
values = [r["comparisons"] for r in record["finite"]]
fig, ax = plt.subplots(figsize=(7.1, 2.3), layout="constrained")
bars = ax.barh(labels[::-1], values[::-1], color="#315c82")
ax.bar_label(bars, fmt="%.0f", padding=4, fontsize=9)
ax.set_xlabel("Complete field-pair comparisons (counts; not work or timing)")
ax.set_xlim(0, max(values) * 1.2)
ax.spines[["top", "right"]].set_visible(False)
ax.set_title("Fixed equivalence coverage — native v1 / PCC-20261116-35f196", fontsize=10)
for ext in ("svg", "pdf"):
    fig.savefig(here / f"coverage.{ext}")
# Normalize renderer path whitespace BEFORE report hashing; mathematical data unchanged.
p = here / "coverage.svg"
p.write_text("\n".join(line.rstrip() for line in p.read_text().splitlines()) + "\n")
