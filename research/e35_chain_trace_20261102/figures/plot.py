"""E35 figures: plots recorded values from experiments/39_chain_trace/summary.json
(written by examples/e35_tables.rs). No computation beyond reading and arranging values.
Run: python plot.py <repo root>"""
import json, sys, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = sys.argv[1] if len(sys.argv) > 1 else "."
S = json.load(open(os.path.join(root, "experiments/39_chain_trace/summary.json")))
out = os.path.dirname(os.path.abspath(__file__))
COL = {"A": "#2a78d6", "B": "#eb6834", "Bpp": "#eda100", "E25": "#e87ba4", "T": "#1baf7a"}
INK, MUTED, GRID = "#0b0b0b", "#52514e", "#e4e3df"
plt.rcParams.update({"font.size": 9, "axes.edgecolor": MUTED, "axes.labelcolor": INK,
                     "xtick.color": MUTED, "ytick.color": MUTED, "axes.grid": True,
                     "grid.color": GRID, "grid.linewidth": 0.6, "axes.spines.top": False,
                     "axes.spines.right": False, "svg.fonttype": "none"})

def save(fig, name):
    fig.savefig(os.path.join(out, name + ".svg"), bbox_inches="tight")
    fig.savefig(os.path.join(out, name + ".png"), dpi=150, bbox_inches="tight")
    plt.close(fig)

# 1. Stage 0: agreement of B's choices with the curve's own trace, and with the other curves' traces.
s0 = S["stage0"]
labels = [f"{r['p']}/{r['seed']}" for r in s0]
cross = S["stage0_cross"]["contested_agree"]
x = range(len(s0))
fig, axs = plt.subplots(1, 2, figsize=(11, 3.8), gridspec_kw={"width_ratios": [1.3, 1]})
ax = axs[0]
ax.plot(x, [100 * r["contested_row_agree"] for r in s0], marker="o", color=COL["T"], label="contested columns: B's row = own trace")
ax.plot(x, [100 * r["contested_choice_agree"] for r in s0], marker="s", color=COL["A"], label="contested: (row, position) = own trace")
ax.plot(x, [100 * max(cross[i][j] for j in range(len(s0)) if j != i) for i in x], marker="^", linestyle="--",
        color=MUTED, label="contested: row = best other curve's trace")
ax.plot(x, [100 * r["sequence_equal"] for r in s0], marker="v", color=COL["B"], label="whole sequence = own trace")
ax.set_ylim(-3, 100)
ax.set_ylabel("% (1,000 calls per curve)")
ax.set_xticks(list(x), labels, rotation=60, fontsize=7)
ax.set_xlabel("curve (p / seed)")
ax.legend(fontsize=7, frameon=False, loc="center right")
ax.set_title("Stage 0: does B's pivot choice depend on the target?", fontsize=9, color=INK)
ax = axs[1]
im = ax.imshow([[100 * v for v in row] for row in cross], cmap="Blues", vmin=60, vmax=95)
ax.set_xticks(list(x), labels, rotation=60, fontsize=7)
ax.set_yticks(list(x), labels, fontsize=7)
ax.set_xlabel("trace learned on curve")
ax.set_ylabel("calls of curve")
ax.grid(False)
fig.colorbar(im, ax=ax, fraction=0.046, label="contested-column agreement, %")
ax.set_title("Across curves", fontsize=9, color=INK)
save(fig, "stage0")

# 2. Stage A: per-call mean S ratios against T, per curve, one panel per weight.
sa = S["stage_a"]
labels = [f"{r['p']}/{r['seed']}" for r in sa]
fig, axs = plt.subplots(1, 3, figsize=(11, 3.4), sharey=False)
for g, ax in enumerate(axs):
    ax.axhline(1.0, color=INK, linewidth=0.8)
    ax.plot(range(len(sa)), [r["A_over_T"][g] for r in sa], marker="o", color=COL["A"], label="$S_A / S_T$")
    ax.plot(range(len(sa)), [r["Bpp_over_T"][g] for r in sa], marker="v", color=COL["Bpp"], label="$S_{B''} / S_T$")
    if g > 0:
        pass
    ax.set_title(f"{[0, 0.1, 1][g]} GAE per native operation", fontsize=9, color=INK)
    ax.set_xticks(range(len(sa)), labels, rotation=60, fontsize=7)
    if g == 0:
        ax.set_ylabel("ratio (above 1: T cheaper)")
        ax.legend(fontsize=7, frameon=False)
save(fig, "stage_a_ratios")

# 3. End to end: per row, S_X / S_T for X = A, B'', E25 at each weight (B and B' are far off-scale: see tables.md).
e2e = S.get("e2e", [])
if e2e:
    labels = [f"{r['p']}/{r['seed']}/{r['arm']}" for r in e2e]
    keys = [("A_over_T", "A", "$S_A / S_T$ (A: E27)", "o"), ("Bpp_over_T", "Bpp", "$S_{B''} / S_T$ (E33)", "v"),
            ("E25_over_T", "E25", "$S_{E25} / S_T$ (schoolbook)", "s")]
    fig, axs = plt.subplots(1, 3, figsize=(12, 3.8))
    for g, ax in enumerate(axs):
        ax.axhline(1.0, color=INK, linewidth=0.8)
        for k, c, lab, mk in keys:
            if g == 0 and k == "E25_over_T":
                continue
            ax.plot(range(len(e2e)), [r[k][g] for r in e2e], marker=mk, markersize=4, linewidth=1, color=COL[c], label=lab)
        ax.set_title(f"{[0, 0.1, 1][g]} GAE per native operation", fontsize=9, color=INK)
        ax.set_xticks(range(len(e2e)), labels, rotation=90, fontsize=6)
        if g == 2:
            ax.set_ylim(0.99, 1.07)
        if g == 0:
            ax.set_ylabel("ratio (above 1: T cheaper)")
        if g == 2:
            ax.legend(fontsize=7, frameon=False, loc="center right")
    save(fig, "band")
