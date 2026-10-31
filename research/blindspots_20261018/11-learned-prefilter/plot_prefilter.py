"""Plots recorded values from raw/summary_rows.tsv only (extracted from raw/n*-m*-*.jsonl; no computation beyond one subtraction). Usage: plot_prefilter.py <tsv> <out-stem>"""
import csv, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
rows = list(csv.DictReader(open(sys.argv[1]), delimiter="\t"))
fig, axs = plt.subplots(1, 2, figsize=(11, 4.6))
ax = axs[0]
col = {"T": "#1b9e77", "G": "#d95f02", "P": "#7570b3"}
for r in rows:
    if r["pop"] != "U" or r["model"] != "gbt" or r["type"] not in col: continue
    ax.scatter(float(r["rate_b"]), float(r["rate_c"]), color=col[r["type"]], s=28, alpha=.8)
for t, c in col.items():
    ax.scatter([], [], color=c, label=f"base type {t}")
ax.plot([0, 1], [0, 1], "k--", lw=1, label="learned = trace filter")
ax.axhline(0.05, color="r", lw=.8, ls=":")
ax.set_xlabel("trace filter refutation rate rate_b (U, 8 cells)")
ax.set_ylabel("learned (gbt) credited refutation rate rate_c")
ax.set_title("Learned filter vs trace filter, population U, held-out bases", fontsize=10)
ax.legend(fontsize=8); ax.grid(alpha=.3)
ax = axs[1]
mk = {("U", "lr"): ("o", "#1b9e77"), ("U", "gbt"): ("s", "#d95f02"), ("S", "lr"): ("^", "#7570b3"), ("S", "gbt"): ("D", "#e7298a")}
for (pop, model), (m, c) in mk.items():
    xs = [float(r["null_auc_max"]) for r in rows if r["pop"] == pop and r["model"] == model and r["type"] in ("G", "P")]
    ys = [float(r["auc"]) for r in rows if r["pop"] == pop and r["model"] == model and r["type"] in ("G", "P")]
    ax.scatter(xs, ys, marker=m, color=c, s=26, alpha=.8, label=f"{pop}, {model}")
ax.plot([.48, .54], [.48, .54], "k--", lw=1, label="AUC = null band max")
ax.set_xlabel("max AUC over 20 within-base label permutations")
ax.set_ylabel("held-out AUC")
ax.set_title("Generic (G) and geometric (P) bases: AUC against null band", fontsize=10)
ax.legend(fontsize=8); ax.grid(alpha=.3)
fig.suptitle("Item 11: all 16 cell-populations measured; n = 17..29, m = 2,3; 20,000 targets per cell-population", fontsize=9)
fig.tight_layout()
fig.savefig(sys.argv[2] + ".svg"); fig.savefig(sys.argv[2] + ".png", dpi=150)
