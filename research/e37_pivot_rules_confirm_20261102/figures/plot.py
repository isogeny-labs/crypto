"""E37 figures: plots recorded values from experiments/44_pivot_rules_confirm/summary.json
(written by examples/e37_tables.rs). Python only reads, arranges and divides recorded values;
every ratio plotted is also in the summary. Run: python plot.py <repo root>"""
import json, os, sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = sys.argv[1] if len(sys.argv) > 1 else "."
S = json.load(open(os.path.join(root, "experiments/44_pivot_rules_confirm/summary.json")))
out = os.path.dirname(os.path.abspath(__file__))
COL = {"chain-fifo": "#2a78d6", "chain-static": "#1baf7a", "chain-fifo-static": "#8a5cd1",
       "chain-lifo": "#e87ba4", "chain-sparsest-est": "#eb6834", "chain-counted": "#52514e",
       "macaulay": "#a0a09a"}
NAME = {"chain-fifo": "F (arrival order)", "chain-static": "S (static)", "chain-fifo-static": "H (hybrid)",
        "chain-lifo": "L (neg. control)", "chain-sparsest-est": "B″ (E33)", "chain-counted": "A (E27)",
        "macaulay": "E25 (Macaulay)"}
MK = {"chain-fifo": "o", "chain-static": "s", "chain-fifo-static": "D", "chain-lifo": "x",
      "chain-sparsest-est": "v", "chain-counted": "^", "macaulay": "P"}
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


# 1. Stage A: S_A / S_X per curve at each weight (per-call means, 1,000 fresh calls per curve).
rows = S["stage_a"]
labels = [f"{r['p']}/{r['seed']}" for r in rows]
rules = ["chain-fifo", "chain-static", "chain-fifo-static", "chain-lifo", "chain-sparsest-est"]
fig, axs = plt.subplots(1, 3, figsize=(11, 3.4), sharey=False)
for gi, ax in enumerate(axs):
    for x in rules:
        v = [r["per"]["chain"]["s"][gi] / r["per"][x]["s"][gi] for r in rows]
        ax.plot(range(len(rows)), v, marker=MK[x], color=COL[x], label=NAME[x], lw=1, ms=4)
    ax.axhline(1, color=INK, lw=0.8)
    ax.set_xticks(range(len(rows)), labels, rotation=60, fontsize=7)
    ax.set_title(f"{GS[gi]} GAE", fontsize=9)
    ax.set_xlabel("curve p / seed")
axs[0].set_ylabel("S_A / S_X  (>1: X cheaper than A)")
axs[2].legend(fontsize=7, loc="lower left")
fig.suptitle("E37 Stage A: per-call S(g) of A over each rule, 1,000 fresh calls per curve (salt 0xE37A0001)", fontsize=9)
save(fig, "stage_a_ratios")

# 2. End to end: S_A / S_X per row at each weight, GLS (20) and E24 (12) rows.
E = S["e2e"]
rl = [f"{r['set']} {r['p']}/{r['seed']}{'' if r['arm'] == '—' else ' ' + r['arm']}" for r in E]
fig, axs = plt.subplots(3, 1, figsize=(11, 8), sharex=True)
for gi, ax in enumerate(axs):
    for x in ["chain-fifo", "chain-static", "chain-fifo-static", "chain-sparsest-est", "macaulay"]:
        v = [r["chain-counted"]["S"][gi] / r[x]["S"][gi] for r in E]
        ax.plot(range(len(E)), v, marker=MK[x], color=COL[x], label=NAME[x], lw=0.8, ms=4)
    ax.axhline(1, color=INK, lw=0.8)
    ax.axvline(19.5, color=MUTED, lw=0.6, ls="--")
    ax.set_ylabel(f"S_A / S_X at {GS[gi]} GAE")
    if gi == 2:
        ax.set_xticks(range(len(E)), rl, rotation=75, fontsize=6.5)
axs[0].legend(fontsize=7, ncol=5, loc="upper right")
fig.suptitle("E37 end to end, paired on the fresh stream 0xE37E0001: A's S(g) over each arm's (>1: arm cheaper than A); left of the dashed line E25's 20 GLS rows, right E24's 12 curves", fontsize=8.5)
save(fig, "e2e_band")

# 3. Distance to the matched rho (walk (iii)) per row, at each weight, for A, F and E25.
fig, axs = plt.subplots(1, 3, figsize=(11, 3.6), sharey=True)
for gi, ax in enumerate(axs):
    for x in ["chain-counted", "chain-fifo", "macaulay"]:
        v = [r[x]["over_rho"][gi] for r in E]
        ax.semilogy(range(len(E)), v, marker=MK[x], color=COL[x], label=NAME[x], lw=0.8, ms=3.5)
    ax.axhline(1, color=INK, lw=0.8)
    ax.axvline(19.5, color=MUTED, lw=0.6, ls="--")
    ax.set_title(f"{GS[gi]} GAE", fontsize=9)
    ax.set_xlabel("row (0–19 GLS, 20–31 E24)")
axs[0].set_ylabel("S_X / rho  (1 = parity with rho)")
axs[0].legend(fontsize=7)
fig.suptitle("E37 distance remaining to the matched rho (walk (iii)); every row is above 1 (rho ahead)", fontsize=9)
save(fig, "rho_distance")

# 4. Controls: positive-check and detection rates against the registered thresholds.
C = S["controls"]
fig, ax = plt.subplots(figsize=(8, 3.4))
lab = [f"{c['control'].upper()} {c['rule']}" + ("" if c["gating"] else " (cal.)") for c in C]
xs = range(len(C))
w = 0.27
ax.bar([i - w for i in xs], [c["positive_rate"] for c in C], w, color="#2a78d6", label="positive check")
ax.bar(list(xs), [c["detection_rate"] for c in C], w, color="#1baf7a", label="detected (all calls)")
ax.bar([i + w for i in xs], [c["nonempty_detection_rate"] for c in C], w, color="#8a5cd1", label="detected (non-empty calls)")
for y, t in [(0.95, "0.95"), (0.75, "0.75")]:
    ax.axhline(y, color=INK, lw=0.7, ls="--")
    ax.text(len(C) - 0.5, y + 0.01, t, fontsize=7, ha="right")
ax.set_xticks(list(xs), lab, fontsize=8)
ax.set_ylim(0, 1.05)
ax.set_ylabel("rate over 320 control calls")
ax.legend(fontsize=7, loc="lower left")
ax.set_title("E37 negative controls (G2.165): thresholds 0.95 (positive, non-empty) and 0.75 (all calls)", fontsize=9)
save(fig, "controls")
