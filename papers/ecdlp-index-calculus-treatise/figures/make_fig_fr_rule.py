"""Koblitz rule (leaderboard Table A's comparison) at baselines v3, v4, v6, v7, v8 and the
v8 same-binary control v8c: cold IC over cold rho per cell with its 95% interval (a), and the
gap with rho priced at the canonical batched step (b). Reads data-fr-rule.json only.

    python3 make_fig_fr_rule.py   # writes fig-fr-rule.{pdf,svg}
"""
import json
import os

from make_fig_fr_common import C, plt, save

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data-fr-rule.json")))
vers = ["v3", "v4", "v6", "v7", "v8", "v8c"]
col = dict(zip(vers, [C["grey"], C["sky"], C["green"], C["blue"], C["red"], C["orange"]]))
mk = dict(zip(vers, ["o", "v", "s", "^", "D", "P"]))
fig, (a, b) = plt.subplots(1, 2, figsize=(7.2, 3.0), sharey=True)
for i, v in enumerate(vers):
    g = sorted([c for c in D["cells"] if c["version"] == v], key=lambda c: c["log2_r"])
    dx = (i - 2.5) * 0.17
    for ax, key, cik in ((a, "cold_ratio", "cold_ratio_ci95"), (b, "canonical_gap", "canonical_gap_ci95")):
        for c in g:
            if c[cik]:
                ax.plot([c["log2_r"] + dx] * 2, c[cik], color=col[v], lw=0.9)
        ax.plot([c["log2_r"] + dx for c in g], [c[key] for c in g], mk[v], color=col[v], ms=3.8,
                ls="none", label=v)
for ax, lab in ((a, "(a) rho as built"), (b, "(b) rho at the canonical step")):
    ax.axhline(1, color=C["black"], ls="--", lw=0.9)
    ax.set_yscale("log")
    ax.set_ylim(0.8, 50)
    ax.set_xlabel(r"$\log_2 r$  (Koblitz, $n=47,57,41,53,59,61$)")
    ax.text(0.03, 0.95, lab, transform=ax.transAxes, va="top")
a.set_ylabel("cold IC / cold rho, same public point")
a.legend(fontsize=6.5, loc="upper left", bbox_to_anchor=(0.0, 0.88), ncol=3)
save(fig, HERE, "fr-rule")
