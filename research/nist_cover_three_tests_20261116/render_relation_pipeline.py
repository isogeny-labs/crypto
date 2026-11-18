"""Render recorded counts only; performs no curve or solver arithmetic."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
data = json.loads((root / "evidence/relation-e2e-v1/relations/result.json").read_text())
labels = ["Projected columns", "Collected rows", "Row rank modulo r"]
values = [data["projected_base_size"], data["relations"], data["rank_mod_r"]]
fig, ax = plt.subplots(figsize=(7.0, 2.5), layout="constrained")
ax.barh(labels, values, color=["#64748b", "#2563eb", "#0d9488"])
ax.invert_yaxis()
ax.set_xlim(0, 250)
ax.set_xlabel("Count / matrix dimension")
ax.set_title("Fixed-window P-192 relation matrix")
for i, value in enumerate(values):
    ax.text(value + 3, i, str(value), va="center")
ax.spines[["top", "right"]].set_visible(False)
fig.text(0.5, -0.03, "relation-e2e-v1; one functional run; no timing or uncertainty estimate",
         ha="center", fontsize=8)
for suffix in ("svg", "pdf"):
    fig.savefig(root / f"figures/relation-pipeline.{suffix}", bbox_inches="tight")
