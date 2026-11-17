#!/usr/bin/env python3
"""Lay out the recorded growth dataset (ecbench.growth-dataset/v1).

The degrees and the growth fits are computed and recorded by the Rust engine
(`f4_fp` for the degrees, `ecbench::growth` for the least-squares fits); this
script only draws the recorded points and the recorded best-fit line.
"""
import argparse
import json

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt


def predict(fit, x):
    m = fit["model"]
    a, b = fit["intercept"], fit["slope"]
    if m == "linear":
        return a + b * x
    if m == "power":
        return pow(2.718281828459045, a) * pow(x, b)
    return pow(2.718281828459045, a + b * x)  # exponential


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--input", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--run-id", default="")
    args = ap.parse_args()

    data = json.load(open(args.input))
    pts = data["points"]
    field = data.get("field")
    ns = [p["n_vars"] for p in pts]
    dreg = [p["degree_of_regularity_observed"] for p in pts]
    sdeg = [p["solving_degree"] for p in pts]
    fits = data["fits"]

    fig, ax = plt.subplots(figsize=(7.2, 4.4))

    # Solving degree is 0 across this family (the generators {x_i^2 - 1} are
    # already a Gröbner basis, so no S-pair is productive); the degree of
    # regularity is the informative growth series here.
    series = [
        ("observed d_reg", ns, dreg, fits["degree_of_regularity_vs_n"], "#1f3b8f", "o"),
    ]
    xline = [min(ns), max(ns)]
    for name, xs, ys, fitlist, color, marker in series:
        ax.plot(xs, ys, marker, color=color, markersize=8, label=name, zorder=3)
        if fitlist:
            best = fitlist[0]  # recorded, ranked by r2
            yl = [predict(best, x) for x in xline]
            ax.plot(xline, yl, "--", color=color, alpha=0.7, zorder=2)
            ci = best.get("slope_ci95")
            ci_txt = (
                f" (95% CI {ci[0]:.2f}..{ci[1]:.2f})" if ci else ""
            )
            ax.annotate(
                f"{name}: {best['model']} fit, slope {best['slope']:.3f}{ci_txt}, R²={best['r2']:.3f}",
                xy=(0.02, 0.98 - 0.07 * series.index(
                    next(s for s in series if s[0] == name))),
                xycoords="axes fraction", fontsize=8, color=color, va="top",
            )

    ax.set_xlabel("instance size  (number of variables n)")
    ax.set_ylabel("degree")
    ax.set_xticks(ns)
    ax.set_title("Growth of the degree of regularity with instance size")
    ax.grid(True, alpha=0.3)
    ax.legend(loc="lower right", fontsize=9)
    sub = f"regular sequence {{x_i² − 1}} over F_{field} · degrees measured by f4_fp, slopes fitted by ecbench::growth"
    if args.run_id:
        sub += f" · {args.run_id}"
    fig.text(0.5, 0.005, sub, ha="center", fontsize=8, color="#444")
    fig.tight_layout(rect=(0, 0.04, 1, 1))

    fig.savefig(f"{args.out}.svg")
    fig.savefig(f"{args.out}.png", dpi=150)
    print(f"wrote {args.out}.svg and {args.out}.png")


if __name__ == "__main__":
    main()
