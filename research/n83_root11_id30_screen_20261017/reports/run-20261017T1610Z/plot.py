#!/usr/bin/env python3
"""Plot recorded screen summaries; preserve censored and unrun statuses."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
summary = json.loads((HERE / "../../results/summary.json").read_text())
row = summary["rows"][0]
plt.rcParams.update({"font.size": 11, "svg.fonttype": "none"})
fig, axes = plt.subplots(1, 2, figsize=(9.5, 3.8), layout="constrained")
values = [summary["baseline_k3072_geometric_target_ms"], row["geometric_target_ms"]]
axes[0].bar(["K3072\nprior control", "K4096\nID30"], values, color=["#176b87", "#927a3d"], width=0.55)
for i, value in enumerate(values):
    axes[0].text(i, value + 0.4, f"{value:.3f}", ha="center")
axes[0].set_ylim(0, 27)
axes[0].set_ylabel("Geometric target-online interval (ms)")
axes[0].set_title("Same 128 public points; local diagnostic")
ratio = row["paired_geometric_speedup_vs_k3072"]
low, high = row["descriptive_95_interval_vs_k3072"]
axes[1].errorbar([ratio], [0], xerr=[[ratio-low], [high-ratio]], fmt="o", color="#176b87", capsize=6)
axes[1].axvline(1, color="#6b7280", linestyle="--")
axes[1].set_xlim(0.4, 1.15)
axes[1].set_ylim(-0.6, 0.6)
axes[1].set_yticks([])
axes[1].set_xlabel("K3072 / K4096 paired geometric ratio")
axes[1].set_title("Recorded descriptive 95% interval")
axes[1].text(ratio, 0.16, f"{ratio:.6f}", ha="center")
axes[1].text(0.775, -0.3, "Below 1: K4096 takes longer", ha="center", fontsize=10)
for ax in axes:
    ax.spines[["top", "right"]].set_visible(False)
fig.savefig(HERE / "observations.svg")
fig.savefig(HERE / "observations.png", dpi=150)
