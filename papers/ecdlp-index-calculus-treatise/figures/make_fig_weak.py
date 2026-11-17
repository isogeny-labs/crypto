"""Weak-family figure from data-new.json (recorded values only): genus-3 GHS IC over
the ideal strong rho per instance, and plane-quartic (N4 v2) fitted exponents.

    python3 make_fig_weak.py   # writes fig-weak.{pdf,svg,png}
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data-new.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25,
                     "svg.fonttype": "none"})
C = ["#0072B2", "#E69F00", "#009E73", "#CC79A7", "#999999", "#D55E00"]
fig, (a, b) = plt.subplots(1, 2, figsize=(7.2, 3.0), gridspec_kw={"width_ratios": [1.2, 1]})
rows = D["genus3_vs_rho"]["rows"]
a.scatter([r["log2q"] + 0.12 * (r["curve"] - 1) for r in rows], [r["ic_over_R_ideal"] for r in rows],
          color=C[0], s=18, label="IC / ideal strong rho ($c_\\ell=1$)", zorder=3)
a.scatter([r["log2q"] + 0.12 * (r["curve"] - 1) for r in rows], [r["ic_over_R_ideal_cl9"] for r in rows],
          color=C[1], marker="s", s=14, label="same, $c_\\ell=9$ sensitivity", zorder=3)
a.axhline(1, color="#222222", ls="--", lw=0.9)
a.set_yscale("log")
a.set_xlabel(r"$\log_2 q$  ($E_0(\mathbb{F}_{q^3})$, 3 curves per size)")
a.set_ylabel("IC / rho")
a.set_title("Genus-3 GHS (hyperelliptic) vs strongest rho", fontsize=9)
a.legend(fontsize=6.5, loc="upper left", frameon=False)
q = D["quartic_n4v2"]
y = range(len(q["arms"]))
for i, (e, lo, hi, name) in enumerate(zip(q["exp"], q["lo"], q["hi"], q["arms"])):
    col = C[5] if "rho" in name else C[2]
    b.plot([lo, hi], [i, i], color=col, lw=2)
    b.plot(e, i, "o", color=col)
b.set_yticks(list(y))
b.set_yticklabels(q["arms"])
b.axvline(1.0, color="#222222", ls=":", lw=0.9)
b.axvline(1.5, color="#222222", ls="--", lw=0.9)
b.set_xlabel("fitted exponent of cost in $q$ (95% CI)")
b.set_title("Plane quartic (Diem), N4 v2", fontsize=9)
fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(HERE, f"fig-weak.{ext}"), bbox_inches="tight")
fig.savefig(os.path.join(HERE, "fig-weak.png"), dpi=150, bbox_inches="tight")
