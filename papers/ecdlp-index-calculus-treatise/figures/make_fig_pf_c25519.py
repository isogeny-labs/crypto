"""Curve25519-rule miniatures: walk-table incumbents over matched rho-neg per curve
(data-pf-c25519.json only; ratios S_inc/S_rho are computed here from the per-curve means).

    python3 make_fig_pf_c25519.py   # writes fig-pf-c25519.{pdf,svg}
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data-pf-c25519.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25,
                     "svg.fonttype": "none", "svg.hashsalt": "pf"})
C = ["#0072B2", "#E69F00", "#009E73"]
W = [0.0, 0.1, 1.0]
fig, ax = plt.subplots(figsize=(6.6, 2.9))
ks = sorted({r["k"] for r in D["rows"]})
for j, (wi, mk) in enumerate(zip(range(3), ["o", "s", "D"])):
    dx = (j - 1) * 0.45
    means = []
    for k in ks:
        rr = [r["S_inc"][wi] / r["S_rho"][wi] for r in D["rows"] if r["k"] == k]
        ax.plot([k + dx] * len(rr), rr, mk, ms=3.5, mfc="none", color=C[j], alpha=0.9,
                label=f"$w={W[wi]:g}$, one curve" if k == ks[0] else None)
        means.append(sum(rr) / len(rr))
    ax.plot([k + dx for k in ks], means, "-", color=C[j], lw=1.2,
            label=f"$w={W[wi]:g}$, mean of 5 curves")
ax.axhline(1, color="#222222", ls="--", lw=0.9)
ax.axhline(D["parity_floor"]["F=2"], color="#CC79A7", ls=":", lw=1.0)
ax.text(32.9, D["parity_floor"]["F=2"], "$F=2$ floor 1.875", va="center", fontsize=6.5, color="#CC79A7", bbox=dict(fc="white", ec="none", pad=0.5))
ax.axhline(D["parity_floor"]["F=1"], color="#999999", ls=":", lw=1.0)
ax.text(32.9, D["parity_floor"]["F=1"], "$F=1$ floor 1.5", va="center", fontsize=6.5, color="#777777", bbox=dict(fc="white", ec="none", pad=0.5))
ax.set_xticks(ks)
ax.set_xlim(18.8, 35.4)
ax.set_ylim(0.6, 2.4)
ax.set_xlabel("$k$ (miniature prime $p<2^k$, $p\\equiv5$ mod 8; five family curves per $k$)")
ax.set_ylabel(r"$S_{\mathrm{inc}}/S_{\mathrm{rho\text{-}neg}}$")
ax.legend(fontsize=6.3, ncol=3, loc="lower center", frameon=False)
fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(HERE, f"fig-pf-c25519.{ext}"), metadata={"Date": None} if ext == "svg" else {"CreationDate": None})
