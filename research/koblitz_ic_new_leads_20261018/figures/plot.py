"""Layout only: draws figures/columns.csv (written by check/columns.rs) and the
lead-screening table in ../README.md. Computes no research result.

    /tmp/report-venv/bin/python plot.py   (from this directory)
"""
import csv

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
from matplotlib.patches import FancyBboxPatch  # noqa: E402

INK, INK2, GRID, SURF = "#0b0b0b", "#52514e", "#e4e3df", "#fcfcfb"
BLUE, ORANGE, AQUA = "#2a78d6", "#eb6834", "#1baf7a"
GIB = 1 << 30
plt.rcParams.update({"font.size": 9, "axes.edgecolor": INK2, "axes.labelcolor": INK,
                     "xtick.color": INK2, "ytick.color": INK2, "svg.fonttype": "none"})


def memory_figure():
    rows = list(csv.DictReader(open("columns.csv")))
    fig, ax = plt.subplots(figsize=(7.2, 4.3), facecolor=SURF)
    ax.set_facecolor(SURF)
    for d, col, ls in ((4, BLUE, "-"), (5, ORANGE, "-"), (6, AQUA, "-")):
        pts = [(int(r["N"]), int(r["dense_full_bytes"]) / GIB) for r in rows if int(r["D"]) == d]
        ax.plot(*zip(*pts), color=col, lw=2, ls=ls, marker="o", ms=4)
        ax.annotate(f"degree {d}, full dense echelon", pts[-1], xytext=(4, 0),
                    textcoords="offset points", va="center", color=INK, fontsize=8)
    for frac, key, ls in ((0.25, "rank_0.25_bytes", "--"), (0.10, "rank_0.10_bytes", ":")):
        pts = [(int(r["N"]), int(r[key]) / GIB) for r in rows if int(r["D"]) == 5]
        ax.plot(*zip(*pts), color=ORANGE, lw=1.5, ls=ls)
        ax.annotate(f"degree 5, rank = {frac:g} x cols (hypothetical L2)", pts[-1],
                    xytext=(4, 9 if frac == 0.25 else -2),
                    textcoords="offset points", va="center", color=INK2, fontsize=8)
    for cap, label in ((15, "this container: 15 GB"), (250, "registered big node: 250 GB")):
        ax.axhline(cap * 1e9 / GIB, color=INK2, lw=1, ls=(0, (6, 3)))
        ax.text(30.2, cap * 1e9 / GIB * 1.12, label, color=INK2, fontsize=8)
    ax.plot([40], [72218811225 / GIB], marker="s", ms=8, mfc="none", mec=INK, mew=1.5)
    ax.annotate("recorded: N = 40, degree 5,\n760k columns, about 70 GB\n(dreg_closure_driver_20261016)",
                (40, 72218811225 / GIB), xytext=(44.3, 0.16), textcoords="data",
                fontsize=8, color=INK, arrowprops=dict(arrowstyle="-", color=INK2, lw=0.8))
    ax.set_yscale("log")
    ax.set_xlim(30, 61)
    ax.set_xticks(range(30, 53, 2))
    ax.set_xlabel("unknowns N = n + m·ℓ of the descended system")
    ax.set_ylabel("dense GF(2) echelon memory (GiB, log scale)")
    ax.grid(True, which="major", color=GRID, lw=0.6)
    for s in ("top", "right"):
        ax.spines[s].set_visible(False)
    ax.set_title("Lead L2: where a partial-enlargement or memory-frugal F4 closure matters",
                 loc="left", fontsize=10, color=INK)
    fig.text(0.01, 0.005, "Derived: cols = sum over k <= D of C(N, k); bytes = rank x cols / 8 "
             "(check/columns.rs). Rank fractions are hypothetical, not measured.",
             fontsize=7, color=INK2)
    fig.tight_layout(rect=(0, 0.03, 1, 1))
    fig.savefig("memory_reach.svg")
    fig.savefig("memory_reach.png", dpi=150)


SCREEN = [
    # (label, status, note)
    ("λ-coordinate factor bases", "new: closed on arrival", "L1, derived + native check"),
    ("Partial enlargement (MXL3/MGB), YAGS", "new: instrument", "L2, memory wall at degree 5"),
    ("Agreeing–Gluing on the S₃ chain", "new: proved cost model", "L3, sparse-system bounds"),
    ("Iterated-system solving-degree bounds", "new: desk check", "L4, Steiner ToSC 2024"),
    ("u = x + 1/x quotient base", "covered", "golden_factor_base_sat_20260921"),
    ("(π − 1) transport, Z/4 on a = 0", "covered", "RESEARCH_EXOTIC_COORDINATES §10–11"),
    ("Transformer border-basis oracle", "queued", "lit2026_speedup_program W9"),
    ("HPST / Kodera–Cheng–Miyaji models", "covered", "symmetrisation, exotic coordinates"),
    ("Gorla–Mueller–Petit HFE bounds", "cited", "CORR-20260916-0c9c0a"),
    ("2023–26 GB, Crossbred, GGM literature", "covered", "literature_search_20261011 (61 papers)"),
]


def lead_map():
    fig, ax = plt.subplots(figsize=(7.2, 4.6), facecolor=SURF)
    ax.set_facecolor(SURF)
    ax.set_xlim(0, 10)
    ax.set_ylim(-0.6, len(SCREEN) + 0.6)
    ax.axis("off")
    colours = {"new": BLUE, "covered": INK2, "queued": INK2, "cited": INK2}
    for i, (label, status, note) in enumerate(SCREEN):
        y = len(SCREEN) - 1 - i
        kind = status.split(":")[0]
        c = colours[kind]
        new = kind == "new"
        ax.add_patch(FancyBboxPatch((0.05, y - 0.38), 4.1, 0.76, boxstyle="round,pad=0.02,rounding_size=0.08",
                                    fc="#eaf2fb" if new else "#f1f0ed", ec=c, lw=1.4 if new else 0.8))
        ax.text(0.2, y, label, va="center", fontsize=8.5, color=INK, weight="bold" if new else "normal")
        ax.annotate("", xy=(4.75, y), xytext=(4.2, y), arrowprops=dict(arrowstyle="->", color=c, lw=1))
        ax.text(4.85, y + 0.13, status, va="center", fontsize=8.5, color=c if new else INK2,
                weight="bold" if new else "normal")
        ax.text(4.85, y - 0.2, note, va="center", fontsize=7.5, color=INK2)
    ax.set_title("Candidate leads screened against the four repositories, 2026-10-18",
                 loc="left", fontsize=10, color=INK)
    fig.text(0.01, 0.01, "Blue: no prior record in crypto, cryptanalysis, crypto-autoresearcher or "
             "ml-cryptanalysis. Grey: already examined, queued or cited; pointer given.",
             fontsize=7, color=INK2)
    fig.tight_layout(rect=(0, 0.03, 1, 1))
    fig.savefig("lead_map.svg")
    fig.savefig("lead_map.png", dpi=150)


if __name__ == "__main__":
    memory_figure()
    lead_map()
