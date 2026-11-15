#!/usr/bin/env python3
"""Render report figures from the committed frontier snapshot only."""

from __future__ import annotations

import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


ROOT = Path(__file__).resolve().parent
OUT = ROOT / "figures"
DATA = json.loads((ROOT / "frontier_snapshot.json").read_text())

NAVY = "#17324d"
BLUE = "#2878b5"
TEAL = "#2a9d8f"
ORANGE = "#e07a1f"
RED = "#c44e52"
PURPLE = "#7a5195"
GREY = "#6b7280"
PALE = "#e8eef3"


def style() -> None:
    plt.rcParams.update({
        "font.family": "DejaVu Sans",
        "font.size": 9.5,
        "axes.titlesize": 13,
        "axes.labelsize": 10,
        "axes.edgecolor": "#52606d",
        "axes.linewidth": 0.8,
        "grid.color": "#d8dee4",
        "grid.linewidth": 0.7,
        "legend.frameon": False,
        "figure.facecolor": "white",
        "axes.facecolor": "white",
    })


def save(fig: plt.Figure, name: str) -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    fig.savefig(OUT / f"{name}.png", dpi=210, bbox_inches="tight")
    fig.savefig(OUT / f"{name}.pdf", bbox_inches="tight")
    plt.close(fig)


def boundary_map() -> None:
    rows = DATA["boundary_markers"]
    fig, ax = plt.subplots(figsize=(10.5, 5.2))
    y = np.arange(len(rows))[::-1]
    ax.axvspan(0.003, 1, color="#e7f5ef", zorder=0)
    ax.axvspan(1, 100, color="#fff1ed", zorder=0)
    ax.axvline(1, color=NAVY, lw=1.4, ls="--")
    for yi, row in zip(y, rows):
        ratio = row["ratio"]
        restricted = row["domain"].startswith("restricted")
        color = TEAL if restricted else (RED if ratio > 1 else BLUE)
        if "ci95" in row:
            lo, hi = row["ci95"]
            ax.errorbar(ratio, yi, xerr=[[ratio - lo], [hi - ratio]], fmt="o",
                        ms=7, color=color, capsize=3, lw=1.4, zorder=3)
        else:
            marker = "D" if restricted else "o"
            ax.scatter(ratio, yi, s=48, marker=marker, color=color, zorder=3)
        ax.text(ratio * (1.12 if ratio < 1 else 1.06), yi,
                f"{ratio:g}×", va="center", color=color, fontweight="bold")
    ax.set_xscale("log")
    ax.set_xlim(0.003, 110)
    ax.set_yticks(y, [r["label"] for r in rows])
    ax.set_xlabel("candidate / matched reference (log scale; parity = 1)")
    ax.set_title("Observed boundary markers by exact problem class")
    ax.text(0.015, len(rows) - 0.15, "candidate lower", color=TEAL, fontweight="bold")
    ax.text(2.0, len(rows) - 0.15, "reference lower", color=RED, fontweight="bold")
    ax.grid(True, which="both", axis="x")
    ax.text(0.0, -0.22,
            "Ratios are meaningful only within each row's domain and unit. GHS-weak is restricted-family; generator-orbit is fixed-setup BSGS engineering.",
            transform=ax.transAxes, color=GREY, fontsize=8.5)
    save(fig, "boundary-map")


def prime_ladder() -> None:
    d = DATA["prime_bitop_ladder"]
    bits = np.array(d["bits"])
    fig, ax = plt.subplots(figsize=(10.5, 5.4))
    series = [
        ("rho.negation", d["rho_neg"], NAVY, "--", 1.8),
        ("BSGS negation", d["bsgs_neg"], TEAL, "-", 1.7),
        ("kangaroo", d["kangaroo"], PURPLE, "-", 1.5),
        ("IC pair rule", d["ic_pair_rule"], ORANGE, "-", 2.0),
        ("IC rootfind m=3", d["ic_rootfind_m3_rule"], RED, "-", 2.0),
    ]
    for label, values, color, ls, lw in series:
        vals = np.array([np.nan if v is None else v for v in values], dtype=float)
        ax.plot(bits, vals, label=label, color=color, ls=ls, lw=lw,
                marker="o" if label.startswith("IC") else None, ms=3.2)
    ax.axhline(1, color=NAVY, lw=1, ls=":")
    ax.set_yscale("log")
    ax.set_xlim(16, 40)
    ax.set_ylim(0.45, 110)
    ax.set_xticks(np.arange(16, 41, 2))
    ax.set_xlabel("prime subgroup size (bits)")
    ax.set_ylabel("mean bitop/v1 cost / rho.negation")
    ax.set_title("Prime generic track: optimized references and IC distance")
    ax.grid(True, which="both")
    ax.legend(ncol=2, loc="upper left")
    ax.text(0.0, -0.2,
            "Public synthetic a=-3 prime-order curves; counted bitop/v1 only. BSGS is the finite-size leader on several toy cells.",
            transform=ax.transAxes, color=GREY, fontsize=8.5)
    save(fig, "prime-bitop-ladder")


