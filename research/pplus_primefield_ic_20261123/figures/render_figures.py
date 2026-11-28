#!/usr/bin/env python3
"""Render existing research statements and recorded counts; run no research code."""
import csv
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Circle, FancyArrowPatch, FancyBboxPatch
from matplotlib.ticker import FixedLocator, FuncFormatter, NullFormatter

OUT = Path(__file__).resolve().parent
NAVY, TEAL, INK = "#17334D", "#087F8C", "#20364A"
MUTED, GRID, PALE = "#5D6E7B", "#DDE5EA", "#F4F7F9"
AMBER, WARM = "#9B621F", "#FFF6E7"
plt.rcParams.update({
    "font.family": "DejaVu Sans", "font.size": 11,
    "svg.fonttype": "none", "svg.hashsalt": "pplus-primefield-ic-20261123",
    "axes.unicode_minus": True, "figure.facecolor": "white",
    "savefig.facecolor": "white", "mathtext.fontset": "dejavusans",
})

def save(fig, name):
    fig.savefig(OUT / f"{name}.svg", metadata={"Date": None})
    fig.savefig(OUT / f"{name}.png", dpi=300)
    plt.close(fig)

def route():
    fig = plt.figure(figsize=(8.8, 8.2))
    ax = fig.add_axes([0, 0, 1, 1], xlim=(0, 8.8), ylim=(0, 8.2))
    ax.axis("off")
    ax.text(.35, 7.88, "A p+1 route to prime-field ECDLP", color=NAVY,
            fontsize=17, weight="bold", va="center")
    ax.text(.35, 7.55, "Coordinate construction, qualified arithmetic, and open algorithmic steps",
            color=MUTED, fontsize=11.2, va="center")

    left, width, height = .35, 4.85, .91
    stages = [
        (6.35, "Norm-one torus", [r"$T_2(\mathbb{F}_p)$ has order $p+1$",
                                  r"For the zero fibre below, require $4N\mid(p+1)$."]),
        (5.25, "Full Dickson fibre", [r"$V_k=\{x:D_{2^k}(x,1)=0\}$; $N=2^k$",
                                     r"Trace coordinates $x=z+z^{-1}$ in $\mathbb{F}_p$."]),
        (4.15, "Rational elliptic factor base", [r"Keep rational lifts and subgroup membership.",
                                                r"One column per $\{P,-P\}$; retain the signs."]),
        (3.05, r"Retained $S_3$ relation extraction", ["OPEN COST BOTTLENECK",
                                                      "Keep selector constraints and intermediate x-values."]),
        (1.95, "Signed relations and useful rank", ["Verify elliptic sums and sign choices.",
                                                  "Remove duplicates and trivial cancellations."]),
        (.85, "Scalar recovery", ["Eliminate factor-base logarithms and recover s.",
                                  r"Verify the result by checking $[s]G=Q$."]),
    ]
    for index, (y, title, lines) in enumerate(stages):
        open_stage = index >= 3
        special = index == 3
        edge = AMBER if special else (MUTED if open_stage else NAVY)
        fill = WARM if special else ("white" if open_stage else PALE)
        box = FancyBboxPatch((left, y), width, height,
                             boxstyle="round,pad=0.015,rounding_size=0.07",
                             linewidth=1.25, edgecolor=edge, facecolor=fill,
                             linestyle=(0, (4, 3)) if open_stage else "solid")
        ax.add_patch(box)
        ax.add_patch(Circle((left + .26, y + .66), .145,
                            facecolor=edge, edgecolor="none"))
        ax.text(left + .26, y + .66, str(index + 1), ha="center", va="center",
                color="white", fontsize=9.6, weight="bold")
        ax.text(left + .51, y + .66, title, fontsize=12.2, weight="bold",
                color=NAVY, va="center")
        for line_index, line in enumerate(lines):
            is_label = special and line_index == 0
            ax.text(left + .51, y + .38 - .22 * line_index, line,
                    fontsize=9.4 if is_label else 10.4,
                    weight="bold" if is_label else "normal",
                    color=AMBER if is_label else INK, va="center")
        if index + 1 < len(stages):
            target_y = stages[index + 1][0] + height
            ax.add_patch(FancyArrowPatch((left + width / 2, y - .025),
                                        (left + width / 2, target_y + .025),
                                        arrowstyle="-|>", mutation_scale=10,
                                        linewidth=1.25, color=MUTED,
                                        linestyle=(0, (3, 2)) if index >= 2 else "solid"))

    # The only FFT connection is to stage 2, the full unfiltered fibre.
    side_x, side_y, side_w, side_h = 5.65, 4.64, 2.8, 2.13
    ax.add_patch(FancyBboxPatch((side_x, side_y), side_w, side_h,
                               boxstyle="round,pad=0.02,rounding_size=0.08",
                               facecolor="#EAF6F5", edgecolor=TEAL, linewidth=1.5))
    ax.add_patch(FancyArrowPatch((left + width + .02, 5.705), (side_x - .03, 5.705),
                                arrowstyle="-|>", mutation_scale=12,
                                linewidth=1.7, color=TEAL))
    ax.text(side_x + .18, side_y + 1.87, "Binary line FFT", color=TEAL,
            fontsize=14, weight="bold", va="center")
    ax.text(side_x + .18, side_y + 1.58, "FULL UNFILTERED FIBRE ONLY", color=TEAL,
            fontsize=8.8, weight="bold", va="center")
    ax.text(side_x + .18, side_y + 1.27, r"$\mathbb{F}_p[X]/(D_{2^k})$", color=INK,
            fontsize=12, va="center")
    ax.text(side_x + .18, side_y + .91, "Evaluate · interpolate\nMultiply in the quotient ring",
            color=INK, fontsize=10.4, linespacing=1.55, va="center")
    ax.text(side_x + .18, side_y + .36, "Native correctness checks: k = 1–8",
            color=TEAL, fontsize=9.2, va="center")
    ax.text(side_x + .04, side_y - .32,
            "The filtered base and elliptic\npair spectrum need not retain\nthis transform structure.",
            color=MUTED, fontsize=10.2, linespacing=1.45, va="top")
    ax.text(.35, .43, "Arrows describe a workflow; no torus-to-elliptic group homomorphism is assumed.",
            color=MUTED, fontsize=10.2, va="center")
    ax.text(.35, .18, "Dashed stages remain algorithmic obligations. No relation-speed or recovery result is claimed.",
            color=MUTED, fontsize=9.5, va="center")
    save(fig, "route")

