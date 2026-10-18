# Plots recorded values only (raw/*.stdout.log); computes no result.
import json, glob, math, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
raw = "research/blindspots_20261018/05-elimination-orders/raw/"
recs = []
for f in sorted(glob.glob(raw + "card-*.stdout.log")):
    for l in open(f):
        l = l.strip()
        if l.startswith("{") and '"arm"' in l:
            try: recs.append(json.loads(l))
            except Exception: pass
cells = ["c17-4-3", "c19-4-3", "c13-3-4", "c15-3-4", "c17-4-4", "c31-15-2"]
def g(c, a, t):
    for r in recs:
        if r["cell"] == c and r["arm"] == a and r["target"] == t: return r
fig, ax = plt.subplots(1, 2, figsize=(11, 4.4))
for i, c in enumerate(cells):
    ys = [r["touches"] for r in recs if r["cell"] == c and r["arm"] == "grevlex"]
    ax[0].scatter([i] * len(ys), ys, c="#1f77b4", s=22, label="grevlex (refuted, D_mat=4)" if i == 0 else None)
ax[0].set_yscale("log"); ax[0].set_xticks(range(6)); ax[0].set_xticklabels(cells, rotation=30, fontsize=8)
ax[0].set_ylabel("touches (matrix-entry toggles), per target"); ax[0].set_title("A. grevlex baseline per cell (measured)", fontsize=10); ax[0].legend(fontsize=8)
labels, vals, cols = [], [], []
def add(lbl, v, col): labels.append(lbl); vals.append(v); cols.append(col)
for c, t in [("c13-3-4", 0), ("c13-3-4", 1), ("c15-3-4", 0), ("c15-3-4", 1)]:
    a, b = g(c, "blkLI", t), g(c, "grevlex", t)
    add(f"blkLI {c} t{t}", b["touches"] / a["touches"], "#d62728")
for c in ("c17-4-3", "c19-4-3"):
    for t in (0, 1):
        a, b = g(c, "blkLI", t), g(c, "grevlex", t)
        add(f"blkLI {c} t{t}\n(capped, upper bound)", b["touches"] / a["touches"], "#ff9896")
for t in (0, 1):
    a, b = g("c17-4-4", "wdeg", t), g("c17-4-4", "grevlex", t)
    add(f"wdeg c17-4-4 t{t}\n(capped; wall ratio)", b["ms"] / a["ms"], "#ff9896")
for arm in ("mindeg", "tw", "rand0", "rand1"):
    a, b = g("c31-15-2", arm, 0), g("c31-15-2", "grevlex", 0)
    add(f"{arm} c31-15-2 t0", b["touches"] / a["touches"], "#2ca02c" if arm in ("mindeg", "tw") else "#7f7f7f")
y = range(len(vals))
ax[1].barh(list(y), vals, color=cols); ax[1].set_xscale("log"); ax[1].axvline(1, c="k", lw=0.8); ax[1].axvline(1.5, c="g", ls="--", lw=0.8)
ax[1].set_yticks(list(y)); ax[1].set_yticklabels(labels, fontsize=6); ax[1].invert_yaxis()
ax[1].set_xlabel("grevlex / arm (wall for capped wdeg); >1 = arm cheaper\ndashed = 1.5 criterion", fontsize=8)
ax[1].set_title("B. paired ratios actually measured (frozen cards)", fontsize=10)
fig.suptitle("BLINDSPOT-05 elimination orders: grevlex baseline and the few arms that ran. Verdict INCONCLUSIVE (0/6 countable cells)", fontsize=9)
fig.tight_layout()
out = "research/blindspots_20261018/05-elimination-orders/figures/order_ratios"
fig.savefig(out + ".svg"); fig.savefig(out + ".png", dpi=150)
for l, v in zip(labels, vals): print(l.replace("\n", " "), round(v, 4))
