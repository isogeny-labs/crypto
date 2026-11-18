"""Layout only: load already-recorded native equation-revision counts."""
from pathlib import Path
import json
import tarfile
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
archive = here.parent.parent / "worklist-solver-v1/native-v1.tar.xz"
with tarfile.open(archive) as tree:
    recorded = json.load(tree.extractfile("native-v1/controls.json"))
panels = [recorded["chain"], recorded["actual_q1_k2"]["original"], recorded["actual_q1_k2"]["indexed"]]
labels = ["Reverse64 chain", "Actual q1 original", "Actual q1 indexed"]
fig, ax = plt.subplots(figsize=(8.0, 2.65), layout="constrained")
for i, (label, reports) in enumerate(zip(labels, panels)):
    for j, report in enumerate(reports):
        value = report["counts"]["equation_revisions"]
        at = i + (-0.19 if j == 0 else 0.19)
        ax.bar(at, value, width=0.34, color=["#607c91", "#218974"][j], label=report["schedule"] if i == 0 else None)
        ax.text(at, value + 110, str(value), ha="center", fontsize=9)
ax.set_xticks(range(3), labels)
ax.set_ylabel("Equation revisions (count)")
ax.set_ylim(0, 9200)
ax.spines[["top", "right"]].set_visible(False)
ax.legend(frameon=False, ncol=2)
fig.suptitle("Fixed v1 diagnostics; not time, complete work or live memory", fontsize=11)
fig.savefig(here / "revisions.svg")
fig.savefig(here / "revisions.pdf")
