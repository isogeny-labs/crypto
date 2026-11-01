#!/usr/bin/env python3
"""Render figures/cost_landscape.{svg,png} from cost_landscape.py's model (layout only; every value is
recomputed by importing cost_landscape, whose inputs are cited there)."""
import math
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

import cost_landscape as cl

HERE = Path(__file__).resolve().parent
FIG = HERE / "figures"
FIG.mkdir(exist_ok=True)

xs = [28 + 0.25 * i for i in range(0, 289)]          # log2 c from 28 to 100
U1, U10 = 0.0, math.log2(10)
full1 = [cl.full_base(x, U1)[0] for x in xs]
full10 = [cl.full_base(x, U10)[0] for x in xs]
tun1 = [cl.tuned_base(x, U1)[0] for x in xs]
tun10 = [cl.tuned_base(x, U10)[0] for x in xs]
dlp = [cl.double_large_prime(x, U1)[0] for x in xs]

C = {"full": "#0072B2", "tuned": "#009E73", "dlp": "#D55E00", "jv": "#CC79A7", "rho": "#000000",
     "target": "#E69F00"}
fig, ax = plt.subplots(figsize=(9.2, 5.2))
ax.fill_between(xs, tun1, tun10, color=C["tuned"], alpha=0.25, lw=0)
ax.plot(xs, tun10, color=C["tuned"], lw=2, label="Gaudry–Diem, factor base tuned to c (u = 1–10)")
ax.plot(xs, full10, color=C["full"], lw=1.4, ls="--", label="Gaudry–Diem, full factor base (u = 10)")
ax.plot(xs, dlp, color=C["dlp"], lw=1.6, label="Gaudry–Diem with large primes (n!·c·q^{8/5})")
lo, hi = cl.LOG_RHO
ax.axhspan(lo, hi, color="#999999", alpha=0.35, lw=0)
ax.text(68, hi + 0.9, f"Pollard rho: 2^{cl.LOG_RHO_STEPS:.1f} steps = 2^{lo:.1f}–2^{hi:.1f} Mp", fontsize=8.5)
jv_lo, jv_hi = 131.6 + 31, 131.6 + 34
ax.axhspan(jv_lo, jv_hi, xmin=0, xmax=0.18, color=C["jv"], alpha=0.35, lw=0)
ax.text(29, jv_lo - 3.2, "JV (n−1)-point, c̃ from JV's 8.5 s timing", fontsize=8, color=C["jv"])
ax.axhline(128, color=C["target"], lw=1.4, ls=":")
ax.text(87, 129, "designer's 128-bit target", fontsize=8.5, color=C["target"])
ax.axhline(142, color="#555555", lw=1, ls="-.")
ax.text(87, 143, "designer's ≥ 2^142 claim", fontsize=8.5, color="#555555")
bands = [("2-torsion quotient D = 2^16 (conjectured)", 32.0, 50.3, "#56B4E9"),
         ("S5-symmetrized D = 2^20", 40.0, 62.3, "#E69F00"),
         ("raw D = 2^26.9", 53.8, 83.0, "#D55E00")]
for i, (name, a, b, col) in enumerate(bands):
    y = 112 - 3.4 * i
    ax.annotate("", xy=(a, y), xytext=(b, y), arrowprops=dict(arrowstyle="<->", color=col, lw=1.6))
    ax.text(b + 0.6, y - 0.8, f"c ∈ [D², 5·D³]: {name}", fontsize=8, color=col)
ax.set_xlim(28, 100)
ax.set_ylim(100, 185)
ax.set_xlabel("per-system solving cost c (log2 of GF(p) multiplications)")
ax.set_ylabel("total cost (log2 of GF(p) multiplications)")
ax.set_title("EcGFp5: cost of the Gaudry–Diem family vs per-system cost (model, not measured)\n"
             "the tuned-base variant stays below rho until c ≈ 2^95 and crosses the 128-bit target near c ≈ 2^48–2^55", fontsize=10)
ax.legend(loc="upper left", fontsize=8.5, frameon=False, bbox_to_anchor=(0.0, 0.93))
for s in ("top", "right"):
    ax.spines[s].set_visible(False)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(FIG / f"cost_landscape.{ext}", dpi=160, bbox_inches="tight")
print("wrote", sorted(p.name for p in FIG.iterdir()))
