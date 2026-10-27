"""Layout only: plots values recorded in summary.json (research-visuals skill).
summary.json is written by the native scorer (summarize/); this file computes nothing.

Usage: python3 plot.py reports/run-<UTC>/
"""
import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

run = Path(sys.argv[1])
S = json.loads((run / "summary.json").read_text())

INK, MUTED, GRID = "#1f2328", "#6e7781", "#d0d7de"
CURVE, NULL, PLANT, STRUCT = "#0969da", "#8c959f", "#bf8700", "#8250df"
plt.rcParams.update({"font.size": 9, "axes.edgecolor": MUTED, "axes.labelcolor": INK,
                     "xtick.color": INK, "ytick.color": INK, "svg.fonttype": "none"})


def save(fig, name):
    for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
        fig.savefig(run / f"{name}.{ext}", bbox_inches="tight", **kw)
    plt.close(fig)


def style(ax):
    ax.grid(True, color=GRID, lw=0.6)
    ax.set_axisbelow(True)
    for s in ("top", "right"):
        ax.spines[s].set_visible(False)


# 1. composite-degree lines: lift fraction and yield ratio per cell
cells = sorted(S["lines"].values(), key=lambda c: (c["N"] // c["d"], c["N"]))
fig, axs = plt.subplots(1, 2, figsize=(9.6, 3.4))
labels = [f"N={c['N']}\nd={c['d']}, n={c['N'] // c['d']}" for c in cells]
x = range(len(cells))
for i, c in enumerate(cells):
    for cur in c["curves"].values():
        sd = cur.get("standard")
        tw = cur.get("twisted")
        if sd:
            axs[0].scatter(i - 0.12, sd["lifting"] / sd["nonzero"], s=14, color=NULL, zorder=3)
        if tw:
            col = CURVE if cur["Tr_a"] == 0 else PLANT
            axs[0].scatter(i + 0.12, tw["lifting"] / tw["nonzero"], s=14, color=col, zorder=3)
axs[0].set_xticks(list(x), labels, fontsize=7)
axs[0].set_ylabel("fraction of line abscissae lifting to E")
axs[0].set_title("standard F_{2^d} (grey) vs twisted u·F_{2^d}:\nTr(a)=0 (blue), Tr(a)=1 (gold)", fontsize=9)
axs[0].set_ylim(-0.05, 1.05)
style(axs[0])
for i, c in enumerate(cells):
    for r in c["yield_ratio_tr0"]:
        axs[1].scatter(i, r["measured"], s=16, color=CURVE, zorder=3)
        axs[1].scatter(i, r["predicted"], s=40, facecolors="none", edgecolors=INK, lw=0.8, zorder=4)
    m = c["m"]
    axs[1].hlines(2 ** m, i - 0.3, i + 0.3, color=STRUCT, lw=1.2)
axs[1].set_yscale("log", base=2)
axs[1].set_xticks(list(x), labels, fontsize=7)
axs[1].set_ylabel("relation yield ratio, twisted / standard")
axs[1].set_title("m = n decompositions (dots measured, rings |F|^m/#E,\nbar 2^m); Tr(a)=0 curves with a twisted line", fontsize=9)
style(axs[1])
fig.tight_layout()
save(fig, "fig1_lines")

# 2. prime-degree repulsion: density-normalised all-lift 4-subspace counts
hc = S.get("H_C", {})
ns = sorted(int(n) for n in S["kmax"] if "count_ratio" in S["kmax"][n]["curve"])
fig, axs = plt.subplots(1, len(ns), figsize=(2.6 * len(ns), 3.2), squeeze=False)
for ax, n in zip(axs[0], ns):
    o = S["kmax"][str(n)]
    for j, (arm, col) in enumerate((("curve", CURVE), ("null", NULL))):
        ys = o[arm]["count_ratio"]["4"]
        xs = [j + (k % 7 - 3) * 0.04 for k in range(len(ys))]
        ax.scatter(xs, ys, s=10, color=col, zorder=3)
    ax.axhline(1.0, color=MUTED, lw=0.8, ls="--")
    ax.set_xticks([0, 1], ["curves", "nulls"])
    h = hc.get(str(n))
    ttl = f"N = {n}"
    if h:
        ttl += f"\np = {h['p_perm']:.2g}"
    ax.set_title(ttl, fontsize=9)
    style(ax)
axs[0][0].set_ylabel("#all-lift 4-dim subspaces / E_4(|L|)")
fig.suptitle("Lift sets repel linear structure (exact counts, density-paired nulls)", fontsize=9)
fig.tight_layout()
save(fig, "fig2_repulsion")

# 3. kmax by N: curves, nulls, planted, structured
fig, ax = plt.subplots(figsize=(6.4, 3.3))
for n_s, o in sorted(S["kmax"].items(), key=lambda t: int(t[0])):
    n = int(n_s)
    ke = o["curve"]["k_exact"]
    kc = ke if ke[0] is not None else o["curve"]["k_budget"]
    kn = o["null"]["k_exact"] if o["null"]["k_exact"][0] is not None else o["null"]["k_budget"]
    ax.scatter([n - 0.15] * len(kc), kc, s=12, color=CURVE, alpha=0.6, zorder=3)
    ax.scatter([n + 0.15] * len(kn), kn, s=12, color=NULL, alpha=0.6, zorder=3)
    sk = [k for k in o["curve"]["struct_k"] if k]
    if sk:
        ax.scatter([n] * len(sk), sk, marker="D", s=18, color=STRUCT, zorder=4)
    ax.scatter(n, o["planted"]["planted_k"], marker="_", s=120, color=PLANT, zorder=4)
ax.set_xlabel("field degree N")
ax.set_ylabel("largest all-lift subspace dim")
ax.set_title("kmax: curves (blue), paired nulls (grey), subfield construction (purple),\n"
             "planted dim (gold bar); exact for N <= 13, under budget above", fontsize=9)
style(ax)
save(fig, "fig3_kmax")

# 4. c2pnb cost-model delta (model output)
D = [r for r in S.get("c2pnb_delta_historical_python", []) if r["lp"] == "2LP" and r["log2_c_trial"] == 30.0]
if D:
    fig, ax = plt.subplots(figsize=(6.4, 3.0))
    curves = sorted({r["curve"] for r in D})
    mets = ["time", "TxM-a-p0", "mesh-a", "MM1-a"]
    for j, met in enumerate(mets):
        vals = [next(r for r in D if r["curve"] == c and r["metric"] == met) for c in curves]
        ax.bar([i + (j - 1.5) * 0.18 for i in range(len(curves))], [v["delta_bits"] for v in vals], width=0.17,
               label=met, color=[CURVE, NULL, STRUCT, PLANT][j],
               alpha=[1.0 if v["applies"] else 0.3 for v in vals][0])
    ax.set_xticks(range(len(curves)), [c + ("" if any(r["applies"] for r in D if r["curve"] == c) else "\n(Tr a = 1)")
                                       for c in curves], fontsize=7)
    ax.set_ylabel("margin change, bits (model)")
    ax.set_title("C1 Gaudry-Diem n=4 cost model (historical Python model output), 2LP, c_trial = 2^30:\n"
                 "all-lift line vs x in F_Q; applies only where Tr(a) = 0; not a measurement", fontsize=9)
    ax.legend(fontsize=7, frameon=False, ncol=4)
    style(ax)
    save(fig, "fig4_c2pnb_delta")
print("figures written to", run)
