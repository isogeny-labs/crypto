"""k=4 over F_{(2^m)^4}, Frobenius-reduced D4 oracle: IC over rho per target
(data-pf-d4.json only).

    python3 make_fig_pf_d4.py   # writes fig-pf-d4.{pdf,svg}
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data-pf-d4.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25,
                     "svg.fonttype": "none", "svg.hashsalt": "pf"})
fig, ax = plt.subplots(figsize=(6.6, 2.9))
clean = [r for r in D["rows"] if not r["session"].startswith(("interrupted", "IC interrupted"))]
intr = [r for r in D["rows"] if r not in clean]
for rows, filled, tag in ((clean, True, "clean session"), (intr, False, "interrupted session (indicated)")):
    ax.plot([r["log2_n"] - 0.35 for r in rows], [r["ic_over_rho_theory"] for r in rows], "o", ms=4.5,
            color="#0072B2", mfc="#0072B2" if filled else "none", label=f"IC / rho theory (0.886), {tag}")
    ax.plot([r["log2_n"] + 0.35 for r in rows], [r["ic_over_rho_measured"] for r in rows], "s", ms=4,
            color="#E69F00", mfc="#E69F00" if filled else "none", label=f"IC / same-session rho run, {tag}")
ax.axhline(1, color="#222222", ls="--", lw=0.9)
ax.set_yscale("log")
ax.set_xlim(22, 70)
ax.set_ylim(0.3, 1.2e4)
ms = sorted({(r["m"], r["log2_n"]) for r in D["rows"]})
ax.set_xticks([l for _, l in ms])
ax.set_xticklabels([f"{l}\n$m={m}$" for m, l in ms], fontsize=7.5)
ax.set_xlabel(r"$\log_2 n$, where $n = |E(\mathbb{F}_{(2^m)^4})|$")
ax.set_ylabel("IC $S$ / rho $S$")
ax.legend(fontsize=6.5, loc="upper right", frameon=False)
fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(HERE, f"fig-pf-d4.{ext}"), metadata={"Date": None} if ext == "svg" else {"CreationDate": None})
