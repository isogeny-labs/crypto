"""Plot E27's recorded values (experiments/32_walk_karatsuba/summary.json).
Lays out recorded numbers only; computes nothing new."""
import json, sys, pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = pathlib.Path(__file__).parent
s = json.load(open(sys.argv[1]))
walks = s["walks"]
labels = [f"{w['p']}/{w['seed']}" for w in walks]
fig, ax = plt.subplots(figsize=(8, 3.6))
x = range(len(walks))
for k, (key, name, col) in enumerate([("schoolbook", "(i) E25 arithmetic", "#8c8c8c"),
                                      ("karatsuba", "(ii) Karatsuba", "#4a7fb5"),
                                      ("batched", "(iii) Karatsuba + batched inversions", "#d9772b")]):
    ax.bar([i + (k - 1) * 0.27 for i in x], [w["gls"][key] for w in walks], 0.27, label=name, color=col)
ax.set_yscale("log"); ax.set_xticks(list(x)); ax.set_xticklabels(labels)
ax.set_xlabel("curve p / seed (E25's GLS twists)"); ax.set_ylabel("F_p mults per <±ψ> walk (mean of 16)")
ax.set_title("E27 Stage W: the same walks, three arithmetics (all 320 walks identical)", fontsize=10)
ax.legend(fontsize=8, frameon=False); fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
    fig.savefig(here / f"walk_cost.{ext}", **kw)

rows = s["rows"]
fig, axs = plt.subplots(1, 3, figsize=(10, 3.4), sharey=False)
for wi, wt in enumerate(["0", "0.1", "1"]):
    ax = axs[wi]
    old = [r["dist_old"][wi] for r in rows]; new = [r["dist_new"][wi] for r in rows]
    ax.scatter(range(len(rows)), old, s=14, color="#8c8c8c", label="E31 A vs E25 walk (§9.24)")
    ax.scatter(range(len(rows)), new, s=14, color="#d9772b", label="E27: A (Karatsuba) vs walk (iii)")
    ax.set_yscale("log"); ax.set_title(f"{wt} GAE per Macaulay entry and walk step", fontsize=9)
    ax.set_xticks(range(0, len(rows), 2)); ax.set_xticklabels([f"{r['p']}/{r['seed']}" for r in rows[::2]], rotation=60, fontsize=7)
    if wi == 0: ax.set_ylabel("S / matched rho (distance remaining)")
axs[0].legend(fontsize=7, frameon=False)
fig.suptitle("E27: distance to the matched <±ψ> rho, 20 rows (arms A, B per curve); rho ahead of every row", fontsize=10)
fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
    fig.savefig(here / f"distance.{ext}", **kw)
