"""E40 figures: plots recorded values only (stage0.json, summary.json written by
examples/e40_stage0.rs and examples/e40_tables.rs); computes nothing new.
usage: python plot.py <experiments/47_macaulay_entries> <out dir>"""
import json
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

src, out = sys.argv[1], sys.argv[2]
s0 = json.load(open(f"{src}/stage0.json"))
sm = json.load(open(f"{src}/summary.json"))


def save(fig, name):
    fig.tight_layout()
    fig.savefig(f"{out}/{name}.svg")
    fig.savefig(f"{out}/{name}.png", dpi=150)
    plt.close(fig)


# 1. Stage 0: cells per call by the lead's weighted degree (GLS p=101 seed 1, 200 calls).
c = s0[0]
by = c["by_lead_degree_sum"]
n = c["calls"]
degs = [d for d, v in enumerate(by) if v[0]]
prefix = [by[d][1] / n for d in degs]
tail = [by[d][2] / n for d in degs]
fig, ax = plt.subplots(figsize=(7, 3.6))
ax.bar(degs, tail, color="#3b6ea8", label="tail (stored by V)")
ax.bar(degs, prefix, bottom=tail, color="#d9a441", label="prefix left of the lead (never read)")
ax.set_xlabel("weighted degree of the row's lead monomial")
ax.set_ylabel("Macaulay cells per call")
ax.set_title(f"Stage 0: where the 176,064 cells sit (GLS p={c['p']} seed {c['seed']}, {n} calls)", fontsize=9)
ax.legend(fontsize=8)
save(fig, "stage0_cells")

# 2. Band: S_A/S_V and S_E25/S_V per row at 0, 0.1, 1 GAE.
rows = sm["rows"]
x = list(range(len(rows)))
labels = [f"{r['set']} {r['p']}s{r['seed']}{'' if r['arm'] == '—' else r['arm']}" for r in rows]
fig, axs = plt.subplots(1, 2, figsize=(11, 3.8), sharex=True)
for ax, key, title in [(axs[0], "a_over_v", "S_A / S_V"), (axs[1], "e25_over_v", "S_E25 / S_V")]:
    for g, (w, m) in enumerate([(0, "o"), (0.1, "s"), (1, "^")]):
        ax.plot(x, [r[key][g] for r in rows], m, ms=4, label=f"{w} GAE")
    ax.axhline(1.0, color="k", lw=0.6)
    ax.set_title(f"{title} per row (32 rows, stream E37E0001)", fontsize=9)
    ax.set_xticks(x)
    ax.set_xticklabels(labels, rotation=90, fontsize=5)
    ax.legend(fontsize=7)
save(fig, "band")

# 3. Distance to rho (walk (iii)), A and V at 0 and 1 GAE.
fig, ax = plt.subplots(figsize=(8, 3.8))
for key, g, m, lab in [("a_over_rho", 0, "o", "A, 0 GAE"), ("v_over_rho", 0, ".", "V, 0 GAE"),
                       ("a_over_rho", 2, "^", "A, 1 GAE"), ("v_over_rho", 2, "v", "V, 1 GAE")]:
    ax.plot(x, [r[key][g] for r in rows], m, ms=4, label=lab)
ax.set_yscale("log")
ax.set_ylabel("S / rho (walk (iii))")
ax.set_xticks(x)
ax.set_xticklabels(labels, rotation=90, fontsize=5)
ax.set_title("Distance remaining to the matched rho, per row (rho ahead on every row)", fontsize=9)
ax.legend(fontsize=7)
save(fig, "rho")
