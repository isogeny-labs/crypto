"""Lay out recorded manifest counts; no research arithmetic or execution."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
run = root / "evidence/relation-portable-v2"
values = [json.loads((run / name).read_text())["manifest_subjects"]
          for name in ("result.json", "relocated-result.json")]
fig, ax = plt.subplots(figsize=(6.8, 2.25), layout="constrained")
ax.barh(["Original checkout", "Relocated subset"], values, color=["#2563eb", "#0d9488"])
ax.set_xlim(0, 130)
ax.set_xlabel("Manifest subjects checked (count)")
ax.set_title("Portable replay artifact bindings")
for i, value in enumerate(values):
    ax.text(value + 2, i, str(value), va="center")
ax.spines[["top", "right"]].set_visible(False)
fig.text(0.5, -0.03, "relation-portable-v2; metadata qualification; no timing comparison",
         ha="center", fontsize=8)
for suffix in ("svg", "pdf"):
    fig.savefig(root / f"figures/portable-replay.{suffix}", bbox_inches="tight")
