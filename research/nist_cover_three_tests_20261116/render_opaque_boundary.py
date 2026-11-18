"""Layout recorded native RR dimensions; performs no mathematical recompute."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
record = json.loads((root / "evidence/opaque-km-boundary-v1/result.json").read_text())
values = [row["decoded"]["rr_dimension"] for row in record["cases"]]
labels = ["Canonical minus point", "Triple point", "Double + single"]
fig, ax = plt.subplots(figsize=(7.0, 2.6))
bars = ax.bar(labels, values, color=["#4779bb", "#7fa9d7", "#7fa9d7"], width=0.55)
ax.set_ylim(0, 2.6)
ax.set_yticks([0, 1, 2])
ax.set_ylabel("RR-space dimension")
ax.set_title("Recorded native opaque-KM boundary cases")
for bar, value in zip(bars, values):
    ax.text(bar.get_x() + bar.get_width() / 2, value + 0.06, str(value), ha="center")
ax.spines[["top", "right"]].set_visible(False)
fig.text(0.5, 0.015, "opaque-km-boundary-v1; producer-unverified; exact dimensions", ha="center", fontsize=8)
fig.tight_layout(rect=[0, 0.07, 1, 1])
for suffix in ("svg", "pdf"):
    fig.savefig(root / f"figures/opaque-boundary.{suffix}")
