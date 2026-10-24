#!/usr/bin/env python3
"""Plot the LEAD-19 comparison from `analysis.json` (recorded values only).

usage: plot_lead19.py <analysis.json> <out-dir> [--commit <hash>]

Writes `s_by_arm.svg` (S per arm and rung, every repeat as a point, the
mean as a bar, the matched rho as a line) and `ratios.svg` (the paired
ratio S(A')/S(B) and S(A')/S(C) per row and rung with the bootstrap
interval, against the 1.10 success line).  Layout only: every number is
read from the analysis file, which read it from the run record.
"""
import json
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

ARMS = ["A", "A'", "B", "C"]
COLORS = {"A": "#7f7f7f", "A'": "#4c72b0", "B": "#dd8452", "C": "#55a868"}


def arg(flag, default=None):
    if flag in sys.argv:
        return sys.argv[sys.argv.index(flag) + 1]
    return default


def main():
    res = json.load(open(sys.argv[1]))
    out = sys.argv[2]
    os.makedirs(out, exist_ok=True)
    commit = arg("--commit", str(res["inputs"].get("char2_commit")))
    insts = [i for i in res["instances"] if i["rungs"]]
    panels = sum(len(i["rungs"]) for i in insts)
    nrows = 2 if panels > 5 else 1
    ncols = -(-panels // nrows)
    fig, axes = plt.subplots(nrows, ncols, figsize=(2.7 * ncols + 1, 4.0 * nrows), sharey=False)
    axes = list(axes.flat) if panels > 1 else [axes]
    for ax in axes[panels:]:
        ax.set_visible(False)
    k = 0
    for inst in insts:
        for rung, rec in inst["rungs"].items():
            ax = axes[k]
            k += 1
            s = rec["s_per_repeat"]["0.0"]
            for j, arm in enumerate(ARMS):
                vals = s[arm]
                ax.bar(j, sum(vals) / len(vals), color=COLORS[arm], alpha=0.35, width=0.7)
                ax.scatter([j] * len(vals), vals, color=COLORS[arm], s=18, zorder=3)
            ax.axhline(inst["rho_s_mean"], color="black", ls="--", lw=1)
            ax.set_xticks(range(len(ARMS)))
            ax.set_xticklabels(ARMS)
            mark = " (published)" if inst.get("published_rung") == rung else ""
            ax.set_title(f"n={inst['degree']}, h={inst['cofactor']}\n{rung}{mark}, |F|={rec['abscissae']['A']} abscissae", fontsize=9)
            ax.set_ylim(bottom=0)
            if (k - 1) % ncols == 0:
                ax.set_ylabel("S = GAE / √r (every phase, cold)")
    fig.suptitle(
        "LEAD-19 arms: S per repeat (points), mean (bars), matched rho (dashed; 8 walks per target)\n"
        f"run {os.path.basename(res['inputs']['char2'])}, commit {commit[:10]}",
        fontsize=9,
    )
    fig.tight_layout(rect=(0, 0, 1, 0.94))
    fig.savefig(os.path.join(out, "s_by_arm.svg"))
    fig.savefig(os.path.join(out, "s_by_arm.png"), dpi=130)
    plt.close(fig)

    labels, rb, lb, hb, rc = [], [], [], [], []
    for inst in insts:
        for rung, rec in inst["rungs"].items():
            pb = rec["paired_ratios"]["A'/B@0.0"]
            pc = rec["paired_ratios"]["A'/C@0.0"]
            mark = "*" if inst.get("published_rung") == rung else ""
            labels.append(f"n={inst['degree']} {rung}{mark}")
            rb.append(pb["geometric_mean"])
            lo, hi = pb["bootstrap_95"]
            lb.append(pb["geometric_mean"] - lo)
            hb.append(hi - pb["geometric_mean"])
            rc.append(pc["geometric_mean"])
    fig, ax = plt.subplots(figsize=(1.0 * len(labels) + 3, 4))
    x = list(range(len(labels)))
    ax.errorbar(x, rb, yerr=[lb, hb], fmt="o", color=COLORS["B"], capsize=4, label="S(A')/S(B), paired, 95 % bootstrap")
    ax.scatter([i + 0.25 for i in x], rc, color=COLORS["C"], marker="s", label="S(A')/S(C), paired (point estimate)")
    ax.axhline(1.0, color="black", lw=1)
    ax.axhline(1.10, color="red", ls=":", lw=1, label="1.10 (success line)")
    ax.set_xticks(x)
    ax.set_xticklabels(labels, rotation=30, ha="right", fontsize=8)
    ax.set_ylabel("paired ratio (geometric mean over repeats)")
    ax.set_title(
        "LEAD-19: raw base without its class test (A') over the projected (B)\n"
        "and subgroup-only (C) base, paired by repeat; * = published rung",
        fontsize=9,
    )
    ax.legend(fontsize=8, loc="best")
    fig.tight_layout()
    fig.savefig(os.path.join(out, "ratios.svg"))
    fig.savefig(os.path.join(out, "ratios.png"), dpi=130)
    plt.close(fig)
    print("wrote", out)


if __name__ == "__main__":
    main()
