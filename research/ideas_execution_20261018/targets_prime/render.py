"""Render recorded native CSV values only; no experiment or result computation."""
import csv
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
with (root / "runs/run-20261018-native-v1/summary.csv").open() as f:
    rows = list(csv.DictReader(f))
with (root / "runs/run-20261018-strict4-v1/summary.csv").open() as f:
    strict = list(csv.DictReader(f))
fig, axes = plt.subplots(1, 3, figsize=(12, 3.6))
for seed, marker in [(1, "o"), (2, "s"), (3, "^"), (4, "v"), (5, "D"), (6, "x")]:
    selected = [r for r in rows if r["item"] == "1" and r["seed"] == str(seed)]
    axes[0].scatter([int(r["p"]) for r in selected], [int(r["base_points"]) for r in selected], marker=marker, label=f"seed {seed}")
    selected = [r for r in rows if r["item"] == "14" and r["seed"] == str(seed)]
    axes[1].scatter([int(r["p"]) for r in selected], [int(r["hits"]) for r in selected], marker=marker)
    selected = [r for r in strict if r["seed"] == str(seed)]
    axes[2].scatter([int(r["p"]) for r in selected], [int(r["natural_hits"]) for r in selected], marker=marker)
axes[0].set(title="Fp² fixture base", xlabel="Base-field prime p", ylabel="Projected factor-base points")
axes[0].legend(fontsize=7, ncol=2)
axes[1].set(title="Ordered four-point decompositions", xlabel="Prime-field p", ylabel="Verified hits / 32 natural targets")
axes[2].set(title="Four distinct abscissae", xlabel="Prime-field p", ylabel="Verified hits / 32 natural targets")
for ax in axes:
    ax.grid(alpha=.2)
axes[1].set_xscale("log")
axes[2].set_xscale("log")
fig.suptitle("Native fixture/decomposition diagnostics · 20261018 · operation counts only", fontsize=11)
fig.tight_layout()
dest = root / "reports"
dest.mkdir(exist_ok=True)
for ext in ["svg", "png", "pdf"]:
    fig.savefig(dest / f"recorded-values.{ext}", dpi=150)
