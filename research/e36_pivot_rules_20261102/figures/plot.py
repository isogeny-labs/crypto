"""E36 figures: plots recorded values from experiments/40_pivot_rules/summary.json
(written by examples/e36_tables.rs). No computation beyond reading, arranging and
dividing recorded values. Run: python plot.py <repo root>"""
import json, os, sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = sys.argv[1] if len(sys.argv) > 1 else "."
S = json.load(open(os.path.join(root, "experiments/40_pivot_rules/summary.json")))
out = os.path.dirname(os.path.abspath(__file__))
COL = {"chain-fifo": "#2a78d6", "chain-static": "#1baf7a", "chain-fifo-static": "#8a5cd1",
       "chain-est-k2": "#eda100", "chain-lifo": "#e87ba4", "chain-sparsest-est": "#eb6834",
       "chain": "#52514e"}
NAME = {"chain-fifo": "F (arrival order)", "chain-static": "S (static)", "chain-fifo-static": "H (hybrid)",
        "chain-est-k2": "K (bounded k = 2)", "chain-lifo": "L (neg. control)",
        "chain-sparsest-est": "B″ (E33)", "chain": "A"}
MK = {"chain-fifo": "o", "chain-static": "s", "chain-fifo-static": "D", "chain-est-k2": "^",
      "chain-lifo": "x", "chain-sparsest-est": "v"}
INK, MUTED, GRID = "#0b0b0b", "#52514e", "#e4e3df"
plt.rcParams.update({"font.size": 9, "axes.edgecolor": MUTED, "axes.labelcolor": INK,
                     "xtick.color": MUTED, "ytick.color": MUTED, "axes.grid": True,
                     "grid.color": GRID, "grid.linewidth": 0.6, "axes.spines.top": False,
                     "axes.spines.right": False, "svg.fonttype": "none"})
GS = [0, 0.1, 1]


def save(fig, name):
    fig.savefig(os.path.join(out, name + ".svg"), bbox_inches="tight")
    fig.savefig(os.path.join(out, name + ".png"), dpi=150, bbox_inches="tight")
    plt.close(fig)


rows = S["stage_a"]
labels = [f"{r['p']}/{r['seed']}" for r in rows]
rules = ["chain-fifo", "chain-static", "chain-fifo-static", "chain-est-k2", "chain-lifo"]

# 1. Stage A: S_A / S_X per curve at each weight (per-call means, 1,000 calls per curve).
fig, axs = plt.subplots(1, 3, figsize=(11, 3.6))
for g, ax in enumerate(axs):
    for x in rules:
        ax.plot(range(len(rows)), [r["s_mean"]["chain"][g] / r["s_mean"][x][g] for r in rows],
                marker=MK[x], markersize=5, linewidth=1.0, color=COL[x], label=NAME[x])
    ax.plot(range(len(rows)), [r["s_mean"]["chain"][g] / r["s_mean_est"][g] for r in rows],
            marker=MK["chain-sparsest-est"], markersize=5, linewidth=1.0, linestyle="--",
            color=COL["chain-sparsest-est"], label=NAME["chain-sparsest-est"])
    ax.axhline(1.0, color=INK, linewidth=0.8)
    ax.set_title(f"{GS[g]} GAE per native operation", fontsize=9, color=INK)
    ax.set_xticks(range(len(rows)), labels, rotation=60, fontsize=7)
    ax.set_xlabel("curve (p / seed)")
    if g == 0:
        ax.set_ylabel("S_A / S_X per call (> 1: X cheaper than A)")
fig.legend(*axs[2].get_legend_handles_labels(), fontsize=7, loc="lower center", ncol=6, frameon=False, bbox_to_anchor=(0.5, -0.12))
fig.suptitle("E36 Stage A: per-call cost against solver A across the §5a band "
             "(1,000 calls per curve, means; binary df82b2e4b3)", fontsize=9)
save(fig, "stage_a_ratios")

# 2. The budget: selection work per call against the saving over A (C_A − C_X), with the
#    lines where the saving equals the selection work priced at 0.1 and 1 GAE.
fig, ax = plt.subplots(figsize=(6.2, 4.2))
xs_all = []
for x in rules + ["chain-sparsest-est"]:
    xs = [max(r["per"][x]["sel"][3], 1.0) for r in rows]
    ys = [r["per"]["chain"]["c_mean"] - r["per"][x]["c_mean"] for r in rows]
    xs_all += xs
    ax.scatter(xs, ys, marker=MK[x], s=22, color=COL[x], label=NAME[x] + (" (sel. 0, drawn at 1)" if max(xs) <= 1 else ""))
lo, hi = 0.8, max(xs_all) * 1.5
for g, ls in [(0.1, ":"), (1.0, "--")]:
    ax.plot([lo, hi], [97 * g * lo, 97 * g * hi], linestyle=ls, color=MUTED, linewidth=1)
    ax.text(hi, 97 * g * hi, f" break-even at {g} GAE", fontsize=7, color=MUTED, va="center")
ax.axhline(0, color=INK, linewidth=0.8)
ax.set_xscale("log")
ax.set_xlabel("selection work per call (native operations, Stage A mean)")
ax.set_ylabel("C_A − C_X per call (F_p multiplications)")
ax.set_title("A rule beats A at weight g only above the line 97·g·selection", fontsize=9)
ax.legend(fontsize=7, frameon=False, loc="upper left")
save(fig, "budget")

# 3. End to end: S_A / S_X and S_X / rho per row (20 rows) at each weight, carried rules.
e2e = S.get("e2e", [])
solvers = S.get("e2e_solvers", [])
if e2e and solvers:
    rl = [f"{r['p']}/{r['seed']}{r['arm']}" for r in e2e]
    fig, axs = plt.subplots(2, 3, figsize=(11, 6.2), sharex=True)
    for g in range(3):
        ax = axs[0][g]
        for x in solvers:
            ax.plot(range(len(e2e)), [r[x]["A_over"][g] for r in e2e], marker=MK[x], markersize=4,
                    linewidth=0.9, color=COL[x], label=NAME[x])
        ax.axhline(1.0, color=INK, linewidth=0.8)
        ax.set_title(f"S_A / S_X, {GS[g]} GAE", fontsize=9)
        ax = axs[1][g]
        for x in solvers:
            ax.plot(range(len(e2e)), [r[x]["over_rho"][g] for r in e2e], marker=MK[x], markersize=4,
                    linewidth=0.9, color=COL[x], label=NAME[x])
        ax.plot(range(len(e2e)), [r["A_over_rho"][g] for r in e2e], marker=".", markersize=4,
                linewidth=0.9, color=COL["chain"], label=NAME["chain"])
        ax.set_yscale("log")
        ax.set_title(f"S / rho (walk (iii)), {GS[g]} GAE", fontsize=9)
        ax.set_xticks(range(len(e2e)), rl, rotation=75, fontsize=6)
    axs[0][0].set_ylabel("> 1: X cheaper than A")
    axs[1][0].set_ylabel("distance to the matched rho")
    axs[0][2].legend(fontsize=7, frameon=False)
    axs[1][2].legend(fontsize=7, frameon=False)
    fig.suptitle("E36 end to end: E25's ten curves × two arms, Karatsuba products "
                 "(experiments/40_pivot_rules/e2e_*.json)", fontsize=9)
    save(fig, "e2e_band")
print("figures written to", out)
