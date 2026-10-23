"""Lay out the recorded values of the 2026-10-23 speed-ups.

Reads summary.json (written by summarize.sh from the sessions, ecbench
compare, the external-tool log and the timing logs) and draws it; no
computation beyond arranging those numbers (AGENTS.md "Implementation
language: no Python").

usage: python plot.py SPEEDUPS_DIR OUT_DIR
"""
import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

here = Path(sys.argv[1])
out = Path(sys.argv[2])
out.mkdir(parents=True, exist_ok=True)
S = json.loads((here / "summary.json").read_text())
INK, MUTED, BEFORE, AFTER, EXT = "#1f2328", "#59636e", "#9aa4b1", "#2f6fdf", "#d4762c"
plt.rcParams.update({"font.size": 9, "axes.edgecolor": MUTED, "axes.labelcolor": INK,
                     "xtick.color": INK, "ytick.color": INK, "axes.spines.top": False,
                     "axes.spines.right": False})


def save(fig, name):
    fig.tight_layout()
    fig.savefig(out / f"{name}.svg")
    fig.savefig(out / f"{name}.png", dpi=150)
    plt.close(fig)


# 1. Rho: ns per group operation and median solve time, three implementations.
r = S["rho"]
arms = [("rho.negation\nserial\n(before)", r["serial"], BEFORE),
        ("rho.negation_lanes\n32 lanes\n(after)", r["lanes"], AFTER),
        ("external\nca --alg rho\n(1aa48b5)", r["external"], EXT)]
fig, axes = plt.subplots(1, 3, figsize=(10, 3.2))
for ax, key, label in [(axes[0], "median_ns_per_op", "median ns per group operation"),
                       (axes[1], "median_solve_ms", "median solve time (ms)"),
                       (axes[2], "mean_ops_over_sqrt_r", "mean operations / √r")]:
    vals = [a[1][key] for a in arms]
    bars = ax.bar(range(3), vals, color=[a[2] for a in arms], width=0.6)
    for b, v in zip(bars, vals):
        ax.text(b.get_x() + b.get_width() / 2, v, f"{v:.3g}", ha="center", va="bottom", fontsize=8)
    ax.set_xticks(range(3), [a[0] for a in arms], fontsize=7)
    ax.set_ylabel(label)
fig.suptitle(f"Pollard rho on the deep-volcano class (r ≈ 2^40.78), 48 curves × 4 targets × 3 rounds = "
             f"{r['lanes']['runs']} runs per ecbench arm, {r['external']['runs']} external solves; "
             f"session {r['session_id']}", fontsize=7.5, color=MUTED)
save(fig, "rho-speed")

# 2. Pipeline stages, before and after (log scale: seconds).
st = S["stages"]
fig, ax = plt.subplots(figsize=(8.5, 3.8))
names = [s["name"] for s in st]
y = range(len(st))
ax.barh([i + 0.2 for i in y], [s["before_s"] for s in st], height=0.38, color=BEFORE, label="before")
ax.barh([i - 0.2 for i in y], [s["after_s"] for s in st], height=0.38, color=AFTER, label="after")
for i, s in enumerate(st):
    ax.text(s["before_s"], i + 0.2, f" {s['before_s']:.3g} s", va="center", fontsize=7, color=MUTED)
    ax.text(s["after_s"], i - 0.2, f" {s['after_s']:.3g} s  (×{s['speedup']:.3g})",
            va="center", fontsize=7, color=INK)
ax.set_yticks(list(y), names, fontsize=7.5)
ax.invert_yaxis()
ax.set_xscale("log")
ax.set_xlim(right=ax.get_xlim()[1] * 30)
ax.set_xlabel("wall time (s, log scale); outputs identical before and after. Rows 1-5: interleaved, this host, "
              "medians;\nlast row: recorded session spans, 2026-10-22 (before) and 2026-10-23 (after)", fontsize=7.5)
ax.legend(frameon=False, fontsize=7.5, loc="upper right")
save(fig, "pipeline-stages")
print("wrote", sorted(p.name for p in out.iterdir()))
