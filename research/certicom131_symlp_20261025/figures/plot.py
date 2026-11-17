#!/usr/bin/env python3
"""Render the recorded values in figures/plotdata.json to SVG + 150 dpi PNG.

Plots recorded values only (research-visuals: Python lays out, never
computes a result). Run with the report venv's python.
"""
import json
import math
import pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = pathlib.Path(__file__).resolve().parent
data = json.loads((HERE / "plotdata.json").read_text())


def save(fig, stem):
    fig.savefig(HERE / f"{stem}.svg")
    fig.savefig(HERE / f"{stem}.png", dpi=150)
    plt.close(fig)


# ── Fig 1: step rates, symmetric oracle vs per-point baseline vs rho ──
rate = data["rate"]
binr = [r for r in rate if r["family"] == "binary"]
prir = [r for r in rate if r["family"] == "prime"]
fig, axes = plt.subplots(1, 2, figsize=(10, 4.2))
for ax, rows, title in ((axes[0], binr, "binary (ECC2-131 + calibration)"),
                        (axes[1], prir, "prime (ECCp-131 + calibration)")):
    rows = sorted(rows, key=lambda r: r["n"])
    ns = [r["n"] for r in rows]
    ax.plot(ns, [r["base"] / 1e6 for r in rows], "o-", label="per-point baseline")
    ax.plot(ns, [r["sym"] / 1e6 for r in rows], "s-", label="symmetric (e₁) oracle")
    rr = rows[0]["rho"] / 1e6
    ax.axhline(rr, ls="--", color="k", label=f"rho adding-walk ({rr:.1f})")
    ax.set_title(title)
    ax.set_xlabel("field bits n")
    ax.set_ylabel("Msteps / s (1 thread)")
    ax.grid(alpha=0.3)
    ax.legend(fontsize=8)
fig.suptitle("Decomposition-oracle step rate vs the rho reference step\n"
             "(same field code, same host; seed 20261025)")
fig.tight_layout()
save(fig, "fig1_step_rate")

# ── Fig 2: decomposition yield large-prime split ──
yl = sorted(data["yield"], key=lambda y: y["n"])
fig, ax = plt.subplots(figsize=(7, 4.2))
ns = [y["n"] for y in yl]
d0 = [max(y["d0"], 0.5) for y in yl]
d1 = [max(y["d1"], 0.5) for y in yl]
d2 = [max(y["d2"], 0.5) for y in yl]
ax.semilogy(ns, d2, "s-", label="2 large primes")
ax.semilogy(ns, d1, "o-", label="1 large prime")
ax.semilogy(ns, d0, "^-", label="direct (0 large primes)")
ax.set_xlabel("field bits n (l = ⌈n/3⌉, l' per record)")
ax.set_ylabel("decompositions over all targets (log)")
ax.set_title("Decomposition yield by large-prime count\n"
             "direct relations are rare; the 2-lp bulk needs the graph")
ax.grid(alpha=0.3, which="both")
ax.legend()
fig.tight_layout()
save(fig, "fig2_yield_split")

# ── Fig 3: distance to rho, exponent view ──
fig, ax = plt.subplots(figsize=(7, 4.2))
ns = list(range(16, 136, 2))
ic = [(2.0 / 3.0) * n for n in ns]        # m=2 subspace optimum ~ 2^{2n/3}
rho = [0.5 * n for n in ns]               # rho ~ 2^{n/2}
ax.plot(ns, ic, label="m=2 subspace IC  ~ 2^{2n/3}  (measured slope 0.647, PR #2493)")
ax.plot(ns, rho, label="Pollard rho  ~ 2^{n/2}")
for d in data["distance"]:
    ax.annotate(f"{d['curve']}\ngap ≈ 2^{{{d['dist']:.0f}}} steps",
                xy=(d["n"], (2.0/3.0)*d["n"]), xytext=(d["n"]-48, (2.0/3.0)*d["n"]+4),
                fontsize=8, arrowprops=dict(arrowstyle="->", lw=0.8))
ax.axvline(131, ls=":", color="grey")
ax.set_xlabel("field bits n")
ax.set_ylabel("log₂ steps")
ax.set_title("Distance to rho is the 2n/3 vs n/2 exponent gap\n"
             "symmetry breaking and the 2-lp graph are constant-factor, measured")
ax.grid(alpha=0.3)
ax.legend(fontsize=8)
fig.tight_layout()
save(fig, "fig3_distance_to_rho")

print("rendered fig1_step_rate, fig2_yield_split, fig3_distance_to_rho (svg+png)")
