# Layout only: plots the paired ratios recorded by `ecbench compare --weight` in
# <study>/P<b>/analysis/compare-*.json (288 pairs each). Nothing is computed here.
# The compare JSON is stored through tools/artifacts where large; pull first.
# Run from the study directory:  python3 figures/plot.py
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

W = ("0", "0.1", "1")
COL = {"0": "#2563eb", "0.1": "#7c3aed", "1": "#dc2626"}
SIZES = (24, 28, 32)
BASE = {24: "walk", 28: "srenew", 32: "walk"}

def ops(st, b, a, c, w):
    o = json.load(open(f"{st}/P{b}/analysis/compare-{a}__{c}-w{w}.json"))["ops"]
    return o["ratio_b_over_a"], o["ci95_bootstrap"]

fig, axs = plt.subplots(1, 3, figsize=(13, 3.9))
for k, (st, title) in enumerate((("v3", "v3 (confirmatory): Σ S_dg-small / Σ S_baseline"),
                                 ("first-study", "first study (descriptive only, §6.1 failed)"))):
    ax = axs[k]
    for j, w in enumerate(W):
        for i, b in enumerate(SIZES):
            r, (lo, hi) = ops(st, b, BASE[b], "dg-small", w)
            ax.errorbar(i + (j - 1) * 0.2, r, yerr=[[r - lo], [hi - r]], fmt="o", color=COL[w], capsize=3,
                        label=f"w = {w}" if i == 0 else None)
    ax.axhline(1.0, color="#6b7280", lw=0.8, ls="--", label="baseline")
    ax.set_ylim(0.7, 1.05)
    ax.set_title(title + ", 95% cluster bootstrap", fontsize=8)
    ax.set_xticks(range(3)); ax.set_xticklabels([f"{b} bits\n({BASE[b]})" for b in SIZES])
ax = axs[2]
for j, (arm, lab, col, mk) in enumerate((("base", "baseline", "#6b7280", "s"), ("dg-small", "dg-small", "#dc2626", "o"))):
    for i, b in enumerate(SIZES):
        r, (lo, hi) = ops("v3", b, "rho-neg", BASE[b] if arm == "base" else arm, "0")
        ax.errorbar(i + (j - 0.5) * 0.2, r, yerr=[[r - lo], [hi - r]], fmt=mk, color=col, capsize=3,
                    label=lab if i == 0 else None)
ax.set_yscale("log")
ax.set_title("v3: Σ S_arm / Σ S_rho-neg at w = 0 (distance remaining to rho)", fontsize=8)
ax.set_xticks(range(3)); ax.set_xticklabels([f"{b} bits" for b in SIZES])
for ax in axs:
    ax.legend(fontsize=7)
fig.suptitle("XP-20261029 deferred-guard stride-renew at a smaller factor base, prime field, "
             "2 curves × 24 targets × 6 rounds per size (isolab-eks, counted units only)", fontsize=9)
fig.tight_layout()
fig.savefig("figures/ratios.svg"); fig.savefig("figures/ratios.png", dpi=150)
