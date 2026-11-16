# Plots recorded values only: raw/summary-primary-r2.txt (card 10-r2). Run from repo root.
import re, matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
D = "research/blindspots_20261018/08-learned-sat-branching"
rows = {}
for l in open(f"{D}/raw/summary-primary-r2.txt"):
    m = re.match(r"rule (\w+) vs a kappa=([\d.]+): pairs (\d+) median ratio ([\d.]+) CI \[([\d.]+), ([\d.]+)\]", l)
    if m: rows[(m[1], m[2])] = tuple(map(float, m.groups()[3:]))
arms = ["b", "b2", "c", "bs", "b2s", "d", "a2"]
ks = ["0", "0.1", "1"]; col = ["#4c78a8", "#f58518", "#54a24b"]
fig, ax = plt.subplots(figsize=(8, 4.6))
for j, k in enumerate(ks):
    for i, a in enumerate(arms):
        med, lo, hi = rows[(a, k)]; x = i + (j - 1) * 0.25
        ax.errorbar(x, med, yerr=[[med - lo], [hi - med]], fmt="o", color=col[j], capsize=3, label=f"weight k={k}" if i == 0 else None)
ax.axhline(1, color="k", lw=0.8); ax.axhline(1.3, color="r", ls="--", lw=1, label="ADVANCES threshold 1.3")
ax.set_xticks(range(len(arms))); ax.set_xticklabels(["b learned\nstatic", "b2 learned\ndynamic", "c cubes", "bs\nshuffled", "b2s\nshuffled", "d random", "a2 structural"], fontsize=8)
ax.set_yscale("log"); ax.set_ylim(0.3, 1.6); ax.set_ylabel("median cost ratio  a / arm  (>1 = arm cheaper)")
ax.set_title("Item 08 (r2): held-out n=31-37, l=5, 128 targets, 95% paired bootstrap\ncost = D+P+Q+k*W (search events, unitless); no arm reaches 1.3", fontsize=9)
ax.legend(fontsize=7, loc="lower left"); fig.tight_layout()
fig.savefig(f"{D}/graph-primary-r2.svg"); fig.savefig(f"{D}/graph-primary-r2.png", dpi=150)
