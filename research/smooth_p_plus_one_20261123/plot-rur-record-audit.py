"""Plot recorded counts only; no polynomial or relation computation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
records = [json.loads(line) for line in
           (root / "evidence/rur_record_audit_v1/audit.jsonl").read_text().splitlines()]
plt.rcParams.update({"svg.fonttype": "none", "font.size": 10})
fig, ax = plt.subplots(figsize=(7.5, 2.65), layout="constrained")
keys = ["equation_valid_roots", "form_valid_roots", "all_residuals_zero_roots"]
labels = ["Eight original equations", "Recorded fixed form", "Both"]
for offset, record, label, color in zip(
        [-0.18, 0.18], records, ["Older output", "Newer output"], ["#2563a6", "#bb4b32"]):
    bars = ax.bar([i + offset for i in range(3)], [record[k] for k in keys],
                  width=0.35, label=label, color=color)
    ax.bar_label(bars, padding=3)
ax.set_xticks(range(3), labels)
ax.set_ylim(0, 15)
ax.set_yticks([0, 1, 5, 10, 13])
ax.set_ylabel("Roots satisfying the check (of 13)")
ax.set_title("Saved F127 parametrizations: degree equality does not certify residuals")
ax.legend(loc="upper right", frameon=False)
ax.spines[["top", "right"]].set_visible(False)
fig.savefig(root / "rur-record-residuals.pdf")
fig.savefig(root / "rur-record-residuals.svg")
svg = root / "rur-record-residuals.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
