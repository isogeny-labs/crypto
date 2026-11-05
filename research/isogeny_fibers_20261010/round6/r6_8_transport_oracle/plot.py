"""Plot recorded values of the R6.8 census (no computation beyond ratios of
recorded numbers).  Reads census_*.json beside this script and writes
class_factor_per_unknown.svg and cost_ratio.svg."""
import glob
import json
import os

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
FILES = sorted(glob.glob(os.path.join(HERE, "census_*_s144.json")))
ARMS = ["plain", "pullback_l2", "pullback_l3", "union", "union_pullbacks",
        "neighbour_l2", "neighbour_l3", "filtered_l2E", "filtered_l3E"]
COLORS = {"plain": "#4C72B0", "pullback_l2": "#DD8452", "pullback_l3": "#C44E52",
          "union": "#55A868", "union_pullbacks": "#8172B3", "neighbour_l2": "#F0C27A",
          "neighbour_l3": "#E89A9B", "filtered_l2E": "#A6C8E8", "filtered_l3E": "#B5D7B0"}


def label(d):
    c = d["curve_E"]
    return f"{d['family']}: p={c['p']}, N={c['h']}r"


def main():
    data = [json.load(open(f)) for f in FILES]
    fig, axes = plt.subplots(1, 2, figsize=(12, 4.6), sharey=False)
    for ax, m in zip(axes, (2, 3)):
        width = 0.8 / len(ARMS)
        for gi, d in enumerate(data):
            rows = {r["arm"]: r for r in d["census"] if r["m"] == m}
            fal = next(f for f in d["falsifier"] if f["m"] == m)
            for ai, arm in enumerate(ARMS):
                if arm not in rows:
                    continue
                v = rows[arm]["class_factor_per_unknown"]
                x = gi + (ai - len(ARMS) / 2) * width + width / 2
                ax.bar(x, v, width, color=COLORS[arm], edgecolor="black", linewidth=0.3,
                       label=arm if gi == 0 else None)
            thr = fal["best_single_class_factor_per_unknown"] * fal["psi_count"]
            ax.plot([gi - 0.4, gi + 0.4], [thr, thr], color="red", linestyle="--", linewidth=1.2,
                    label="falsifier threshold: best single × |Ψ|" if gi == 0 else None)
        ax.set_yscale("log")
        ax.set_xticks(range(len(data)))
        ax.set_xticklabels([label(d) for d in data], fontsize=7)
        ax.set_title(f"m = {m}: class factor per unknown, c/U (exact census)")
        ax.set_ylabel("c / U")
        ax.grid(axis="y", alpha=0.3)
    axes[0].legend(fontsize=6.5, ncol=2, loc="lower left")
    fig.suptitle("R6.8 transport oracles: single bases, pulled-back bases, union, controls "
                 f"(|F| = {data[0]['size']} signed points per base; census_*_s144.json)", fontsize=9)
    fig.tight_layout()
    fig.savefig(os.path.join(HERE, "class_factor_per_unknown.svg"))

    fig, axes = plt.subplots(1, 2, figsize=(12, 4.4))
    for ax, m in zip(axes, (2, 3)):
        width = 0.8 / len(ARMS)
        for gi, d in enumerate(data):
            rows = {r["arm"]: r for r in d["census"] if r["m"] == m}
            plain = rows["plain"]["cost_model"]
            for ai, arm in enumerate(ARMS):
                if arm not in rows:
                    continue
                v = rows[arm]["cost_model"] / plain
                x = gi + (ai - len(ARMS) / 2) * width + width / 2
                ax.bar(x, v, width, color=COLORS[arm], edgecolor="black", linewidth=0.3,
                       label=arm if gi == 0 else None)
            k = next(f for f in d["falsifier"] if f["m"] == m)["union_size_over_plain_size"]
            ax.plot([gi - 0.4, gi + 0.4], [1 / k, 1 / k], color="black", linestyle=":", linewidth=1,
                    label="1/k, k = |union|/|plain| (bigger-base law)" if gi == 0 else None)
        ax.set_yscale("log")
        ax.set_xticks(range(len(data)))
        ax.set_xticklabels([label(d) for d in data], fontsize=7)
        ax.set_title(f"m = {m}: derived cost U·(r/D)·L_m relative to plain")
        ax.set_ylabel("cost / cost(plain)")
        ax.grid(axis="y", alpha=0.3)
    axes[0].legend(fontsize=6.5, ncol=2, loc="lower left")
    fig.suptitle("R6.8: §15.2 cost model per arm (derived from the exact counts in census_*_s144.json)",
                 fontsize=9)
    fig.tight_layout()
    fig.savefig(os.path.join(HERE, "cost_ratio.svg"))


if __name__ == "__main__":
    main()
