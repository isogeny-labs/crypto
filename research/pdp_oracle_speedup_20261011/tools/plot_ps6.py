#!/usr/bin/env python3
"""Plot recorded PS-6 values (plotting only; no research computation).

  plot_ps6.py summary.json outdir

Reads `ps6_tool analyze`'s summary.json and draws:
  s_over_floor.svg   S / floor per rung and arm (mean over targets, 95% bootstrap CI over targets)
  exponents.svg      fitted exponent e (total operations ~ r^e) per arm with its bootstrap CI
  wall.svg           median wall time per run by arm and rung (timed sessions), with isolation levels
  memory.svg         pair-table entries and resident-memory growth per rung
Every plotted number is a field of summary.json; nothing is recomputed here.
"""
import json
import sys

import matplotlib

matplotlib.use("svg")
import matplotlib.pyplot as plt

summary, outdir = sys.argv[1:3]
S = json.load(open(summary))
rungs = S["rungs"]
ns = [r["n"] for r in rungs]

STYLE = {
    "rho-negation": ("rho (negation), matched", "black", "o", "-"),
    "s4-baseline": ("S4 pairs-and-solve baseline", "C3", "s", "-"),
    "s4-grouplaw|lower bound (recorded)": ("group-law kernel, lower bound (pairs unpriced)", "C0", "v", "--"),
    "s4-grouplaw|pair-count (kernel pair = baseline pair)": ("group-law kernel, pair = baseline pair (NOT pinned)", "C9", "^", ":"),
    "s4-grouplaw|anchored estimate": ("group-law kernel, pinned baseline pair x measured time ratio (NOT pinned)", "C1", "D", "-"),
    "s4-grouplaw|add-ratio estimate": ("group-law kernel, measured pair / measured add (NOT pinned)", "C6", "d", ":"),
    "mitm": ("meet in the middle, m=3 (full table)", "C2", "P", "-"),
    "mitm-negfold": ("meet in the middle, m=3, negation-folded", "C4", "X", "--"),
}


def key(a):
    return f'{a["arm"]}|{a["convention"]}' if a["arm"] == "s4-grouplaw" else a["arm"]


fig, ax = plt.subplots(figsize=(7.4, 4.6))
for k, (lab, col, mk, ls) in STYLE.items():
    xs, ys, lo, hi = [], [], [], []
    for r in rungs:
        for a in r["arms"]:
            if key(a) == k and a["mean_S"] is not None:
                f = r["floor_s"]
                xs.append(r["n"])
                ys.append(a["mean_S"] / f)
                ci = a["ci95_mean_S"] or (a["mean_S"], a["mean_S"])
                lo.append(a["mean_S"] / f - ci[0] / f)
                hi.append(ci[1] / f - a["mean_S"] / f)
    if xs:
        ax.errorbar(xs, ys, yerr=[lo, hi], label=lab, color=col, marker=mk, ls=ls, ms=4, lw=1.1, capsize=2)
ax.set_yscale("log")
ax.set_xticks(ns)
ax.set_xlabel("field degree n of the ledger's random binary curve (l = n/3, m = 3)")
ax.set_ylabel("S / floor   (floor S = sqrt(pi/4) = 0.886)")
ax.grid(alpha=0.3, which="both")
ax.legend(fontsize=6.2, loc="upper left")
ax.set_title("S per rung in the counted unit: mean over targets, 95% bootstrap CI over targets", fontsize=8)
fig.text(0.01, 0.01, "source: ps6_charge/raw/summary.json (from the sealed counted sessions); kernel rows other than the lower bound use a conversion that is not pinned", fontsize=6)
fig.tight_layout(rect=(0, 0.03, 1, 1))
fig.savefig(f"{outdir}/s_over_floor.svg")
plt.close(fig)

# Exponents
fig, ax = plt.subplots(figsize=(7.4, 3.6))
fits = [f for f in S["fits"] if f["exponent_e"] is not None]
for i, f in enumerate(fits):
    k = f["arm"] if f["arm"] != "s4-grouplaw" else f'{f["arm"]}|{f["convention"]}'
    lab, col, mk, _ = STYLE.get(k, (k, "gray", "o", "-"))
    ci = f["ci95"] or (f["exponent_e"], f["exponent_e"])
    ax.errorbar([f["exponent_e"]], [i], xerr=[[f["exponent_e"] - ci[0]], [ci[1] - f["exponent_e"]]], color=col, marker=mk, capsize=3)
