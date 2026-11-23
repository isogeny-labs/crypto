# Lays out recorded values (raw/*.stdout.log) and the geometric means that
# `b05 summarize` prints; recomputes nothing else. Run from the repo root.
import json, glob, math
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
raw = "research/blindspots_20261018/05-elimination-orders/raw/"
recs = {}
for f in sorted(glob.glob(raw + "card-*.stdout.log")):
    for l in open(f):
        l = l.strip()
        if l.startswith("{") and '"arm"' in l:
            r = json.loads(l); recs.setdefault((r["cell"], r["arm"], r["target"]), r)
cells = ["c17-4-3", "c19-4-3", "c13-3-4", "c15-3-4"]
nvar = {"c17-4-3": 29, "c19-4-3": 31, "c13-3-4": 38, "c15-3-4": 42}
def ratios(c, a):
    return [recs[(c, "grevlex", t)]["touches"] / recs[(c, a, t)]["touches"]
            for t in range(8) if (c, a, t) in recs and (c, "grevlex", t) in recs]
gm = lambda x: math.exp(sum(map(math.log, x)) / len(x))
fig, ax = plt.subplots(1, 2, figsize=(11.5, 4.6))
xs = range(len(cells))
band_lo = [min(gm(ratios(c, f"rand{d}")) for d in range(8)) for c in cells]
band_hi = [max(gm(ratios(c, f"rand{d}")) for d in range(8)) for c in cells]
ax[0].fill_between(xs, band_lo, band_hi, color="#bbbbbb", alpha=.6, label="random-ranking band (8 draws)")
for a, col, mk in [("wdeg", "#d62728", "o"), ("blkIL", "#1f77b4", "s"), ("mindeg", "#2ca02c", "^")]:
    ax[0].plot(xs, [gm(ratios(c, a)) for c in cells], marker=mk, color=col, label=a + (" (= tw)" if a == "mindeg" else ""))
ax[0].axhline(1, c="k", lw=.8); ax[0].axhline(1.5, c="g", ls="--", lw=.8)
ax[0].set_yscale("log"); ax[0].set_xticks(list(xs))
ax[0].set_xticklabels([f"{c}\nN={nvar[c]}" for c in cells], fontsize=8)
ax[0].set_ylabel("geometric-mean touches ratio, grevlex / arm\n(>1: arm cheaper; dashed = 1.5 rule)", fontsize=8)
ax[0].set_title("A. paired ratio per countable cell (8 targets)", fontsize=10); ax[0].legend(fontsize=7)
for i, c in enumerate(cells):
    r = ratios(c, "wdeg")
    ax[1].scatter([i] * len(r), r, c="#d62728", s=18)
    ax[1].plot([i - .25, i + .25], [gm(r)] * 2, c="k", lw=1.2)
wall = [recs[("c17-4-4", "grevlex", t)]["ms"] / recs[("c17-4-4", "wdeg", t)]["ms"] for t in (0, 1)]
ax[1].scatter([4, 4], wall, marker="v", c="#ff9896", s=30, label="c17-4-4 N=50: wdeg hit the 370 s cap;\nwall ratio grevlex/wdeg (upper bound)")
ax[1].axhline(1, c="k", lw=.8); ax[1].axhline(1.5, c="g", ls="--", lw=.8)
ax[1].set_yscale("log"); ax[1].set_xticks(range(5))
ax[1].set_xticklabels([f"{c}\nN={nvar[c]}" for c in cells] + ["c17-4-4\nN=50"], fontsize=8)
ax[1].set_ylabel("grevlex / wdeg, touches (bar = geometric mean)", fontsize=8)
ax[1].set_title("B. wdeg per target: gain shrinks with N, reverses at N=50", fontsize=10); ax[1].legend(fontsize=7, loc="upper right")
fig.suptitle("BLINDSPOT-05 elimination orders: wdeg ADVANCES (constant) on the 4 countable cells (N=29-42); "
             "no order lowers D_mat. Stage diagnostic, toy scale", fontsize=8.5)
fig.tight_layout()
out = "research/blindspots_20261018/05-elimination-orders/figures/order_ratios"
fig.savefig(out + ".svg"); fig.savefig(out + ".png", dpi=150)
