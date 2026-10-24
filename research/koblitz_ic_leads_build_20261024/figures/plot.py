"""Layout only: draws the committed records in ../runs/. Computes no result.

    python3 plot.py   (from this directory)
"""
import glob
import json
import math
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

INK, INK2, GRID, SURF = "#0b0b0b", "#52514e", "#e4e3df", "#fcfcfb"
BLUE, ORANGE, AQUA = "#2a78d6", "#eb6834", "#1baf7a"
RUNS = os.path.join(os.path.dirname(__file__) or ".", "..", "runs")
plt.rcParams.update({"font.size": 9, "axes.edgecolor": INK2, "axes.labelcolor": INK,
                     "xtick.color": INK2, "ytick.color": INK2, "svg.fonttype": "none"})


def load(pattern):
    out = []
    for p in sorted(glob.glob(os.path.join(RUNS, pattern))):
        with open(p) as f:
            out += [json.loads(l) for l in f if l.strip()]
    return out


def style(ax):
    ax.set_facecolor(SURF)
    ax.grid(True, color=GRID, lw=0.6)
    for s in ("top", "right"):
        ax.spines[s].set_visible(False)


def fig_l2():
    part = [r for r in load("g1-*.jsonl") if r.get("built")]
    full = [r for r in load("p1full-*.jsonl") if r.get("built")]
    cells = sorted({r["cell"] for r in part}, key=lambda c: (c.endswith("m4"), c))
    fig, ax = plt.subplots(figsize=(7.2, 3.8), facecolor=SURF)
    style(ax)
    xs = {c: i for i, c in enumerate(cells)}
    for rows, col, mark, lab, dx in ((part, BLUE, "o", "partial enlargement", -0.12),
                                     (full, ORANGE, "s", "full order (same engine)", 0.12)):
        x = [xs[r["cell"]] + dx for r in rows]
        y = [r["peak_basis_rows"] / r["columns"] for r in rows]
        ax.scatter(x, y, s=34, color=col, marker=mark, label=lab, zorder=3,
                   edgecolor=SURF, linewidth=1.5)
    for v, lab in ((0.5, "falsifier 0.5"), (0.25, "payoff 0.25")):
        ax.axhline(v, color=INK2, lw=1, ls=(0, (6, 3)))
        ax.text(len(cells) - 0.55, v + 0.015, lab, color=INK2, fontsize=8, ha="right")
    ax.set_xlim(-0.5, len(cells) - 0.5)
    ax.set_xticks(range(len(cells)))
    names = {"n22l6": "m=3 (22,6)", "n24l6": "m=3 (24,6)", "n11l5m4": "m=4 (11,5)",
             "n12l5m4": "m=4 (12,5)"}
    ax.set_xticklabels([names.get(c, c) for c in cells])
    ax.set_ylim(0, 1.08)
    ax.set_ylabel("peak basis rows / columns at refutation")
    ax.legend(loc="lower left", frameon=False, fontsize=8)
    ax.set_title("L2: partial enlargement fills the space before 1 appears (degree 4)",
                 loc="left", fontsize=10, color=INK)
    fig.text(0.01, 0.01, f"Recorded: runs/g1-*.jsonl ({len(part)} draws), runs/p1full-*.jsonl "
             f"({len(full)} draws). Each point is one draw.", fontsize=7, color=INK2)
    fig.tight_layout(rect=(0, 0.04, 1, 1))
    fig.savefig("l2_peak_rows.svg")
    fig.savefig("l2_peak_rows.png", dpi=150)


def fig_l3():
    rows = load("l3-gluing.jsonl")
    fig, axes = plt.subplots(1, 2, figsize=(7.2, 3.4), facecolor=SURF, sharey=True)
    for ax, m in zip(axes, (3, 4)):
        style(ax)
        sel = [r for r in rows if r["m"] == m]
        ells = sorted({r["ell"] for r in sel})
        for key, col, lab in (("pairs", ORANGE, "pairs-and-solve"),
                              ("gluing", BLUE, "Agreeing–Gluing")):
            y = [math.log2(sum(r[key]["root_solves"] for r in sel if r["ell"] == e) /
                           sum(1 for r in sel if r["ell"] == e)) for e in ells]
            ax.plot(ells, y, color=col, lw=2, marker="o", ms=5, label=lab)
        ax.plot(ells, [2 * e for e in ells], color=INK2, lw=1, ls=(0, (6, 3)), label="2ℓ")
        ax.set_xlabel("ℓ (subspace dimension)")
        ax.set_title(f"m = {m}, n = {m}ℓ", loc="left", fontsize=9, color=INK)
    axes[0].set_ylabel("log₂ root solves per target (mean of 4)")
    axes[0].legend(frameon=False, fontsize=8)
    fig.suptitle("L3: Agreeing–Gluing is the pair-table meet-in-the-middle", x=0.01,
                 ha="left", fontsize=10, color=INK)
    fig.text(0.01, 0.01, "Recorded: runs/l3-gluing.jsonl, 52 draws; both solvers return the "
             "same solution count on every draw.", fontsize=7, color=INK2)
    fig.tight_layout(rect=(0, 0.04, 1, 0.95))
    fig.savefig("l3_root_solves.svg")
    fig.savefig("l3_root_solves.png", dpi=150)


def fig_l4():
    rows = [r for r in load("l4-bounds.jsonl") if r["macaulay_bound"] is not None]
    fig, ax = plt.subplots(figsize=(7.2, 3.4), facecolor=SURF)
    style(ax)
    labels = [f"m={r['m']} ({r['n']},{r['ell']})" for r in rows]
    x = range(len(rows))
    ax.bar([i - 0.18 for i in x], [r["macaulay_bound"] for r in rows], width=0.36,
           color=ORANGE, label="Macaulay bound (Caminata–Gorla)")
    meas = [5 if r["measured_closure_degree"].startswith(">=") else
            int(r["measured_closure_degree"]) for r in rows]
    ax.bar([i + 0.18 for i in x], meas, width=0.36, color=BLUE,
           label="measured closure degree (≥5 drawn as 5)")
    ax.set_xticks(list(x))
    ax.set_xticklabels(labels, rotation=30, ha="right", fontsize=8)
    ax.set_ylabel("degree")
    ax.legend(frameon=False, fontsize=8, loc="upper left")
    ax.set_title("L4: the proved bound sits 12–22× above the measured degree", loc="left",
                 fontsize=10, color=INK)
    fig.text(0.01, 0.01, "Recorded: runs/l4-bounds.jsonl; measured degrees from the cited "
             "closure studies.", fontsize=7, color=INK2)
    fig.tight_layout(rect=(0, 0.04, 1, 1))
    fig.savefig("l4_bounds.svg")
    fig.savefig("l4_bounds.png", dpi=150)


if __name__ == "__main__":
    fig_l3()
    fig_l4()
    if glob.glob(os.path.join(RUNS, "g1-*.jsonl")):
        fig_l2()
