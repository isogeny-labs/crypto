"""Graphs for the S2b recorded run (record 20261114T2233Z-hec-s2b-dense-solve).

Layout only: every plotted number is a field of ../../s2b-20261114/summary.jsonl
or ../../s2b-20261114/pairs.jsonl, which summarise_s2b.sh and pairs_s2b.sh
derive from the raw run output. Writes <name>.svg and a 150 dpi <name>.png.

    python3 plots.py
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
DATA = HERE / "../../s2b-20261114"
ROWS = [json.loads(l) for l in (DATA / "summary.jsonl").read_text().splitlines() if l.strip()]
PAIRS = [json.loads(l) for l in (DATA / "pairs.jsonl").read_text().splitlines() if l.strip()]
LABELS = [f"g{r['genus']}\np={r['p']}" for r in ROWS]
X = list(range(len(ROWS)))
RUN = ("record 20261114T2233Z-hec-s2b-dense-solve; 5 repeats x 5 seeds = 25 pairs per row; "
       "arm64 macOS, 1 thread, shared host (load average 81-95)")
C_FAST, C_DENSE = "#2f6fb0", "#3a9a5b"


def save(fig, name):
    fig.tight_layout()
    fig.savefig(HERE / f"{name}.svg")
    fig.savefig(HERE / f"{name}.png", dpi=150)
    plt.close(fig)


def ratios():
    fig, ax = plt.subplots(figsize=(9, 4.4))
    for x, r in zip(X, ROWS):
        ys = [q["ratio"] for q in PAIRS if q["genus"] == r["genus"] and q["p"] == r["p"]]
        ax.scatter([x] * len(ys), ys, s=9, color="#999", alpha=0.6, zorder=2)
        ax.plot([x - 0.3, x + 0.3], [r["ratio_med"]] * 2, color=C_DENSE, lw=2.5, zorder=3)
        ax.text(x + 0.32, r["ratio_med"], f"{r['ratio_med']:.1f}×", va="center", fontsize=8)
    ax.axhline(2, color="#b00", lw=1, ls="--")
    ax.text(-0.4, 1.6, "E6 bar 2× (genus 2, p ≥ 503)", color="#b00", fontsize=8)
    ax.set_yscale("log")
    ax.set_xticks(X, LABELS, fontsize=8)
    ax.set_ylabel("solve_ms(Fast) / solve_ms(Dense), per pair (log)")
    ax.set_title("S2b: paired solve ratio (dots = pairs, bar = median)", fontsize=10)
    ax.text(0, -0.2, RUN, transform=ax.transAxes, fontsize=7, color="#555")
    save(fig, "ratios")


def scaling():
    fig, ax = plt.subplots(figsize=(7, 4.4))
    for g, mk in ((2, "o"), (3, "s"), (4, "^")):
        rs = [r for r in ROWS if r["genus"] == g]
        ms = [r["m"] for r in rs]
        ax.plot(ms, [r["fast_solve_med"] for r in rs], mk + "-", color=C_FAST, label=f"Fast (S2a), g{g}")
        ax.plot(ms, [r["dense_solve_med"] for r in rs], mk + "--", color=C_DENSE, label=f"Dense (S2b), g{g}")
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel("factor-base size m (log)")
    ax.set_ylabel("median solve time per run (ms, log)")
    ax.set_title("Solve time against factor-base size", fontsize=10)
    ax.legend(fontsize=7, ncol=3, loc="upper left")
    ax.text(0, -0.2, RUN, transform=ax.transAxes, fontsize=6, color="#555")
    save(fig, "scaling")


def shares():
    fig, ax = plt.subplots(figsize=(9, 4.2))
    w = 0.38
    ax.bar([x - w / 2 for x in X], [100 * r["fast_share_med"] for r in ROWS], w, color=C_FAST,
           label="Fast (S2a)")
    ax.bar([x + w / 2 for x in X], [100 * r["dense_share_med"] for r in ROWS], w, color=C_DENSE,
           label="Dense (S2b)")
    for x, r in zip(X, ROWS):
        ax.text(x + w / 2, 100 * r["dense_share_med"] + 1, f"{100 * r['dense_share_med']:.0f}",
                ha="center", fontsize=7)
    ax.axhline(35, color="#b00", lw=1, ls="--")
    ax.text(6.0, 36.5, "E7 bar: Dense share < 35 % (genus 2, p ≥ 503)", color="#b00", fontsize=8)
    ax.set_xticks(X, LABELS, fontsize=8)
    ax.set_ylabel("median over runs of solve_ms / total_ms (%)")
    ax.set_ylim(0, 80)
    ax.set_title("Solve share of native wall time, per run, median of 25", fontsize=10)
    ax.legend(fontsize=8, loc="upper right")
    ax.text(0, -0.2, RUN, transform=ax.transAxes, fontsize=7, color="#555")
    save(fig, "shares")


ratios()
scaling()
shares()
