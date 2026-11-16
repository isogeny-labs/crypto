"""Multi-target IC over the best multi-target rho against group size, unbatched and batched inversion.
Reads only data-u16grob-mt.json; writes fig-u16grob-mt.{pdf,svg}."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

H = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(H, "data-u16grob-mt.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25, "svg.fonttype": "none"})
C = {"1024": "#0072B2", "4096": "#D55E00", "16384": "#009E73"}
MK = {"1024": "o", "4096": "s", "16384": "^"}
OFF = {"1024": -0.15, "4096": 0.0, "16384": 0.15}
fig, ax = plt.subplots(figsize=(4.6, 2.9))
for kind, filled in (("unbatched", True), ("batched", False)):
    for T, cells in D[kind].items():
        xs = sorted(cells, key=int)
        x = [int(b) + OFF[T] + (0.05 if not filled else 0.0) for b in xs]
        y = [cells[b][0] for b in xs]
        lo = [cells[b][0] - cells[b][1] for b in xs]
        hi = [cells[b][2] - cells[b][0] for b in xs]
        ax.errorbar(x, y, yerr=[lo, hi], marker=MK[T], color=C[T], ms=4.5, capsize=2, lw=0.9,
                    ls="-" if filled else ":", mfc=C[T] if filled else "white", zorder=3)
ax.axhline(1, color="#444444", ls="--", lw=0.9)
ax.set_xticks([32, 34, 36])
ax.set_xticklabels(["$2^{32}$", "$2^{34}$", "$2^{36}$"])
ax.set_xlabel("group order $n$")
ax.set_ylabel("IC wall / best rho wall")
for T in ("1024", "4096", "16384"):
    ax.plot([], [], marker=MK[T], color=C[T], ls="none", label=f"$T={int(T):,}$".replace(",", "{,}"))
ax.plot([], [], marker="o", color="#666666", ls="-", label="both unbatched")
ax.plot([], [], marker="o", color="#666666", mfc="white", ls=":", label="both batched")
ax.plot([], [], color="#444444", ls="--", lw=0.9, label="parity")
ax.legend(fontsize=6.5, frameon=False, loc="upper left", ncol=2)
ax.set_ylim(0.9, 2.1)
fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(H, f"fig-u16grob-mt.{ext}"))
