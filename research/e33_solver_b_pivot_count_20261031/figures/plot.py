"""E33 figures: plots recorded values from experiments/37_solver_b_pivot_count/summary.json
(written by examples/e33_tables.rs). No computation beyond reading and arranging values.
Run: python plot.py <repo root>"""
import json, sys, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = sys.argv[1] if len(sys.argv) > 1 else "."
S = json.load(open(os.path.join(root, "experiments/37_solver_b_pivot_count/summary.json")))
out = os.path.dirname(os.path.abspath(__file__))
# Validated default categorical slots (dataviz palette.md), fixed order per entity.
COL = {"chain": "#2a78d6", "chain-sparsest-counted": "#eb6834", "chain-sparsest-inc": "#1baf7a",
       "chain-sparsest-est": "#eda100", "existing": "#e87ba4"}
NAME = {"existing": "E25 (existing)", "chain": "A", "chain-sparsest-counted": "B, selection counted",
        "chain-sparsest-inc": "B′ (incremental)", "chain-sparsest-est": "B″ (estimate)"}
INK, MUTED, GRID = "#0b0b0b", "#52514e", "#e4e3df"
plt.rcParams.update({"font.size": 9, "axes.edgecolor": MUTED, "axes.labelcolor": INK,
                     "xtick.color": MUTED, "ytick.color": MUTED, "axes.grid": True,
                     "grid.color": GRID, "grid.linewidth": 0.6, "axes.spines.top": False,
                     "axes.spines.right": False, "svg.fonttype": "none"})

def save(fig, name):
    fig.savefig(os.path.join(out, name + ".svg"), bbox_inches="tight")
    fig.savefig(os.path.join(out, name + ".png"), dpi=150, bbox_inches="tight")
    plt.close(fig)

# 1. Stage A: per-call S(g) means, curve by curve, one panel per weight (markers, log axis).
rows = S["stage_a"]
labels = [f"{r['p']}/{r['seed']}" for r in rows]
arms = ["existing", "chain", "chain-sparsest-counted", "chain-sparsest-inc", "chain-sparsest-est"]
MK = {"existing": "s", "chain": "o", "chain-sparsest-counted": "^", "chain-sparsest-inc": "D", "chain-sparsest-est": "v"}
fig, axs = plt.subplots(1, 3, figsize=(11, 3.6))
for g, ax in enumerate(axs):
    for a in arms:
        ax.plot(range(len(rows)), [r["s_mean"][a][g] for r in rows], marker=MK[a], markersize=6,
                linewidth=1.2, color=COL[a], label=NAME[a])
    ax.set_yscale("log")
    ax.set_title(f"{[0, 0.1, 1][g]} GAE per native operation", fontsize=9, color=INK)
    ax.set_xticks(range(len(rows)), labels, rotation=60, fontsize=7)
    ax.set_xlabel("p / seed")
axs[0].set_ylabel("mean per-call S(g), F_p mults (log)")
h, l = axs[0].get_legend_handles_labels()
fig.legend(h, l, fontsize=8, frameon=False, loc="lower center", ncol=5, bbox_to_anchor=(0.5, -0.17))
fig.suptitle("E33 Stage A: per-call cost with pivot-selection work priced (1,000 calls per curve)", fontsize=10)
save(fig, "stage_a_per_call")

# 2. Stage A: selection work per call by part (B, B′, B″), pooled mean over curves shown per curve.
fig, ax = plt.subplots(figsize=(7.5, 3.2))
parts = ["initial counts", "comparisons", "count updates"]
pcol = ["#2a78d6", "#eb6834", "#1baf7a"]
sel = ["chain-sparsest-counted", "chain-sparsest-inc", "chain-sparsest-est"]
r0 = rows  # per curve
xs, ticks = [], []
for k, a in enumerate(sel):
    vals = [[r["selection"][a][j] for r in r0] for j in range(3)]
    mean = [sum(v) / len(v) for v in vals]
    bottom = 0
    for j in range(3):
        ax.bar(k, mean[j], 0.5, bottom=bottom, color=pcol[j], label=parts[j] if k == 0 else None,
               edgecolor="white", linewidth=1)
        bottom += mean[j]
    ax.text(k, bottom * 1.02, f"{bottom:,.0f}", ha="center", va="bottom", fontsize=8, color=INK)
ax.set_xticks(range(3), [NAME[a] for a in sel])
ax.set_ylabel("native operations per call (mean of 10 curves)")
ax.legend(fontsize=8, frameon=False)
ax.set_title("Pivot-selection work per call, by part (Stage A)", fontsize=10, color=INK)
save(fig, "selection_parts")

# 3. End to end: ratios across the band (min-max over 20 rows), three panels.
e2e = S["e2e"]
solvers = S["e2e_solvers"]
G = [0, 0.1, 1]
MK3 = {"chain-sparsest-counted": "^", "chain-sparsest-inc": "D", "chain-sparsest-est": "v"}
fig, axs = plt.subplots(1, 3, figsize=(11, 3.6))
for ax, (key, title) in zip(axs, [("A_over", "S_A / S_X  (Karatsuba)"), ("E25_over", "S_E25 / S_X  (schoolbook)"),
                                    ("over_rho", "S_X / rho  (walk iii)")]):
    for k, a in enumerate(solvers):
        lo = [min(r[a][key][g] for r in e2e) for g in range(3)]
        hi = [max(r[a][key][g] for r in e2e) for g in range(3)]
        mid = [(l * h) ** 0.5 for l, h in zip(lo, hi)]
        xs = [g + (k - 1) * 0.2 for g in range(3)]
        ax.errorbar(xs, mid, yerr=[[m - l for m, l in zip(mid, lo)], [h - m for m, h in zip(mid, hi)]],
                    fmt=MK3[a], markersize=7, color=COL[a], capsize=4, linewidth=1.5, label=NAME[a])
    if key != "over_rho":
        ax.axhline(1, color=INK, linewidth=0.8)
        ax.text(2.5, 1.04, "parity: above = X cheaper", fontsize=7, color=MUTED, va="bottom", ha="right")
    ax.set_yscale("log")
    ax.set_xticks(range(3), [f"{g} GAE" for g in G])
    ax.set_xlim(-0.5, 2.6)
    ax.set_title(title, fontsize=9, color=INK)
axs[0].set_ylabel("min–max over 20 rows (log)")
h, l = axs[0].get_legend_handles_labels()
fig.legend(h, l, fontsize=8, frameon=False, loc="lower center", ncol=3, bbox_to_anchor=(0.5, -0.06))
fig.suptitle("E33 end to end, E25's ten curves × two arms (one GAE = 97 F_p mults per native operation)", fontsize=10)
save(fig, "band")
print("ok")
