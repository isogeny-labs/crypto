#!/usr/bin/env python3
"""Log-log T against |V| for m = 3 and m = 4 at k = 20 and k = 22: parent cells and extension cells,
censored cells as vertical dotted lines. Input data/analysis.json; output exponent_ext.svg / .png."""
import json, os
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
D = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
A = json.load(open(os.path.join(D, "data", "analysis.json")))["groups"]
col = {3: "#c0501a", 4: "#2a8a4a"}
fig, axs = plt.subplots(1, 2, figsize=(11, 4.8), sharey=True)
for ax, k in zip(axs, (20, 22)):
    for m in (3, 4):
        g = A.get(f"k{k}_m{m}_sym_g0")
        if not g: continue
        ok = [r for r in g["rows"] if not r["capped"] and r["T"]]
        ax.plot([r["v_size"] for r in ok], [r["T"] for r in ok], "-", color=col[m], lw=0.8, alpha=.5)
        for r in ok:
            ext = r["origin"] == "ext"
            ax.plot(r["v_size"], r["T"], "D" if ext else "o", color=col[m], mfc=col[m] if ext else "white", ms=6 if ext else 5)
        ax.plot([], [], "o", color=col[m], mfc="white", label=f"m = {m}, parent cell")
        ax.plot([], [], "D", color=col[m], label=f"m = {m}, extension cell")
        for r in g["rows"]:
            if r["capped"]:
                ax.axvline(r["d"], color=col[m], ls=":" if r["origin"] == "ext" else "-.", lw=1)
                ax.text(r["d"], 2e3, f" m={m} d={r['d']} censored ({r['origin']})", color=col[m], rotation=90, fontsize=6.5, va="bottom")
        fe = g.get("fit_d_ge_9")
        txt = (f"slope {fe['slope']:.2f} [{fe['lo95']:.2f}, {fe['hi95']:.2f}]" if fe.get("applicable", True)
               else f"estimator not applicable ({fe['n']} cells, span {fe['span']:.1f}x)")
        ax.text(0.02, 0.97 - 0.06 * (m - 3), f"m={m}: {txt}", transform=ax.transAxes, color=col[m], fontsize=8, va="top")
    x = np.array([3, 100.0])
    ax.plot(x, 1e3 * (x / 3) ** 0.5, "k:", label="slope 1/2 (reference)")
    ax.plot(x, 1e3 * (x / 3) ** 1.0, "k--", lw=0.6, label="slope 1 (reference)")
    ax.set_xscale("log"); ax.set_yscale("log"); ax.grid(True, which="both", alpha=.2)
    ax.set_xlabel("|V| = deg L = d")
    ax.set_title(f"k = {k}: T(E,m,L), subgroup bases, G = 0, 12 GB / 10,800 s")
axs[0].set_ylabel("T: F_p multiplications per system (mean of 6 targets)")
axs[1].legend(fontsize=7, loc="lower right")
fig.tight_layout()
fig.savefig(os.path.join(D, "exponent_ext.svg")); fig.savefig(os.path.join(D, "exponent_ext.png"), dpi=150)
print("wrote exponent_ext.svg/.png")
