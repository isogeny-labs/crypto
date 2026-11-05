"""Plots recorded values from raw/cells.tsv only (no computation). Usage: plot_gap.py <cells.tsv> <out-stem>"""
import csv, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
rows = list(csv.DictReader(open(sys.argv[1]), delimiter="\t"))
ns = [131, 163, 233, 283, 409, 571]
def series(fam, model, m, w, K):
    d = {int(r["n"]): float(r["gap_to_rho_bits"]) for r in rows
         if r["family"] == fam and r["model"] == model and r["m"] == str(m) and r["w"] == str(w) and r["K"] == K}
    return [d[n] for n in ns]
arms = [("floor", "na", "free-oracle floor", "#444444", "--"),
        ("D=4", "1", "D=4 (F4)", "#1b9e77", "-"),
        ("D=6", "1", "D=6 (XL)", "#d95f02", "-"),
        ("D=7", "1", "D=7", "#7570b3", "-"),
        ("D=l+4", "1", "D=l+4 (measured fit)", "#e7298a", "-")]
fig, axs = plt.subplots(1, 2, figsize=(11, 4.6), sharey=True)
for ax, (fam, title) in zip(axs, [("k1", "Koblitz family (A = 2n), vs matched rho"), ("r1", "Random family (A = 2), vs matched rho")]):
    for model, K, lab, c, ls in arms:
        ax.plot(ns, series(fam, model, 4, 1, K), marker="o", color=c, ls=ls, label=lab + ", m=4")
    ax.plot(ns, series(fam, "D=6", 3, 1, "1"), marker="s", color="#d95f02", ls=":", label="D=6, m=3")
    ax.plot(ns, series(fam, "floor", 2, 1, "na"), marker="x", color="#999999", ls=":", label="floor, m=2 (negative control)")
    ax.axhline(0, color="k", lw=1)
    ax.set_yscale("symlog", linthresh=30)
    ax.set_title(title, fontsize=10)
    ax.set_xlabel("field degree n (sect131 ... sect571)")
    ax.set_xticks(ns); ax.tick_params(labelsize=8)
    ax.grid(alpha=.3)
axs[0].set_ylabel("gap to matched rho, log2 group ops (K=1, w=1)\n<= 0 : model at or below rho")
h,l=axs[0].get_legend_handles_labels()
fig.legend(h,l,fontsize=7,loc="lower center",ncol=4,bbox_to_anchor=(0.5,0.04))
fig.suptitle("Item 02: derived cost-model gap to Pollard rho at deployed binary sizes (m=4 unless noted; symlog y)", fontsize=10)
fig.text(0.01, 0.008, "Source: raw/cells.tsv (CARD-BLINDSPOT-02-01). Derived, not measured. Both sides are lower bounds (unpriced counters).", fontsize=7)
fig.tight_layout(rect=(0, 0.12, 1, 0.95))
fig.savefig(sys.argv[2] + ".svg"); fig.savefig(sys.argv[2] + ".png", dpi=150)
