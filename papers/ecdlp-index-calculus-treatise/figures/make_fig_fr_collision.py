"""Rho's collision count against the Gamma floor. (a) Empirical distribution of per-run cost
S (GAE per sqrt r) of the negation-map and plain rho walks in the negation audit (256 runs
each, 4 curves of 28-40 bits) against the first-collision law of the Poisson model,
P(S <= s) = 1 - exp(-s^2) for negation classes and 1 - exp(-s^2/2) for plain points.
(b) Monte Carlo mean of the k-th collision against sqrt(r) Gamma(k+1/2)/Gamma(k).
(c) Mean S of each arm per curve across the pricing band w = 0, 0.1, 1.
Reads data-fr-collision.json only; the model curves, means and coefficients of variation are
computed here and printed.

    python3 make_fig_fr_collision.py   # writes fig-fr-collision.{pdf,svg}
"""
import json
import math
import os
import statistics as st

from make_fig_fr_common import C, plt, save

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data-fr-collision.json")))
neg = sorted(w["rho_neg_s"] for w in D["workloads"])
pla = sorted(w["rho_plain_s"] for w in D["workloads"])
fig, (a, b, c) = plt.subplots(1, 3, figsize=(7.2, 2.7), gridspec_kw={"width_ratios": [1.3, 1, 1]})
for xs, col, lab in ((neg, C["blue"], "negation rho, measured"), (pla, C["orange"], "plain rho, measured")):
    n = len(xs)
    a.step(xs, [(i + 1) / n for i in range(n)], where="post", color=col, lw=1.1, label=lab)
    print(lab, "n", n, "mean %.4f" % st.mean(xs), "cv %.4f" % (st.stdev(xs) / st.mean(xs)))
s = [i / 100 for i in range(0, 451)]
a.plot(s, [1 - math.exp(-x * x) for x in s], ls="--", color=C["blue"], lw=0.9, label="model, $r/2$ classes")
a.plot(s, [1 - math.exp(-x * x / 2) for x in s], ls="--", color=C["orange"], lw=0.9, label="model, $r$ points")
print("model cv", math.sqrt(4 / math.pi - 1))
a.set_xlabel(r"$S$ per run (GAE per $\sqrt{r}$)")
a.set_ylabel("fraction of runs with cost $\\leq S$")
a.legend(fontsize=6, loc="lower right")
a.text(0.03, 0.95, "(a)", transform=a.transAxes, va="top")
sim = D["sim"]["k"]
ks = sorted(sim, key=int)
b.errorbar([int(k) for k in ks], [sim[k]["mean_over_sqrt_r"] for k in ks],
           yerr=[1.96 * sim[k]["sem"] / math.sqrt(D["sim"]["r"]) for k in ks], fmt="o", color=C["green"],
           ms=4, capsize=2, label="Monte Carlo, 4,000 trials")
kk = [1 + i / 20 for i in range(61)]
b.plot(kk, [math.exp(math.lgamma(k + 0.5) - math.lgamma(k)) for k in kk], color=C["black"], lw=0.9, ls="--",
       label=r"$\Gamma(k+\frac{1}{2})/\Gamma(k)$")
b.set_xlabel("collisions needed $k=F+1$")
b.set_ylabel(r"mean walk length / $\sqrt{r}$")
b.set_xticks([1, 2, 3, 4])
b.legend(fontsize=6, loc="lower right")
b.text(0.03, 0.95, "(b)", transform=b.transAxes, va="top")
rows = D["band"]["rows"]
curves = sorted({r["curve"] for r in rows}, key=lambda cv: [r["log2_r"] for r in rows if r["curve"] == cv][0])
for arm, col, m in (("rho-neg", C["blue"], "o"), ("rho-plain", C["orange"], "s")):
    for i, w in enumerate(D["band"]["weights"]):
        g = [next(r for r in rows if r["curve"] == cv and r["arm"] == arm) for cv in curves]
        c.plot([r["log2_r"] + (i - 1) * 0.35 for r in g], [r["mean_s"][i] for r in g], m, color=col,
               alpha=[1, 0.7, 0.45][i], ms=3.8, ls="none",
               label=(f"{arm}, $w={w:g}$" if arm == "rho-neg" else ("rho-plain, every $w$" if i == 0 else None)))
c.set_xlabel(r"$\log_2 r$")
c.set_ylabel(r"mean $S$ at weight $w$")
c.legend(fontsize=5.8, loc="center left", bbox_to_anchor=(0.02, 0.64))
c.text(0.03, 0.95, "(c)", transform=c.transAxes, va="top")
save(fig, HERE, "fr-collision")
