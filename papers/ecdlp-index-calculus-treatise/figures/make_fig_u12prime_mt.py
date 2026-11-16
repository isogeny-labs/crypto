"""Multi-target IC over multi-target rho, counted and timed (data-u12prime-mt.json only).

    python3 make_fig_u12prime_mt.py   # writes fig-u12prime-mt.{pdf,svg}
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data-u12prime-mt.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25,
                     "svg.fonttype": "none", "svg.hashsalt": "u12prime-mt"})
SERIES = [("h0", "counted, $h=0$", "#0072B2", "o", -0.24),
          ("h01", "counted, $h=0.1$", "#009E73", "s", -0.08),
          ("h1", "counted, $h=1$", "#D55E00", "^", 0.08),
          ("wall", "wall time (one CPU)", "#000000", "D", 0.24)]
rows = D["rows"]
fig, ax = plt.subplots(figsize=(6.6, 3.0))
for key, lab, col, mk, dx in SERIES:
    xs = [i + dx for i in range(len(rows))]
    y = [r[key][0] for r in rows]
    lo = [r[key][0] - r[key][1] for r in rows]
    hi = [r[key][2] - r[key][0] for r in rows]
    ax.errorbar(xs, y, yerr=[lo, hi], fmt=mk, ms=4, color=col, mfc="none" if key == "wall" else col,
                capsize=2, lw=1, label=lab, zorder=3)
ax.axhline(1, color="#222222", ls="--", lw=0.9)
for b in (2.5, 5.5):
    ax.axvline(b, color="#999999", lw=0.6)
ax.set_xticks(range(len(rows)))
ax.set_xticklabels([f"$2^{{{r['bits']}}}$\n$T={r['T']}$" for r in rows], fontsize=7)
ax.set_xlabel("group order and number of targets $T$")
ax.set_ylabel("IC / rho per target (ratio)")
ax.set_ylim(0.65, 1.4)
ax.legend(fontsize=6.8, ncol=4, loc="upper left", frameon=False)
fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(HERE, f"fig-u12prime-mt.{ext}"),
                metadata={"Date": None} if ext == "svg" else {"CreationDate": None})
