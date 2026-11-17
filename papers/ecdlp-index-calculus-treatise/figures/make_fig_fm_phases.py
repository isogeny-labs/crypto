"""Phase shares of cold S for every leaderboard row (best recipe), from
data-fm-phases.json. Shares phases_s[p]/s are computed here.

    python3 make_fig_fm_phases.py   # writes fig-fm-phases.{pdf,svg}
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data-fm-phases.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "svg.fonttype": "none"})
# Merge phases into five groups so that the three families share one legend.
GROUPS = [("set-up, base, table", ["precomputation", "setup", "constructions", "factor_base", "table"], "#0072B2"),
          ("relations", ["relations"], "#E69F00"),
          ("linear algebra", ["linear_algebra"], "#009E73"),
          ("descent", ["descent"], "#CC79A7"),
          ("verification", ["verify"], "#999999")]
FAM = {"B": "B: matched-rho ladder (prime, binary)", "A": "A: Koblitz, one target", "C": "C: Koblitz, collection"}
rows = D["rows"]
order = [r for f in ("B", "A", "C") for r in sorted((x for x in rows if x["family"] == f), key=lambda x: (x["regime"], x["log2_r"]))]
fig, ax = plt.subplots(figsize=(7.2, 4.6))
labels = []
for i, r in enumerate(order):
    left = 0.0
    for name, keys, col in GROUPS:
        v = sum(r["phases_s"].get(k, 0.0) for k in keys) / r["s"]
        ax.barh(i, v, left=left, color=col, height=0.75, label=name if i == 0 else None)
        left += v
    tag = "" if r["standing"] == "validated" else " (pre-gate)"
    tok = r["slug"].split("-")[1]
    labels.append(f"{r['family']}  {tok}, {r['log2_r']:.1f}{tag}")
ax.set_yticks(range(len(order)))
ax.set_yticklabels(labels, fontsize=6.3)
ax.invert_yaxis()
ax.set_xlim(0, 1)
ax.set_xlabel(r"share of cold $S$ (row: family, field, $\log_2 r$; fp$b$ = $b$-bit prime, f2m$m$ = $\mathbb{F}_{2^m}$)")
for f in ("B", "A"):
    last = max(i for i, r in enumerate(order) if r["family"] == f)
    ax.axhline(last + 0.5, color="#222222", lw=0.6)
# A-family rows have no relation phase of their own: their reusable set-up holds it.
handles = [plt.Rectangle((0, 0), 1, 1, color=c) for _, _, c in GROUPS]
ax.legend(handles, [g[0] for g in GROUPS], fontsize=6.5, ncol=5, frameon=False,
          loc="upper center", bbox_to_anchor=(0.5, 1.07))
fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(HERE, f"fig-fm-phases.{ext}"))
