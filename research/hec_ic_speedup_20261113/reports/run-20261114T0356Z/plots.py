"""Graphs for the recorded rerun (record 20261114T0356Z-hec-native-ic).

Layout only: every plotted number is a field of ../../rec-20261113/summary.jsonl,
which summarise.sh derives (means of five seeds) from the raw run output.
Writes <name>.svg and a 150 dpi <name>.png for each graph beside this file.

    python3 plots.py
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
ROWS = [json.loads(l) for l in (HERE / "../../rec-20261113/summary.jsonl").read_text().splitlines() if l.strip()]
LABELS = [f"g{r['genus']}\np={r['p']}" for r in ROWS]
X = list(range(len(ROWS)))
RUN = "record 20261114T0356Z-hec-native-ic; mean of 5 seeds per row; one arm64 macOS host, 1 thread"
C_BIG, C_REF, C_FAST = "#8c8c8c", "#d08c2f", "#2f6fb0"


def save(fig, name):
    fig.tight_layout()
    fig.savefig(HERE / f"{name}.svg")
    fig.savefig(HERE / f"{name}.png", dpi=150)
    plt.close(fig)


def solve_times():
    fig, ax = plt.subplots(figsize=(9, 4.2))
    w = 0.38
    ax.bar([x - w / 2 for x in X], [r["ref_solve_ms"] for r in ROWS], w, color=C_REF, label="Solver::Reference (S1)")
    ax.bar([x + w / 2 for x in X], [r["fast_solve_ms"] for r in ROWS], w, color=C_FAST, label="Solver::Fast (S2a)")
    for x, r in zip(X, ROWS):
        ax.text(x, r["ref_solve_ms"] * 1.15, f"{r['solve_ratio']:.1f}×", ha="center", fontsize=8)
    ax.set_yscale("log")
    ax.set_ylim(top=ax.get_ylim()[1] * 1.6)
    ax.set_xticks(X, LABELS, fontsize=8)
    ax.set_ylabel("solve time per run (ms, log)")
    ax.set_title("Native solve: same pivots and arithmetic, reference vs fast (label = ratio)", fontsize=10)
    ax.legend(fontsize=8, loc="upper left")
    ax.text(0, -0.2, RUN, transform=ax.transAxes, fontsize=7, color="#555")
    save(fig, "solve_times")


def end_to_end():
    fig, ax = plt.subplots(figsize=(9, 4.2))
    w = 0.27
    ax.bar([x - w for x in X], [r["big_ms"] for r in ROWS], w, color=C_BIG, label="BigUint engine (round eight)")
    ax.bar(X, [r["ref_total_ms"] for r in ROWS], w, color=C_REF, label="native, S1 solver")
    ax.bar([x + w for x in X], [r["fast_total_ms"] for r in ROWS], w, color=C_FAST, label="native, S2a solver")
    for x, r in zip(X, ROWS):
        ax.text(x, r["big_ms"] * 1.15, f"{r['big_over_fast_total']:.0f}×", ha="center", fontsize=8)
    ax.set_yscale("log")
    ax.set_ylim(top=ax.get_ylim()[1] * 1.6)
    ax.set_xticks(X, LABELS, fontsize=8)
    ax.set_ylabel("index-calculus wall time per run (ms, log)")
    ax.set_title("End to end: BigUint vs native (label = BigUint / native S2a)", fontsize=10)
    ax.legend(fontsize=8, loc="upper left")
    ax.text(0, -0.2, RUN + "; instance selection excluded", transform=ax.transAxes, fontsize=7, color="#555")
    save(fig, "end_to_end")


def phases():
    fig, ax = plt.subplots(figsize=(9, 4.2))
    # Solve at the bottom, so its height reads directly against the 50 % line.
    parts = [("fast_solve_ms", "solve", C_FAST), ("fast_row_ms", "rows", "#e3c26b"),
             ("fast_oracle_ms", "oracle", "#b59ad1"), ("fast_walk_ms", "walk", "#7fb07f"),
             ("fast_fb_ms", "factor base", "#c9c9c9")]
    bottom = [0.0] * len(ROWS)
    for key, label, colour in parts:
        vals = [100 * r[key] / r["fast_total_ms"] for r in ROWS]
        ax.bar(X, vals, 0.6, bottom=bottom, color=colour, label=label)
        bottom = [b + v for b, v in zip(bottom, vals)]
    ax.axhline(50, color="#b00", lw=1, ls="--")
    ax.text(len(ROWS) - 0.5, 51, "S2b trigger: solve > 50 %", color="#b00", fontsize=8, ha="right")
    ax.set_xticks(X, LABELS, fontsize=8)
    ax.set_ylabel("share of native (S2a) wall time (%)")
    ax.set_ylim(0, 105)
    ax.set_title("Where the native engine's time goes after S2a", fontsize=10)
    ax.legend(fontsize=8, ncol=5, loc="upper center", bbox_to_anchor=(0.5, -0.17))
    ax.text(0, -0.32, RUN + "; phases may not sum to 100 % (untimed glue)", transform=ax.transAxes,
            fontsize=7, color="#555")
    save(fig, "phases")


solve_times()
end_to_end()
phases()
