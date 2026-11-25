"""Plot previously recorded counts; no polynomial computation or solver call."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
prior = {r["source_path"]: r for r in map(json.loads,
    (root / "evidence/rur_record_audit_v1/audit.jsonl").read_text().splitlines())}
records = list(map(json.loads,
    (root / "evidence/msolve_input_audit_v1/audit.jsonl").read_text().splitlines()))
plt.rcParams.update({"svg.fonttype": "none", "font.size": 10})
fig, ax = plt.subplots(figsize=(7.5, 2.6), layout="constrained")
for offset, record, label, color in zip(
        [-0.18, 0.18], records, ["Older output", "Newer output"], ["#2563a6", "#bb4b32"]):
    counts = [prior[record["source_path"]]["form_valid_roots"],
              record["effective_form_valid_assignments"],
              record["effective_equation_valid_assignments"]]
    bars = ax.bar([i + offset for i in range(3)], counts, width=0.35, label=label, color=color)
    ax.bar_label(bars, padding=3)
ax.set_xticks(range(3), ["Literal form: A - t0", "Parsed form: A - t0 + a1 + a0", "Eight original equations"])
ax.set_ylabel("Assignments satisfying check (of 13)")
ax.set_ylim(0, 17)
ax.set_yticks([0, 1, 5, 10, 13])
ax.set_title("The effective form accounts for the older output's form residuals")
ax.legend(loc="upper left", ncol=2, frameon=False)
ax.spines[["top", "right"]].set_visible(False)
fig.savefig(root / "msolve-input-residuals.pdf")
fig.savefig(root / "msolve-input-residuals.svg")
svg = root / "msolve-input-residuals.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
