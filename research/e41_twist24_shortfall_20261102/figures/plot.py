"""E41 figures. Lays out recorded values from experiments/48_twist24_shortfall/*.json only
(every number is computed by examples/e41_stage0.rs and examples/e41_tables.rs); no computation here."""
import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

ROOT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(".")
D = ROOT / "experiments/48_twist24_shortfall"
OUT = ROOT / "research/e41_twist24_shortfall_20261102/figures"
summ = json.loads((D / "summary.json").read_text())
rec = json.loads((D / "stage0_records.json").read_text())
s0 = json.loads((D / "stage0_run.json").read_text())

# 1. steps / ideal per curve: E34's records (16 walks) against E41's main stream (128 walks).
fig, axes = plt.subplots(1, 2, figsize=(10, 3.8), sharey=True)
for ax, (e34arm, e41arm, title) in zip(axes, [("Order6Cheap", "O6B", "order 6"), ("Order24Cheap", "O24", "order 24")]):
    ps = [c["p"] for c in rec["curves"]]
    x = range(len(ps))
    ax.errorbar([i - 0.15 for i in x], [c[e34arm]["steps_over_ideal"] for c in rec["curves"]],
                yerr=[1.96 * c[e34arm]["se"] for c in rec["curves"]], fmt="o", color="#b85c38",
                label="E34 records (seeds 1000–1015, 16 walks)", capsize=2)
    rows = summ["steps_over_ideal_per_curve"][e41arm]
    ax.errorbar([i + 0.15 for i in x], [r["steps_over_ideal"] for r in rows],
                yerr=[1.96 * r["se"] for r in rows], fmt="s", color="#2c6e91",
                label="E41 main stream (seeds 48000–48127, 128 walks)", capsize=2)
    ax.axhline(1.0, color="0.4", lw=0.8, ls="--")
    ax.set_xticks(list(x), [str(p) for p in ps])
    ax.set_xlabel("p (E18's j = 0 curve)")
    ax.set_title(f"{title}: steps / √(π(n−1)/2m)")
axes[0].set_ylabel("steps / ideal (mean, 95 % bars)")
axes[0].legend(fontsize=7, loc="upper left")
fig.tight_layout()
fig.savefig(OUT / "shortfall.svg")

# 2. the band: pooled ratios with 95 % intervals.
fig, ax = plt.subplots(figsize=(8, 4))
pairs = [("O6B/O24|schoolbook", "E29 order 6 / E34 order 24", "#2c6e91"),
         ("O6C/O24|schoolbook", "cheap order 6 / E34 order 24", "#4a9c5d"),
         ("O24/O24F|schoolbook", "E34 order 24 / fix F", "#b85c38"),
         ("O24/O24AA|schoolbook", "A/A: order 24, stream A / stream B", "0.45")]
for k, (key, lab, col) in enumerate(pairs):
    rows = summ["band"][key]["rows"]
    xs = [i + (k - 1.5) * 0.12 for i in range(3)]
    ax.errorbar(xs, [r["ratio"] for r in rows], yerr=[[r["ratio"] - r["lo"] for r in rows], [r["hi"] - r["ratio"] for r in rows]],
                fmt="o", color=col, label=lab, capsize=2)
e34 = {"E29 / cheap (E34, 16 walks)": [1.563, 1.254, 1.126], "cheap / cheap (E34, 16 walks)": [1.505, 1.167, 1.026]}
for (lab, v), col in zip(e34.items(), ["#2c6e91", "#4a9c5d"]):
    ax.plot([i + 0.3 for i in range(3)], v, marker="D", mfc="none", ls="none", color=col, label=lab)
ax.axhline(1.0, color="0.4", lw=0.8, ls="--")
ax.set_xticks(range(3), ["0 GAE", "0.1 GAE", "1 GAE"])
ax.set_ylabel("pooled cost ratio (> 1: second arm cheaper)")
ax.set_title("§5a band, schoolbook, 8 curves × 128 walks; bars 95 %")
ax.legend(fontsize=7, ncol=2)
fig.tight_layout()
fig.savefig(OUT / "band.svg")

# 3. Stage 0: the shortfall by cause (fractions of the ideal, 512 fresh walks per order).
fig, ax = plt.subplots(figsize=(8, 3.4))
causes = ["steps/ideal − 1", "fruitless cycles", "distinguished-point losses", "representative ties"]
for k, (arm, col) in enumerate([("Order6Cheap", "#4a9c5d"), ("Order24Cheap", "#2c6e91")]):
    p = s0["pooled"][arm]
    vals = [p["steps_over_ideal"] - 1.0, p["excess_from_fruitless"], 0.0, 0.0]
    ax.bar([i + (k - 0.5) * 0.35 for i in range(4)], vals, width=0.35, color=col,
           yerr=[1.96 * p["se"], 0, 0, 0], capsize=2, label=f"{arm} ({p['walks']} walks)")
rd = s0["pooled"]["Order24Cheap"]
ax.bar([-0.7], [rec["pooled"]["Order24Cheap"]["steps_over_ideal"] - 1.0], width=0.3, color="#b85c38",
       yerr=[1.96 * rec["pooled"]["Order24Cheap"]["se"]], capsize=2, label="order 24, E34 records (128 walks)")
ax.set_xticks([-0.7] + list(range(4)), ["E34 order 24\nsteps/ideal − 1"] + causes, fontsize=7)
ax.axhline(0, color="0.4", lw=0.8)
ax.set_ylabel("fraction of the ideal")
ax.set_title("Stage 0: where the order-24 excess comes from (seeds 41000–41063)")
ax.legend(fontsize=7)
fig.tight_layout()
fig.savefig(OUT / "stage0.svg")
