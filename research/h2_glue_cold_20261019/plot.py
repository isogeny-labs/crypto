#!/usr/bin/env python3
"""Plots recorded values only (analysis.json / cold.csv of one run)."""
import csv, json, sys, os
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

run, out = sys.argv[1], sys.argv[2]
A = json.load(open(os.path.join(run, "analysis.json")))
rid = os.path.basename(run.rstrip("/"))
os.makedirs(out, exist_ok=True)
def save(fig, name):
    fig.savefig(os.path.join(out, name + ".svg"), metadata={"Date": None})
    fig.savefig(os.path.join(out, name + ".png"), dpi=150)
    plt.close(fig)

# 1. ratio grid per nu
cells = A["cells"]
sig = sorted({c["sigma"] for c in cells}); iot = sorted({c["iota"] for c in cells})
fig, axs = plt.subplots(1, 3, figsize=(12.5, 3.9), sharey=True)
for ax, nu in zip(axs, ["0.0", "0.1", "1.0"]):
    M = np.zeros((len(sig), len(iot))); 
    for c in cells:
        M[sig.index(c["sigma"]), iot.index(c["iota"])] = c["nu"][nu]["J_over_cmp"][0]
    im = ax.imshow(np.log2(M), cmap="RdBu_r", vmin=-1.3, vmax=1.3, aspect="auto")
    for i in range(len(sig)):
        for j in range(len(iot)):
            c = [x for x in cells if x["sigma"] == sig[i] and x["iota"] == iot[j]][0]
            lo, hi = c["nu"][nu]["J_over_cmp"][1:]
            ax.text(j, i, f"{M[i,j]:.2f}\n[{lo:.2f},{hi:.2f}]", ha="center", va="center", fontsize=7.5)
    ax.set_xticks(range(len(iot)), [f"{x:g}" for x in iot]); ax.set_xlabel("I/M")
    ax.set_title(f"native charge ν = {nu} GAE/op")
axs[0].set_yticks(range(len(sig)), [f"{x:g}" for x in sig]); axs[0].set_ylabel("S/M")
fig.suptitle(f"Genus-2 IC (glued, h=1) cost / matched rho cost: ratio [95% CI]; >1 = rho cheaper. 16 instances, {A['N_targets']} targets. Red = rho cheaper.", fontsize=9)
fig.text(0.01, 0.01, f"run {rid}; comparator = cheaper of default negation rho and equal-memory rho; verdicts: rho-wins {A['tally']['rho']}, unresolved {A['tally']['unresolved']}, IC-wins {A['tally']['ic']} of 12", fontsize=7)
fig.tight_layout(rect=(0, 0.04, 1, 0.94)); save(fig, "ratio_grid")

# 2. time x memory
rows = list(csv.DictReader(open(os.path.join(run, "cold.csv"))))
fig, axs = plt.subplots(1, 2, figsize=(10.5, 4.2), sharey=True)
for ax, bits in zip(axs, ["36", "40"]):
    arms = {}
    for r in rows:
        if r["bits"] != bits or "@" in r["arm"]: continue
        arms.setdefault(r["arm"], []).append((float(r["steps"]) / np.sqrt(float(r["ell"])), float(r["words"]) / float(r["p"])))
    for a, col, mk in [("J", "C3", "s"), ("R", "C0", "o"), ("Pl", "C2", "^")]:
        v = np.array(arms[a]); ax.scatter(v[:, 1], v[:, 0], s=6, alpha=0.25, color=col)
        ax.scatter(v[:, 1].mean(), v[:, 0].mean(), s=70, marker=mk, color=col, edgecolor="k", label={"J": "genus-2 IC (J)", "R": "neg. rho, default DP", "Pl": "plain rho"}[a], zorder=5)
    dp = [(d, np.array(arms[f"D{d}"])) for d in [0, 2, 4, 6, 8, 10, 12]]
    ax.plot([v[:, 1].mean() for d, v in dp], [v[:, 0].mean() for d, v in dp], "-o", color="C0", ms=4, label="neg. rho, DP density 2^-d")
    for d, v in dp:
        ax.annotate(f"d={d}", (v[:, 1].mean(), v[:, 0].mean()), fontsize=7, xytext=(3, 4), textcoords="offset points")
    ax.set_xscale("log"); ax.set_xlabel("stored 64-bit words / p"); ax.set_title(f"log2 #E ≈ {bits}"); ax.grid(alpha=0.3)
axs[0].set_ylabel("group-op steps / √ℓ  (time axis)"); axs[0].legend(fontsize=7, loc="center left")
fig.suptitle("Time × memory: J uses 0.45√ℓ steps but 1.5p words; rho at the same memory still takes 0.9√ℓ steps — step count is not cost", fontsize=9)
fig.text(0.01, 0.01, f"run {rid}; small dots = targets, large = mean; step = Jacobian addition (J) or E(F_p²) addition (rho)", fontsize=7)
fig.tight_layout(rect=(0, 0.04, 1, 0.94)); save(fig, "time_memory")

# 3. break-even
be = A["breakeven_by_nu"]
fig, ax = plt.subplots(figsize=(8, 4.2))
lab = [f"σ={b['sigma']:g}\nι={b['iota']:g}" for b in be if b["iota"] in (1, 10)]
sel = [b for b in be if b["iota"] in (1, 10)]
x = np.arange(len(sel)); w = 0.2
ax.bar(x - 1.5*w, [b["measured"] for b in sel], w, color="C3", label="measured Jacobian add (this backend)")
ax.bar(x - 0.5*w, [b["breakeven_by_nu"][0] for b in sel], w, color="C0", label="break-even, ν=0")
ax.bar(x + 0.5*w, [b["breakeven_by_nu"][1] for b in sel], w, color="C1", label="break-even, ν=0.1")
ax.bar(x + 1.5*w, [b["breakeven_by_nu"][2] for b in sel], w, color="C7", label="break-even, ν=1 (negative: even a free add loses)")
ax.plot(x, [b["hyp22"] for b in sel], "k_", ms=22, mew=2, label="22M + I")
ax.axhline(0, color="k", lw=0.6); ax.set_xticks(x, lab); ax.set_ylabel("Jacobian addition cost (M-equivalents)")
ax.set_title("Break-even Jacobian addition cost against the comparator (I/M ∈ {1,10} shown)", fontsize=9); ax.legend(fontsize=7)
fig.text(0.01, 0.01, f"run {rid}; break-even = cost per walk addition at which J equals the comparator, other J costs held at measured values", fontsize=7)
fig.tight_layout(rect=(0, 0.04, 1, 1)); save(fig, "breakeven")
