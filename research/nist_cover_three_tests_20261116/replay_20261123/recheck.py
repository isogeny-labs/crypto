"""Render the recorded CI recheck counts only."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
data = json.loads((root / "evidence/ci-unique-path/certificate-summary.json").read_text())
plt.rcParams["svg.fonttype"] = "none"
fig, ax = plt.subplots(figsize=(8, 1.9), layout="constrained")
labels = ["Public bindings", "Relation rows", "Rank modulo r", "Corruptions rejected"]
keys = ["known_scalar_bindings", "relation_rows", "rank_mod_r", "adapter_corruptions_rejected"]
bars = ax.bar(labels, [data[k] for k in keys], color="#28658c")
ax.bar_label(bars, padding=2)
ax.set_ylim(0, 7)
ax.set_ylabel("Recorded count")
ax.set_title("P-192 · CI 45130692659 · fixed fixtures, n=1")
ax.spines[["top", "right"]].set_visible(False)
fig.savefig(root / "recheck.svg")
fig.savefig(root / "recheck.png", dpi=150)
