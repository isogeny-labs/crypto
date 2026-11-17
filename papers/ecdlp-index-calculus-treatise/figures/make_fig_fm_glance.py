"""Programme-at-a-glance figure from data-fm-glance.json (counts copied or counted
from committed files; see the JSON's 'rule' fields).

    python3 make_fig_fm_glance.py   # writes fig-fm-glance.{pdf,svg}
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data-fm-glance.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25,
                     "svg.fonttype": "none"})
C = ["#0072B2", "#E69F00", "#009E73", "#CC79A7", "#D55E00", "#56B4E9"]
months = D["months"]
# The ledger index covers Jul-Sep only; its October rows are late-September files.
ledger_months = ["2026-07", "2026-08", "2026-09"]
series = [
    ("ledger decisions (Jul--Sep index)", {m: D["ledger"]["decisions"].get(m) for m in ledger_months}),
    ("ledger run directories (Jul--Sep index)", {m: D["ledger"]["run_dirs"].get(m) for m in ledger_months}),
    ("research/ study directories", D["studies"]["by_month"]),
    ("ecbench sessions", D["ecbench"]["sessions"]),
    ("admitted benchmark rows", D["benchmarks"]["admitted_rows"]),
    ("gate VALIDATION.json files", D["validation_files"]["by_month"]),
]
fig, ax = plt.subplots(figsize=(7.2, 3.0))
w = 0.13
for k, (name, vals) in enumerate(series):
    xs, ys = [], []
    for i, m in enumerate(months):
        v = vals.get(m)
        if v:
            xs.append(i + (k - (len(series) - 1) / 2) * w)
            ys.append(v)
    ax.bar(xs, ys, width=w, color=C[k], label=name, zorder=3)
ax.set_yscale("log")
ax.set_ylim(1, 3000)
ax.set_xticks(range(len(months)))
ax.set_xticklabels(["Jul", "Aug", "Sep", "Oct", "Nov (to 6th)"])
ax.set_xlabel("month of 2026")
ax.set_ylabel("count per month")
ax.legend(fontsize=6.5, ncol=2, frameon=False, loc="upper left")
fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(HERE, f"fig-fm-glance.{ext}"))
