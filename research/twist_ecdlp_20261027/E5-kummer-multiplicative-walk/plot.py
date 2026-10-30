# plot.py -- plots recorded values only (out/costs.csv and out/summary.csv written by analyze.gp). No computation of new numbers.
import csv, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

costs_path, = [sys.argv[2]] if len(sys.argv) > 2 else ["out/costs.csv"]
outdir = "/".join(costs_path.split("/")[:-1]) or "."
costs = {}
with open(costs_path) as f:
    for row in csv.DictReader(f):
        def num(x):
            try:
                return float(x)
            except ValueError:
                return None
        costs[row["name"]] = (num(row["point"]), num(row["lo95"]), num(row["hi95"]))

# figure 1: normalised step count R = mean T / sqrt(M_arm) per bit length, pooled over good instances
fig, ax = plt.subplots(figsize=(6.4, 4.0))
for arm, label in [("kpure", "K-pure"), ("kmix", "K-mix"), ("kpureT", "K-pure on twist"), ("a0", "A0 additive"), ("a1", "A1 additive+neg")]:
    xs, ys, lo, hi = [], [], [], []
    for b in (20, 24, 28, 32, 36, 40):
        v = costs.get("Rb|%s|%d" % (arm, b))
        if v and v[0] is not None and v[1] is not None:
            xs.append(b); ys.append(v[0]); lo.append(v[0] - v[1]); hi.append(v[2] - v[0])
    if xs:
        ax.errorbar(xs, ys, yerr=[lo, hi], marker="o", capsize=3, label=label)
ax.set_xlabel("bit length b of n"); ax.set_ylabel("R = mean steps / sqrt(states)")
ax.set_title("E5: normalised step counts (95% bootstrap CI)")
ax.legend(fontsize=8); ax.grid(alpha=0.3)
fig.tight_layout(); fig.savefig(outdir + "/fig_R_vs_b.png", dpi=130); fig.savefig(outdir + "/fig_R_vs_b.svg")

# figure 2: counted-cost ratio K/A versus I/M, b >= 28 pooled, S = 0.8 M
fig, ax = plt.subplots(figsize=(6.4, 4.0))
rhos = (3, 10, 30, 100)
for pair, label in [("kpure/a0", "K-pure / A0"), ("kpure/a1", "K-pure / A1"), ("kmix/a0", "K-mix / A0"), ("kmix/a1", "K-mix / A1")]:
    xs, ys, lo, hi = [], [], [], []
    for r in rhos:
        v = costs.get("CR|%s|w0.8|rho%d|ge28" % (pair, r))
        if v and v[0] is not None and v[1] is not None:
            xs.append(r); ys.append(v[0]); lo.append(v[0] - v[1]); hi.append(v[2] - v[0])
    if xs:
        ax.errorbar(xs, ys, yerr=[lo, hi], marker="o", capsize=3, label=label)
ax.axhline(1.0, color="k", lw=0.8)
ax.set_xscale("log"); ax.set_xlabel("inversion cost I/M"); ax.set_ylabel("counted cost ratio Kummer / additive")
ax.set_title("E5: total counted cost ratio (b >= 28, S=0.8 M); ratio < 1 would be an improvement")
ax.title.set_fontsize(8)
ax.legend(fontsize=8); ax.grid(alpha=0.3)
fig.tight_layout(); fig.savefig(outdir + "/fig_cost_ratio.png", dpi=130); fig.savefig(outdir + "/fig_cost_ratio.svg")
print("wrote figures in", outdir)
