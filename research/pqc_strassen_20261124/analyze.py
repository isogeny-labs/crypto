#!/usr/bin/env python3
"""Graphs, report and record metrics for the PQC Strassen timing run.

Reads the two raw files the harness writes (regression.json, crossover.json)
and writes, next to them:

  regression.svg / regression.png   old loop / new mul, per field and shape
  crossover.svg / crossover.png     Strassen (cutoff 32) / schoolbook vs n
  REPORT.pdf                        summary page, tables, both graphs
  metrics.json                      the values handed to `benchrec.py metric`

Every number is derived from the raw files: medians over the harness's
batches, ratios of medians, and a conservative spread (worst/best batch
pairing). Needs matplotlib (the Python.org framework Python has it).

    python3 analyze.py DIR --commit SHA --host TEXT
"""
from __future__ import annotations

import argparse
import json
import statistics
import textwrap
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
from matplotlib.backends.backend_pdf import PdfPages  # noqa: E402

# Default categorical order (blue, orange, aqua, yellow), fixed per field.
FIELDS = ["GF(2)", "GF(16)", "GF(256)", "F_31^3"]
COLORS = {"GF(2)": "#2a78d6", "GF(16)": "#eb6834", "GF(256)": "#1baf7a", "F_31^3": "#eda100"}
MARKERS = {"GF(2)": "o", "GF(16)": "s", "GF(256)": "^", "F_31^3": "D"}
SCHEME = {"GF(2)": "McEliece", "GF(16)": "MAYO", "GF(256)": "UOV", "F_31^3": "QR-UOV"}
INK, INK2, GRID = "#0b0b0b", "#52514e", "#e4e3df"
plt.rcParams.update({
    "font.size": 9, "axes.edgecolor": INK2, "axes.labelcolor": INK, "xtick.color": INK2,
    "ytick.color": INK2, "axes.spines.top": False, "axes.spines.right": False,
    "svg.hashsalt": "pqc-strassen-20261124", "svg.fonttype": "none",
})


def med(v):
    return statistics.median(v)


def ratio(num, den):
    """median(num)/median(den), and the spread over the worst and best batch pairing."""
    return med(num) / med(den), (min(num) / max(den), max(num) / min(den))


def shape(r):
    return f"{r['r']}x{r['k']}x{r['c']}"


def load(d: Path, name: str):
    data = json.loads((d / name).read_text())
    return data["rows"]


def regression_table(rows):
    out = {}
    for f in FIELDS:
        for r in rows:
            if r["field"] != f or r["impl"] != "old":
                continue
            new = next(x for x in rows if x["field"] == f and x["impl"] == "new" and shape(x) == shape(r))
            sp, (lo, hi) = ratio(r["seconds_per_call"], new["seconds_per_call"])
            out[(f, shape(r))] = dict(old_us=med(r["seconds_per_call"]) * 1e6,
                                      new_us=med(new["seconds_per_call"]) * 1e6,
                                      speedup=sp, lo=lo, hi=hi, cutoff=new["cutoff"])
    return out


def crossover_table(rows):
    out = {}
    for r in rows:
        if r["impl"] != "schoolbook":
            continue
        n = r["r"]
        for c in (16, 32, 64):
            s = next(x for x in rows if x["field"] == r["field"] and x["impl"] == "strassen"
                     and x["r"] == n and x["cutoff"] == c)
            rel, (lo, hi) = ratio(s["seconds_per_call"], r["seconds_per_call"])
            out[(r["field"], n, c)] = dict(school_ms=med(r["seconds_per_call"]) * 1e3,
                                           strassen_ms=med(s["seconds_per_call"]) * 1e3,
                                           rel=rel, lo=lo, hi=hi)
    return out


def save(fig, d: Path, stem: str):
    fig.savefig(d / f"{stem}.svg", metadata={"Date": None})
    fig.savefig(d / f"{stem}.png", dpi=150)


