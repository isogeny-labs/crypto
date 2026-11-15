"""Figures for EXP-PKM-ODD-001 from the recorded rows in ../runs (plots only).

    python3 make_figures.py
writes work_vs_S.svg, partB_ratio.svg, form_ratio.svg (+ .png at 150 dpi).
"""
import glob
import json
import math
import os
import statistics

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
RUNS = os.path.join(HERE, "..", "runs")
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none"})
COL = {"A2": "#1f4e9c", "A3": "#c0392b", "A5": "#7d3c98", "A23": "#1e8449", "A32": "#b9770e",
       "I2": "#5d8fd6", "I3": "#e57f74", "I23": "#6fcf97"}


def rows(schema):
    out = []
    for f in sorted(glob.glob(os.path.join(RUNS, "*.jsonl"))):
        out += [json.loads(l) for l in open(f) if l.strip()]
    return [r for r in out if r.get("schema") == schema]


def save(fig, name):
    fig.savefig(os.path.join(HERE, name + ".svg"), bbox_inches="tight")
    fig.savefig(os.path.join(HERE, name + ".png"), dpi=150, bbox_inches="tight")


def work_vs_S():
    rs = [r for r in rows("pkm-odd-tower-f4.row/v1") if r["kind"] == "refute" and r["cell"].startswith("M2-")]
    fig, axes = plt.subplots(1, 2, figsize=(10, 3.8), sharey=True)
    for ax, control in zip(axes, ("tower", "null")):
        by = {}
        for r in rs:
            if r["control"] != control or r["censored"]:
                continue
            arm = r["cell"].split("-")[1]
            by.setdefault(arm, {}).setdefault(r["S"], []).append(r["field_ops"])
        for arm, d in sorted(by.items()):
            xs = sorted(d)
            ys = [math.log2(statistics.median(d[s])) for s in xs]
            ax.plot([math.log2(s) for s in xs], ys, "o-" if arm.startswith("A") else "s--",
                    color=COL.get(arm, "k"), label=arm, ms=4)
        ax.set_title(f"m = 2, {control} (median of 3 refutation systems)")
        ax.set_xlabel("log2 S = log2 |V|")
    axes[0].set_ylabel("log2 F4 field_ops")
    axes[1].legend(fontsize=7, ncol=2)
    fig.suptitle("EXP-PKM-ODD-001 part A: F4 work against information, by degree mix and form (p = 1399680001)")
    fig.tight_layout()
    save(fig, "work_vs_S")


def part_b():
    rs = rows("pkm-algebraic-oracle.row/v1")
    fig, axes = plt.subplots(1, 2, figsize=(10, 3.8), sharey=True)
    for ax, m in zip(axes, (2, 3)):
        by = {}
        for r in rs:
            if r["m"] != m or r["alg_undetermined"]:
                continue
            arm = r["cell"].split("-")[2]
            by.setdefault(arm, {}).setdefault(r["S"], []).append(r["ratio_alg_over_enum"])
        for arm, d in sorted(by.items()):
            xs = sorted(d)
            for s in xs:
                ax.scatter([math.log2(s)] * len(d[s]), [math.log10(v) for v in d[s]], s=8,
                           color=COL.get(arm, "k"), alpha=0.5)
            ax.plot([math.log2(s) for s in xs], [math.log10(statistics.median(d[s])) for s in xs],
                    "-", color=COL.get(arm, "k"), label=arm)
        ax.axhline(0, color="#333", lw=0.8)
        ax.axhline(math.log10(0.5), color="#999", lw=0.6, ls=":")
        ax.set_title(f"m = {m}: algebraic oracle / enumeration")
        ax.set_xlabel("log2 S")
    axes[0].set_ylabel("log10 (F_p mults, algebraic / enumeration)")
    axes[1].legend(fontsize=7)
    fig.suptitle("EXP-PKM-ODD-001 part B: solving slide 17's system vs enumerating V (same targets; above 0 = algebraic costs more)")
    fig.tight_layout()
    save(fig, "partB_ratio")


if __name__ == "__main__":
    work_vs_S()
    part_b()
