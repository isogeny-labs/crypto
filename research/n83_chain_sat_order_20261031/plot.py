"""Lay out recorded values from ANALYSIS.json (native n83_chain_sat_analyze).
Plotting only. usage: plot.py <ANALYSIS.json> <out_dir>"""
import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

a = json.loads(Path(sys.argv[1]).read_text())
out = Path(sys.argv[2])
out.mkdir(parents=True, exist_ok=True)
ARMS = ["unordered", "all-summands", "all-summands+t2.2", "all-summands+affine+t2.2"]
LABEL = {
    "unordered": "A0 unordered (shipped)",
    "all-summands": "A1 all-summands order",
    "all-summands+t2.2": "A2 order + root-finder tables",
    "all-summands+affine+t2.2": "A3 A2, affine pattern only",
}
MS = list(range(2, 10))
cell = {(c["m"], c["arm"], c["kind"]): c for c in a["cells"]}


def series(kind, field):
    return {arm: [cell.get((m, arm, kind), {}).get(field) for m in MS] for arm in ARMS}


fig, axes = plt.subplots(1, 3, figsize=(15, 4.6))
w = 0.2
for k, (ax, kind, title) in enumerate([
    (axes[0], "public", "public [a]G: queries decided (of 3)"),
    (axes[1], "planted", "planted: verified witnesses (of 2)"),
]):
    field = "refuted" if kind == "public" else "verified"
    s = series(kind, field)
    for i, arm in enumerate(ARMS):
        ys = [v or 0 for v in s[arm]]
        ax.bar([x + (i - 1.5) * w for x in MS], ys, w, label=LABEL[arm])
    ax.set_xticks(MS)
    ax.set_xlabel("summands m")
    ax.set_title(title, fontsize=9)
    ax.grid(alpha=0.3, axis="y")
axes[0].legend(fontsize=7)
ax = axes[2]
s = series("public", "median_seconds_all")
for arm in ARMS:
    ax.plot(MS, [v if v is not None else float("nan") for v in s[arm]], marker="o", label=LABEL[arm])
ax.axhline(120, color="k", ls=":", lw=1)
ax.text(2, 125, "120 s cap", fontsize=7)
ax.set_yscale("log")
ax.set_xticks(MS)
ax.set_xlabel("summands m")
ax.set_ylabel("median seconds per public query (all, capped)")
ax.set_title("public query wall time (L0, includes tables)", fontsize=9)
ax.grid(alpha=0.3)
fig.suptitle("Chained-S3 SAT on N83 (icv1-f2m83-t6151469093347-cdcc5432), 167-point union base — run n83_chain_sat_order_20261031", fontsize=10)
fig.tight_layout()
fig.savefig(out / "n83_chain_sat_m2_9.svg")
fig.savefig(out / "n83_chain_sat_m2_9.png", dpi=150)

# Conflicts on decided queries (counted unit).
fig, ax = plt.subplots(figsize=(7.5, 4.6))
for kind, ls in (("public", "-"), ("planted", "--")):
    s = series(kind, "median_conflicts_decided")
    for arm in ARMS:
        ys = [v if v is not None else float("nan") for v in s[arm]]
        ax.plot(MS, [max(y, 0.5) if y == y else y for y in ys], ls=ls, marker="o", label=f"{LABEL[arm]} · {kind}")
ax.set_yscale("log")
ax.set_xticks(MS)
ax.set_xlabel("summands m")
ax.set_ylabel("median solver conflicts, decided queries (0 drawn at 0.5)")
ax.set_title("Conflicts per decided query — run n83_chain_sat_order_20261031", fontsize=9)
ax.legend(fontsize=6)
ax.grid(alpha=0.3)
fig.tight_layout()
fig.savefig(out / "n83_chain_sat_conflicts.svg")
fig.savefig(out / "n83_chain_sat_conflicts.png", dpi=150)
print("ok")
