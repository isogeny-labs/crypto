"""Large-prime arms against matched rho on the Koblitz m=3 pair-table pipeline.
Reads only data-bin-lp.json; writes fig-bin-lp.{pdf,svg}."""
import json, os, re
import matplotlib
import matplotlib.ticker
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

H = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(H, "data-bin-lp.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25, "svg.fonttype": "none"})
C = {"plain": "#0072B2", "lp1": "#D55E00", "lp2": "#009E73"}
MK = {"plain": "o", "lp1": "s", "lp2": "^"}
fig, axes = plt.subplots(1, 3, figsize=(7.2, 2.8), sharey=True)
titles = {"n53": "$n=53$ (16 workloads)", "n61a": "$n=61$, session 1 (4)", "n61b": "$n=61$, session 2 (3)"}
for ax, g in zip(axes, ("n53", "n61a", "n61b")):
    P = D["pairs"][g]
    for key, p in sorted(P.items()):
        m = re.match(r"(plain|lp1|lp2)-k(\d+)(?:-c(\d+))?/vs/rho-strong$", key)
        if not m:
            continue
        kind, K, c = m[1], int(m[2]), m[3]
        r, lo, hi = p["ratio"], p["ci95"][0], p["ci95"][1]
        ax.errorbar(K, r, yerr=[[r - lo], [hi - r]], marker=MK[kind], color=C[kind], ms=4.5, capsize=2,
                    ls="none", mfc=C[kind] if kind != "lp2" or c == "10" else "white", zorder=3)
    ax.axhline(1, color="#444444", ls="--", lw=0.9)
    ax.set_xscale("log"); ax.set_yscale("log")
    ax.xaxis.set_major_locator(matplotlib.ticker.FixedLocator([20, 50, 100, 200, 500, 1000]))
    ax.xaxis.set_major_formatter(matplotlib.ticker.FixedFormatter(["20", "50", "100", "200", "500", "1000"]))
    ax.xaxis.set_minor_formatter(matplotlib.ticker.NullFormatter())
    ax.set_xlim(14, 1500)
    ax.set_xlabel("factor-base columns $K$")
    ax.text(0.97, 0.10, titles[g], transform=ax.transAxes, fontsize=7.5, ha="right")
axes[0].set_ylabel("arm $S$ / rho $S$ (paired, $w=0$)")
for k, lab in (("plain", "plain $m=3$ table"), ("lp1", "one large prime, $c=10$"),
               ("lp2", "two large primes, $c=10$ (filled), $c\\ne10$ (open)")):
    axes[0].plot([], [], marker=MK[k], color=C[k], ls="none", label=lab)
axes[0].plot([], [], color="#444444", ls="--", lw=0.9, label="ratio 1 (matched rho)")
fig.legend(loc="upper center", ncol=4, fontsize=6.5, frameon=False, bbox_to_anchor=(0.5, 1.06))
fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(H, f"fig-bin-lp.{ext}"), bbox_inches="tight")
