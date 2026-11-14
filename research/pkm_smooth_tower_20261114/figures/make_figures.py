"""Lay out the recorded pkm-smooth-tower outputs in ../data as SVG figures.

Plots recorded values only (AGENTS.md "Implementation language"): nothing
here computes a tower, an order or a relation.

    python3 make_figures.py   # writes pipeline.svg, cm_orders.svg, method1_trials.svg
"""
import json
import math
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch

HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.path.join(HERE, "..", "data")
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none"})


def load(name):
    with open(os.path.join(DATA, name)) as f:
        return json.load(f)


def box(ax, x, y, w, h, text, fc="#eef3fb", ec="#2c5aa0", fs=8.5):
    ax.add_patch(FancyBboxPatch((x - w / 2, y - h / 2), w, h, boxstyle="round,pad=0.02",
                                fc=fc, ec=ec, lw=1.1))
    ax.text(x, y, text, ha="center", va="center", fontsize=fs)


def arrow(ax, x0, y0, x1, y1, label=None, color="#333"):
    ax.annotate("", xy=(x1, y1), xytext=(x0, y0),
                arrowprops=dict(arrowstyle="-|>", color=color, lw=1.1))
    if label:
        ax.text((x0 + x1) / 2, (y0 + y1) / 2 + 0.12, label, ha="center", fontsize=7.5, color=color)


def pipeline():
    cm = load("cm_p1000003.json")
    t = cm["tower"]
    aux = t["kind"]["aux"]
    fig, ax = plt.subplots(figsize=(10, 7.2))
    ax.set_xlim(0, 10)
    ax.set_ylim(0, 7.4)
    ax.axis("off")
    ax.text(5, 7.2, "PKM factor base from smooth maps (pkm-smooth-tower)", ha="center",
            fontsize=12, weight="bold")

    # Row 1: input and the three routes
    box(ax, 5, 6.55, 5.4, 0.45, "input: prime p (given), target size S ≈ p^{1/m}, smoothness bound B")
    box(ax, 1.7, 5.45, 3.0, 0.85,
        "Kummer form (slide 14)\nS | p − 1\nL_j(x) = x^{q_j},  V = g·μ_S", fc="#f3f8ec", ec="#4a7d2a")
    box(ax, 5.0, 5.45, 3.0, 0.85,
        "Method 1 (slide 16)\nrandom E'/F_p until #E' has a\nB-smooth cyclic part S", fc="#fff5e8", ec="#b36b00")
    box(ax, 8.3, 5.45, 3.0, 0.85,
        "Method 2, CM (slide 16), p fixed\nclass number 1: 4p = t² + |D|v²\n(Cornacchia), all twists", fc="#fff5e8",
        ec="#b36b00")
    for x in (1.7, 5.0, 8.3):
        arrow(ax, 5 if x == 5.0 else (4.0 if x < 5 else 6.0), 6.32, x, 5.9)

    # Row 2: auxiliary curve
    box(ax, 6.65, 4.25, 5.0, 0.7,
        "auxiliary curve E': #E'(F_p) = S·N',  G = ⟨Q⟩ cyclic of order S = ∏ q_j\n"
        "R' with 2R' ∉ G,   V = x(R' + G),   |V| = S", fc="#eef3fb")
    arrow(ax, 5.0, 5.0, 5.6, 4.62)
    arrow(ax, 8.3, 5.0, 7.7, 4.62)

    # Row 3: the chain, concrete example
    degs = t["degrees"]
    ax.text(6.6, 3.6, "isogeny chain (slide 15), recorded example", ha="center", fontsize=8.5, weight="bold")
    ax.text(6.6, 3.36, f"p = {cm['p']}, D = {cm['tower_D']}, E': a = {aux['a']}, b = {aux['b']}, "
            f"#E' = {aux['order']}, S = {t['S']} = 2^7·11", ha="center", fontsize=7.5, style="italic")
    n = len(degs)
    x0, x1 = 3.65, 9.75
    xs = [x0 + i * (x1 - x0) / n for i in range(n + 1)]
    for i, x in enumerate(xs):
        lbl = "E'" if i == 0 else f"E'{i + 1}"
        box(ax, x, 2.9, 0.44, 0.34, lbl, fc="white", fs=7.5)
        if i < n:
            col = "#8a1c1c" if degs[i] != 2 else "#333"
            arrow(ax, x + 0.23, 2.9, xs[i + 1] - 0.23, 2.9, color=col)
            ax.text((x + xs[i + 1]) / 2, 3.07, f"{degs[i]}", ha="center", fontsize=7.5, color=col)
    arrow(ax, 6.65, 3.9, 6.65, 3.72)
    ax.text(6.6, 2.45, "edge label = deg φ_j;  L_j = x-map N_j/D_j of φ_j (Vélu)", ha="center", fontsize=7.5)
    ax.text(6.6, 2.25, f"L = L_n' ∘ … ∘ L_1,   c = x(Φ(R')) = {t['c']}", ha="center", fontsize=7.5)
    arrow(ax, 1.7, 5.0, 1.7, 2.0, label=None)
    ax.text(1.8, 3.4, "L_j(x) = x^{q_j}\nc = g^S", fontsize=7.5, color="#4a7d2a")

    # Row 4: factor base and system
    box(ax, 2.6, 1.6, 4.2, 0.75,
        "verify: |V| = S distinct, L(v) = c on V,\nno other root in F_p (p < 2^22)\n"
        "factor base F = {(x, y) ∈ E(F_p) : x ∈ V}", fc="#f3f8ec", ec="#4a7d2a")
    box(ax, 7.4, 1.6, 4.6, 0.75,
        "relation system (slide 17), msolve format\nS_{m+1}(x_{1,1},…,x_{m,1}, X) as an S_3 chain\n"
        "x_{i,j+1}·D_j(x_{i,j}) = N_j(x_{i,j}),  L_n'(x_{i,n'}) = c")
    arrow(ax, 4.6, 2.15, 3.6, 1.98)
    arrow(ax, 4.7, 1.6, 5.1, 1.6)
    box(ax, 7.4, 0.55, 4.6, 0.5, "m = 2: exact finder over V, every relation checked by point addition",
        fc="white", fs=8)
    arrow(ax, 7.4, 1.22, 7.4, 0.8)
    ax.text(0.1, 0.12, f"Source: research/pkm_smooth_tower_20261114/data/cm_p1000003.json "
            "(pkm-smooth-tower at a894bbdaa9). Red arrows: odd-degree steps.", fontsize=7, color="#555")
    fig.savefig(os.path.join(HERE, "pipeline.svg"), bbox_inches="tight")
    fig.savefig(os.path.join(HERE, "pipeline.png"), dpi=150, bbox_inches="tight")


