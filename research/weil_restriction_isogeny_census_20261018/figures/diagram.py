"""Draw the mechanism diagram (layout only; no computation).

Usage: python diagram.py <out-dir>
Writes mechanism.svg and mechanism.png.
"""

import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
from matplotlib.patches import FancyBboxPatch  # noqa: E402

BLUE, ORANGE, AQUA, PURPLE, RED = "#2a78d6", "#eb6834", "#1baf7a", "#8e5bd6", "#c0392b"
INK, INK2, GRID, PALE = "#0b0b0b", "#52514e", "#d9d8d4", "#f2f1ee"

out = Path(sys.argv[1])
out.mkdir(parents=True, exist_ok=True)
fig, ax = plt.subplots(figsize=(12.5, 6.2))
ax.set_xlim(0, 100)
ax.set_ylim(0, 60)
ax.axis("off")


def box(x, y, w, h, text, color, fs=8, fc=None):
    ax.add_patch(
        FancyBboxPatch(
            (x, y), w, h, boxstyle="round,pad=0.4,rounding_size=1.2", linewidth=1.4, edgecolor=color, facecolor=fc or "white"
        )
    )
    ax.text(x + w / 2, y + h / 2, text, ha="center", va="center", fontsize=fs, color=INK, wrap=True)


def arrow(x1, y1, x2, y2, color=INK2, text=None, fs=7, dy=1.2):
    ax.annotate(
        "", xy=(x2, y2), xytext=(x1, y1), arrowprops=dict(arrowstyle="-|>", color=color, lw=1.3, shrinkA=2, shrinkB=2)
    )
    if text:
        ax.text((x1 + x2) / 2, (y1 + y2) / 2 + dy, text, ha="center", va="bottom", fontsize=fs, color=INK2)


# Row 1: the isogeny route (this round).
y = 44
box(1, y, 16, 10, "E / F_{q^3}\n(#E prime allowed)", BLUE, fs=8.5)
box(21, y, 20, 10, "W = Res_{F_{q^3}/F_q} E, dim 3\nW[ℓ] = V_0 ⊕ V_1 ⊕ V_2\nπ(v_0,v_1,v_2) = (φv_2, v_0, v_1), π^3 = φ", BLUE)
box(45, y, 19, 10, "Tian kernel Λ ⊂ W[ℓ]:\nF_q-rational, Lagrangian for ω_V = ⊕ q̄^i ω,\nΛ ∩ V_i = 0", RED)
box(68, y, 14, 10, "A = W/Λ, ppav of dim 3\n→ Jac(C), C a plane quartic\nwhen #E is prime", PURPLE)
box(86, y, 13, 10, "Diem–Thomé\nindex calculus Õ(q)\n(rho q^{3/2}, Gaudry q^{4/3})", ORANGE)
arrow(17, y + 5, 21, y + 5)
arrow(41, y + 5, 45, y + 5)
arrow(64, y + 5, 68, y + 5)
arrow(82, y + 5, 86, y + 5)
ax.text(50, y - 1.6, "arrows, left to right: Weil restriction · Part G (does Λ exist? by the class of φ) · (ℓ,ℓ,ℓ)-isogeny from Λ, Õ(ℓ^3) [Tian] · DLP transfer", ha="center", fontsize=7.2, color=INK2)

# Row 2: Theorem 4 (the decision by the class of φ).
y2 = 24
box(21, y2, 61, 14, "", INK2, fc=PALE)
ax.text(51.5, y2 + 12.3, "Theorem 4 (n = 3, ℓ ≠ p): a Tian kernel exists iff — by the class of φ = π_{q^3}|E[ℓ] in GL_2(F_ℓ)", ha="center", fontsize=8.3, color=INK)
cells = [
    ("nonsplit\n(χ_φ irreducible mod ℓ)", "never:\nno submodule of dim 3", GRID),
    ("split diag(λ, μ)", "iff λ (⇔ μ) is a cube in F_ℓ\n(always when ℓ ≢ 1 mod 3)", RED),
    ("scalar λ·I", "always\n(λ is a cube automatically)", RED),
    ("Jordan J(λ)", "iff ℓ ≡ 1 (mod 3)\n(T^3 − λ splits)", ORANGE),
]
for i, (h, b, col) in enumerate(cells):
    x = 23 + i * 15
    ax.add_patch(FancyBboxPatch((x, y2 + 1.2), 13.5, 9.6, boxstyle="round,pad=0.3,rounding_size=0.8", linewidth=1.2, edgecolor=col, facecolor="white"))
    ax.text(x + 6.75, y2 + 8.6, h, ha="center", va="center", fontsize=7.4, color=INK, fontweight="bold")
    ax.text(x + 6.75, y2 + 4.2, b, ha="center", va="center", fontsize=7.2, color=INK2)
arrow(54.5, y, 54.5, y2 + 14, color=RED)

# Row 3: the cover route (round one) for contrast.
y3 = 4
box(1, y3, 16, 10, "cover route (round one):\nC → E of degree 2^m,\nJac(C) ⊇ W", AQUA)
box(21, y3, 30, 10, "ℓ = 2 kernels exist iff φ = I on E[2]\n(check 1, check 7): full rational 2-torsion,\nso #E ≡ 0 (mod 4) — never a prime-order curve", AQUA)
box(55, y3, 44, 10, "the auditor's test for the isogeny route is polynomial-time:\nq mod ℓ, t mod ℓ and the factor pattern of ψ_ℓ over F_{q^3} (Part H);\nthe affected set is a constant fraction (not hideable: margin a < 0)", INK2)
arrow(17, y3 + 5, 21, y3 + 5)
ax.text(1, 57.5, "Weil-restriction isogeny census (n = 3): where the isogeny association reaches curves no cover reaches", fontsize=10, color=INK, fontweight="bold")
ax.text(1, 1.2, "source: PROTOCOL.md, results/run-20261018T1605Z (Parts G, H); Theorem 4 in REPORT.md §3", fontsize=7, color=INK2)
fig.savefig(out / "mechanism.svg", bbox_inches="tight")
fig.savefig(out / "mechanism.png", dpi=150, bbox_inches="tight")
print("wrote mechanism.svg/png")
