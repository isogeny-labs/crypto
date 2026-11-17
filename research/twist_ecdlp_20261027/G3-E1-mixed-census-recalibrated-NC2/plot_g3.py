# plot_g3.py -- plots recorded values only (summary.tsv, nc2_cells.tsv, f4_table.tsv); no computation of new results
import sys, csv
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

summ = sys.argv[1] if len(sys.argv) > 1 else "summary.tsv"
def fl(x):
    try: return float(x)
    except: return None
rows = list(csv.DictReader(open(summ), delimiter="\t"))

# Fig 1: observed mixed (nd) vs structural predictor S_nd, per m=3 config
fig, ax = plt.subplots(figsize=(5.2, 4.6))
xs = [(fl(r["S_nd"]), fl(r["M_nd"])) for r in rows if r["m"] == "3"]
ax.scatter([a + 0.5 for a, b in xs], [b + 0.5 for a, b in xs], s=10, alpha=.6)
mx = max([max(a, b) for a, b in xs] + [1]) + 1
ax.plot([0.5, mx], [0.5, mx], "k--", lw=.8, label="N_M = S (offset 0.5)")
ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("structural predictor S_nd (+0.5)"); ax.set_ylabel("observed mixed non-degenerate N_M (+0.5)")
ax.set_title("m=3: mixed vs structural predictor (one point per config)")
ax.legend(); fig.tight_layout(); fig.savefig("fig_mixed_vs_predictor.png", dpi=130); fig.savefig("fig_mixed_vs_predictor.svg")

# Fig 2: NC2-calibrated D per cell with 95% hierarchical intervals and the +-0.05 margin
try:
    nc = list(csv.DictReader(open("nc2_cells.tsv"), delimiter="\t"))
    fig, ax = plt.subplots(figsize=(8, 4.2))
    labs = [f'{r["bits"]}/{r["fb"][:4]}/m{r["m"]}' for r in nc]
    D = [fl(r["D"]) for r in nc]; lo = [fl(r["D_lo"]) for r in nc]; hi = [fl(r["D_hi"]) for r in nc]
    col = ["tab:green" if r["pass"] == "1" else "tab:red" for r in nc]
    for i in range(len(nc)):
        ax.errorbar(i, D[i], yerr=[[D[i] - lo[i]], [hi[i] - D[i]]], fmt="o", ms=3, color=col[i], lw=1)
    ax.axhspan(-0.05, 0.05, color="g", alpha=.08); ax.axhline(0, color="k", lw=.6)
    ax.set_xticks(range(len(nc))); ax.set_xticklabels(labs, rotation=90, fontsize=6)
    ax.set_ylabel("D = observed minus conditional-null mixed share"); ax.set_title("NC2-calibrated (G3): D per cell, 95% hierarchical bootstrap; band = +-0.05")
    fig.tight_layout(); fig.savefig("fig_nc2_D.png", dpi=130); fig.savefig("fig_nc2_D.svg")
except Exception as e:
    print("nc2 plot skipped:", e)

# Fig 3: F4 table ratio c_hat/c_ref with intervals (resolved strata only)
try:
    f4 = [r for r in csv.DictReader(open("f4_table.tsv"), delimiter="\t") if r["resolved"] == "1" and fl(r["boot_valid"]) and fl(r["boot_valid"]) >= 10]
    fig, ax = plt.subplots(figsize=(8, 4.6))
    for i, r in enumerate(f4):
        v, lo, hi = fl(r["ratio_cref"]), fl(r["lo"]), fl(r["hi"])
        ax.errorbar(i, v, yerr=[[max(0, v - lo)], [max(0, hi - v)]], fmt="o", ms=3, lw=1)
    ax.axhspan(0.8, 1.25, color="g", alpha=.1); ax.axhline(1, color="k", lw=.6); ax.axhline(2, color="r", lw=.5, ls="--"); ax.axhline(0.5, color="r", lw=.5, ls="--")
    ax.set_xticks(range(len(f4))); ax.set_xticklabels([r["stratum"] for r in f4], rotation=90, fontsize=6)
    ax.set_ylabel("pooled c_hat / c_ref"); ax.set_title("F4: pooled mixed rate vs refined baseline (resolved strata), 95% hierarchical bootstrap")
    fig.tight_layout(); fig.savefig("fig_f4_ratio.png", dpi=130); fig.savefig("fig_f4_ratio.svg")
except Exception as e:
    print("f4 plot skipped:", e)
print("plots written")
