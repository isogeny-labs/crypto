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

# 4. E26: the cofactor census (experiments/31_gls_cofactor/census.json).
census = [r for r in json.load(open("experiments/31_gls_cofactor/census.json")) if "error" not in r]
ps = sorted({r["p"] for r in census})
fig, ax = plt.subplots(figsize=(8, 3.8))
for i, (cls, lab, col) in enumerate(((2, "t ≡ 2 (mod 4): v₂(#E) = 2, halves of T₂ NOT x-rational", C_A),
                                     (0, "4 | t: v₂(#E) ≥ 4, halves of T₂ x-rational", C_B))):
    ys = [sum(1 for r in census if r["p"] == p and r["t_mod_4"] == cls) for p in ps]
    ax.bar([j + (i - 0.5) * 0.4 for j in range(len(ps))], ys, 0.4, label=lab, color=col)
ax.set_xticks(range(len(ps)), [str(p) for p in ps], fontsize=8)
ax.set_xlabel("p")
ax.set_ylabel("curves (of 40 per p)")
ax.set_title("E26: 560 GLS twists over F_p⁴ — the V₄ line's condition and h = 4 never meet", fontsize=10)
ax.legend(fontsize=8)
save(fig, "census")
