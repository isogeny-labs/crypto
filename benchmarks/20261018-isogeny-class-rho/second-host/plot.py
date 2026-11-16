#!/usr/bin/env python3
"""Render descriptive second/original aggregate ns-per-iteration ratios."""
import csv
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parents[3]
ORIGINAL = ROOT / "research/isogeny-class-rho/runs/run-20261018T0857Z"
SECOND = ROOT / "research/isogeny-class-rho-n45/second-host"
CLASSES = ("P1", "P0", "B1", "K")


def aggregate(path):
    total_ns = total_iters = 0
    with path.open(newline="") as f:
        for row in csv.DictReader(f):
            total_ns += int(row["ns"])
            total_iters += int(row["iters"])
    return total_ns / total_iters


ratios = [aggregate(SECOND / c / "class/trials.csv") /
          aggregate(ORIGINAL / c / "class/trials.csv") for c in CLASSES]
fig, ax = plt.subplots(figsize=(8, 4.6), constrained_layout=True)
bars = ax.bar(CLASSES, ratios, color="#4776a8")
ax.axhline(1.0, color="#b33a3a", linestyle="--", linewidth=1.2, label="ratio = 1")
ax.set_ylabel("aggregate ns per iteration ratio (second / original)")
ax.set_title("Second-host replay: descriptive ratios only")
ax.text(0.01, -0.23,
        "Different VMs; timing ratios are descriptive only. "
        "22,700 matched rows had identical work counters.",
        transform=ax.transAxes, fontsize=8, va="top")
ax.set_ylim(0, max(1.15, max(ratios) * 1.15))
ax.legend(frameon=False)
for bar, ratio in zip(bars, ratios):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height()+0.02,
            f"{ratio:.4f}", ha="center", va="bottom", fontsize=9)
for ext in ("svg", "png"):
    fig.savefig(Path(__file__).with_name(f"second_host_ns_per_iter.{ext}"),
                dpi=160 if ext == "png" else None)
