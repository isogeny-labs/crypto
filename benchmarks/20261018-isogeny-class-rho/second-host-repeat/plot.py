#!/usr/bin/env python3
"""Same-host repeat noise: aggregate ns/iter of each run relative to the original host (descriptive)."""
import csv
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parents[3]
ORIG = ROOT / "research/isogeny-class-rho/runs/run-20261018T0857Z"
H2 = ROOT / "research/isogeny-class-rho-n45"
RUNS = {"run 1": H2 / "second-host", "run 2": H2 / "second-host-repeat/r2", "run 3": H2 / "second-host-repeat/r3"}
CLASSES = ("P0", "K")


def agg(p):
    ns = it = 0
    with p.open(newline="") as f:
        for r in csv.DictReader(f):
            ns += int(r["ns"]); it += int(r["iters"])
    return ns / it


fig, ax = plt.subplots(figsize=(7, 4.4), constrained_layout=True)
w = 0.25
colors = ["#4776a8", "#d98a2b", "#5b9a6a"]
for i, (name, base) in enumerate(RUNS.items()):
    vals = [agg(base / c / "class/trials.csv") / agg(ORIG / c / "class/trials.csv") for c in CLASSES]
    xs = [j + (i - 1) * w for j in range(len(CLASSES))]
    bars = ax.bar(xs, vals, w, label=name, color=colors[i])
    for x, v in zip(xs, vals):
        ax.text(x, v + 0.01, f"{v:.3f}", ha="center", fontsize=8)
ax.axhline(1.0, color="#b33a3a", ls="--", lw=1.1)
ax.set_xticks(range(len(CLASSES))); ax.set_xticklabels(CLASSES)
ax.set_ylabel("aggregate ns/iter, this host / original host")
ax.set_title("Same-host repeats on the second VM (descriptive)")
ax.set_ylim(0, 1.15); ax.legend(frameon=False, loc="lower right")
fig.text(0.01, -0.02, "Shared cloud VMs, back-to-back runs; work counters identical in all runs.", fontsize=8)
for ext in ("svg", "png", "pdf"):
    fig.savefig(Path(__file__).with_name(f"second_host_repeat.{ext}"), dpi=160 if ext == "png" else None, bbox_inches="tight")
