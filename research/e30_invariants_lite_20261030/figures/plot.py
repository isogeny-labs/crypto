"""Plot E30's recorded values (experiments/35_invariants_lite/summary.json); computes nothing new."""
import json, sys, pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
here = pathlib.Path(__file__).parent
rows = json.load(open(sys.argv[1]))["rows"]
lab = [f"{r['p']}/{r['seed']}{r['arm']}" for r in rows]; x = range(len(rows))
fig, ax = plt.subplots(figsize=(8.5, 3.4))
for wi, (wt, col) in enumerate((("0 GAE", "#d9772b"), ("0.1 GAE", "#4a7fb5"), ("1 GAE", "#8c8c8c"))):
    ax.plot(x, [100 * r["distance_change"][wi] for r in rows], "o", color=col, label=wt, ms=4)
ax.axhline(0, color="k", lw=0.6)
ax.set_xticks(list(x)); ax.set_xticklabels(lab, rotation=70, fontsize=7)
ax.set_ylabel("change of S / rho (%)"); ax.set_xlabel("row (p / seed, arm)")
ax.set_title("E30: the trivial-product audit moves the distance to rho by at most 0.31 % (accounting)", fontsize=10)
ax.legend(fontsize=8, frameon=False); fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})): fig.savefig(here / f"distance_change.{ext}", **kw)
