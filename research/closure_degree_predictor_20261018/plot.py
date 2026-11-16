"""Layout only: plots table.csv written by compare.py (no result computed here)."""
import csv, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
rows = list(csv.DictReader(open(os.path.join(here, "table.csv"))))
rows.sort(key=lambda r: (int(r["m"]), int(r["N"]), int(r["ell"])))
fig, ax = plt.subplots(figsize=(13, 5.2))
for i, r in enumerate(rows):
    obs = [o.split("x") for o in r["observed_counts"].split(";")]
    for o, c in obs:
        y = 5 if o == ">4" else int(o)
        ax.scatter(i, y, s=40 + 25 * int(c), color="tab:blue", alpha=.7, zorder=3,
                   marker="^" if o == ">4" else "o",
                   label="observed (size = draws; triangle = '>4', plotted at 5)" if i == 0 and o == obs[0][0] else None)
    ax.scatter(i, min(int(r["pred_multi"]), 15), marker="s", color="tab:red", s=34, zorder=4,
               label="multigraded prediction (15 = cap)" if i == 0 else None)
    ax.scatter(i, int(r["pred_semireg"]), marker="x", color="tab:green", s=34, zorder=4,
               label="univariate semi-regular D_reg (control)" if i == 0 else None)
ax.axhline(4, color="grey", ls=":", lw=1)
ax.set_xticks(range(len(rows)))
ax.set_xticklabels([f"{r['m']},{r['ell']},{r['n']}" for r in rows], rotation=90, fontsize=7)
ax.set_xlabel("cell (m, ell, n), sorted by m, N, ell")
ax.set_ylabel("closure degree")
ax.set_title("Observed MutantXL closure degree (148 draws, 37 cells) vs frozen multigraded Hilbert-series prediction")
ax.legend(loc="upper left", fontsize=8)
fig.tight_layout()
fig.savefig(os.path.join(here, "observed_vs_predicted.svg"))
fig.savefig(os.path.join(here, "observed_vs_predicted.png"), dpi=150)
