"""Render recorded native results; this file performs no research experiment."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

study = Path(__file__).resolve().parent.parent
rows = json.loads((study / "summary.json").read_text())
fig, axes = plt.subplots(1, 2, figsize=(10, 4.1), constrained_layout=True)
for policy, color, marker in [("first", "#285f8f", "o"), ("min-weight", "#b05a21", "s")]:
    arm = [row["result"] for row in rows if row["result"]["policy"] == policy]
    x = [row["n_vars"] for row in arm]
    axes[0].plot(x, [row["rank"] for row in arm], marker=marker, color=color, label=policy)
    axes[1].plot(x, [row["sparse_entries_visited_by_xor"] for row in arm], marker=marker, color=color, label=policy)
for ax in axes:
    ax.set_xlabel("Boolean variables N (one target per shape)")
    ax.grid(alpha=.22)
    ax.legend(fontsize=9)
axes[0].set_ylabel("Recorded forward-echelon rank (rows)")
axes[1].set_ylabel("Recorded committed sparse entries per XOR (count)")
axes[1].set_yscale("log")
axes[0].set_title("Fixed degree D=4 row spaces")
axes[1].set_title("Deterministic legal pivot controls")
fig.suptitle("Disk-backed Boolean Macaulay passes — run 20261018 native-v2 + continuation-v1", fontsize=11)
fig.savefig(study / "reports" / "passes.svg")
fig.savefig(study / "reports" / "passes.png", dpi=150)
fig.savefig(study / "reports" / "passes.pdf")
