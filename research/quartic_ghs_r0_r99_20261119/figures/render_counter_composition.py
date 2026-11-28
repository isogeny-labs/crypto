#!/usr/bin/env python3
"""Render recorded q=223 abstract counter totals; performs no benchmark."""

from __future__ import annotations

import json
from pathlib import Path

import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter


ROOT = Path(__file__).resolve().parents[1]
INPUT = ROOT / "receipts" / "commit_bound_q223.jsonl"
OUTPUT = Path(__file__).with_name("counter_composition.svg")

ARMS = [
    ("strong-b8", "Baseline"),
    ("strong-b8-mul5", "R17 mul5"),
    ("strong-b8-walker-table", "R36 table"),
    ("strong-b8-mul5-walker-table", "R17 + R36"),
]
COUNTERS = [
    ("fq_mul", "Base-field multiplications"),
    ("fq_inv", "Base-field inversions"),
    ("fq_add", "Base-field additions"),
    ("fq_small_div", "Exact small divisions"),
]
PHASES = ("setup", "walk", "verify")


def records() -> dict[str, dict[str, object]]:
    found: dict[str, dict[str, object]] = {}
    with INPUT.open(encoding="utf-8") as handle:
        for line in handle:
            row = json.loads(line)
            if row.get("rec") == "rho" and row.get("arm") in dict(ARMS):
                found[str(row["arm"])] = row
    missing = [arm for arm, _ in ARMS if arm not in found]
    if missing:
        raise SystemExit(f"missing required arms: {', '.join(missing)}")
    return found


def total(row: dict[str, object], counter: str) -> int:
    return sum(int(row[f"{phase}.{counter}"]) for phase in PHASES)


def main() -> None:
    rows = records()
    plt.rcParams.update(
        {
            "font.family": "DejaVu Sans",
            "font.size": 9,
            "svg.fonttype": "none",
        }
    )
    colors = ["#64748b", "#2563eb", "#16a34a", "#7c3aed"]
    fig, axes = plt.subplots(2, 2, figsize=(9.0, 5.8), constrained_layout=True)
    labels = [label for _, label in ARMS]
    for axis, (counter, title) in zip(axes.flat, COUNTERS):
        values = [total(rows[arm], counter) for arm, _ in ARMS]
        bars = axis.bar(labels, values, color=colors, width=0.68)
        axis.set_title(title, loc="left", fontweight="bold")
        axis.set_ylabel("abstract counter events")
        axis.grid(axis="y", color="#e2e8f0", linewidth=0.8)
        axis.set_axisbelow(True)
        axis.tick_params(axis="x", rotation=18)
        axis.yaxis.set_major_formatter(FuncFormatter(lambda x, _: f"{x:,.0f}"))
        pad = max(values) * 0.02 if max(values) else 1
        for bar, value in zip(bars, values):
            axis.text(
                bar.get_x() + bar.get_width() / 2,
                bar.get_height() + pad,
                f"{value:,}",
                ha="center",
                va="bottom",
                fontsize=7.5,
            )
        axis.set_ylim(0, max(values + [1]) * 1.16)

    fig.suptitle(
        "q=223 deterministic control: complete setup + walk + verification",
        fontsize=13,
        fontweight="bold",
    )
    fig.text(
        0.5,
        -0.01,
        "Exact recorded operation classes; unweighted and not elapsed-time or performance evidence.",
        ha="center",
        fontsize=9,
        color="#334155",
    )
    fig.savefig(OUTPUT, format="svg", bbox_inches="tight")


if __name__ == "__main__":
    main()
