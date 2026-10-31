#!/usr/bin/env python3
"""Plot recorded PS-6 values (plotting only; no research computation).

  plot_ps6.py summary.json outdir

Reads `ps6_tool analyze`'s summary.json and draws, each as SVG and as a 150 dpi PNG from the same code:
  s_over_floor     S / floor per rung and arm (mean over targets, 95% bootstrap CI over targets), with the
                   AGENTS.md section 5a band (unpriced native operations at 0 and 1 GAE) shaded
  band_order       sum S_A / sum S_B per rung at w = 0, 0.1, 1 GAE per unpriced operation: where the
                   band straddles 1 the order is unresolved
  exponents        fitted exponent e (total operations ~ r^e) per arm and weight with its bootstrap CI
  wall             median wall time per run by arm and rung (timed sessions), isolation levels in the title
  memory           pair-table entries and resident-memory growth per rung
Every plotted number is a field of summary.json; nothing is recomputed here.
"""
import json
import sys

import matplotlib

matplotlib.use("agg")
import matplotlib.pyplot as plt

summary, outdir = sys.argv[1:3]
S = json.load(open(summary))
rungs = S["rungs"]
ns = [r["n"] for r in rungs]
W0, W01, W1 = "w = 0 (recorded)", "w = 0.1", "w = 1"

COLORS = {
    "rho-negation": "black",
    "s4-baseline": "C3",
    "s4-grouplaw": "C0",
    "mitm": "C2",
    "mitm-negfold": "C4",
}
LABEL = {
    "rho-negation": "rho (negation), matched",
    "s4-baseline": "S4 pairs-and-solve baseline",
    "s4-grouplaw": "group-law kernel (PS-2)",
    "mitm": "meet in the middle, m=3 (full table)",
    "mitm-negfold": "meet in the middle, m=3, negation-folded",
}


def save(fig, name):
    fig.savefig(f"{outdir}/{name}.svg")
    fig.savefig(f"{outdir}/{name}.png", dpi=150)
    plt.close(fig)


def series(arm, conv):
    xs, ys, lo, hi = [], [], [], []
    for r in rungs:
        for a in r["arms"]:
            if a["arm"] == arm and a["convention"] == conv and a["mean_S"] is not None:
                f = r["floor_s"]
                xs.append(r["n"])
                ys.append(a["mean_S"] / f)
                ci = a["ci95_mean_S"] or (a["mean_S"], a["mean_S"])
                lo.append((a["mean_S"] - ci[0]) / f)
                hi.append((ci[1] - a["mean_S"]) / f)
    return xs, ys, lo, hi


# 1. S / floor
fig, ax = plt.subplots(figsize=(7.4, 4.8))
for arm in COLORS:
    xs, ys, lo, hi = series(arm, W0)
    if xs:
        ax.errorbar(xs, ys, yerr=[lo, hi], label=LABEL[arm] + " (w=0, recorded)", color=COLORS[arm], marker="osvPX"[list(COLORS).index(arm)], ms=4, lw=1.1, capsize=2)
    x1, y1, _, _ = series(arm, W1)
    if x1 and len(x1) == len(xs):
        ax.fill_between(xs, ys, y1, color=COLORS[arm], alpha=0.12, lw=0)
        ax.plot(x1, y1, color=COLORS[arm], ls=":", lw=0.8)
for conv, col, mk, lab in (
    ("anchored estimate", "C1", "D", "kernel, pinned baseline pair x measured time ratio (estimate, NOT pinned)"),
    ("pair-count (kernel pair = baseline pair)", "C9", "^", "kernel, each pair priced as a baseline pair (estimate, NOT pinned)"),
):
    xs, ys, _, _ = series("s4-grouplaw", conv)
    if xs:
        ax.plot(xs, ys, color=col, marker=mk, ms=3.5, ls="--", lw=0.9, label=lab)
