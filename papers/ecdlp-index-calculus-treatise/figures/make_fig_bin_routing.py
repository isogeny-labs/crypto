"""Koblitz routing retest: descent and pipeline against rho by group size.
Reads only data-bin-routing.json; writes fig-bin-routing.{pdf,svg}."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

H = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(H, "data-bin-routing.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25, "svg.fonttype": "none"})
S = [("descent_over_rho", "descent / rho", "#D55E00", "s", 0.08),
     ("pipeline_over_rho", "pipeline / rho", "#0072B2", "o", -0.08),
     ("descent_over_pipeline", "descent / pipeline", "#009E73", "^", 0.0)]
fig, ax = plt.subplots(figsize=(4.6, 3.0))
for key, lab, c, mk, dx in S:
    xs = [r["log2_r"] + dx for r in D["ratios"]]
    ys = [r[key][0] for r in D["ratios"]]
    lo = [r[key][0] - r[key][1] for r in D["ratios"]]
    hi = [r[key][2] - r[key][0] for r in D["ratios"]]
    ax.errorbar(xs, ys, yerr=[lo, hi], marker=mk, color=c, ms=4.5, capsize=2, lw=1, label=lab)
for r in D["ratios"]:
    ax.annotate(r["curve"], (r["log2_r"], r["descent_over_rho"][2]), textcoords="offset points",
                xytext=(0, 4), ha="center", fontsize=6.5)
ax.axhline(1, color="#444444", ls="--", lw=0.9, label="ratio 1")
ax.set_yscale("log")
ax.set_xlabel("$\\log_2 r$ (prime subgroup order, bits)")
ax.set_ylabel("ratio of counted $S$ at $w=0$")
ax.legend(fontsize=6.8, frameon=False, loc="center right")
fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(H, f"fig-bin-routing.{ext}"), bbox_inches="tight")
