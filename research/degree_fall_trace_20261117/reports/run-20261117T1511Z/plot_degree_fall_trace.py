#!/usr/bin/env python3
"""Lay out the recorded degree-fall dataset as a coloured trace figure.

This script only *lays out* values recorded by the Rust f4_fp degree-fall
diagnostic (ecbench.degree-fall-dataset/v1); it computes no result. The colour
of each (relations, degree) cell is the recorded Hilbert deviation
`expected - realized` (standard monomials a semi-regular system of the same
shape would have, minus those the computed basis actually leaves) — i.e. how
much structure the added relations removed at that degree. A cell is outlined
when the engine flagged a degree fall there.
"""
import argparse
import json

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.colors import Normalize
from matplotlib.patches import Rectangle


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--input", required=True)
    ap.add_argument("--out", required=True, help="output path prefix (no extension)")
    ap.add_argument("--run-id", default="")
    args = ap.parse_args()

    data = json.load(open(args.input))
    systems = data["systems"]
    field = data.get("field")
    n_vars = data.get("n_vars")

    rels = [s["relations"] for s in systems]
    max_deg = max((st["degree"] for s in systems for st in s["steps"]), default=0)
    degrees = list(range(max_deg + 1))

    # Recorded Hilbert deviation grid (expected - realized); None where the
    # engine recorded no step at that degree for that system.
    grid = [[None] * len(degrees) for _ in systems]
    fall = [[False] * len(degrees) for _ in systems]
    for i, s in enumerate(systems):
        for st in s["steps"]:
            d = st["degree"]
            grid[i][d] = st["expected"] - st["realized"]
            fall[i][d] = bool(st["is_fall"])

    vals = [v for row in grid for v in row if v is not None]
    vmax = max(vals) if vals else 1
    norm = Normalize(vmin=0, vmax=max(vmax, 1))
    cmap = plt.get_cmap("YlOrRd")

    fig, (axh, axl) = plt.subplots(
        1, 2, figsize=(11, 4.2), gridspec_kw={"width_ratios": [2.3, 1]}
    )

    # Panel A: the coloured relations x degree deviation heatmap.
    for i, _ in enumerate(systems):
        for j, d in enumerate(degrees):
            v = grid[i][j]
            if v is None:
                axh.add_patch(
                    Rectangle((j - 0.5, i - 0.5), 1, 1, facecolor="#f2f2f2",
                              edgecolor="white")
                )
                continue
            axh.add_patch(
                Rectangle((j - 0.5, i - 0.5), 1, 1, facecolor=cmap(norm(v)),
                          edgecolor="white")
            )
            txt = "#1a1a1a" if norm(v) < 0.6 else "white"
            axh.text(j, i, str(v), ha="center", va="center", fontsize=8, color=txt)
            if fall[i][j]:
                axh.add_patch(
                    Rectangle((j - 0.5, i - 0.5), 1, 1, fill=False,
                              edgecolor="#1f3b8f", linewidth=2.2)
                )
    # Mark the observed degree of regularity per row (first vanishing degree).
    for i, s in enumerate(systems):
        dreg = s.get("degree_of_regularity_observed")
        if dreg is not None and dreg in degrees:
            axh.plot([dreg], [i], marker="v", color="#1f3b8f", markersize=9,
                     zorder=5, clip_on=False)
    axh.set_xlim(-0.5, len(degrees) - 0.5)
    axh.set_ylim(len(systems) - 0.5, -0.5)
    axh.set_xticks(degrees)
    axh.set_yticks(range(len(systems)))
    axh.set_yticklabels([f"{r} rel." for r in rels])
    axh.set_xlabel("working degree  d")
    axh.set_ylabel("relations in the system")
    axh.set_title("Hilbert deviation  (semi-regular expected − realized)")
    sm = plt.cm.ScalarMappable(cmap=cmap, norm=norm)
    sm.set_array([])
    cb = fig.colorbar(sm, ax=axh, fraction=0.046, pad=0.04)
    cb.set_label("standard monomials removed")

    # Panel B: observed degree of regularity and solving degree vs relations.
    dregs = [s.get("degree_of_regularity_observed") for s in systems]
    sdeg = [s.get("solving_degree") for s in systems]
    ffd = [s.get("first_fall_degree") for s in systems]
    axl.plot(rels, dregs, "o-", color="#1f3b8f", label="observed d_reg")
    axl.plot(rels, sdeg, "s--", color="#d1495b", label="solving degree")
    for x, y in zip(rels, ffd):
        if y is not None:
            axl.annotate(f"FFD {y}", (x, y), textcoords="offset points",
                         xytext=(4, -12), fontsize=8, color="#7a4e00")
    axl.set_xlabel("relations in the system")
    axl.set_ylabel("degree")
    axl.set_xticks(rels)
    axl.set_title("degree of regularity vs relations")
    axl.legend(fontsize=8, loc="best")
    axl.grid(True, alpha=0.3)

    sub = f"f4_fp degree-fall diagnostic · F_{field}, {n_vars} vars · ▽ = observed d_reg · blue outline = degree fall"
    if args.run_id:
        sub += f" · {args.run_id}"
    fig.suptitle("How relations drive the degree of regularity", fontsize=13)
    fig.text(0.5, 0.005, sub, ha="center", fontsize=8, color="#444")
    fig.tight_layout(rect=(0, 0.03, 1, 0.96))

    fig.savefig(f"{args.out}.svg")
    fig.savefig(f"{args.out}.png", dpi=150)
    print(f"wrote {args.out}.svg and {args.out}.png")


if __name__ == "__main__":
    main()
