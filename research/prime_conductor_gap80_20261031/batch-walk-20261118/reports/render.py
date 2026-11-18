"""Plot recorded native counters only; no arithmetic or research execution."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parents[1]
cases = [("blocked17", "Degree 10009 · block 17"),
         ("full64", "Degree 10009 · block 64"),
         ("explicit18", "Degree 10009 · B=18 · block 200"),
         ("small1", "Degree 19 · block 1")]
fig, axes = plt.subplots(2, 2, figsize=(10, 6.5), layout="constrained")
for ax, (name, title) in zip(axes.flat, cases):
    records = [json.loads((root / "results" / f"{name}-batch{cap}.json").read_text())
               for cap in [1, 7, 64, 4096]]
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
