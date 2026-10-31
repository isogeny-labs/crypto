"""Lay out recorded phase counters; no cryptographic computation."""

import argparse
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt


PHASES = ("model", "base", "input", "eigen", "map", "table", "stream", "verification")


def total(record, unit):
    return sum(record.get(f"{phase}_{unit}", 0) for phase in PHASES)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("first", type=Path)
    parser.add_argument("second", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--aa", action="store_true")
    args = parser.parse_args()
    first = json.loads(args.first.read_text())
    second = json.loads(args.second.read_text())
    labels = ["first run", "A/A"] if args.aa else ["full table", "tagged table"]
    fig, axes = plt.subplots(1, 3, figsize=(11, 3.5))
    panels = (
        ("fp_muls", "Total counted Fp multiplications", 1e6, "million"),
        ("group_ops", "Total counted group operations", 1e3, "thousand"),
    )
    for ax, (field, title, divisor, unit) in zip(axes, panels):
        values = [total(first, field) / divisor, total(second, field) / divisor]
        bars = ax.bar(labels, values, color=["#2266aa", "#cc7744"])
        ax.bar_label(bars, labels=[f"{v:.3f}" for v in values], padding=3)
        ax.set_ylim(0, max(values) * 1.18)
        ax.set(title=title, ylabel=unit)
    values = [first["table_fp_muls"] / 1e6, second["table_fp_muls"] / 1e6]
    bars = axes[2].bar(labels, values, color=["#2266aa", "#cc7744"])
    axes[2].bar_label(bars, labels=[f"{v:.3f}" for v in values], padding=3)
    axes[2].set_ylim(0, max(values) * 1.18)
    axes[2].set(title="Pair-table setup Fp multiplications", ylabel="million")
    run = "run-20261022T0758Z-quartic-2d-dense-tagged-p37"
    fig.suptitle(f"{run}{'-aa' if args.aa else ''} · p=37 · one public target · n=1/arm", fontsize=11)
    fig.tight_layout()
    svg = args.output.with_suffix(".svg")
    fig.savefig(svg, bbox_inches="tight")
    svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
    fig.savefig(args.output.with_suffix(".png"), dpi=150, bbox_inches="tight")
    plt.close(fig)


if __name__ == "__main__":
    main()
