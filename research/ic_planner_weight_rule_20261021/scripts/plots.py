"""Lay out analysis/derivation.json and analysis/summary.json as SVG + 150 dpi
PNG. Plotting only: every number comes from those files (jq over ecbench output).
"""
import json
from pathlib import Path

import matplotlib
import matplotlib.ticker

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

here = Path(__file__).resolve().parent.parent
der = json.loads((here / "analysis" / "derivation.json").read_text())
summ = json.loads((here / "analysis" / "summary.json").read_text())
out = here / "figures"
out.mkdir(exist_ok=True)
NOTE = "ecbench GAE; 24 paired verified runs per arm and curve; w = GAE charged per unpriced native operation (AGENTS.md §5a)."


def save(fig, name):
    fig.tight_layout()
    fig.savefig(out / f"{name}.svg")
    fig.savefig(out / f"{name}.png", dpi=150)
    plt.close(fig)


# 1. Phase A: the derivation grid G(m, w).
fig, ax = plt.subplots(figsize=(6.4, 4.2))
cols = ["#4f81bd", "#9bbb59", "#f79646", "#8064a2", "#c0504d"]
for c, d in zip(cols, der["detail"]):
    ms = [g["m"] for g in d["grid"]]
    gs = [g["G"] for g in d["grid"]]
    ax.plot(ms, gs, "o-", color=c, ms=4, label=f"w = {d['w']}  (m* = {d['best']['m']})")
    ax.plot([d["best"]["m"]], [d["best"]["G"]], "*", color=c, ms=12)
ax.axhline(1, color="black", lw=0.8)
ax.set_xscale("log", base=2)
ax.set_yscale("log")
ax.set_xticks([0.5, 0.71, 1, 1.41, 2, 2.83, 4])
ax.set_xticklabels(["0.5", "0.71", "1", "1.41", "2", "2.83", "4"])
ax.set_xlabel("base multiplier m (base = m · round(#E^(1/3)/2) abscissae)")
ax.set_ylabel("G(m, w): geometric mean of S_w(m) / S_w(1)")
ax.yaxis.set_major_formatter(matplotlib.ticker.ScalarFormatter())
ax.yaxis.set_minor_formatter(matplotlib.ticker.NullFormatter())
ax.set_yticks([0.9, 1, 1.5, 2, 3, 4, 5])
ax.set_title("Phase A (r24, r26, r28): the cheapest base moves to 1.41 N₀ only at w = 1", fontsize=9)
ax.legend(fontsize=7)
ax.grid(alpha=0.3, which="both")
fig.text(0.01, 0.005, NOTE, fontsize=5.5)
save(fig, "derivation_grid")

# 2. Phase B: v2-m141 / v1 at w = 0 and w = 1, paired.
fig, ax = plt.subplots(figsize=(6.4, 4.0))
rows = summ["phase_b"]
for k, (w, col) in enumerate((("w0", "#7f7f7f"), ("w1", "#c0504d"))):
    xs = [i + (k - 0.5) * 0.3 for i in range(len(rows))]
    ys = [r["v2_over_v1"][w]["ratio"] for r in rows]
    lo = [y - r["v2_over_v1"][w]["ci95"][0] for y, r in zip(ys, rows)]
    hi = [r["v2_over_v1"][w]["ci95"][1] - y for y, r in zip(ys, rows)]
    ax.errorbar(xs, ys, yerr=[lo, hi], fmt="o", color=col, capsize=3,
                label=f"w = {w[1:]}")
ax.axhline(1, color="black", lw=0.8)
ax.set_xticks(range(len(rows)))
ax.set_xticklabels([f"{r['tag']}\n{r['slug']}\nlog2 r {r['log2_r']:.1f}" for r in rows], fontsize=6)
ax.set_ylabel("S_w(1.41 N₀) / S_w(N₀), paired, 95% interval")
ax.set_title("Held out: the w = 1 base is cheaper at w = 1 (3 of 4 intervals below 1)", fontsize=9)
ax.legend(fontsize=7)
ax.grid(alpha=0.3, axis="y")
fig.text(0.01, 0.005, NOTE + " Below 1: the larger base is cheaper.", fontsize=5)
save(fig, "holdout_ratio")

# 3. Distance remaining to rho at w = 1, both bases.
fig, ax = plt.subplots(figsize=(6.0, 4.0))
pts = [(r["log2_r"], r["v1_over_rho"]["w1"], r["v2_over_rho_w1"], "held out") for r in summ["phase_b"]] + \
      [(r["log2_r"], r["m100_over_rho"]["w1"], r["m141_over_rho_w1"], "derivation") for r in summ["phase_a"]]
for lab, mk in (("held out", "o"), ("derivation", "s")):
    sel = sorted([p for p in pts if p[3] == lab])
    for j, (key, col, name) in enumerate(((1, "#7f7f7f", "N₀ (ic-planner/1)"), (2, "#c0504d", "1.41 N₀ (ic-planner/2 at w = 1)"))):
        xs = [p[0] + (j - 0.5) * 0.12 for p in sel]
        ys = [p[key]["ratio"] for p in sel]
        lo = [y - p[key]["ci95"][0] for y, p in zip(ys, sel)]
        hi = [p[key]["ci95"][1] - y for y, p in zip(ys, sel)]
        ax.errorbar(xs, ys, yerr=[lo, hi], fmt=mk, color=col, capsize=2, label=f"{name}, {lab}")
ax.set_xlabel("log2 r")
ax.set_ylabel("S_1(IC) / S_1(rho.negation), paired")
ax.set_title("Distance remaining to rho at w = 1", fontsize=9)
ax.legend(fontsize=6)
ax.grid(alpha=0.3)
fig.text(0.01, 0.005, NOTE, fontsize=5)
save(fig, "rho_gap_w1")
print("wrote", sorted(p.name for p in out.iterdir()))
