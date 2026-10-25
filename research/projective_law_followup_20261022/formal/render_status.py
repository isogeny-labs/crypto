#!/usr/bin/env python3
"""Render the recorded N6 formal-boundary status table."""

from __future__ import annotations

import csv
from pathlib import Path

import matplotlib.pyplot as plt


ROOT = Path(__file__).resolve().parent


def main() -> None:
    with (ROOT / "formal-status.csv").open(newline="") as handle:
        rows = list(csv.DictReader(handle))

    labels = [row["item"] for row in rows][::-1]
    values = [int(row["count"]) for row in rows][::-1]
    colors = ["#2f855a" if row["class"] == "mechanized" else "#c05621" for row in rows][::-1]

    fig, ax = plt.subplots(figsize=(10.8, 6.2))
    bars = ax.barh(labels, values, color=colors, edgecolor="#243447", linewidth=0.6)
    ax.bar_label(bars, labels=[str(value) for value in values], padding=4, fontsize=10)
    ax.set_xlim(0, 3.45)
    ax.set_xlabel("Named obligations in the packet")
    fig.suptitle(
        "N6 formal proof boundary",
        x=0.39,
        y=0.96,
        ha="left",
        fontsize=16,
        weight="bold",
    )
    fig.text(
        0.39,
        0.905,
        "Green: kernel checked at stated scope   Orange: explicitly not established",
        fontsize=10,
        color="#334155",
    )
    ax.text(
        0,
        -0.18,
        "Counts describe the inventory only; they do not measure proof effort or universal completeness.",
        transform=ax.transAxes,
        fontsize=9,
        color="#475569",
    )
    ax.grid(axis="x", color="#cbd5e1", linewidth=0.7, alpha=0.7)
    ax.spines[["top", "right", "left"]].set_visible(False)
    ax.tick_params(axis="y", length=0)
    fig.subplots_adjust(left=0.39, right=0.96, top=0.82, bottom=0.2)
    svg_path = ROOT / "formal-status.svg"
    fig.savefig(svg_path, metadata={"Date": None})
    svg_path.write_text(
        "\n".join(line.rstrip() for line in svg_path.read_text().splitlines()) + "\n"
    )
    fig.savefig(ROOT / "formal-status.png", dpi=150, metadata={"Software": "matplotlib"})
    plt.close(fig)


if __name__ == "__main__":
    main()
