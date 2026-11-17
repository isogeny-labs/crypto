#!/usr/bin/env python3
"""Draw figures/overhead.svg from callgrind/totals.tsv and wall/wall.json.

Layout only: every number plotted is one the Callgrind run or
`examples/ecbench_child_overhead.rs` recorded.  Run: python3 -I figures/plot.py
"""
import csv
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.join(HERE, "..")
ARMS = ["before", "after", "main", "head"]
COLORS = {"before": "#8c959f", "after": "#54aeff", "after-aa": "#9ecbff", "main": "#d4a72c", "head": "#0969da"}
INK, MUTED, GRID = "#1f2328", "#59636e", "#d8dee4"
INPUTS = [("008-rho-neg", "rho.negation"), ("012-bsgs-neg", "bsgs.negation"), ("001-ic-mitm", "ic.pipeline")]


def log_x(v, lo, hi, x0, w):
    return x0 + (math.log10(v) - math.log10(lo)) / (math.log10(hi) - math.log10(lo)) * w


def bars(out, x0, y0, w, title, rows, lo, hi, ticks, fmt, unit):
    """rows: [(group label, [(arm, value, ci or None)])]"""
    label_w, bar_h, gap = 120, 11, 10
    out.append(f'<text x="{x0}" y="{y0}" font-size="13" font-weight="600" fill="{INK}">{title}</text>')
    top = y0 + 14
    plot_w = w - label_w - 70
    gx = lambda v: log_x(v, lo, hi, x0 + label_w, plot_w)
    height = sum(len(r[1]) * bar_h + gap for r in rows)
    for t in ticks:
        x = gx(t)
        out.append(f'<line x1="{x:.1f}" y1="{top}" x2="{x:.1f}" y2="{top + height}" stroke="{GRID}"/>')
        out.append(f'<text x="{x:.1f}" y="{top + height + 13}" font-size="10" fill="{MUTED}" text-anchor="middle">{fmt(t)}</text>')
    out.append(f'<text x="{x0 + label_w + plot_w}" y="{top + height + 26}" font-size="10" fill="{MUTED}" text-anchor="end">{unit} (log scale)</text>')
    y = top
    for label, items in rows:
        out.append(f'<text x="{x0}" y="{y + len(items) * bar_h / 2 + 4:.1f}" font-size="11" fill="{INK}">{label}</text>')
        for arm, v, ci in items:
            x1 = gx(lo)
            out.append(f'<rect x="{x1:.1f}" y="{y + 1}" width="{gx(v) - x1:.1f}" height="{bar_h - 2}" fill="{COLORS[arm]}"/>')
            if ci:
                out.append(f'<line x1="{gx(ci[0]):.1f}" y1="{y + bar_h / 2:.1f}" x2="{gx(ci[1]):.1f}" y2="{y + bar_h / 2:.1f}" stroke="{INK}" stroke-width="1"/>')
            out.append(f'<text x="{gx(v) + 4:.1f}" y="{y + bar_h - 2}" font-size="9" fill="{MUTED}">{arm} {fmt(v)}</text>')
            y += bar_h
        y += gap
    return y - y0 + 34


def main():
    with open(os.path.join(ROOT, "callgrind", "totals.tsv")) as f:
        ir = {(r["input"], r["arm"]): int(r["total_Ir"]) for r in csv.DictReader(f, delimiter="\t")}
    with open(os.path.join(ROOT, "wall", "wall.json")) as f:
        wall = json.load(f)
    W = 760
    out = []
    y = 66
    rows = [(name, [(a, ir[(stem, a)], None) for a in ARMS]) for stem, name in INPUTS]
    y += bars(out, 16, y, W - 32, "A. Instructions per measured child (Callgrind, whole process)", rows,
              1e6, 1e9, [1e6, 1e7, 1e8, 1e9], lambda v: f"{v / 1e6:.1f}M" if v < 1e9 else f"{v / 1e9:.0f}G", "instructions")
    groups = [("all 18 inputs", wall["pooled"]),
              ("ic.pipeline (001)", wall["by_input"]["001-ic-mitm.json"]),
              ("ic.pipeline (010)", wall["by_input"]["010-ic-mitm.json"])]
    rows = [(name, [(a["label"], a["median_ns"] / 1e6, [c / 1e6 for c in a["median_ci95_ns"]])
                    for a in arms]) for name, arms in groups]
    y += 10
    y += bars(out, 16, y, W - 32, "B. Wall time per child, spawn to exit: median, 95% interval (bootstrap over 30 rounds)", rows,
              1, 100, [1, 2, 5, 10, 20, 50, 100], lambda v: f"{v:g} ms" if v >= 1 and v == int(v) else f"{v:.2f} ms", "milliseconds")
    H = y + 30
    head = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica, Arial, sans-serif">',
        f'<rect width="{W}" height="{H}" fill="#ffffff"/>',
        f'<text x="16" y="22" font-size="15" font-weight="700" fill="{INK}">ecbench exec: the cost of one measured child</text>',
        f'<text x="16" y="40" font-size="11" fill="{MUTED}">before 1b74250e9 · after 0f5d9c272 (#2184) · after-aa (same binary) · main 004b792b7 · head 645666691 (this PR); solves identical across binaries</text>',
    ]
    foot = (f'<text x="16" y="{H - 12}" font-size="10" fill="{MUTED}">smoke-prime.json, 20-bit curve; shared 4-vCPU Xeon VM, L0, wall descriptive; '
            f'data: callgrind/totals.tsv, wall/wall.json</text>')
    with open(os.path.join(HERE, "overhead.svg"), "w") as f:
        f.write("\n".join(head + out + [foot, "</svg>"]) + "\n")


main()
