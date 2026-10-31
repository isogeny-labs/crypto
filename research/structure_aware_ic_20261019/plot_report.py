#!/usr/bin/env python3
"""Plots recorded values only (AGENTS.md "no Python" carve-out for rendering).

Inputs: sessions/*.band.json (ecbench.band/v1) and analysis_B.json written by
`structure_aware_curves analyze`.  The only arithmetic is ratios of sums of the
recorded per-curve means, the same ratios the report tables print.
"""
import json, sys, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

D = os.path.dirname(os.path.abspath(__file__))
OUT = sys.argv[1] if len(sys.argv) > 1 else os.path.join(D, "reports")
os.makedirs(OUT, exist_ok=True)
W = ["w=0", "w=0.1", "w=1"]
COL = {"base": "#4c78a8", "cand": "#e45756", "rho": "#7f7f7f"}


def band(name):
    d = json.load(open(f"{D}/sessions/{name}.band.json"))
    return {(r["curve"], r["arm"]): r["mean_s"] for r in d["rows"]}


# ---------------------------------------------------------------- Test A
fig, axes = plt.subplots(1, 3, figsize=(11, 3.8), sharey=False)
a1, tu = band("e11b_a1"), band("e11b_tuned")
curves = sorted({c for c, _ in a1})
pairs = [  # (label, source, default arm, half arm) per curve index, frozen then tuned
    ("frozen: d9 vs half d8", a1, "mitm-default9", "mitm-half9"),
]
tuned = {0: ("mitm-def-d7", "mitm-half-d6"), 1: ("mitm-def-d7", "mitm-half-d7"), 2: ("mitm-def-d8", "mitm-half-d8")}
for i, c in enumerate(curves):
    ax = axes[i]
    bars = [("default d9\n(frozen)", a1[(c, "mitm-default9")], COL["base"]),
            ("half d8\n(frozen)", a1[(c, "mitm-half9")], COL["cand"]),
            (f"default tuned", tu[(c, tuned[i][0])], COL["base"]),
            (f"half tuned", tu[(c, tuned[i][1])], COL["cand"]),
            ("rho.neg", a1[(c, "rho-neg")], COL["rho"])]
    for k, (lab, v, col) in enumerate(bars):
        ax.bar(k, v[1], color=col, alpha=0.85)
        ax.plot([k, k], [v[0], v[2]], color="k", lw=1.2)
    ax.set_xticks(range(len(bars)))
    ax.set_xticklabels([b[0] for b in bars], fontsize=7)
    ax.set_title(c.split("-")[1] + " " + c.split("-")[2], fontsize=9)
    ax.set_yscale("log")
    ax.set_ylabel("S = GAE / sqrt(r)  (bar w=0.1, whisker w=0..1)" if i == 0 else "")
fig.suptitle("Test A: MITM whole-method S, half base vs default, a=1 cofactor-2 curves (8 targets each)", fontsize=10)
fig.tight_layout()
fig.savefig(f"{OUT}/testA_mitm_S.svg"); fig.savefig(f"{OUT}/testA_mitm_S.png", dpi=150); plt.close(fig)

# ---------------------------------------------------------------- Test B
p = f"{D}/analysis_B.json"
if os.path.exists(p):
    an = json.load(open(p))
    res = an["results"]
    bits = [r["bits"] for r in res]
    fig, axes = plt.subplots(1, 2, figsize=(11, 4))
    for w, mk in zip(range(3), ["o", "s", "^"]):
        q = [r["band"][w]["Q_j0_over_random"] for r in res]
        axes[0].errorbar([b + 0.4 * (w - 1) for b in bits], [x[0] for x in q],
                         yerr=[[x[0] - x[1] for x in q], [x[2] - x[0] for x in q]], fmt=mk, capsize=3, label=W[w])
    axes[0].axhline(1, color="k", lw=0.8)
    axes[0].set_xlabel("bits of p"); axes[0].set_ylabel("Q = (IC/rho on j=0) / (IC/rho on random)")
    axes[0].set_title("Q < 1 means j=0 IC/rho is better"); axes[0].legend()
    for key, lab, mk in [("IC_gain_A2_over_A6_on_j0", "IC gain (A=2 -> A=6 fold)", "o"),
                         ("rho_gain_neg_over_six", "rho gain (neg -> six-fold)", "s")]:
        v = [r["band"][1][key] for r in res]
        axes[1].errorbar(bits, [x[0] for x in v], yerr=[[x[0] - x[1] for x in v], [x[2] - x[0] for x in v]],
                         fmt=mk, capsize=3, label=lab)
    axes[1].axhline(6 ** 0.5, color="gray", ls=":", label="sqrt(6)")
    axes[1].set_xlabel("bits of p"); axes[1].set_ylabel("gain on j=0 curves (w=0.1)")
    axes[1].set_title("what the six-fold buys each side"); axes[1].legend(fontsize=8)
    fig.suptitle("Test B: j=0 against the random control (6 j=0, 12 random curves per size, 8 targets each)", fontsize=10)
    fig.tight_layout()
    fig.savefig(f"{OUT}/testB_Q.svg"); fig.savefig(f"{OUT}/testB_Q.png", dpi=150); plt.close(fig)
print("wrote", OUT)