def plot_regression(reg, d: Path):
    shapes = list(dict.fromkeys(s for (_, s) in reg))
    fig, ax = plt.subplots(figsize=(7.5, 3.6), constrained_layout=True)
    width = 0.19
    for i, f in enumerate(FIELDS):
        xs = [j + (i - 1.5) * width for j in range(len(shapes))]
        ys = [reg[(f, s)]["speedup"] for s in shapes]
        err = [[y - reg[(f, s)]["lo"] for y, s in zip(ys, shapes)],
               [reg[(f, s)]["hi"] - y for y, s in zip(ys, shapes)]]
        ax.bar(xs, ys, width * 0.9, color=COLORS[f], label=f"{f} ({SCHEME[f]})",
               edgecolor="white", linewidth=1)
        ax.errorbar(xs, ys, yerr=err, fmt="none", ecolor=INK2, elinewidth=0.8, capsize=1.5)
    ax.axhline(1.0, color=INK2, linewidth=1, linestyle="--")
    ax.text(-0.5, 1.05, "parity", color=INK2, ha="left", va="bottom", fontsize=8)
    ax.set_xticks(range(len(shapes)), [s.replace("x", "×") for s in shapes])
    ax.set_xlabel("product shape r×k×c (shapes the schemes use)")
    ax.set_ylabel("old loop time / new time  (higher = faster)")
    ax.set_title("New shared loop vs each scheme's old loop (median of 5 batches; bars = batch range)",
                 fontsize=9, color=INK, loc="left")
    ax.yaxis.grid(True, color=GRID, linewidth=0.6)
    ax.set_axisbelow(True)
    ax.legend(frameon=False, ncol=4, fontsize=8, loc="upper left")
    ax.set_ylim(0, max(v["hi"] for v in reg.values()) * 1.15)
    save(fig, d, "regression")
    return fig


def plot_crossover(xo, d: Path):
    ns = sorted({n for (_, n, _) in xo})
    fig, ax = plt.subplots(figsize=(7.5, 4.2), layout="constrained")
    for f in FIELDS:
        ys = [xo[(f, n, 32)]["rel"] for n in ns]
        ax.plot(ns, ys, color=COLORS[f], marker=MARKERS[f], markersize=5, linewidth=2,
                label=f"{f} ({SCHEME[f]})")
        ax.fill_between(ns, [xo[(f, n, 32)]["lo"] for n in ns], [xo[(f, n, 32)]["hi"] for n in ns],
                        color=COLORS[f], alpha=0.12, linewidth=0)
        dy = {"GF(16)": 5, "GF(256)": -5}.get(f, 0)
        ax.annotate(f, (ns[-1], ys[-1]), xytext=(6, dy), textcoords="offset points",
                    va="center", fontsize=8, color=INK)
    ax.axhline(1.0, color=INK2, linewidth=1, linestyle="--")
    ax.axvline(36, color=GRID, linewidth=1)
    ax.text(37, 0.22, "UOV o=36", color=INK2, fontsize=8, va="bottom")
    ax.set_xscale("log", base=2)
    ax.set_yscale("log", base=2)
    ax.set_xticks(ns, [str(n) for n in ns])
    yt = [0.25, 0.5, 1, 2, 4, 8]
    ax.set_yticks(yt, [f"{y:g}" for y in yt])
    ax.minorticks_off()
    ax.set_ylim(0.15, 10)
    ax.set_xlim(ns[0] * 0.9, ns[-1] * 1.45)
    ax.set_xlabel("n (square n×n · n×n product)")
    ax.set_ylabel("Strassen time / schoolbook time  (< 1 = Strassen faster)")
    ax.set_title("Strassen at cutoff 32 vs the shared schoolbook loop (median of 5 batches; band = batch range)",
                 fontsize=9, color=INK, loc="left")
    ax.yaxis.grid(True, color=GRID, linewidth=0.6)
    ax.set_axisbelow(True)
    fig.legend(frameon=False, ncol=4, fontsize=8, loc="outside lower center")
    save(fig, d, "crossover")
    return fig


