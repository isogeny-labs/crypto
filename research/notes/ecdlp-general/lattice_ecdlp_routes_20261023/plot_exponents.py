#!/usr/bin/env python3
"""Lay out the derived exponents of exponents.json (no computation here beyond
reading the recorded values). Writes exponents.svg beside this file.
Run: /tmp/report-venv/bin/python plot_exponents.py"""
import json, pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = pathlib.Path(__file__).resolve().parent
d = json.loads((here / "exponents.json").read_text())
rows = d["rows"]
m = [r["m"] for r in rows]

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(11, 5.6))

# Panel 1: cost exponents against m
ax1.plot(m, [r["e_rel"] for r in rows], "o-", color="#b2182b",
         label="relation collection, Coppersmith ceiling θ_m (derived)")
ax1.plot(m, [r["e_la"] for r in rows], "s-", color="#ef8a62",
         label="linear algebra at β_max = θ_m/m (derived)")
ax1.plot(m, [r["oracle_e_rel"] for r in rows], "o--", color="#2166ac",
         label="relation collection if θ = 1 (oracle; unattainable by lattices)")
ax1.plot(m, [r["oracle_e_la"] for r in rows], "s--", color="#67a9cf",
         label="linear algebra if θ = 1 (oracle)")
ax1.plot(m, [r["gaudry_weil_descent"] for r in rows], "^:", color="#4d4d4d",
         label="Gaudry 2009 Weil descent, E(F_{q^n}), n = m (literature; not over F_p)")
ax1.axhline(d["rho_exponent"], color="black", lw=1.2)
ax1.text(m[0] + 0.1, d["rho_exponent"] + 0.015, "Pollard rho: 1/2", ha="left", fontsize=8)
ax1.set_xlabel("summands m in the decomposition R = P₁ + … + P_m")
ax1.set_ylabel("cost exponent e  (cost ≈ p^e group/lattice operations)")
ax1.set_ylim(0, 1.05)
ax1.set_xticks(m)
ax1.grid(alpha=0.3)
ax1.set_title("Small-x factor base, lattice (Coppersmith) decomposition", fontsize=10)
ax1.legend(fontsize=7, loc="upper center", bbox_to_anchor=(0.5, -0.17), ncol=1, frameon=False)

# Panel 2: root-region exponent available vs needed
ax2.plot(m, [r["theta_m"] for r in rows], "o-", color="#b2182b",
         label="θ_m = 2 / ((m+1)·2^(m−1)): Coppersmith ceiling (derived)")
ax2.plot(m, [r["theta_needed"] for r in rows], "D-", color="#2166ac",
         label="θ needed for e_rel ≤ 1/2: m / (2(m−1)) (derived)")
ax2.axhline(1.0, color="black", lw=1.0, ls="--")
ax2.text(m[-1] + 0.05, 1.015, "uniqueness ceiling θ = 1", ha="right", fontsize=8)
ax2.set_yscale("log")
ax2.set_ylim(1e-3, 1.5)
ax2.set_xticks(m)
ax2.set_xlabel("summands m")
ax2.set_ylabel("root-region exponent θ  (∏ Xᵢ = p^θ)")
ax2.grid(alpha=0.3, which="both")
ax2.set_title("Root region a lattice can reach vs. what rho-parity needs", fontsize=10)
ax2.legend(fontsize=7, loc="upper center", bbox_to_anchor=(0.5, -0.17), ncol=1, frameon=False)

fig.suptitle("Derived exponents, RESEARCH_LATTICE_ECDLP_ROUTES.md §6 (no measurement; "
             "values in exponents.json)", fontsize=9)
fig.tight_layout()
fig.savefig(here / "exponents.svg")
fig.savefig(here / "exponents.png", dpi=150)
print("wrote", here / "exponents.svg")
