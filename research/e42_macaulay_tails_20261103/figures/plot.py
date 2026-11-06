"""E42/E42b figures: plots recorded values only (stage0_v2.json from
examples/e42_stage0.rs, summary.json from examples/e42_tables.rs); computes
nothing new (bars are stacked, not summed).
usage: python plot.py <experiments/49_macaulay_tails> <out dir>"""
import json
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

src, out = sys.argv[1], sys.argv[2]
s0 = json.load(open(f"{src}/stage0_v2.json"))
sm = json.load(open(f"{src}/summary.json"))


def save(fig, name):
    fig.tight_layout()
    fig.savefig(f"{out}/{name}.svg")
    fig.savefig(f"{out}/{name}.png", dpi=150)
    plt.close(fig)


# 1. Stage 0: V's tail cells per call by the lead's weighted degree, split
#    into ever-nonzero, interior zeros of the fill extent, fill-free right end.
c = s0[0]
by, n = c["by_lead_degree_sum"], c["calls"]
degs = [d for d, v in enumerate(by) if v[0]]
ever = [by[d][4] / n for d in degs]
interior = [(by[d][3] - by[d][4]) / n for d in degs]
right = [(by[d][1] - by[d][3]) / n for d in degs]
fig, ax = plt.subplots(figsize=(7, 3.6))
ax.bar(degs, ever, color="#3b6ea8", label="ever nonzero")
ax.bar(degs, interior, bottom=ever, color="#9bb7d4", label="zero inside the fill extent")
ax.bar(degs, right, bottom=[a + b for a, b in zip(ever, interior)], color="#d9a441",
       label="right of the fill extent (never nonzero)")
ax.set_xlabel("weighted degree of the row's lead monomial")
ax.set_ylabel("V's stored tail cells per call")
ax.set_title(f"Stage 0: V's tail (GLS p={c['p']} seed {c['seed']}, {n} calls, stream seed^0xE42D_0000)", fontsize=9)
ax.legend(fontsize=8)
save(fig, "stage0_tail")

# 2. Fill arrival: growth cells by the update column (eighths of 448), 22 curves.
bins = [sum(x["growth_cells_by_bin_sum"][k] for x in s0) for k in range(8)]
calls = sum(x["calls"] for x in s0)
fig, ax = plt.subplots(figsize=(6, 3.2))
ax.bar(range(1, 9), [b / calls for b in bins], color="#3b6ea8")
ax.set_xlabel("update column, eighth of the 448 columns")
ax.set_ylabel("growth cells per call")
ax.set_title(f"Stage 0: when fill pushes a row's end right (22 curves, {calls} calls)", fontsize=9)
save(fig, "fill_arrival")

# 3. Candidate storage rules, per call (curve GLS p=101 s1): recorded components stacked.
f = c["fields"]
m = lambda k: f[k]["mean"]
rules = [
    ("V (tail)", [m("tail_cells"), 0, 0, 0]),
    ("exact growth", [c["reserve_cells_mean"][0], c["reserve_copy_cells_mean"][0], m("updates"), c["reserve_reallocs_mean"][0]]),
    ("reserve k=1", [c["reserve_cells_mean"][1], c["reserve_copy_cells_mean"][1], m("updates"), c["reserve_reallocs_mean"][1]]),
    ("reserve k=2 (W)", [c["reserve_cells_mean"][2], c["reserve_copy_cells_mean"][2], m("updates"), c["reserve_reallocs_mean"][2]]),
    ("two segments", [m("final_extent_cells"), m("overflow_copy_cells"), m("updates") + m("overflow_split_updates"), m("growth_events")]),
]
labels = ["stored cells", "copied cells", "fill checks (+ splits)", "re-allocations"]
colors = ["#3b6ea8", "#d9a441", "#7a9a52", "#a8443b"]
fig, ax = plt.subplots(figsize=(7.5, 3.6))
for i, (name, parts) in enumerate(rules):
    bottom = 0
    for j, v in enumerate(parts):
        ax.barh(i, v, left=bottom, color=colors[j], label=labels[j] if i == 1 else None)
        bottom += v
ax.set_yticks(range(len(rules)), [r[0] for r in rules])
ax.set_xlim(0, 1.6e5)
ax.set_xlabel("charged native operations per call (exact growth runs off the axis: copies ≈ 2.0·10⁵)")
ax.set_title(f"Stage 0: storage rules for V's tails (GLS p={c['p']} seed {c['seed']}, {n} calls); sparse rows ≥ {m('sparse_merge_lower_bound'):.2e} index ops", fontsize=8)
ax.legend(fontsize=7, loc="lower right")
save(fig, "levers")

# 4. Band: S_V/S_W and S_A/S_W per end-to-end row at 0.1 and 1 GAE.
rows = sm["rows"]
lab = [f"{r['set']} {r['p']} s{r['seed']} {r['arm']}" for r in rows]
x = range(len(rows))
fig, axs = plt.subplots(1, 2, figsize=(10, 4.2))
for ax, k, t in [(axs[0], "v_over_w", "S_V / S_W"), (axs[1], "a_over_w", "S_A / S_W")]:
    for g, col, mk in [(0, "#888888", "s"), (1, "#3b6ea8", "o"), (2, "#a8443b", "^")]:
        ax.plot(x, [r[k][g] for r in rows], mk, color=col, ms=4, label=f"{[0, 0.1, 1][g]} GAE")
    ax.axhline(1, color="k", lw=0.6)
    ax.set_xticks(list(x), lab, rotation=90, fontsize=6)
    ax.set_ylabel(t + " (> 1: W cheaper)")
    ax.legend(fontsize=7)
fig.suptitle("E42b end to end, 32 rows, stream E37E0001, Karatsuba (W = chain-tails, V = E40 chain-profile, A = E37 chain-counted)", fontsize=9)
save(fig, "band")

# 5. Distance to the matched rho (walk (iii)): S_W / rho per row.
fig, ax = plt.subplots(figsize=(10, 4))
for g, col, mk in [(0, "#888888", "s"), (1, "#3b6ea8", "o"), (2, "#a8443b", "^")]:
    ax.semilogy(x, [r["w_over_rho"][g] for r in rows], mk, color=col, ms=4, label=f"{[0, 0.1, 1][g]} GAE")
ax.axhline(1, color="k", lw=0.6)
ax.set_xticks(list(x), lab, rotation=90, fontsize=6)
ax.set_ylabel("S_W / rho (walk (iii))")
ax.set_title("Distance remaining to the matched rho, every row (rho ahead where > 1)", fontsize=9)
ax.legend(fontsize=7)
save(fig, "rho")
