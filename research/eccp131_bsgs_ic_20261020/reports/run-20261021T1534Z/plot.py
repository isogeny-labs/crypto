"""Render recorded benchmark values; this script does not compute results."""

import csv
import json
from collections import defaultdict
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
BENCH = HERE.parents[1] / "bench_20261021"


def exact_plot():
    with (BENCH / "exact-values.tsv").open(newline="") as stream:
        rows = list(csv.DictReader(stream, delimiter="\t"))
    arms = ["B256", "B512a", "B512b", "B1024"]
    colors = ["#1f77b4", "#ff7f0e", "#9467bd", "#2ca02c"]
    seen = defaultdict(int)
    fig, axes = plt.subplots(1, 2, figsize=(8.7, 3.3), layout="constrained")
    for row in rows:
        arm = row["arm"]
        i = arms.index(arm)
        ordinal = seen[arm]
        seen[arm] += 1
        x = i + (ordinal - 2) * 0.055
        axes[0].scatter(x, float(row["pair_ms"]), color=colors[i], s=30)
        axes[1].scatter(x, float(row["probe_ms"]), color=colors[i], s=30)
    for ax, title in zip(
        axes,
        ["First pair-table build", "Rebuilt table + 256 candidate checks"],
    ):
        ax.set_xticks(range(4), arms)
        ax.set_xlabel("Exact ECCp-131 base arm (5 runs each)")
        ax.set_ylabel("Elapsed time (ms; L0 descriptive)")
        ax.set_title(title, fontsize=10)
        ax.grid(axis="y", alpha=0.25)
    fig.suptitle("Exact probe, source 18550e9979 · 2026-10-21", fontsize=11)
    fig.savefig(HERE / "exact_stage.svg")
    fig.savefig(HERE / "exact_stage.png", dpi=150)
    plt.close(fig)


def ecbench_plot():
    path = BENCH / "ecbench-session" / "records.jsonl"
    records = [json.loads(line) for line in path.read_text().splitlines() if line]
    measured = [r for r in records if not r["warmup"]]
    arms = ["rho-neg", "ic-mitm", "rho-neg-aa"]
    colors = ["#1f77b4", "#ff7f0e", "#9467bd"]
    seen = defaultdict(int)
    fig, ax = plt.subplots(figsize=(6.7, 3.0), layout="constrained")
    for record in measured:
        arm = record["arm"]
        i = arms.index(arm)
        ordinal = seen[arm]
        seen[arm] += 1
        x = i + (ordinal - 2.5) * 0.055
        ax.scatter(x, record["cost"]["s"], color=colors[i], s=31)
    ax.set_xticks(range(3), arms)
    ax.set_ylabel("Recorded S (GAE; unpriced work remains)")
    ax.set_xlabel("Registered 20-bit prime-curve arm (6 measured records each)")
    ax.set_title("ecbench ECBS1hcc2634f59aca · L0 operation control", fontsize=10)
    ax.grid(axis="y", alpha=0.25)
    fig.savefig(HERE / "ecbench_s.svg")
    fig.savefig(HERE / "ecbench_s.png", dpi=150)
    plt.close(fig)


if __name__ == "__main__":
    exact_plot()
    ecbench_plot()