ax.axvline(0.5, color="gray", lw=0.8, ls="--")
ax.text(0.5, -0.9, "rho: 1/2", fontsize=6, ha="center")
ax.set_yticks(range(len(fits)))
ax.set_yticklabels([STYLE.get(f["arm"] if f["arm"] != "s4-grouplaw" else f'{f["arm"]}|{f["convention"]}', (f["arm"],))[0] for f in fits], fontsize=6)
ax.set_xlabel("fitted exponent e: total operations T = S*sqrt(r) ~ r^e over the five rungs (95% bootstrap CI over targets)")
ax.grid(axis="x", alpha=0.3)
ax.set_title("slope of the ratio to the floor = e - 1/2", fontsize=8)
fig.text(0.01, 0.01, "source: ps6_charge/raw/summary.json (fits)", fontsize=6)
fig.tight_layout(rect=(0, 0.03, 1, 1))
fig.savefig(f"{outdir}/exponents.svg")
plt.close(fig)

# Wall
if S["timed"]:
    fig, ax = plt.subplots(figsize=(7.4, 4.0))
    arms = ["rho-negation", "s4-baseline", "s4-baseline-control", "s4-grouplaw", "mitm", "mitm-negfold"]
    cols = {"rho-negation": "black", "s4-baseline": "C3", "s4-baseline-control": "C5", "s4-grouplaw": "C0", "mitm": "C2", "mitm-negfold": "C4"}
    for a in arms:
        xs, ys = [], []
        for t in S["timed"]:
            for r in t["arms"]:
                if r["arm"] == a and r["median_wall_ms"] is not None:
                    xs.append(t["n"])
                    ys.append(r["median_wall_ms"])
        if xs:
            ax.plot(xs, ys, marker="o", ms=4, color=cols[a], label=a, lw=1)
    ax.set_yscale("log")
    ax.set_xticks([t["n"] for t in S["timed"]])
    ax.set_xlabel("n")
    ax.set_ylabel("median wall time of the solve, ms")
    ax.grid(alpha=0.3, which="both")
    ax.legend(fontsize=6)
    levels = sorted({k for t in S["timed"] for r in t["arms"] for k in r["levels"]})
    ax.set_title(f"timed sessions, whole pipeline per target (isolation levels reached: {', '.join(levels)})", fontsize=8)
    fig.text(0.01, 0.01, "source: ps6_charge/raw/summary.json (timed); wall time is descriptive unless the saved comparison says admitted", fontsize=6)
    fig.tight_layout(rect=(0, 0.03, 1, 1))
    fig.savefig(f"{outdir}/wall.svg")
    plt.close(fig)

# Memory
if S["memory"]:
    fig, (a, b) = plt.subplots(1, 2, figsize=(7.4, 3.2))
    for fold, col in (("full", "C2"), ("negation", "C4")):
        m = sorted([x for x in S["memory"] if x["fold"] == fold], key=lambda x: x["n"])
        a.plot([x["n"] for x in m], [x["table_entries"] for x in m], marker="o", color=col, label=f"table entries, {fold}")
        b.plot([x["n"] for x in m], [x["rss_delta_kib"] / 1024 for x in m], marker="o", color=col, label=f"RSS growth, {fold}")
    for ax in (a, b):
        ax.set_yscale("log")
        ax.set_xlabel("n")
        ax.grid(alpha=0.3, which="both")
        ax.legend(fontsize=6)
    a.set_ylabel("entries")
    b.set_ylabel("MiB")
    fig.suptitle("the repository's pair table: not in S", fontsize=8)
    fig.text(0.01, 0.01, "source: ps6_charge/raw/summary.json (memory)", fontsize=6)
    fig.tight_layout(rect=(0, 0.04, 1, 0.95))
    fig.savefig(f"{outdir}/memory.svg")
    plt.close(fig)