ax.set_yscale("log")
ax.set_xticks(ns)
ax.set_xlabel("field degree n of the ledger's random binary curve (l = n/3, m = 3)")
ax.set_ylabel("S / floor   (floor S = sqrt(pi/4) = 0.886)")
ax.grid(alpha=0.3, which="both")
ax.legend(fontsize=5.6, loc="upper left")
ax.set_title("S per rung, counted unit: mean over targets, 95% bootstrap CI; shaded = section 5a band (unpriced ops at 0 to 1 GAE)", fontsize=7)
fig.text(0.01, 0.01, "source: ps6_charge/raw/summary.json (sealed counted sessions); dotted = w=1 edge of the band", fontsize=6)
fig.tight_layout(rect=(0, 0.03, 1, 1))
save(fig, "s_over_floor")

# 2. Band order
pairs = [("s4-grouplaw", "s4-baseline"), ("mitm", "s4-baseline"), ("s4-grouplaw", "mitm"), ("s4-grouplaw", "rho-negation"), ("mitm", "rho-negation")]
fig, axes = plt.subplots(1, len(pairs), figsize=(8.6, 3.3), sharey=False)
for ax, (a, b) in zip(axes, pairs):
    xs, w0, w01, w1 = [], [], [], []
    for r in rungs:
        for o in r.get("order_across_band", []):
            if o["a"] == a and o["b"] == b and all(v is not None for v in o["ratio_a_over_b_at_w0_w01_w1"]):
                xs.append(r["n"])
                w0.append(o["ratio_a_over_b_at_w0_w01_w1"][0])
                w01.append(o["ratio_a_over_b_at_w0_w01_w1"][1])
                w1.append(o["ratio_a_over_b_at_w0_w01_w1"][2])
    if xs:
        ax.fill_between(xs, w0, w1, color="C0", alpha=0.18, lw=0)
        ax.plot(xs, w0, color="C0", lw=1, marker="v", ms=3, label="w=0")
        ax.plot(xs, w01, color="C0", lw=0.8, ls="--", label="w=0.1")
        ax.plot(xs, w1, color="C0", lw=1, marker="^", ms=3, label="w=1")
    ax.axhline(1.0, color="gray", lw=0.8)
    ax.set_yscale("log")
    ax.set_xticks(ns)
    ax.tick_params(labelsize=6)
    ax.set_title(f"{a}\n/ {b}", fontsize=6.2)
    ax.grid(alpha=0.3, which="both")
axes[0].legend(fontsize=5.5)
fig.suptitle("sum S_A / sum S_B at 0, 0.1 and 1 GAE per unpriced native operation (below 1: A cheaper); a band straddling 1 is unresolved", fontsize=7)
fig.text(0.01, 0.01, "source: ps6_charge/raw/summary.json (order_across_band)", fontsize=6)
fig.tight_layout(rect=(0, 0.04, 1, 0.93))
save(fig, "band_order")

# 3. Exponents
fig, ax = plt.subplots(figsize=(7.4, 4.4))
fits = [f for f in S["fits"] if f["exponent_e"] is not None and f["convention"] in (W0, W1, "anchored estimate")]
for i, f in enumerate(fits):
    ci = f["ci95"] or (f["exponent_e"], f["exponent_e"])
    ax.errorbar([f["exponent_e"]], [i], xerr=[[f["exponent_e"] - ci[0]], [ci[1] - f["exponent_e"]]], color=COLORS.get(f["arm"], "gray"), marker="o", ms=4, capsize=3)
ax.axvline(0.5, color="gray", lw=0.8, ls="--")
ax.text(0.5, -0.9, "rho: 1/2", fontsize=6, ha="center")
ax.set_yticks(range(len(fits)))
ax.set_yticklabels([f'{f["arm"]} [{f["convention"]}]' for f in fits], fontsize=6)
ax.set_xlabel("fitted exponent e: total operations T = S*sqrt(r) ~ r^e over the five rungs (95% bootstrap CI over targets)")
ax.grid(axis="x", alpha=0.3)
ax.set_title("slope of the ratio to the floor = e - 1/2", fontsize=8)
fig.text(0.01, 0.01, "source: ps6_charge/raw/summary.json (fits)", fontsize=6)
fig.tight_layout(rect=(0, 0.03, 1, 1))
save(fig, "exponents")

# 4. Wall
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
    save(fig, "wall")

# 5. Memory
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
    save(fig, "memory")