def backend_tradeoff() -> None:
    d = DATA["f4_f5_f6"]
    cells = d["cells"]
    x = np.arange(len(cells))
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(11.5, 4.9), gridspec_kw={"width_ratios": [1, 1.15]})

    f5 = np.array([np.nan if v is None else v for v in d["f5_over_f4_word_work"]])
    f6_factor = np.power(2.0, -np.array(d["f6_saving_log2_inherited_over_f6"]))
    ax1.bar(x - 0.17, f5, width=0.34, label="F5 / matrix F4 word work", color=BLUE)
    ax1.bar(x + 0.17, f6_factor, width=0.34, label="F6 / inherited-F4 word work", color=TEAL)
    ax1.axhline(1, color=NAVY, lw=1, ls="--")
    ax1.set_xticks(x, cells)
    ax1.set_ylim(0, 1.08)
    ax1.set_ylabel("backend-native work ratio")
    ax1.set_title("Local solver improvements")
    ax1.grid(True, axis="y")
    ax1.legend(fontsize=8)

    e2e = np.array(d["f6_over_inherited_s"])
    rho = np.array(d["f6_over_rho_neg"])
    ax2.plot(x, e2e, marker="o", lw=2, color=ORANGE, label="F6 / inherited F4, S")
    ax2.plot(x, rho, marker="o", lw=2, color=RED, label="F6 / rho.negation, S")
    ax2.axhline(1, color=NAVY, lw=1, ls="--")
    ax2.set_yscale("log")
    ax2.set_xticks(x, cells)
    ax2.set_ylabel("whole-pipeline lower-bound S ratio")
    ax2.set_title("Why the E2E frontier does not move")
    ax2.grid(True, which="both", axis="y")
    ax2.legend(fontsize=8)

    fig.suptitle("F4/F5/F6: fewer solver operations are not an ECDLP speedup", y=1.02, fontsize=13)
    fig.text(0.5, -0.04,
             "K/R = Koblitz/random binary; 13/19 = extension degree. Native word work and S are deliberately shown on separate axes.",
             ha="center", color=GREY, fontsize=8.5)
    save(fig, "solver-tradeoff")


def ghs_boundary() -> None:
    rows = DATA["ghs_weak"]["b0_plain_w0"]
    q = np.array([r["q"] for r in rows])
    ratio = np.array([r["ratio"] for r in rows])
    fig, ax = plt.subplots(figsize=(9.8, 5.0))
    ax.loglog(q, ratio, marker="o", color=TEAL, lw=2, label="B0 / plain rho, w=0")
    ax.axhline(1, color=NAVY, lw=1.4, ls="--", label="parity")
    ax.scatter([DATA["ghs_weak"]["n4v2_dl"]["q"]],
               [DATA["ghs_weak"]["n4v2_dl"]["over_negation_rho_w0"]],
               marker="D", s=70, color=PURPLE, zorder=4, label="N4 v2 DL / negation rho")
    ax.set_xlabel("base field q")
    ax.set_ylabel("IC / matched rho")
    ax.set_title("Restricted GHS-weak family: the separate IC-led frontier")
    ax.grid(True, which="both")
    ax.legend()
    ax.text(0.0, -0.22,
            "Only E0/Fq^3 with #E0(K)=4ℓ. Not generic ECDLP, prime-order E/Fq^3, standard curves, or ECC2K-130.",
            transform=ax.transAxes, color=GREY, fontsize=8.5)
    save(fig, "ghs-weak-frontier")


def main() -> None:
    style()
    boundary_map()
    prime_ladder()
    backend_tradeoff()
    ghs_boundary()


if __name__ == "__main__":
    main()
