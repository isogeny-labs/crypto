"""Lay out what Callgrind and the wall rounds recorded (data/ir_32bit.tsv,
data/ir_20bit.tsv, data/wall_rounds_32bit.tsv).  Plotting only: the per-GAE
quotients and medians are the only arithmetic."""

import csv
import statistics as st
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
OUT = HERE / (sys.argv[1] if len(sys.argv) > 1 else "figures")
OUT.mkdir(parents=True, exist_ok=True)


def rows(name):
    with (HERE / "data" / name).open() as f:
        return list(csv.DictReader(f, delimiter="\t"))


ARMS = ["E", "F", "G"]
LABEL = {"E": "E: before (PR #3144 head)", "F": "F: + walk key ring, jump wrap, 32-bit inversion", "G": "G: + raw-nanosecond phase trace"}
COL = {"E": "#555555", "F": "#9ecae1", "G": "#2f6db3"}
arm = lambda r: r["arm"][0]

fig, axes = plt.subplots(1, 3, figsize=(13.5, 4.2))

ax = axes[0]
r32 = rows("ir_32bit.tsv")
cells = [("rho.negation\nsolve", lambda r: r["child"] == "rho.negation", "solve_Ir"),
         ("rho.negation\nwhole child", lambda r: r["child"] == "rho.negation", "whole_process_Ir"),
         ("ic.pipeline N=1024\nwhole child", lambda r: r["child"].startswith("ic"), "whole_process_Ir")]
w = 0.26
for i, a in enumerate(ARMS):
    for j, (lab, sel, col) in enumerate(cells):
        base = next(int(r[col]) for r in r32 if arm(r) == "E" and sel(r))
        v = next(int(r[col]) for r in r32 if arm(r) == a and sel(r))
        x = j + (i - 1) * w
        ax.bar(x, v / base, width=w, color=COL[a], label=LABEL[a] if j == 0 else None)
        ax.text(x, v / base + 0.01, f"{v/1e6:.1f}M", ha="center", fontsize=6.3, rotation=90)
ax.set_xticks(range(len(cells)))
ax.set_xticklabels([c[0] for c in cells], fontsize=7.2)
ax.set_ylim(0, 1.3)
ax.set_ylabel("Callgrind Ir, relative to E (labels: absolute)", fontsize=7.5)
ax.set_title("32-bit children (exported inputs, identical reports)", fontsize=8.5)
ax.legend(fontsize=6.3, frameon=False, loc="upper center", bbox_to_anchor=(0.5, -0.17), ncol=1)
ax.tick_params(labelsize=7)

ax = axes[1]
r20 = rows("ir_20bit.tsv")
seqs = [("0", "rho.negation"), ("1", "ic.pipeline N=32"), ("2", "ic.pipeline N=45")]
for i, a in enumerate(ARMS):
    for j, (s, lab) in enumerate(seqs):
        r = next(r for r in r20 if arm(r) == a and r["seq"] == s)
        v = int(r["solve_Ir"]) / float(r["total_gae"])
        x = j + (i - 1) * w
        ax.bar(x, v, width=w, color=COL[a])
        ax.text(x, v + 12, f"{v:.0f}", ha="center", fontsize=6.3, rotation=90)
ax.set_xticks(range(len(seqs)))
ax.set_xticklabels([s[1] for s in seqs], fontsize=7.2)
ax.set_ylim(0, 1300)
ax.set_ylabel("Callgrind Ir per GAE inside the solve", fontsize=7.5)
ax.set_title("20-bit children (audit-ir, solve only)", fontsize=8.5)
ax.tick_params(labelsize=7)

ax = axes[2]
wr = rows("wall_rounds_32bit.tsv")
for k, (s, lab) in enumerate([("0", "rho.negation (ms)"), ("1", "ic.pipeline (ms / 40)")]):
    sc = 1e6 if s == "0" else 4e7
    for c, (col, name, mk) in enumerate([("before_E_solve_wall_ns", "E", "o"), ("after_G_solve_wall_ns", "G", "s"), ("before_again_E_solve_wall_ns", "E (A/A)", "^")]):
        vals = [int(r[col]) / sc for r in wr if r["seq"] == s]
        x = k * 4 + c
        ax.scatter([x] * len(vals), vals, s=10, color=COL["G"] if name == "G" else "#777777", marker=mk, alpha=0.6)
        m = st.median(vals)
        ax.plot([x - 0.3, x + 0.3], [m, m], color="black", lw=1.2)
        ax.text(x + 0.35, m, f"{m:.2f}", fontsize=6.3, va="center")
ax.set_xticks([0, 1, 2, 4, 5, 6])
ax.set_xticklabels(["E", "G", "E\n(A/A)", "E", "G", "E\n(A/A)"], fontsize=7)
ax.text(1, -0.2, "rho.negation, ms (9 rounds)", transform=ax.get_xaxis_transform(), ha="center", fontsize=7, va="top")
ax.text(5, -0.2, "ic.pipeline, ms/40 (7 rounds)", transform=ax.get_xaxis_transform(), ha="center", fontsize=7, va="top")
ax.set_ylim(0, 17)
ax.set_ylabel("child-reported solve wall (L0, CPU 3 pinned)", fontsize=7.5)
ax.set_title("Wall per solve, interleaved rounds (descriptive)", fontsize=8.5)
ax.tick_params(labelsize=7)

fig.suptitle("ecbench round 2, 2026-11-01: the tuned walk's key ring and the 32-bit inversion (F), the IC phase trace's raw clock (G); every record identical", fontsize=8.5)
fig.tight_layout(rect=(0, 0, 1, 0.93))
for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
    fig.savefig(OUT / f"round2.{ext}", bbox_inches="tight", **kw)
