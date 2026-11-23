"""Plot already-recorded A4 results (results/runs.csv). No computation of results here:
it only reads recorded log_p values and p, and converts them to log2 for display."""
import csv, math, sys
import matplotlib; matplotlib.use("svg")
import matplotlib.pyplot as plt

rows = [r for r in csv.DictReader(open("results/runs.csv")) if r["status"] == "ok"]
BLUE, ORANGE, INK, MUTED = "#2a78d6", "#eb6834", "#0b0b0b", "#8a8984"
fig, ax = plt.subplots(figsize=(8.6, 5.6))
seen_opt = set()
for ell, col, dx in (("2", BLUE, -0.28), ("3", ORANGE, 0.28)):
    xs, ys, xo, yo = [], [], [], []
    for r in rows:
        if r["ell"] != ell:
            continue
        lp = math.log2(int(r["p"]))
        xs.append(lp + dx); ys.append(float(r["log_p_ell_e"]) * lp)
        key = (r["csv_line"], ell)
        if key not in seen_opt:
            seen_opt.add(key)
            xo.append(lp + dx); yo.append(math.log2(int(r["optimum"])))
    ax.scatter(xs, ys, s=16, color=col, alpha=0.55, edgecolors="none",
               label=f"basic KLPT, ℓ={ell} (5 runs per ideal)")
    ax.scatter(xo, yo, s=26, facecolors="none", edgecolors=col, linewidths=1.2,
               label=f"exact least {ell}^e norm (upstream)")
xr = [10.5, 29.5]
for k, name in ((1, "p"), (3, "p³"), (3.5, "p³·⁵")):
    ax.plot(xr, [k * x for x in xr], color=MUTED, lw=1, ls="--", zorder=0)
    ax.text(xr[1] + 0.15, k * xr[1], name, color=INK, va="center", fontsize=9)
ax.set_xlim(10.5, 31)
ax.set_xlabel("log₂ p   (bands: 12, 20, 28 bits; ℓ=2 drawn left, ℓ=3 right of each p)")
ax.set_ylabel("log₂ of equivalent-ideal norm")
ax.set_title(r"A4: basic KLPT lands near $p^{3.7}$–$p^{4.0}$; the exact optimum sits near $p$", fontsize=11)
ax.grid(True, color="#e6e5e1", lw=0.6); ax.set_axisbelow(True)
for s in ("top", "right"):
    ax.spines[s].set_visible(False)
ax.legend(loc="upper left", fontsize=8.5, frameon=False)
fig.tight_layout()
fig.savefig("klpt_vs_optimum.svg")
