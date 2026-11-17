"""Render recorded quartic follow-up values; never generate research results."""
import argparse
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt


def save(fig, output):
    svg = output.with_suffix(".svg")
    fig.savefig(svg, bbox_inches="tight")
    svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
    fig.savefig(output.with_suffix(".png"), dpi=150, bbox_inches="tight")
    plt.close(fig)


def s5(record, output):
    p = record["p"]
    labels = ["first hit", "first miss"]
    values = [row["solve"]["total_muls"] / 1e12 for row in record["rows"]]
    fig, ax = plt.subplots(figsize=(6.4, 3.8))
    bars = ax.bar(labels, values, color=["#2266aa", "#cc7744"])
    ax.bar_label(bars, labels=[f"{v:.3f}" for v in values], padding=3)
    ax.set_ylim(0, max(values) * 1.15)
    ax.set_ylabel("S5 solve base-field multiplications (10¹²)")
    ax.set_title(f"Canonical twist p={p}: two registered S5 queries")
    ax.text(0.5, -0.24, "Run: one hit and one miss; precomputation excluded from bars",
            ha="center", transform=ax.transAxes, fontsize=8)
    fig.tight_layout()
    save(fig, output)


def census(records, output):
    primes = [r["p"] for r in records]
    lines = [r["line0"] + r["line2"] for r in records]
    spaces = [r["physical_base"] for r in records]
    pairs = [r["signed_pair_entries"] for r in records]
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(10, 3.8))
    ax1.plot(primes, lines, "o-", color="#9a6028", label="two axes")
    ax1.plot(primes, spaces, "s-", color="#2266aa", label="full 2D space")
    ax1.set(xlabel="prime p", ylabel="liftable abscissae")
    ax1.legend(fontsize=8)
    ax2.plot(primes, pairs, "o-", color="#a04152")
    ax2.set_yscale("log")
    ax2.set(xlabel="prime p", ylabel="signed pair entries (count)")
    fig.suptitle("Quartic twist 2D invariant-space census · eight curves")
    fig.tight_layout()
    save(fig, output)


def p13(record, output):
    trace = record["query_trace"]
    selected = record["selected_relations"]
    x = list(range(1, len(trace) + 1))
    hits = [row[2] for row in trace]
    ranks = [row["rank"] for row in selected]
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(9, 3.7))
    ax1.bar(x, hits, color="#2266aa")
    ax1.set(xlabel="processed target", ylabel="exact signed decompositions")
    ax2.step(x, ranks, where="post", color="#a04152", marker="o")
    ax2.set(xlabel="selected relation", ylabel="matrix rank", ylim=(0, 8))
    fig.suptitle("p=13 two-dimensional exact-oracle IC · one target [271]G")
    fig.tight_layout()
    save(fig, output)


def p37(record, output):
    trace = record["query_trace"]
    selected = record["selected_relations"]
    x = list(range(1, len(trace) + 1))
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(9, 3.7))
    ax1.plot(x, [row[2] for row in trace], ".-", color="#2266aa", markersize=3)
    ax1.set(xlabel="processed residual", ylabel="exact signed decompositions")
    ax2.step(list(range(1, len(selected) + 1)), [row["rank"] for row in selected],
             where="post", color="#a04152")
    ax2.set(xlabel="selected relation", ylabel="matrix rank", ylim=(0, 55))
    fig.suptitle("p=37 full two-dimensional exact-oracle IC · one target [271]G")
    fig.tight_layout()
    save(fig, output)


