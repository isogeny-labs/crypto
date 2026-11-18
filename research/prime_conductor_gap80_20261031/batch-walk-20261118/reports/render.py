"""Plot recorded native counters only; no arithmetic or research execution."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parents[1]
summary = json.loads((root / "summary.json").read_text())
cases = [("10009", 17, 50, "Degree 10009 · block 17"),
         ("10009", 64, 50, "Degree 10009 · block 64"),
         ("10009", 200, 18, "Degree 10009 · B=18 · block 200"),
         ("19", 1, 2, "Degree 19 · block 1")]
fig, axes = plt.subplots(2, 2, figsize=(10, 6.5), layout="constrained")
for ax, (degree, block, baby, title) in zip(axes.flat, cases):
    records = sorted([row for row in summary["pipeline_counts"]
                      if row["ell"] == degree and row["block"] == block
                      and row["baby_count"] == baby],
                     key=lambda row: row["walk_backend"]["batch_cap"])
    values = [int(row["walk_backend"]["batch_inverse_calls"]) for row in records]
    bars = ax.bar(["1", "7", "64", "4096"], values, color="#177d9b")
    ax.bar_label(bars, padding=3)
    ax.set_title(title)
    ax.set_xlabel("Configured normalization batch cap (points)")
    ax.set_ylabel("Recorded batch inversion calls")
    ax.set_ylim(0, max(values) * 1.24)
    ax.spines[["top", "right"]].set_visible(False)
fig.suptitle("Small development controls · same outputs, different batch caps", fontsize=13)
fig.savefig(Path(__file__).with_name("normalization.svg"))
fig.savefig("/tmp/conductor-batch-normalization.png", dpi=150)
