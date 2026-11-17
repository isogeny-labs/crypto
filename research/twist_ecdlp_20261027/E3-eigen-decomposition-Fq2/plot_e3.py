# plot_e3.py -- plots recorded values only (reads out/results.csv). No computation beyond axis transforms.
import csv, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

src = sys.argv[1] if len(sys.argv) > 1 else "out/results.csv"
outdir = sys.argv[2] if len(sys.argv) > 2 else "."
rows = list(csv.DictReader(open(src)))
def f(x):
    return float("inf") if x == "inf" else float(x)

# Figure 1: rho_gen (measured, 99% paired-bootstrap CI) and model vs N, one panel per family
fig, axes = plt.subplots(1, 2, figsize=(11, 4.2), sharey=True)
colors = {}
for ax, fam, title in zip(axes, ["1", "2"], ["Family A (m=2: X_4 vs D_2)", "Family B (m=3: X_6 vs D_3)"]):
    curves = sorted({(int(r["p"]), int(r["c"])) for r in rows if r["fam"] == fam})
    for k, (p, c) in enumerate(curves):
        rs = sorted([r for r in rows if r["fam"] == fam and int(r["p"]) == p and int(r["c"]) == c], key=lambda r: int(r["N"]))
        N = [int(r["N"]) for r in rs]
        y = [f(r["rho_gen"]) for r in rs]
        lo = [f(r["rho_gen_lo99"]) for r in rs]
        hi = [f(r["rho_gen_hi99"]) for r in rs]
        mod = [f(r["rho_model_gen"]) for r in rs]
        ok = [i for i in range(len(rs)) if lo[i] > 0 and hi[i] < 1e17]
        line, = ax.plot(N, y, "o-", label=f"p={p} c={c}")
        ax.errorbar([N[i] for i in ok], [y[i] for i in ok],
                    yerr=[[y[i] - lo[i] for i in ok], [hi[i] - y[i] for i in ok]], fmt="none", ecolor=line.get_color(), capsize=2)
        ax.plot(N, mod, "x--", color=line.get_color(), alpha=0.6)
    ax.axhline(1, color="k", lw=0.8)
    ax.axhline(2, color="gray", lw=0.8, ls=":")
    ax.set_yscale("log"); ax.set_xlabel("factor base size N"); ax.set_title(title)
    ax.legend(fontsize=7)
axes[0].set_ylabel("rho_gen = Z_X / Z_D (solid, 99% CI); x-dashed = registered model")
fig.tight_layout()
fig.savefig(f"{outdir}/e3_rho_vs_N.png", dpi=130); fig.savefig(f"{outdir}/e3_rho_vs_N.svg")

# Figure 2: mean j/m2 versus NE/N
fig, ax = plt.subplots(figsize=(4.8, 4.4))
xs = [f(r["NE_over_N"]) for r in rows if f(r["meanj_over_m2"]) >= 0]
ys = [f(r["meanj_over_m2"]) for r in rows if f(r["meanj_over_m2"]) >= 0]
ax.scatter(xs, ys, s=14)
ax.plot([0.2, 0.8], [0.2, 0.8], "k-", lw=0.8)
ax.axhspan(0.35, 0.65, color="gray", alpha=0.15)
ax.set_xlabel("N_E / N"); ax.set_ylabel("mean j / m2 over X relations (cell)")
fig.tight_layout()
fig.savefig(f"{outdir}/e3_jfrac.png", dpi=130); fig.savefig(f"{outdir}/e3_jfrac.svg")
print("plots written to", outdir)
