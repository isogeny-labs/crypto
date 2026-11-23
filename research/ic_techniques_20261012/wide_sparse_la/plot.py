#!/usr/bin/env python3
"""Draw the two figures of the wide-modulus sparse linear algebra benchmark.

    plot.py --results results.json --raw RAW_DIR --out-dir figures

LAYOUT ONLY.  Every computation of the analysis (cell validity and stop rule,
size-law and width-law fits, bootstrap intervals, A/A bands, falsification
tests, extrapolation) lives in the native Rust crate `analysis/`
(`wide-sla-analysis --raw RAW_DIR --out results.json`), as AGENTS.md
"Implementation language: no Python" requires.  This script only reads recorded
values and draws them:

* `ns_per_row_op_vs_limbs`: median/min/max ns per row_op per arm and D, read
  from results.json `width_law.table`.
* `row_ops_per_col_vs_D`: the recorded `row_ops / D` of every `verified: true`
  record in raw/seed*/grid.jsonl (points), and the size-law line
  `2^(a + (beta-1) log2 D)` from the fitted `a`, `beta` in results.json
  `size_law` (dashed; drawn over the sizes the fit used).  Earlier versions of
  this figure joined the per-size medians instead of drawing the fit.

Writes <out-dir>/<name>.svg and <name>.png (dpi 200).  Needs matplotlib.
"""
import argparse
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

SOLVERS = ("full-bw", "full-bl")
ARMS = ["u64@61", "U128@61", "U128@89", "U128@127", "U192@129", "U192@190", "U256@255"]
COL = ["#000000", "#E69F00", "#56B4E9", "#009E73", "#D55E00", "#0072B2", "#CC79A7"]
MK = ["o", "s", "^", "v", "D", "P", "X"]


def arm_label(bits, word):
    return "u64@%d" % bits if word == "u64" else "%s@%d" % (word.upper(), bits)


def recorded_points(raw):
    """(arm, solver) -> {D: [row_ops / D, ...]} from verified records (first per cell)."""
    out, seen = {}, set()
    for seed in (1, 2, 3):
        path = os.path.join(raw, "seed%d" % seed, "grid.jsonl")
        try:
            lines = open(path).read().splitlines()
        except OSError:
            continue
        for line in lines:
            try:
                r = json.loads(line)
                if r.get("failed") or r.get("verified") is not True:
                    continue
                key = (arm_label(int(r["modulus_bits"]), r["word_width"]), r["arm"], int(r["cols"]), seed)
                if key in seen:
                    continue
                seen.add(key)
                out.setdefault(key[:2], {}).setdefault(key[2], []).append(r["row_ops"] / key[2])
            except (ValueError, KeyError, TypeError):
                continue
    return out


def save(fig, outdir, name):
    fig.savefig(os.path.join(outdir, name + ".svg"))
    fig.savefig(os.path.join(outdir, name + ".png"), dpi=200)
    plt.close(fig)


def fig_row_ops(res, raw, outdir):
    pts = recorded_points(raw)
    fig, axes = plt.subplots(1, 2, figsize=(11, 4.5), sharey=True)
    for ax, sol in zip(axes, SOLVERS):
        n = 0
        for i, lab in enumerate(ARMS):
            byD = pts.get((lab, sol), {})
            if not byD:
                continue
            xs = [D for D in sorted(byD) for _ in byD[D]]
            ys = [y for D in sorted(byD) for y in byD[D]]
            n += len(xs)
            ax.scatter(xs, ys, s=14, color=COL[i], marker=MK[i], alpha=0.6, label=lab)
            fit = res["size_law"].get("%s|%s" % (lab, sol), {}).get("row_ops", {})
            if fit.get("status") == "ok":
                lx = [2.0 ** k for k in fit["sizes_log2"]]
                ax.plot(lx, [2.0 ** (fit["a"] + fit["slope"] * k) for k in fit["sizes_log2"]],
                        color=COL[i], lw=1.0, ls="--")
        ax.set_xscale("log", base=2)
        ax.set_yscale("log")
        ax.set_xlabel("D (columns)")
        ax.set_title(sol)
        ax.grid(alpha=0.3, which="both")
        if n == 0:
            ax.text(0.5, 0.5, "no valid data", transform=ax.transAxes, ha="center")
        else:
            ax.legend(fontsize=7, title="width arm (dashed: size-law fit)", title_fontsize=7)
    axes[0].set_ylabel("row_ops / D (counted)")
    fig.suptitle("Counted work per column against D (points: seeds; dashed: fitted size law)")
    fig.tight_layout()
    save(fig, outdir, "row_ops_per_col_vs_D")


def fig_limbs(res, outdir):
    wl = res["width_law"]
    table = wl["table"]
    min_l2 = wl["min_log2_D"]
    fig, axes = plt.subplots(1, 2, figsize=(11, 4.5), sharey=True)
    cm = plt.get_cmap("viridis")
    for ax, sol in zip(axes, SOLVERS):
        ds = sorted(int(k.split("|")[1]) for k in table if k.split("|")[0] == sol)
        for j, D in enumerate(ds):
            c = cm(j / max(1, len(ds) - 1) * 0.9)
            ent = table["%s|%d" % (sol, D)]["arms"]
            xs, med, lo, hi = [], [], [], []
            for lab in ARMS:
                e = ent.get(lab)
                if e:
                    xs.append(e["limbs"])
                    med.append(e["ns_per_row_op_median"])
                    lo.append(e["ns_per_row_op_median"] - e["min"])
                    hi.append(e["max"] - e["ns_per_row_op_median"])
            if xs:
                ax.errorbar(xs, med, yerr=[lo, hi], fmt="o", ms=4, color=c, capsize=2, lw=0.8,
                            label="D=2^%d" % round(__import__("math").log2(D)))
        ax.set_xticks([1, 2, 3, 4])
        ax.set_xlabel("limbs L (u64=1, U128=2, U192=3, U256=4)")
        ax.set_title(sol)
        ax.grid(alpha=0.3)
        if not ds:
            ax.text(0.5, 0.5, "no D >= 2^%d data" % min_l2, transform=ax.transAxes, ha="center")
        else:
            ax.legend(fontsize=7)
    axes[0].set_ylabel("ns per row_op (1e9 * solve_s / row_ops; derived)")
    fig.suptitle("Time per counted operation against limbs (median, min-max over pods)")
    fig.tight_layout()
    save(fig, outdir, "ns_per_row_op_vs_limbs")


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--results", required=True)
    ap.add_argument("--raw", required=True)
    ap.add_argument("--out-dir", required=True)
    a = ap.parse_args()
    with open(a.results) as fh:
        res = json.load(fh)
    os.makedirs(a.out_dir, exist_ok=True)
    fig_row_ops(res, a.raw, a.out_dir)
    fig_limbs(res, a.out_dir)


if __name__ == "__main__":
    main()