def counts():
    with (OUT / "selector_counts.csv").open(newline="") as source:
        rows = list(csv.DictReader(source))
    # Only parse and display recorded values; no elliptic or selector computation.
    depths = [int(r["k"]) for r in rows]
    full = [int(r["root_count"]) for r in rows]
    actual = [int(r["curve_columns"]) for r in rows]
    fig = plt.figure(figsize=(8.8, 5.9))
    fig.text(.04, .945, "Full selector and rational elliptic columns", color=NAVY,
             fontsize=16.8, weight="bold", va="center")
    fig.text(.04, .891, r"$p=2^{31}-1$  ·  $y^2=x^3-3x+21$  ·  Accepted selector attempt 02",
             color=MUTED, fontsize=11.2, va="center")
    ax = fig.add_axes([.09, .19, .84, .57])
    ax.set_xscale("log", base=2)
    ax.set_xlim(8, 8192)
    ax.set_ylim(12.65, 4.35)
    ax.set_yticks(depths, [str(k) for k in depths])
    ax.set_ylabel("Depth k", color=INK, fontsize=11.5, labelpad=13)
    ax.set_xlabel("Recorded count (log₂ scale)", color=INK, fontsize=11.5, labelpad=10)
    ax.xaxis.set_major_locator(FixedLocator([16, 64, 256, 1024, 4096]))
    ax.xaxis.set_major_formatter(FuncFormatter(lambda v, _: f"{int(v):,}"))
    ax.xaxis.set_minor_locator(FixedLocator([8, 32, 128, 512, 2048, 8192]))
    ax.xaxis.set_minor_formatter(NullFormatter())
    ax.grid(axis="x", which="major", color=GRID, linewidth=.8)
    ax.grid(axis="x", which="minor", color=GRID, linewidth=.5, linestyle=(0, (2, 3)))
    ax.tick_params(axis="both", colors=MUTED, labelsize=10.5, length=0, pad=7)
    for spine in ax.spines.values():
        spine.set_visible(False)
    for k, n, b in zip(depths, full, actual):
        ax.plot([b, n], [k, k], color="#B7C6CF", linewidth=2.2, zorder=1)
        ax.scatter([n], [k], s=62, color=NAVY, marker="o", zorder=3)
        ax.scatter([b], [k], s=65, color=TEAL, marker="D", zorder=3)
        ax.annotate(f"{n:,}", (n, k), xytext=(9, 0), textcoords="offset points",
                    color=NAVY, fontsize=11.5, weight="bold", va="center")
        ax.annotate(f"{b:,}", (b, k), xytext=(-10, 0), textcoords="offset points",
                    color=TEAL, fontsize=11.5, weight="bold", ha="right", va="center")
    n_key, = ax.plot([], [], color=NAVY, marker="o", linestyle="none", markersize=7)
    b_key, = ax.plot([], [], color=TEAL, marker="D", linestyle="none", markersize=7)
    fig.legend([n_key, b_key], ["Full selector N", "Rational x / negation-orbit columns B"],
               loc="upper left", bbox_to_anchor=(.065, .848), ncols=2, frameon=False,
               fontsize=11.2, labelcolor=INK, handletextpad=.5, columnspacing=2.2)
    fig.text(.04, .078, "Exact recorded counts; B is the rational x-column count, not the signed affine-point count.",
             color=MUTED, fontsize=9.8, va="center")
    fig.text(.04, .037, "Source: selector attempt 02 normal.stdout · k = 5–12 · No speed, relation-yield, or rank measurement.",
             color=MUTED, fontsize=9.5, va="center")
    save(fig, "counts")

if __name__ == "__main__":
    route()
    counts()
