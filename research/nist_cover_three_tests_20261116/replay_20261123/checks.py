"""Render already-recorded fixture counts; no cryptographic computation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
data = json.loads((root / "evidence/ci-replay/certificate-summary.json").read_text())
labels = ["Public scalar bindings", "Relation rows", "Rank modulo r", "Adapter corruptions rejected"]
values = [data[k] for k in ("known_scalar_bindings", "relation_rows", "rank_mod_r", "adapter_corruptions_rejected")]
plt.rcParams["svg.fonttype"] = "none"
fig, ax = plt.subplots(figsize=(8, 2.6), layout="constrained")
bars = ax.barh(labels[::-1], values[::-1], color="#28658c")
ax.bar_label(bars, padding=4)
ax.set_xlim(0, 7)
ax.set_xlabel("Recorded count (fixed fixtures; n=1, no sampling interval)")
ax.set_title("P-192 replay · CI 45128902122")
ax.spines[["top", "right"]].set_visible(False)
fig.savefig(root / "checks.svg")
fig.savefig(root / "checks.png", dpi=150)
