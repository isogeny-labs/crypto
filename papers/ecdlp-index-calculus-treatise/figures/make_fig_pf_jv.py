"""Joux-Vitse cover vs matched negation rho per p (data-pf-jv.json only).

    python3 make_fig_pf_jv.py   # writes fig-pf-jv.{pdf,svg}
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data-pf-jv.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25,
                     "svg.fonttype": "none", "svg.hashsalt": "pf"})
C = {"U1 group additions": "#0072B2", "U2 textbook F_p mults": "#D55E00"}
LBL = {"U1 group additions": "U1 (group additions)", "U2 textbook F_p mults": "U2 (textbook $\\mathbb{F}_p$ mults)"}
EST = {"(a) per-prime": ("s", -14), "(b) pooled new primes": ("o", 0), "(c) pooled + p=251": ("D", 14)}
fig, ax = plt.subplots(figsize=(6.6, 3.0))
for unit, col in C.items():
    off = -4 if unit.startswith("U1") else 4
    for est, (mk, dx) in EST.items():
        rr = sorted([r for r in D["nagao"] if r["unit"] == unit and r["estimator"] == est], key=lambda r: r["p"])
        xs = [r["p"] + dx + off for r in rr]
        ys = [r["ratio"] for r in rr]
        yerr = [[r["ratio"] - r["ci95"][0] for r in rr], [r["ci95"][1] - r["ratio"] for r in rr]]
        primary = est.startswith("(b)")
        ax.errorbar(xs, ys, yerr=yerr, fmt=mk, ms=4.5 if primary else 3, capsize=2,
                    color=col, mfc=col if primary else "none", lw=1.1 if primary else 0.7,
                    alpha=1 if primary else 0.7,
                    label=f"Nagao arm, {LBL[unit]}, estimator {est[:3]}")
s = D["sieve_p1511"]
for key, col, lab in (("seeds 17-24|pooled|S/rho U1", C["U1 group additions"], "sieve route, U1 (validated row)"),
                      ("seeds 17-24|pooled|S/rho U2", C["U2 textbook F_p mults"], "sieve route, U2")):
    v = s[key]
    ax.errorbar([1511], [v["point"]], yerr=[[v["point"] - v["lo"]], [v["hi"] - v["point"]]], fmt="*",
                ms=8, color=col, capsize=2, label=lab)
ax.axhline(1, color="#222222", ls="--", lw=0.9)
ax.set_yscale("log")
ax.set_ylim(3e-3, 6)
ax.set_xlim(1260, 1650)
ax.set_xticks([1301, 1451, 1511, 1601])
ax.set_xlabel("$p$ (cover of $E(\\mathbb{F}_{p^6})$, genus-3 Jacobian over $\\mathbb{F}_{p^2}$; weak class)")
ax.set_ylabel("cover $S$ / matched negation-rho $S$")
ax.legend(fontsize=5.8, ncol=2, loc="center left", bbox_to_anchor=(0.0, 0.42), frameon=False)
fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(HERE, f"fig-pf-jv.{ext}"), metadata={"Date": None} if ext == "svg" else {"CreationDate": None})
