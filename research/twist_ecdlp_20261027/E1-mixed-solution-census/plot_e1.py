# plot_e1.py -- plots recorded values only (summary.tsv, pooled.tsv); no computation of new results
import sys, csv
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

summ = sys.argv[1] if len(sys.argv) > 1 else "summary.tsv"
rows = list(csv.DictReader(open(summ), delimiter="\t"))
def fl(x):
    try: return float(x)
    except: return None

# Fig 1: observed mixed (nd) vs structural predictor S_nd, per config with m=3
fig, ax = plt.subplots(figsize=(5.2, 4.6))
xs = [(fl(r["S_nd"]), fl(r["M_nd"])) for r in rows if r["m"] == "3"]
ax.scatter([a + 0.5 for a, b in xs], [b + 0.5 for a, b in xs], s=14, alpha=.7)
mx = max([max(a, b) for a, b in xs] + [1]) + 1
ax.plot([0.5, mx], [0.5, mx], "k--", lw=.8, label="N_M = S (offset 0.5)")
ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("structural predictor S_nd (+0.5)"); ax.set_ylabel("observed mixed non-degenerate N_M (+0.5)")
ax.set_title("m=3: mixed solutions vs structural predictor (one point per config)")
ax.legend(); fig.tight_layout(); fig.savefig("fig_mixed_vs_predictor.png", dpi=130); fig.savefig("fig_mixed_vs_predictor.svg")

# Fig 2: E-only obs / B_ref with bootstrap intervals, by factor base
fig, ax = plt.subplots(figsize=(7, 4.2))
fbs = ["interval", "random", "twistmult", "farey"]
for k, fb in enumerate(fbs):
    pts = [(fl(r["ratio_ref"]), fl(r["ratio_ref_lo"]), fl(r["ratio_ref_hi"])) for r in rows if r["fb"] == fb and fl(r["ratio_ref"]) is not None and fl(r["B_ref"]) > 5]
    xs_ = [k + 0.4 * (i / max(1, len(pts)) - .5) for i in range(len(pts))]
    ax.errorbar(xs_, [p[0] for p in pts], yerr=[[p[0] - p[1] for p in pts], [p[2] - p[0] for p in pts]], fmt="o", ms=2.5, lw=.6, alpha=.6)
ax.axhline(1, color="k", lw=.8); ax.axhspan(.7, 1.4, color="g", alpha=.08)
ax.set_xticks(range(4)); ax.set_xticklabels(fbs); ax.set_ylabel("mean E-only (nd) per target / B_ref")
ax.set_title("E-only baseline ratio (configs with B_ref>5), 95% bootstrap over targets")
fig.tight_layout(); fig.savefig("fig_eonly_ratio.png", dpi=130); fig.savefig("fig_eonly_ratio.svg")

# Fig 3: pooled mixed fraction per stratum with intervals, vs 5/q-scale
try:
    pr = list(csv.DictReader(open("pooled.tsv"), delimiter="\t"))
    fig, ax = plt.subplots(figsize=(7.5, 4.2))
    labs = [f'm{r["m"]} nT{r["nT"]} {r["fb"]}' for r in pr]
    f = [fl(r["f_mixed"]) for r in pr]; lo = [fl(r["f_lo"]) for r in pr]; hi = [fl(r["f_hi"]) for r in pr]
    ax.errorbar(range(len(pr)), f, yerr=[[a - b for a, b in zip(f, lo)], [c - a for a, c in zip(f, hi)]], fmt="o", ms=3)
    ax.set_yscale("symlog", linthresh=1e-6); ax.set_xticks(range(len(pr))); ax.set_xticklabels(labs, rotation=90, fontsize=6)
    ax.set_ylabel("pooled mixed fraction N_M/(N_M+N_E) (nd)"); ax.set_title("Pooled mixed fraction by stratum (zero => CP upper bound)")
    fig.tight_layout(); fig.savefig("fig_pooled_fraction.png", dpi=130); fig.savefig("fig_pooled_fraction.svg")
except Exception as e:
    print("pooled plot skipped:", e)
print("plots written")