def cm_orders():
    cm = load("cm_p1000003.json")
    cs = cm["curves"]
    fig, ax = plt.subplots(figsize=(8, 3.6))
    labels = [f"D={c['D']}\nt={c['trace']}" for c in cs]
    sm = [math.log2(c["smooth_part"]) for c in cs]
    ax.bar(range(len(cs)), sm, color=["#2c5aa0" if c["smooth_part"] >= 300 else "#9bb3d6" for c in cs])
    ax.axhline(math.log2(300), color="#b36b00", ls="--", lw=1)
    ax.text(len(cs) - 0.5, math.log2(300) + 0.2, "target S ≥ 300", ha="right", color="#b36b00", fontsize=8)
    ax.set_xticks(range(len(cs)))
    ax.set_xticklabels(labels, fontsize=6, rotation=90)
    ax.set_ylabel("log2 of 13-smooth part of #E")
    ax.set_title(f"Method 2 (CM) at p = {cm['p']}: {len(cs)} class-number-1 CM curves, "
                 "smooth part of each order (recorded)", fontsize=9)
    fig.savefig(os.path.join(HERE, "cm_orders.svg"), bbox_inches="tight")
    fig.savefig(os.path.join(HERE, "cm_orders.png"), dpi=150, bbox_inches="tight")


def method1_trials():
    runs = load("method1_p1000003_seeds1-20.json")
    tries = [r["curves_tried"] for r in runs]
    fig, ax = plt.subplots(figsize=(6, 3))
    ax.bar([i + 1 for i in range(len(runs))], tries, color="#b36b00")
    ax.set_xticks(range(1, len(runs) + 1))
    ax.set_xlabel("seed")
    ax.set_ylabel("curves tried")
    mean = sum(tries) / len(tries)
    ax.axhline(mean, color="#333", ls=":", lw=1)
    ax.text(20.4, mean + 1, f"mean {mean:.1f}", ha="right", fontsize=8)
    ax.set_title("Method 1 at p = 1000003, B = 13, S ≥ 500: curves tried per seed (n = 20, recorded)",
                 fontsize=8.5)
    fig.savefig(os.path.join(HERE, "method1_trials.svg"), bbox_inches="tight")
    fig.savefig(os.path.join(HERE, "method1_trials.png"), dpi=150, bbox_inches="tight")


if __name__ == "__main__":
    pipeline()
    cm_orders()
    method1_trials()
