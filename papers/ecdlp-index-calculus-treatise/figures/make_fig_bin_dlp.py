"""Tuned double large prime vs ic-compact on held-out targets, across the GAE band.
Reads only data-bin-dlp.json; writes fig-bin-dlp.{pdf,svg}."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

H = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(H, "data-bin-dlp.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25, "svg.fonttype": "none"})
C = {37: "#0072B2", 41: "#E69F00", 43: "#009E73", 47: "#CC79A7"}
W = [0, 1, 2]
WL = ["0", "0.1", "1"]
fig, (a, b) = plt.subplots(1, 2, figsize=(7.2, 2.9))
for i, r in enumerate(D["dlp_over_base"]):
    n = r["n"]; dx = (i - 1.5) * 0.07
    v = r["dlp_over_base"]
    a.errorbar([w + dx for w in W], [x[0] for x in v], yerr=[[x[0] - x[1] for x in v], [x[2] - x[0] for x in v]],
               marker="o", color=C[n], ms=4, capsize=2, lw=1, label=f"$n={n}$ ({r['cell']})")
a.axhline(1, color="#444444", ls="--", lw=0.9)
a.set_xticks(W, WL); a.set_xlabel("weight $w$ (GAE per native operation)")
a.set_ylabel("double-LP $S$ / \\texttt{ic-compact} $S$" if False else "double-LP $S$ / ic-compact $S$")
a.legend(fontsize=6.3, frameon=False, loc="lower left")
for i, r in enumerate(D["distance_to_rhofrob"]):
    n = r["n"]; dx = (i - 1.5) * 0.07
    for key, ls, mk in (("base_over_rhofrob", "-", "o"), ("dlp_over_rhofrob", ":", "s")):
        v = r[key]
        b.errorbar([w + dx + (0.02 if mk == "s" else 0) for w in W], [x[0] for x in v],
                   yerr=[[x[0] - x[1] for x in v], [x[2] - x[0] for x in v]], marker=mk, ls=ls, color=C[n],
                   ms=3.8, capsize=2, lw=1, mfc=C[n] if mk == "o" else "white")
b.plot([], [], "o-", color="#333333", label="ic-compact baseline")
b.plot([], [], "s:", color="#333333", mfc="white", label="double LP (selected cell)")
b.axhline(1, color="#444444", ls="--", lw=0.9, label="ratio 1 (matched rho)")
b.set_yscale("log")
b.set_xticks(W, WL); b.set_xlabel("weight $w$ (GAE per native operation)")
b.set_ylabel("arm $S$ / rho.frob $S$")
b.legend(fontsize=6.3, frameon=False, loc="center right")
fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(H, f"fig-bin-dlp.{ext}"), bbox_inches="tight")
