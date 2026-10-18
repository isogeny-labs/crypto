"""E25 figures: lays out recorded values from experiments/30_gls_v4 and
experiments/29_quartic_pipeline_v4 (no computation beyond ratios of
recorded fields).  Run from the repository root:
    python3 research/glv_gls_v4_20261018/figures/plot.py
"""
import glob
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

OUT = os.path.dirname(os.path.abspath(__file__))
rows = [r for f in sorted(glob.glob("experiments/30_gls_v4/p*s*.json")) for r in json.load(open(f))]
e24 = [r for f in sorted(glob.glob("experiments/29_quartic_pipeline_v4/p*s*.json")) for r in json.load(open(f))]
A = {(r["p"], r["seed"]): r for r in rows if r["arm"] == "A"}
B = {(r["p"], r["seed"]): r for r in rows if r["arm"] == "B"}
keys = sorted(k for k in A if k in B)
C_A, C_B, C_E24, C_RHO = "#4C72B0", "#DD8452", "#8C8C8C", "#55A868"


def save(fig, name):
    fig.tight_layout()
    fig.savefig(os.path.join(OUT, name + ".svg"))
    fig.savefig(os.path.join(OUT, name + ".png"), dpi=150)
    plt.close(fig)


# 1. S per curve, both arms.
fig, ax = plt.subplots(figsize=(8, 4))
x = range(len(keys))
w = 0.38
ax.bar([i - w / 2 for i in x], [A[k]["total_muls"] for k in keys], w, label="A: V₄ fold (control)", color=C_A)
ax.bar([i + w / 2 for i in x], [B[k]["total_muls"] for k in keys], w, label="B: ⟨V₄, ψ⟩ fold", color=C_B)
for i, k in enumerate(keys):
    ax.annotate(f"{A[k]['total_muls'] / B[k]['total_muls']:.2f}×", (i, B[k]["total_muls"]), ha="center",
                va="bottom", fontsize=8, xytext=(0, 18), textcoords="offset points")
ax.set_xticks(list(x), [f"p={p}\ns{s}" for p, s in keys], fontsize=8)
ax.set_yscale("log")
ax.set_ylabel("S, end to end (F_p multiplications)")
ax.set_title(f"E25: both arms on one GLS curve, one base, one target stream ({len(keys)} curves)", fontsize=10)
ax.legend(fontsize=8)
save(fig, "arms_s")

# 2. Distance to rho against log2 n, with E24.
fig, ax = plt.subplots(figsize=(7, 4.2))
ax.scatter([A[k]["bits"] for k in keys], [A[k]["total_muls"] / A[k]["rho_negation_muls"] for k in keys],
           color=C_A, label="E25 A / negation walk", marker="o")
ax.scatter([B[k]["bits"] for k in keys], [B[k]["ratio"] for k in keys], color=C_B,
           label="E25 B / ⟨±ψ⟩ walk (matched)", marker="s")
ax.scatter([r["bits"] for r in e24], [r["ratio"] for r in e24], color=C_E24, label="E24 (h = 4) / negation walk",
           marker="^")
ax.set_yscale("log")
ax.set_xlabel("log₂ n")
ax.set_ylabel("S / rho (F_p multiplications)")
ax.set_title("Distance to rho, every curve (16 walks per curve and fold)", fontsize=10)
ax.legend(fontsize=8)
save(fig, "distance_to_rho")

# 3. The walks: steps and multiplications, negation vs ±ψ.
fig, axs = plt.subplots(1, 2, figsize=(9, 3.8))
for ax, field, lab in ((axs[0], "rho_{}_steps", "mean steps"), (axs[1], "rho_{}_muls", "mean F_p multiplications")):
    neg = [B[k][field.format("negation")] for k in keys]
    psi = [B[k][field.format("matched")] for k in keys]
    ax.bar([i - w / 2 for i in x], neg, w, label="negation", color=C_E24)
    ax.bar([i + w / 2 for i in x], psi, w, label="⟨±ψ⟩", color=C_RHO)
    ax.set_xticks(list(x), [f"{p}\ns{s}" for p, s in keys], fontsize=7)
    ax.set_yscale("log")
    ax.set_ylabel(lab)
    ax.legend(fontsize=8)
axs[0].set_title("Walk steps (16 walks each)", fontsize=10)
axs[1].set_title("Walk cost, set-up and class representative included", fontsize=10)
save(fig, "walks")
print("figures:", keys)
