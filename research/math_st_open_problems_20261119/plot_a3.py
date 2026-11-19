"""Diagram of the A3 reduction chain (layout only; no computation)."""
import matplotlib; matplotlib.use("svg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
boxes = [
    ("Q025: detect that walk states P, Q\nlie in one ω-orbit (ω non-automorphism)", "#fdebd0"),
    ("(B) [PROVED]  same orbit  ⇔  χ(log P) = χ(log Q)\nχ: character of order k = [F_r* : ⟨λ⟩]", "#d6eaf8"),
    ("(C) [PROVED]  generic model + ω-oracle:\npredicting χ(log P) with advantage ε needs Ω(√(εr))", "#d5f5e3"),
    ("Closed generically. Open only non-generically:\na coordinate route to χ(log P) costing ≪ r^(1/6)\n(would beat √r via power-residue-PRF key recovery)", "#f2f3f4"),
]
fig, ax = plt.subplots(figsize=(8, 6.4)); ax.axis("off")
for i, (t, c) in enumerate(boxes):
    y = 0.84 - i * 0.255
    ax.add_patch(FancyBboxPatch((0.08, y - 0.085), 0.84, 0.17, boxstyle="round,pad=0.01", fc=c, ec="#555"))
    ax.text(0.5, y, t, ha="center", va="center", fontsize=9.5)
    if i < len(boxes) - 1:
        ax.annotate("", (0.5, y - 0.1), (0.5, y - 0.155), arrowprops=dict(arrowstyle="<-", color="#333"))
ax.text(0.5, 0.995, "(A) [PROVED]: ω of degree d ≥ 2 has orbit length m ≥ 2·log(√r − 1)/log d on ⟨G⟩",
        ha="center", va="top", fontsize=8.5, style="italic")
ax.set_xlim(0, 1); ax.set_ylim(-0.04, 1)
fig.suptitle("A3 (math-st P5.2/Q025): orbit collisions reduce to residue characters of the logarithm", fontsize=10, y=0.02)
fig.savefig("a3_reduction.svg", bbox_inches="tight")
