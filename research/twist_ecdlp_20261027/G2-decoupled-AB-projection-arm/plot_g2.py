# plot_g2.py -- plots recorded values only (reads out/results.csv). No computation beyond axis transforms.
import csv, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

src = sys.argv[1] if len(sys.argv) > 1 else "out/results.csv"
outdir = sys.argv[2] if len(sys.argv) > 2 else "."
rows = [r for r in csv.DictReader(open(src)) if r["fam"] in ("A", "B")]
def f(x):
    if x == "NA":
        return float("nan")
    return float("inf") if x == "inf" else float(x)

def series(fam, p, scope, arm):
    rs = [r for r in rows if r["fam"] == fam and int(r["p"]) == p and r["scope"] == scope and r["arm"] == arm and int(r["N"]) > 0]
    rs.sort(key=lambda r: int(r["N"]))
    return rs

arms = [("PF(j=m;k=m)", "o-"), ("PF(j=m;k=1)", "s-"), ("PF best (sweep)", "^-"), ("X coupled E-sets", "d--")]
fig, axes = plt.subplots(1, 2, figsize=(12, 4.6), sharey=True)
for ax, fam, title in zip(axes, ["A", "B"], ["Family A (m=2)", "Family B (m=3)"]):
    ps = sorted({int(r["p"]) for r in rows if r["fam"] == fam and int(r["p"]) > 0})
    cols = {p: c for p, c in zip(ps, ["tab:blue", "tab:orange", "tab:green"])}
    for p in ps:
        for arm, style in arms:
            rs = series(fam, p, "primary", arm)
            if not rs:
                continue
            N = [int(r["N"]) for r in rs]
            y = [f(r["point"]) for r in rs]
            ok = [i for i in range(len(rs)) if y[i] < 1e17 and y[i] > 0 and rs[i]["powered"] == "1"]
            if not ok:
                continue
            ax.plot([N[i] for i in ok], [y[i] for i in ok], style, color=cols[p], alpha=0.8, ms=4,
                    label=f"p={p} {arm}" if p == ps[0] else None)
    ax.axhline(1, color="k", lw=1.2)
    ax.set_yscale("log"); ax.set_xlabel("factor base size N"); ax.set_title(title + " (powered cells only; colour = p; black line rho=1)")
    ax.title.set_fontsize(8)
axes[0].set_ylabel("rho_E = Z_arm / Z_D* (Semaev work units per independent E-relation)")
axes[0].legend(fontsize=6)
fig.tight_layout()
for ext in ("png", "svg", "pdf"):
    fig.savefig(f"{outdir}/g2_rho_vs_N.{ext}", dpi=130)

# pair-metric accounting (relabelling) vs independent E-relation metric, powered primary cells
fig, axes = plt.subplots(1, 2, figsize=(12, 4.2), sharey=True)
for ax, fam, title in zip(axes, ["A", "B"], ["Family A (m=2)", "Family B (m=3)"]):
    ps = sorted({int(r["p"]) for r in rows if r["fam"] == fam and int(r["p"]) > 0})
    for p, c in zip(ps, ["tab:blue", "tab:orange", "tab:green"]):
        for arm, style in [("pair(m;m) accounting", "o-"), ("PF(j=m;k=m)", "x--")]:
            rs = series(fam, p, "primary", arm)
            ok = [r for r in rs if r["powered"] == "1" and 0 < f(r["point"]) < 1e17]
            if ok:
                ax.plot([int(r["N"]) for r in ok], [f(r["point"]) for r in ok], style, color=c, alpha=0.8, ms=4,
                        label=f"p={p} {arm}")
    ax.axhline(1, color="k", lw=1.2)
    ax.set_yscale("log"); ax.set_xlabel("factor base size N"); ax.set_title(title + " (pair-count accounting is a relabelling, not an improvement)")
    ax.title.set_fontsize(8)
    ax.legend(fontsize=6)
axes[0].set_ylabel("rho vs D*")
fig.tight_layout()
for ext in ("png", "svg", "pdf"):
    fig.savefig(f"{outdir}/g2_pair_accounting.{ext}", dpi=130)
print("plots written to", outdir)
