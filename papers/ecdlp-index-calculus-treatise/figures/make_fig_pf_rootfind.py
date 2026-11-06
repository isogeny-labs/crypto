"""Root-finding oracle series (data-pf-rootfind.json only).
Left: median wall per target of one decomposition query on binary curves n=3l.
Right: ecbench S of the root-finding IC arm over the strong signed-Frobenius rho on Koblitz curves.

    python3 make_fig_pf_rootfind.py   # writes fig-pf-rootfind.{pdf,svg}
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data-pf-rootfind.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25,
                     "svg.fonttype": "none", "svg.hashsalt": "pf"})
C = {"baseline": "#999999", "group-scalar": "#009E73", "rf-pairs": "#E69F00", "rf-mitm": "#0072B2"}
MK = {"baseline": "v", "group-scalar": "s", "rf-pairs": "D", "rf-mitm": "o"}
fig, (a, b) = plt.subplots(1, 2, figsize=(6.6, 2.8), gridspec_kw={"width_ratios": [1.15, 1]})
for arm in C:
    rr = sorted([r for r in D["binary"] if r["arm"] == arm and r["n"] == 3 * r["l"]], key=lambda r: r["l"])
    a.plot([r["l"] for r in rr], [r["ns_median"] / 1e3 for r in rr], "-" + MK[arm], ms=3.5, lw=1,
           color=C[arm], label=arm)
a.set_yscale("log")
a.set_xlabel(r"$\ell$ (binary curve, $n=3\ell$)")
a.set_ylabel(r"median wall per target ($\mu$s)")
a.legend(fontsize=6.5, frameon=False, loc="upper left")
kr = sorted(D["koblitz_ratio"], key=lambda r: r["n"])
b.errorbar([r["n"] for r in kr], [r["rootfind_over_strong_rho_S"] for r in kr],
           yerr=[[r["rootfind_over_strong_rho_S"] - r["ci95"][0] for r in kr],
                 [r["ci95"][1] - r["rootfind_over_strong_rho_S"] for r in kr]],
           fmt="o-", ms=3.5, lw=1, capsize=2, color="#0072B2", label="IC + rootfind-mitm, 95% interval")
b.axhline(1, color="#222222", ls="--", lw=0.9)
b.set_yscale("log")
b.set_ylim(0.5, 1e5)
b.set_xticks([r["n"] for r in kr])
b.set_xlabel(r"Koblitz curve degree $n$")
b.set_ylabel(r"$S_{\mathrm{IC}}/S_{\mathrm{strong\ rho}}$")
b.legend(fontsize=6.5, frameon=False, loc="upper left")
fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(HERE, f"fig-pf-rootfind.{ext}"), metadata={"Date": None} if ext == "svg" else {"CreationDate": None})
