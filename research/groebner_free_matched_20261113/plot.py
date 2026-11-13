#!/usr/bin/env python3
"""Plots recorded values only (AGENTS.md: Python lays out, never computes a result).

Reads SUMMARY.json (ic_algebraic_floor linsummary), cells.csv (ic_algebraic_floor
analyze) and, for the SAT lower bounds of the earlier session, the committed
research/ic_m3_algebraic_vs_mitm_20261014/cells.csv. Writes figures/*.svg and *.png.
"""
import csv
import json
import math
import pathlib

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = pathlib.Path(__file__).resolve().parent
fig_dir = here / "figures"
fig_dir.mkdir(exist_ok=True)
summary = json.load(open(here / "SUMMARY.json"))
cells = summary["cells"]
rows = list(csv.DictReader(open(here / "cells.csv")))
old = list(csv.DictReader(open(here.parent / "ic_m3_algebraic_vs_mitm_20261014" / "cells.csv")))

INK, MUTED = "#1f2933", "#7b8794"
COL = {"floor": "#2b6cb0", "lin21-ht": "#c05621", "lin21": "#b7791f", "f4-chain": "#6b46c1", "sat": "#718096"}


def save(fig, name):
    fig.savefig(fig_dir / f"{name}.svg", bbox_inches="tight")
    fig.savefig(fig_dir / f"{name}.png", dpi=150, bbox_inches="tight")
    plt.close(fig)


def arm(name, source):
    pts = [(int(r["n"]), float(r["log2_cost_ns"]), r["status"]) for r in source
           if r["m"] == "3" and r["arm"] == name and r["log2_cost_ns"] not in ("", "NaN")]
    return sorted(pts)


# 1. Cost per relation against n.
fig, ax = plt.subplots(figsize=(8.2, 5.2))
fl = [(c["n"], math.log2(c["floor"]["ns_per_relation_online"])) for c in cells if c["floor"]]
fa = [(c["n"], math.log2(c["floor"]["ns_per_relation_amortized"])) for c in cells if c["floor"]]
ln = [(c["n"], math.log2(c["lin_ns_per_relation_online"])) for c in cells]
ax.plot(*zip(*fa), "o-", color=COL["floor"], label="pair-table floor, amortized (table build spread over the base)")
ax.plot(*zip(*fl), "o--", color=COL["floor"], alpha=0.6, label="pair-table floor, online (table already built)")
ax.plot(*zip(*ln), "s-", color=COL["lin21-ht"], label="lin21-ht, whole pool (no table)")
p = arm("lin21", rows)
ax.plot([x[0] for x in p], [x[1] for x in p], "^-", color=COL["lin21"], label="lin21, 12 targets (no table)")
p = arm("f4-chain", rows)
ax.plot([x[0] for x in p], [x[1] for x in p], "D-", color=COL["f4-chain"], label="F4 on the S3 chain, 12 targets (this session)")
p = arm("sat", old)
if p:
    ax.plot([x[0] for x in p], [x[1] for x in p], "v:", color=COL["sat"],
            label="SAT, 2026-10-14 session (n ≥ 17 are lower bounds)")
last_floor = max(c["n"] for c in cells if c["floor"])
ax.axvline(last_floor + 3, color=MUTED, lw=0.8, ls=":")
ax.text(last_floor + 3.4, ax.get_ylim()[0] + 0.6, "pair table does not fit\n15 GB beyond here", color=MUTED, fontsize=8)
ax.set_xlabel("n (field degree; d = ⌈n/3⌉)")
ax.set_ylabel("log₂ wall ns per relation")
ax.set_title("m = 3 decomposition, cost per relation (one core, L0)", color=INK)
ax.grid(alpha=0.25)
ax.legend(fontsize=7.5, frameon=False)
save(fig, "cost_per_relation")

# 2. Peak memory.
fig, ax = plt.subplots(figsize=(8.2, 4.4))
fm = [(c["n"], c["floor"]["vm_hwm_kb"] / 1e3) for c in cells if c["floor"]]
lm = [(c["n"], c["lin_vm_hwm_kb"] / 1e3) for c in cells]
ax.semilogy(*zip(*fm), "o-", color=COL["floor"], label="pair-table floor (process peak resident set)")
ax.semilogy(*zip(*lm), "s-", color=COL["lin21-ht"], label="lin21-ht (process peak resident set)")
ax.set_xlabel("n")
ax.set_ylabel("peak resident set, MB")
ax.set_title("Peak memory per cell", color=INK)
ax.grid(alpha=0.25, which="both")
ax.legend(fontsize=8, frameon=False)
save(fig, "peak_memory")

# 3. Ratio to the floor and cost of one trial.
fig, (a1, a2) = plt.subplots(1, 2, figsize=(10, 4.2))
fc = [c for c in cells if c["floor"]]
ns = [c["n"] for c in fc]
for key, style, lab in (("lin_over_floor_online", "s-", "online"), ("lin_over_floor_amortized", "o--", "amortized")):
    v = [c["floor"][key] for c in fc]
    lo = [c["floor"][key] - c["floor"][key + "_ci95"][0] for c in fc]
    hi = [c["floor"][key + "_ci95"][1] - c["floor"][key] for c in fc]
    a1.errorbar(ns, v, yerr=[lo, hi], fmt=style, color=COL["lin21-ht"], capsize=3,
                alpha=1.0 if lab == "online" else 0.55, label=f"{lab} (95% paired bootstrap)")
a1.axhline(1, color=MUTED, lw=0.8)
a1.set_xlabel("n")
a1.set_ylabel("lin21-ht ÷ floor, wall per relation")
a1.set_title("Wall-time ratio to the floor", color=INK)
a1.grid(alpha=0.25)
a1.legend(fontsize=8, frameon=False)
a2.plot([c["n"] for c in cells], [c["lin_ns_per_trial"] for c in cells], "s-", color=COL["lin21-ht"])
a2.set_xlabel("n")
a2.set_ylabel("ns per trial (one third summand)")
a2.set_ylim(bottom=0)
a2.set_title("lin21-ht: cost of one trial", color=INK)
a2.grid(alpha=0.25)
save(fig, "ratio_and_trial_cost")
print("figures written to", fig_dir)