def pair(record, output):
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(9, 3.8))
    labels = ["setup", "seven queries"]
    x = [0, 1]
    full_m = [record["full_setup_fp_muls"], record["full_query_fp_muls"]]
    quotient_m = [record["quotient_setup_fp_muls"], record["quotient_query_fp_muls"]]
    full_g = [record["full_setup_group_ops"], record["full_query_group_ops"]]
    quotient_g = [record["quotient_setup_group_ops"], record["quotient_query_group_ops"]]
    for ax, full, quot, ylabel in [
        (ax1, full_m, quotient_m, "base-field multiplications"),
        (ax2, full_g, quotient_g, "group operations"),
    ]:
        ax.bar([v - 0.18 for v in x], full, width=0.36, label="full table", color="#2266aa")
        ax.bar([v + 0.18 for v in x], quot, width=0.36, label="orbit quotient", color="#cc7744")
        ax.set_xticks(x, labels)
        ax.set_yscale("log")
        ax.set_ylabel(ylabel)
        ax.legend(fontsize=8)
    fig.suptitle("p=13 twist pair-table quotient · exact same seven targets")
    fig.tight_layout()
    save(fig, output)


def tagged(record, output):
    labels = ["setup", f"{len(record['queries'])} queries"]
    fields = [
        ("fp_muls", "Base-field multiplications"),
        ("group_ops", "Group operations"),
    ]
    fig, axes = plt.subplots(1, 3, figsize=(11, 3.5))
    for ax, (field, title) in zip(axes[:2], fields):
        full = [record[f"full_setup_{field}"], record[f"full_query_{field}"]]
        folded = [record[f"tagged_setup_{field}"], record[f"tagged_query_{field}"]]
        ax.bar([-0.18, 0.82], full, width=0.36, color="#2266aa", label="full")
        ax.bar([0.18, 1.18], folded, width=0.36, color="#cc7744", label="tagged")
        ax.set_xticks([0, 1], labels)
        ax.set_yscale("log")
        ax.set_title(title)
        ax.legend(fontsize=8)
    ax = axes[2]
    ax.bar(["full", "tagged"], [record["full_table_live_requested_bytes"],
            record["tagged_table_live_requested_bytes"]], color=["#2266aa", "#cc7744"])
    ax.set_title("Retained requested bytes")
    ax.ticklabel_format(axis="y", style="sci", scilimits=(0, 0))
    fig.suptitle(f"p={record['p']} · pair-table arms · {len(record['queries'])} queries · L0 counts")
    fig.tight_layout()
    save(fig, output)


def dense(record, prior, output):
    phases = ["model", "base", "input", "eigen", "map", "table", "stream", "verification"]
    def total(data, suffix):
        return sum(data.get(f"{phase}_{suffix}", 0) for phase in phases)
    fig, axes = plt.subplots(1, 3, figsize=(10, 3.5))
    for ax, values, title in [
        (axes[0], [total(prior, "fp_muls"), total(record, "fp_muls")], "Counted base-field multiplications"),
        (axes[1], [total(prior, "group_ops"), total(record, "group_ops")], "Counted group operations"),
        (axes[2], [prior["residuals"], record["residuals"]], "Residual queries"),
    ]:
        ax.bar(["one row/query", "dense + anchor"], values, color=["#2266aa", "#cc7744"])
        ax.set_title(title, fontsize=10)
        ax.tick_params(axis="x", labelrotation=20)
        if ax != axes[2]:
            ax.set_yscale("log")
    fig.suptitle(f"p={record['p']} · one public target [271]G · exact oracle · n=1 per arm")
    fig.tight_layout()
    save(fig, output)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=["s5", "census", "p13", "p37", "pair", "tagged", "dense"])
    parser.add_argument("input", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--reference", type=Path)
    args = parser.parse_args()
    if args.mode == "dense":
        if args.reference is None:
            parser.error("dense mode requires --reference")
        dense(json.loads(args.input.read_text()), json.loads(args.reference.read_text()), args.output)
    elif args.mode == "census":
        data = [json.loads(line) for line in args.input.read_text().splitlines()]
        census(data, args.output)
    else:
        data = json.loads(args.input.read_text())
        {"s5": s5, "p13": p13, "p37": p37, "pair": pair, "tagged": tagged}[args.mode](data, args.output)


if __name__ == "__main__":
    main()
