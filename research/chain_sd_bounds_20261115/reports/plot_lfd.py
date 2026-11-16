#!/usr/bin/env python3
"""Plot recorded values only (AGENTS.md: Python lays out recorded values, computes no result).

Figure 1: last fall degree LFD(k) of the normalized two-summand Kummer chain system against depth k:
  - certified here (replay certificates verified by the independent C checker),
  - computed here without certificate file (engine run, verify_closed sweep),
  - lane-P values quoted from research/leads_20261112/BRIEFING.md (unverified here),
  - the proved bounds: LFD >= 4 (k >= 2), LFD <= 2k+2, and the transferred lower bounds.
Figure 2: dim V_D(k) for D = 3, 4, 5 against k (recorded ranks) with the ambient column counts.
Values are read from values.json beside this script.
"""
import json, os, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
V = json.load(open(os.path.join(here, "values.json")))

# ---------------------------------------------------------------- figure 1
fig, ax = plt.subplots(figsize=(7.2, 4.4))
ks = list(range(1, 17))
ax.plot(ks, [2 * k + 1 for k in ks], color="#888888", ls="--", lw=1, label="proved upper bound 2k+1 (Theorem 7)")
ax.plot(ks, [(k + 14) // 4 for k in ks], color="#1f77b4", ls=":", lw=1.2, label="lane-P law floor((k+14)/4) (empirical)")
cert = V["certified"]
ax.scatter([c["k"] for c in cert], [c["lfd"] for c in cert], marker="o", s=46, color="#d62728", zorder=5,
           label="certified exact LFD (upper+lower replay certificates, p=12289 n=11)")
comp = V["computed"]
if comp: ax.scatter([c["k"] for c in comp], [c["lfd"] for c in comp], marker="s", s=40, facecolors="none", edgecolors="#d62728", zorder=4,
           label="computed exact LFD (engine, verify_closed; no certificate file)")
lane = V["laneP_unverified"]
ax.scatter([c["k"] for c in lane], [c["lfd"] for c in lane], marker="x", s=40, color="#1f77b4", zorder=4,
           label="lane-P values (ml-cryptanalysis, not re-verified here)")
lb = V["lower_bounds_transferred"]
for c in lb:
    ax.annotate("", xy=(c["k_to"], c["bound"]), xytext=(c["k_from"], c["bound"]),
                arrowprops=dict(arrowstyle="->", color="#2ca02c", lw=1.3))
ax.scatter([c["k_from"] for c in lb], [c["bound"] for c in lb], marker="^", s=40, color="#2ca02c", zorder=6,
           label="certified lower bound LFD >= D+1, transferred to larger k by depth monotonicity (Lean)")
ax.axhline(4, color="#9467bd", lw=1, ls="-.", label="proved LFD >= 4 for all k >= 2 (Theorem 6)")
ax.set_xlabel("depth k  (N = 2^k, 2k variables)")
ax.set_ylabel("last fall degree = solving degree (unit ideal)")
ax.set_xticks(ks)
ax.set_ylim(2.5, 9.5)
ax.set_xlim(0.5, 16.5)
ax.grid(alpha=0.25)
ax.legend(fontsize=6.6, loc="upper left")
ax.set_title("LFD(k) of the normalized two-summand Kummer chain, F_p, run 2026-11-15", fontsize=10)
fig.tight_layout()
fig.savefig(os.path.join(here, "lfd_vs_k.svg"))
fig.savefig(os.path.join(here, "lfd_vs_k.png"), dpi=150)

# ---------------------------------------------------------------- figure 2
fig, ax = plt.subplots(figsize=(7.2, 4.4))
dims = V["dims"]
for D, color in [(3, "#2ca02c"), (4, "#d62728"), (5, "#1f77b4")]:
    rows = [r for r in dims if r["D"] == D]
    ax.plot([r["k"] for r in rows], [r["rank"] for r in rows], marker="o", color=color, label=f"dim V_{D}(k) (recorded rank)")
    ax.plot([r["k"] for r in rows], [r["columns"] for r in rows], ls="--", color=color, alpha=0.6, label=f"columns C(2k,<={D})")
ax.set_yscale("log")
ax.set_xlabel("depth k")
ax.set_ylabel("dimension")
ax.grid(alpha=0.25, which="both")
ax.legend(fontsize=7)
ax.set_title("Closure dimensions below the LFD: dim V_D(k) against the ambient weight-<=D space", fontsize=10)
fig.tight_layout()
fig.savefig(os.path.join(here, "dims_vs_k.svg"))
fig.savefig(os.path.join(here, "dims_vs_k.png"), dpi=150)
print("wrote lfd_vs_k.{svg,png} dims_vs_k.{svg,png}")
