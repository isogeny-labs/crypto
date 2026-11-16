"""The matched rho reference's evaluation ladder (research/ic_rho_reference_20260923):
S per sqrt(r) of the frozen plain walk, the tuned plain walk and the negation-map walk on 16
fresh curves (a), and the paired tuned-plain over negation walk ratio against sqrt 2 (b).
Reads data-fr-rholadder.json only.

    python3 make_fig_fr_rholadder.py   # writes fig-fr-rholadder.{pdf,svg}
"""
import json
import math
import os

from make_fig_fr_common import C, plt, save

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data-fr-rholadder.json")))
L = D["ladder"]
fig, (a, b) = plt.subplots(1, 2, figsize=(7.2, 3.0), gridspec_kw={"width_ratios": [1.3, 1]})
walks = [("frozen_s", "frozen plain ($A=1$, set-up as through §17)", C["grey"]),
         ("tuned_plain_s", "tuned plain ($A=1$)", C["orange"]),
         ("negation_s", "negation map ($A=2$), the matched reference", C["blue"])]
for key, lab, col in walks:
    for reg, m in (("prime", "o"), ("char2", "s")):
        g = sorted([r for r in L if r["regime"] == reg], key=lambda r: r["log2_r"])
        a.plot([r["log2_r"] for r in g], [r[key] for r in g], m + "-", color=col, ms=3.6, lw=0.8,
               mfc=col if reg == "prime" else "white", label=lab if reg == "prime" else None)
a.axhline(D["floors"]["plain"], color=C["orange"], ls=":", lw=0.9)
a.axhline(D["floors"]["negation"], color=C["blue"], ls=":", lw=0.9)
a.text(10.0, D["floors"]["plain"] * 0.97, r"$\sqrt{\pi/2}$", fontsize=7, va="top")
a.text(10.0, D["floors"]["negation"] * 0.97, r"$\sqrt{\pi/4}$", fontsize=7, va="top")
a.set_yscale("log")
a.set_xlabel(r"$\log_2 r$  (filled prime, open random binary)")
a.set_ylabel(r"$S$ (GAE per $\sqrt{r}$), mean of 128 runs")
a.legend(fontsize=6.3, loc="upper right")
a.text(0.02, 0.95, "(a)", transform=a.transAxes, va="top")
for reg, m, col in (("prime", "o", C["blue"]), ("char2", "s", C["green"])):
    g = sorted([r for r in L if r["regime"] == reg], key=lambda r: r["log2_r"])
    b.plot([r["log2_r"] for r in g], [r["plain_over_negation_walk"] for r in g], m, color=col, ms=4,
           label=reg.replace("char2", "random binary"))
b.axhline(math.sqrt(2), color=C["black"], ls="--", lw=0.9)
b.text(10.2, math.sqrt(2) * 1.01, r"$\sqrt{2}$", fontsize=7, va="bottom")
b.set_xlabel(r"$\log_2 r$")
b.set_ylabel("tuned-plain walk / negation walk")
b.legend(fontsize=6.5, loc="lower right")
b.text(0.02, 0.04, "(b)", transform=b.transAxes)
save(fig, HERE, "fr-rholadder")