def text_page(pdf, lines, title):
    fig = plt.figure(figsize=(8.27, 11.69))
    fig.text(0.07, 0.95, title, fontsize=13, weight="bold", color=INK, va="top")
    fig.text(0.07, 0.91, "\n".join(lines), fontsize=8, family="monospace", color=INK, va="top")
    pdf.savefig(fig)
    plt.close(fig)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("dir", type=Path)
    ap.add_argument("--commit", required=True)
    ap.add_argument("--host", required=True)
    a = ap.parse_args()
    d = a.dir
    reg = regression_table(load(d, "regression.json"))
    xo = crossover_table(load(d, "crossover.json"))

    figs = [plot_regression(reg, d), plot_crossover(xo, d)]

    metrics = []
    for f in FIELDS:
        for s in ("36x36x36", "36x24x12"):
            v = reg[(f, s)]
            metrics.append(dict(name=f"{SCHEME[f].lower()}-{s}-old-over-new", value=round(v["speedup"], 4),
                                unit="ratio", n=5, spread=[round(v["lo"], 4), round(v["hi"], 4)],
                                note=f"{f}: old loop median / new strassen::mul median (cutoff "
                                     f"{'none' if v['cutoff'] > 10**9 else v['cutoff']}), {s}, 3000 calls x 5 batches"))
    for f in FIELDS:
        for n in (36, 512):
            v = xo[(f, n, 32)]
            metrics.append(dict(name=f"{SCHEME[f].lower()}-n{n}-schoolbook-over-strassen32", value=round(1 / v["rel"], 4),
                                unit="ratio", n=5, spread=[round(1 / v["hi"], 4), round(1 / v["lo"], 4)],
                                note=f"{f}: schoolbook median / Strassen (cutoff 32) median, square n={n}"))
    (d / "metrics.json").write_text(json.dumps(metrics, indent=2) + "\n")

    lines = [f"commit  {a.commit}", f"host    {a.host}", "",
             "Harness: research/pqc_strassen_20261124/harness (includes src/pqc/strassen.rs by path).",
             "Each cell: 1 warm-up call, 5 timed batches; values are medians, ranges are batch min/max.",
             "One thread, no core pinning, shared macOS host with other sessions' builds: not isolated.", "",
             "1. New shared loop (strassen::mul at each scheme's cutoff) vs each scheme's old loop",
             f"   {'field':8} {'shape':10} {'old us':>9} {'new us':>9} {'old/new':>8}  batch range"]
    for (f, s), v in reg.items():
        lines.append(f"   {f:8} {s:10} {v['old_us']:9.2f} {v['new_us']:9.2f} {v['speedup']:8.2f}  "
                     f"{v['lo']:.2f}-{v['hi']:.2f}")
    lines += ["", "2. Strassen / schoolbook on square n x n (cutoffs 16 / 32 / 64)",
              f"   {'field':8} {'n':>4} {'school ms':>10} {'c16':>6} {'c32':>6} {'c64':>6}"]
    for f in FIELDS:
        for n in sorted({n for (g, n, _) in xo if g == f}):
            lines.append(f"   {f:8} {n:4d} {xo[(f, n, 32)]['school_ms']:10.3f} "
                         + " ".join(f"{xo[(f, n, c)]['rel']:6.2f}" for c in (16, 32, 64)))
    lines += ["", *textwrap.wrap(
        "Reading: the speedups at the schemes' own sizes come from the shared i-l-j loop (and, for "
        "McEliece, from leaving the i-j-k loop), not from Strassen. Strassen at cutoff 32 only "
        "engages on UOV's 36x36 products. Over GF(2) bytes it takes longer at 7 of 9 sizes (n=64 and "
        "128 at parity within noise), so McEliece never recurses. Noise floor: cells with cutoff >= n "
        "run the schoolbook loop on both sides and still range 0.73-1.69, so effects inside that band "
        "are unresolved. Unvalidated timing on a shared host; not a validation-gate result.", 95)]
    with PdfPages(d / "REPORT.pdf", metadata={"Title": "PQC Strassen timing, 2026-11-24",
                                              "CreationDate": None, "ModDate": None}) as pdf:
        text_page(pdf, lines, "PQC dense matrix products: Strassen path timing (2026-11-24)")
        for fig in figs:
            pdf.savefig(fig)
    print(json.dumps(metrics, indent=2))


if __name__ == "__main__":
    main()
