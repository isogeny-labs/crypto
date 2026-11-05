"""Layout only: plot recorded values of XP-20261029-packed-filter (no computation beyond reading).

Panel a: the frozen wall statistic, corrected dgbf/dgb whole-solve wall per charged GAE,
read from each size's native `ecbench compare` output (wall/analysis/compare-dgb__dgbf-w0.json).
Panel b: the counted distance to matched rho, mean S' ratio against rho-neg at w = 0, 0.1, 1,
read from the validator's validation/code/out/j4.json (P24, P32) and validator-report.json (P28).
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

S = Path(__file__).resolve().parent.parent
OUT = Path(__file__).resolve().parent
SIZES = ("P24", "P28", "P32")
BITS = {"P24": 24, "P28": 28, "P32": 32}

wall = {s: json.loads((S / s / "wall/analysis/compare-dgb__dgbf-w0.json").read_text())["wall"] for s in SIZES}
j4 = json.loads((S / "validation/code/out/j4.json").read_text())
vr = json.loads((S / "validation/validator-report.json").read_text())
dist = {}
for s in ("P24", "P32"):
    dist[s] = {arm: [j4[f"{s}-counted"][f"{arm}/rho-neg"][w]["ratio_b_over_a"] for w in ("0.0", "0.1", "1.0")]
               for arm in ("dgb", "dgbf")}
p28 = vr["p28_counted_session_of_record"]["sprime"]
dist["P28"] = {"dgb": p28["dgb/rho-neg"], "dgbf": p28["dgbf/rho-neg"]}
json.dump({"wall_corrected": {s: {"ratio": wall[s]["corrected_ratio"], "ci95": wall[s]["corrected_ci95"]} for s in SIZES},
           "distance_to_rho_neg": dist}, open(OUT / "summary.data.json", "w"), indent=1)

fig, (a, b) = plt.subplots(1, 2, figsize=(11, 4.4))
for s in SIZES:
    r, (lo, hi) = wall[s]["corrected_ratio"], wall[s]["corrected_ci95"]
    desc = s == "P28"
    a.errorbar([BITS[s]], [r], yerr=[[r - lo], [hi - r]], fmt="o", ms=7, capsize=5,
               color="#888888" if desc else ("#2b6cb0" if hi < 1 else "#c05621"),
               mfc="white" if desc else None)
    label = {"P24": "wall regression\n1.0032", "P28": "undecidable\n(infrastructure;\ndescriptive 0.9441)", "P32": "wall improvement\n0.9601"}[s]
    a.annotate(label, (BITS[s], r), textcoords="offset points", xytext=(12, -4), fontsize=8)
a.axhline(1.0, color="black", lw=0.8, ls="--")
a.set_xticks([24, 28, 32]); a.set_xlim(22, 35)
a.set_xlabel("field size b (bits)")
a.set_ylabel("corrected wall per charged GAE, dgbf / dgb")
a.set_title("a. wall statistic (95% CI, 288 pairs per size)", fontsize=10)

w = [0, 0.1, 1]
x0 = {"P24": 0, "P28": 4, "P32": 8}
for s in SIZES:
    xs = [x0[s] + i for i in range(3)]
    b.bar([x - 0.2 for x in xs], dist[s]["dgb"], 0.4, color="#a0aec0", label="dgb / rho-neg" if s == "P24" else None,
          hatch="//" if s == "P28" else None, edgecolor="#4a5568")
    b.bar([x + 0.2 for x in xs], dist[s]["dgbf"], 0.4, color="#2b6cb0", label="dgbf / rho-neg" if s == "P24" else None,
          hatch="//" if s == "P28" else None, edgecolor="#1a365d")
b.set_xticks([x0[s] + i for s in SIZES for i in range(3)])
b.set_xticklabels([f"{s}\nw={v}" if i == 1 else f"w={v}" for s in SIZES for i, v in enumerate(w)], fontsize=7)
b.set_ylabel("counted S′ ratio to matched rho (distance remaining)")
b.set_title("b. distance to rho-neg, counted S′ at w = 0 / 0.1 / 1", fontsize=10)
b.legend(fontsize=8, loc="upper left")
fig.text(0.01, 0.005, "XP-20261029-packed-filter, c8a.2xlarge, single session per size. P28 (hollow / hatched): wall undecidable "
         "(infrastructure), counted from session of record -r2, descriptive. Counted figures decide nothing; "
         "table_insert_attempts unpriced on every IC arm.", fontsize=6.5)
fig.tight_layout(rect=(0, 0.04, 1, 1))
fig.savefig(OUT / "summary.svg")
fig.savefig(OUT / "summary.png", dpi=150)
