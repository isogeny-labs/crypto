"""Residual collectors vs matched negation rho (data-pf-residual.json only).

    python3 make_fig_pf_residual.py   # writes fig-pf-residual.{pdf,svg}
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data-pf-residual.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25,
                     "svg.fonttype": "none", "svg.hashsalt": "pf"})
C = {"p36": "#0072B2", "p37": "#D55E00"}
arms = ["res4", "res2", "res1", "res4dp", "res2dp", "res1dp", "res4eq", "res2eq", "res1eq"]
fig, ax = plt.subplots(figsize=(6.6, 3.0))
for cur, dx in (("p36", -0.17), ("p37", 0.17)):
    rows = {r["arm"]: r for r in D["rows"] if r["curve"] == cur}
    xs = [i + dx for i in range(len(arms))]
    y0 = [rows[a]["ratio_w0"] for a in arms]
    lo = [rows[a]["ratio_w0"] - rows[a]["ci95_t"][0] for a in arms]
    hi = [rows[a]["ci95_t"][1] - rows[a]["ratio_w0"] for a in arms]
    ax.errorbar(xs, y0, yerr=[lo, hi], fmt="o", ms=4, color=C[cur], capsize=2, lw=1,
                label=f"{cur}: $w=0$, 95% interval", zorder=3)
    ax.plot(xs, [rows[a]["bytes_excluded"][2] for a in arms], "^", ms=4.5, mfc="none",
            color=C[cur], label=f"{cur}: $w=1$, byte counter excluded", zorder=3)
    ax.plot(xs, [rows[a]["band_as_built"][2] for a in arms], "x", ms=4, color="#777777",
            label="$w=1$ as built (bytes priced)" if cur == "p36" else None, zorder=2)
ax.axhline(1, color="#222222", ls="--", lw=0.9)
ax.set_yscale("log")
ax.set_ylim(0.8, 70)
ax.set_xticks(range(len(arms)))
ax.set_xticklabels(arms, family="monospace", fontsize=7.5)
ax.set_xlabel("residual-collector arm (base size 4, 2, 1; plain, distinguished points, equivariant)")
ax.set_ylabel(r"$S_{\mathrm{arm}}/S_{\mathrm{rho}}$ (matched rho.negation)")
ax.legend(fontsize=6.5, ncol=2, loc="upper right", frameon=False)
fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(HERE, f"fig-pf-residual.{ext}"), metadata={"Date": None} if ext == "svg" else {"CreationDate": None})
