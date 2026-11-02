"""Lay out what `ecbench audit-ir`, the 32-bit Callgrind runs and the child wall
runs recorded (data/audit_ir_20bit.tsv, data/audit_ir_32bit.tsv when present,
data/child_wall_20bit.tsv).  Plotting only; the per-GAE quotients are the
ones audit-ir printed."""

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
    p = HERE / "data" / name
    if not p.exists():
        return []
    with p.open() as f:
        return list(csv.DictReader(f, delimiter="\t"))


COLOUR = {"baseline main 8a208fa570": "#555555", "A: warm fix (curve_id)": "#9ecae1", "B: A + walk (ic_boundary)": "#2f6db3", "baseline main 5ac38ef4e9": "#9a9a9a", "C: B + registry index (main 5ac38ef4e9)": "#0b2e5c"}
SHORT = {"baseline main 8a208fa570": "baseline (main)", "A: warm fix (curve_id)": "A: alias table", "B: A + walk (ic_boundary)": "B: A + walk", "baseline main 5ac38ef4e9": "baseline\u2032 (main 5ac38ef4e9)", "C: B + registry index (main 5ac38ef4e9)": "C: B + registry index"}

fig, axes = plt.subplots(1, 4, figsize=(15.2, 3.9))

# 1. before-solve Ir per child, 20-bit, rho and IC children.
ax = axes[0]
r20 = rows("audit_ir_20bit.tsv")
arms = [a for a in COLOUR if any(r["binary"] == a for r in r20)]
seqs = [("0", "rho.negation child"), ("1", "ic.pipeline N=32 child")]
w = 0.8 / len(arms)
for i, a in enumerate(arms):
    vals = [next((int(r["before_solve_Ir"]) for r in r20 if r["binary"] == a and r["seq"] == s), 0) for s, _ in seqs]
    xs = [j + (i - (len(arms) - 1) / 2) * w for j in range(len(seqs))]
    ax.bar(xs, vals, width=w, color=COLOUR[a], label=SHORT[a])
    for x, v in zip(xs, vals):
        ax.text(x, v * 1.15, f"{v/1e6:.1f}M", ha="center", fontsize=6.5, rotation=90)
ax.set_yscale("log")
ax.set_xticks(range(len(seqs)))
ax.set_xticklabels([t for _, t in seqs], fontsize=7.5)
ax.set_ylabel("Callgrind Ir before the solve marker (log)", fontsize=7.5)
ax.set_title("Per child, outside the solve (20-bit inputs)", fontsize=8.5)
ax.legend(fontsize=6.3, frameon=False, loc="upper center", bbox_to_anchor=(0.5, -0.12), ncol=2)
ax.set_ylim(1e6, 2e9)
ax.tick_params(labelsize=7)

# 2. Ir per GAE inside the solve, 20-bit and 32-bit, both methods.
ax = axes[1]
r32 = rows("audit_ir_32bit.tsv")
cells = [("20-bit rho.negation", r20, "0"), ("20-bit ic.pipeline", r20, "1"), ("32-bit rho.negation", r32, "0"), ("32-bit ic.pipeline", r32, "1")]
cells = [c for c in cells if any(r["seq"] == c[2] and r["ir_per_gae"] for r in c[1])]
w = 0.8 / len(arms)
for i, a in enumerate(arms):
    vals = [next((float(r["ir_per_gae"]) for r in rr if r["binary"] == a and r["seq"] == s and r["ir_per_gae"]), 0) for _, rr, s in cells]
    xs = [j + (i - (len(arms) - 1) / 2) * w for j in range(len(cells))]
    ax.bar(xs, vals, width=w, color=COLOUR[a], label=SHORT[a])
    for x, v in zip(xs, vals):
        if v:
            ax.text(x, v + 12, f"{v:.0f}", ha="center", va="bottom", fontsize=5.6, rotation=90)
ax.set_xticks(range(len(cells)))
ax.set_xticklabels([c[0] for c in cells], fontsize=7, rotation=12)
ax.set_ylim(0, 1350)
ax.set_ylabel("Callgrind Ir per GAE inside the solve", fontsize=7.5)
ax.set_title("Per counted group operation (solve only)", fontsize=8.5)
ax.tick_params(labelsize=7)

# 3. child wall, 20-bit.
ax = axes[2]
rw = rows("child_wall_20bit.tsv")
labels, meds, cols = [], [], []
for r in rw:
    vals = [float(r[f"run{i}_s"]) for i in range(1, 6)]
    labels.append(f'{SHORT[r["binary"]]}\n{r["input"]}')
    meds.append(st.median(vals) * 1000)
    cols.append(COLOUR[r["binary"]])
ax.barh(range(len(labels)), meds, color=cols)
for i, v in enumerate(meds):
    ax.text(v + 0.5, i, f"{v:.0f} ms", va="center", fontsize=7)
ax.set_yticks(range(len(labels)))
ax.set_yticklabels(labels, fontsize=6.5)
ax.invert_yaxis()
ax.set_xlim(0, max(meds) * 1.3)
ax.set_xlabel("median wall of `ecbench exec` over 5 runs, ms (L0)", fontsize=7.5)
ax.set_title("Per child, spawn to exit (20-bit inputs)", fontsize=8.5)
ax.tick_params(labelsize=7)

# 4. whole-process Ir per child, 32-bit inputs (Callgrind "Collected", every part).
ax = axes[3]
seqs32 = [("0", "rho.negation child\n(34,156 GAE)"), ("1", "ic.pipeline N=1024 child\n(1,446,809 GAE)")]
arms32 = [a for a in COLOUR if any(r["binary"] == a for r in r32)]
if r32:
    w = 0.8 / len(arms32)
    for i, a in enumerate(arms32):
        vals = [next((int(r["total_Ir"]) for r in r32 if r["binary"] == a and r["seq"] == s and r["total_Ir"]), 0) for s, _ in seqs32]
        xs = [j + (i - (len(arms32) - 1) / 2) * w for j in range(len(seqs32))]
        ax.bar(xs, vals, width=w, color=COLOUR[a], label=SHORT[a])
        for x, v in zip(xs, vals):
            if v:
                ax.text(x, v * 1.12, f"{v/1e6:.1f}M", ha="center", fontsize=6.5, rotation=90)
    ax.set_yscale("log")
    ax.set_xticks(range(len(seqs32)))
    ax.set_xticklabels([t for _, t in seqs32], fontsize=7)
    ax.set_ylim(1e7, 2e10)
    ax.legend(fontsize=6.5, frameon=False, loc="upper left")
ax.set_ylabel("Callgrind Ir, whole child process (log)", fontsize=7.5)
ax.set_title("Per child, spawn to exit (32-bit inputs)", fontsize=8.5)
ax.tick_params(labelsize=7)

fig.suptitle("ecbench, 2026-11-01: the alias-table parse (A) and the registry scan (C) out of every child, the tuned rho walk's bookkeeping (B); every record identical",
             fontsize=8.5)
fig.tight_layout(rect=(0, 0, 1, 0.93))
for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
    fig.savefig(OUT / f"child_and_walk.{ext}", bbox_inches="tight", **kw)
