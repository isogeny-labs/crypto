"""Render the figures of the index-calculus improvement-ledger paper from data.json.

Plots recorded values only (AGENTS.md: Python lays out what was recorded; it
computes no result).  Each figure is written as PDF (for LaTeX), SVG and a
150 dpi PNG from the same source.

    python3 make_figures.py
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data.json")))

plt.rcParams.update({
    "font.size": 8.5,
    "font.family": "serif",
    "axes.spines.top": False,
    "axes.spines.right": False,
    "axes.grid": True,
    "grid.alpha": 0.25,
    "svg.fonttype": "none",
})

# Okabe-Ito, colour-blind safe; one palette for the whole paper.
C = ["#0072B2", "#E69F00", "#009E73", "#CC79A7", "#56B4E9", "#D55E00", "#999999"]
INK = "#222222"


def save(fig, name):
    for ext in ("pdf", "svg"):
        fig.savefig(os.path.join(HERE, f"{name}.{ext}"), bbox_inches="tight")
    fig.savefig(os.path.join(HERE, f"{name}.png"), dpi=150, bbox_inches="tight")
    plt.close(fig)


def parity(ax):
    ax.axhline(1.0, color=INK, lw=0.9, ls="--", zorder=1)
    ax.text(ax.get_xlim()[1], 1.0, " rho parity", va="center", ha="left",
            fontsize=7.5, color=INK)


def fig_koblitz_rule():
    k = D["koblitz_rule"]
    x = k["log2r"]
    fig, ax = plt.subplots(figsize=(6.4, 3.0))
    ax.plot(x, k["v3"], "s--", color=C[6], ms=4, lw=1, label="v3 (old rho)")
    ax.plot(x, k["v7"], "o--", color=C[4], ms=4, lw=1, label="v7 (IC 1.52-1.89x faster, same rho)")
    ax.plot(x, k["v8c"], "^:", color=C[1], ms=4, lw=1, label="v8c control (rho as at v7)")
    lo = [m - l for m, l in zip(k["v8"], k["v8_lo"])]
    hi = [h - m for m, h in zip(k["v8"], k["v8_hi"])]
    ax.errorbar(x, k["v8"], yerr=[lo, hi], fmt="o-", color=C[0], ms=5, lw=1.6,
                capsize=2.5, label="v8, matched rho (95% CI)", zorder=4)
    ax.plot(x, k["canonical_step"], "d-.", color=C[5], ms=4, lw=1,
            label="model: rho at canonical step")
    for i, (xi, n) in enumerate(zip(x, k["n"])):
        ax.annotate(f"n={n}", (xi, 2.3 if i != 4 else 1.9), ha="center",
                    fontsize=6.5, color=INK)
    ax.set_yscale("log")
    ax.set_ylim(1.0 / 1.15, 40)
    ax.set_xlabel(r"$\log_2 r$")
    ax.set_ylabel("cold IC / cold rho (same point)")
    parity(ax)
    ax.legend(fontsize=6.8, loc="upper left", frameon=False)
    save(fig, "fig-koblitz-rule")


def fig_families():
    f = D["families"]
    fig, ax = plt.subplots(figsize=(6.4, 3.2))
    kc = f["koblitz_campaign"]
    ax.plot(kc["log2r"], kc["ratio"], "o-", color=C[0], ms=4, label="Koblitz compact orbit (campaign, w=0.1)")
    kr = D["koblitz_rule"]
    ax.plot(kr["log2r"], kr["v8"], "o:", color=C[0], mfc="white", ms=4, label="Koblitz descent, rule v8 (cold, matched)")
    pc = f["prime_campaign"]
    ax.plot(pc["log2r"], pc["ratio"], "s-", color=C[5], ms=4, label="prime pair table (campaign, w=0.1)")
    pp = f["prime_planner"]
    ax.plot(pp["log2r"], pp["ratio"], "s:", color=C[5], mfc="white", ms=4, label="prime, planner cells (w=0)")
    k4 = f["k4_char2"]
    ax.plot(k4["log2n"], k4["ratio_theory"], "D-", color=C[2], ms=4, label=r"$k=4$ over $\mathbb{F}_{(2^m)^4}$ / rho model")
    ax.plot(k4["log2n"], k4["ratio_measured"], "D:", color=C[2], mfc="white", ms=4, label=r"$k=4$ / measured rho")
    ax.set_yscale("log")
    ax.set_xlim(8, 60)
    ax.set_ylim(0.8, 1e4)
    ax.set_xlabel(r"$\log_2$ of the subgroup order")
    ax.set_ylabel("IC / matched rho")
    parity(ax)
    ax.legend(fontsize=6.6, loc="upper right", frameon=False)
    save(fig, "fig-families")


def fig_ratchet():
    rows = D["ratchet"]["rows"]
    groups = []
    for r in rows:
        if r["group"] not in groups:
            groups.append(r["group"])
    fig, ax = plt.subplots(figsize=(6.4, 6.6))
    y = 0
    yt, yl = [], []
    for gi, g in enumerate(groups):
        for r in [r for r in rows if r["group"] == g]:
            filled = r["level"] == "whole"
            col = C[gi % len(C)]
            ax.plot([r["lo"], r["hi"]], [y, y], color=col, lw=1.6, zorder=2)
            for v in ([r["lo"], r["hi"]] if r.get("range") else [r["ratio"]]):
                ax.plot(v, y, "o" if filled else "s", ms=5, color=col,
                        mfc=col if filled else "white", zorder=3)
            yt.append(y)
            yl.append(r["label"])
            y -= 1
        y -= 0.6
    ax.set_xscale("log")
    ax.set_yticks(yt)
    ax.set_yticklabels(yl, fontsize=6.6)
    ax.axvline(1.0, color=INK, lw=0.9, ls="--")
    ax.set_xlabel("candidate / incumbent (log scale; < 1 is an improvement)")
    ax.set_xlim(3e-6, 2)
    ax.grid(axis="y", alpha=0)
    from matplotlib.lines import Line2D
    ax.legend(handles=[
        Line2D([], [], marker="o", ls="", color=INK, label="whole solve or cold S"),
        Line2D([], [], marker="s", ls="", color=INK, mfc="white", label="one stage"),
    ], loc="lower left", fontsize=7, frameon=False)
    save(fig, "fig-ratchet")


def fig_audit():
    rows = D["audit"]["rows"]
    fig, ax = plt.subplots(figsize=(6.4, 2.9))
    for i, r in enumerate(rows):
        y = -i
        ax.plot([r["before"], r["after"]], [y, y], color=C[6], lw=1.2, zorder=1)
        ax.plot(r["before"], y, "o", color=C[1], ms=5, zorder=2)
        ax.plot(r["after"], y, "o", color=C[0], ms=5, zorder=2)
    ax.set_yticks([-i for i in range(len(rows))])
    ax.set_yticklabels([r["label"] for r in rows], fontsize=6.6)
    ax.set_xscale("log")
    ax.axvline(1.0, color=INK, lw=0.9, ls="--")
    ax.set_xlabel("IC / rho (log scale; > 1 means rho is cheaper)")
    ax.grid(axis="y", alpha=0)
    from matplotlib.lines import Line2D
    ax.legend(handles=[
        Line2D([], [], marker="o", ls="", color=C[1], label="as first recorded"),
        Line2D([], [], marker="o", ls="", color=C[0], label="after correction"),
    ], loc="lower right", fontsize=7, frameon=False)
    save(fig, "fig-audit")


if __name__ == "__main__":
    fig_koblitz_rule()
    fig_families()
    fig_ratchet()
    fig_audit()
