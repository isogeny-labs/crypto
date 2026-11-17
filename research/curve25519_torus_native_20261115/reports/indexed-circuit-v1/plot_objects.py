"""Layout recorded native representation counts; no research arithmetic."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
native = json.loads((here / "../../indexed-circuit-v1/native-v1/controls.json").read_text())
old = native["actual"]["original_stats"]
new = native["actual"]["stats"]
keys = ["variables", "equations", "terms"]
fig, ax = plt.subplots(figsize=(7.1, 2.35), layout="constrained")
a = ax.bar([-0.18, 0.82, 1.82], [old[k] for k in keys], width=0.35, label="Original", color="#547b8b")
b = ax.bar([0.18, 1.18, 2.18], [new[k] for k in keys], width=0.35, label="Augmented", color="#cf8736")
ax.bar_label(a, labels=[f"{old[k]:,}" for k in keys], padding=2, fontsize=9)
ax.bar_label(b, labels=[f"{new[k]:,}" for k in keys], padding=2, fontsize=9)
ax.set_xticks([0, 1, 2], ["Variables", "Equations", "Terms"])
ax.set_ylabel("Represented objects (count)")
ax.set_ylim(0, 8300)
ax.spines[["top", "right"]].set_visible(False)
ax.legend(frameon=False, loc="upper left")
ax.set_title("One actual six-member system: recorded construction counts", fontsize=11)
fig.savefig(here / "objects.svg", metadata={"Date": None})
fig.savefig(here / "objects.pdf", metadata={"CreationDate": None, "ModDate": None})
