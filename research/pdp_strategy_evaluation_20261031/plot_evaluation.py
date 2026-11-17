#!/usr/bin/env python3
"""Render the two figures of the PDP strategy evaluation (layout only; every number is read
from a committed file: phase_shares.json and atlas_match_log.tsv, plus the verdict tally
typed in VERDICTS below from cards/e1.md .. cards/e5.md).

    python3 plot_evaluation.py     # writes figures/*.svg and figures/*.png
"""
import json
from collections import Counter
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
FIG = HERE / "figures"
FIG.mkdir(exist_ok=True)

# Okabe-Ito colour-blind-safe palette
C = {"walk": "#0072B2", "factor": "#E69F00", "matrix": "#D55E00", "verify": "#999999",
     "closed": "#56B4E9", "open": "#009E73", "unexamined": "#CC79A7", "kr": "#BBBBBB",
     "promote": "#009E73", "revise": "#E69F00", "close": "#56B4E9"}

# ---- Figure 1: where S goes on the arm-B ladder rows (measured) ----
ps = json.loads((HERE / "phase_shares.json").read_text())
rows = ps["rows"]
fig, ax = plt.subplots(figsize=(8.6, 3.4))
labels = [f"{r['row']}  (h={r['cofactor']}, S={r['S_mean']:.2f})" for r in rows]
left = [0.0] * len(rows)
for key, name, col in (("relations", "walk", C["walk"]), ("factor_base", "factor base", C["factor"]),
                       ("verify", "verify", C["verify"]), ("linear_algebra", "matrix", C["matrix"])):
    vals = [r["share_percent"][key] for r in rows]
    ax.barh(labels, vals, left=left, color=col, label=name, edgecolor="white", linewidth=0.6)
    for i, v in enumerate(vals):
        if v >= 8:
            ax.text(left[i] + v / 2, i, f"{v:.0f}%", ha="center", va="center", color="white", fontsize=9)
    left = [a + b for a, b in zip(left, vals)]
for i, r in enumerate(rows):
    ax.text(101, i, f"matrix {r['share_percent']['linear_algebra']:.3f}%", va="center", fontsize=8,
            color=C["matrix"])
ax.set_xlim(0, 118)
ax.set_xlabel("share of total group-addition equivalents (mean of 20 repeats)")
ax.set_title("Arm B at the published rung: the walk dominates and the matrix is negligible\n"
             "(measured, research/lead19_projected_base/char2_repeats20.json)", fontsize=10)
ax.invert_yaxis()
ax.legend(ncol=4, loc="upper center", bbox_to_anchor=(0.45, -0.28), frameon=False, fontsize=9)
for s in ("top", "right"):
    ax.spines[s].set_visible(False)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(FIG / f"s_phase_shares.{ext}", dpi=160, bbox_inches="tight")
plt.close(fig)

# ---- Figure 2: novelty audit and verdicts ----
import csv
with open(HERE / "candidate_judgements.tsv") as f:
    judged = list(csv.DictReader(f, delimiter="\t"))
cnt = Counter(r["my_reading"] for r in judged)
order = ["collides", "partial", "none"]
names = ["collides with an existing lead\nor known-result row", "partial overlap\n(neighbour covers part)", "no real match"]
colors = [C["closed"], C["revise"], C["unexamined"]]

VERDICTS = [("PROMOTE", 1, C["promote"]), ("REVISE", 6, C["revise"]), ("CLOSE", 7, C["close"])]

fig, (a, b) = plt.subplots(1, 2, figsize=(10.2, 3.6), gridspec_kw={"width_ratios": [1.25, 1]})
vals = [cnt.get(k, 0) for k in order]
a.barh(names, vals, color=colors)
for i, v in enumerate(vals):
    a.text(v + 0.2, i, str(v), va="center")
a.invert_yaxis()
a.set_xlim(0, max(vals) + 3)
a.set_xlabel(f"candidate strategies (n = {len(judged)})")
a.set_title("Novelty audit: do the candidates already exist in the atlas?\n(my reading of lead_atlas.py --match hits, candidate_judgements.tsv)", fontsize=10)
b.bar([v[0] for v in VERDICTS], [v[1] for v in VERDICTS], color=[v[2] for v in VERDICTS])
for i, v in enumerate(VERDICTS):
    b.text(i, v[1] + 0.15, str(v[1]), ha="center")
b.set_ylim(0, 9)
b.set_ylabel("lead cards")
b.set_title("Desk-evaluation verdicts (n = 14 cards)\nnone is a measured improvement", fontsize=10)
for ax_ in (a, b):
    for s in ("top", "right"):
        ax_.spines[s].set_visible(False)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(FIG / f"novelty_and_verdicts.{ext}", dpi=160)
plt.close(fig)
print("wrote", sorted(p.name for p in FIG.iterdir()))
print("judgements:", dict(cnt))
