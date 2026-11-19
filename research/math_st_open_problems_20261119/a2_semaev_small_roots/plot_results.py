"""Plot recorded A2 numbers only (results/cells_summary.csv). No computation."""
import csv, sys
from collections import defaultdict
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

SER = ["#2a78d6", "#eb6834", "#1baf7a", "#eda100", "#e87ba4", "#008300"]
INK, INK2, GRID, SURF = "#0b0b0b", "#52514e", "#e4e3df", "#fcfcfb"
rows = list(csv.DictReader(open("results/cells_summary.csv")))
data = defaultdict(lambda: defaultdict(list))
for r in rows:
    a = float(r["alpha"])
    if round(a * 100) % 2:  # alpha=0.05 control cells are not part of the grid
        continue
    data[(r["poly"], int(r["p_bits"]))][int(r["dim"])].append((a, int(r["successes"]) / int(r["instances"])))
thr = {"S3": 0.5, "S4": 1 / 3}
plt.rcParams.update({"font.size": 9, "axes.edgecolor": INK2, "axes.labelcolor": INK, "xtick.color": INK2, "ytick.color": INK2})
fig, axes = plt.subplots(2, 3, figsize=(12, 6.6), sharey=True, facecolor=SURF)
for i, poly in enumerate(["S3", "S4"]):
    for j, bits in enumerate([32, 48, 64]):
        ax = axes[i][j]
        ax.set_facecolor(SURF)
        dims = sorted(data[(poly, bits)])
        for k, d in enumerate(dims):
            pts = sorted(data[(poly, bits)][d])
            ax.plot([p[0] for p in pts], [p[1] for p in pts], "-o", color=SER[k], lw=2, ms=4.5,
                    mec=SURF, mew=1, label=f"dim {d}")
        t = thr[poly]
        ax.axvline(t, color=INK, lw=1.2, ls="-")
        ax.axvline(0.6 * t, color=INK, lw=1.2, ls="--")
        ax.text(t, 1.04, " threshold", fontsize=8, color=INK, ha="left", va="bottom")
        ax.text(0.6 * t, 1.04, "0.6x ", fontsize=8, color=INK, ha="right", va="bottom")
        ax.axhline(0.5, color=GRID, lw=1, zorder=0)
        ax.set_xlim(0, 0.55 if poly == "S3" else 0.4)
        ax.set_ylim(-0.03, 1.12)
        ax.grid(axis="y", color=GRID, lw=0.6)
        for s in ("top", "right"): ax.spines[s].set_visible(False)
        ax.set_title(f"{'S_3 (2 unknowns)' if poly == 'S3' else 'S_4 (3 unknowns)'}, p of {bits} bits", color=INK, fontsize=10, loc="left")
        if i == 1: ax.set_xlabel("alpha  (|x_i| <= p^alpha)")
        if j == 0: ax.set_ylabel("planted roots recovered (of 20)")
        ax.set_yticks([0, 0.25, 0.5, 0.75, 1.0]); ax.set_yticklabels(["0", "5", "10", "15", "20"])
        if dims: ax.legend(frameon=False, fontsize=8, loc="center left", bbox_to_anchor=(0.56, 0.5), labelcolor=INK)
        if poly == "S4":
            ax.text(0.01, 0.12, "0/20 in every cell run\nonly lattice <= 150:\nm = 1, dim 125", fontsize=8, color=INK2, va="bottom")
fig.suptitle("A2: Coppersmith recovery of planted small roots of Semaev polynomials mod p "
             "(solid: counting threshold; dashed: 0.6x threshold = H2 kill line)", fontsize=10.5, color=INK, x=0.01, ha="left")
fig.tight_layout()
fig.savefig("success_vs_alpha.svg", facecolor=SURF)
fig.savefig("success_vs_alpha.png", dpi=150, facecolor=SURF)
print("wrote success_vs_alpha.svg/.png")
