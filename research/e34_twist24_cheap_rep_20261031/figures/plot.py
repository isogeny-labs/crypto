"""Plot E34's recorded values (experiments/38_twist24_cheap_rep/summary.json); computes nothing new."""
import json, sys, pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
here = pathlib.Path(__file__).parent
s = json.load(open(sys.argv[1]))
ps = s["curves"]
band = s["band"]
fig, axes = plt.subplots(1, 2, figsize=(10, 3.6), sharey=True)
pairs = [("Order6Built/Order24Built", "#8c8c8c", "E29 order 6 / E29 order 24"),
         ("Order6Built/Order24Cheap", "#4a7fb5", "E29 order 6 / cheap order 24"),
         ("Order6Cheap/Order24Cheap", "#d9772b", "cheap order 6 / cheap order 24")]
for ax, arith in zip(axes, ("schoolbook", "karatsuba")):
    for pair, col, lab in pairs:
        rows = band[f"{pair}|{arith}|full|gae97"]["rows"]
        ws = [r["weight"] for r in rows]
        ax.plot(range(3), [r["pooled"] for r in rows], "o-", color=col, label=lab + " (pooled)")
        for i, r in enumerate(rows):
            ax.scatter([i] * len(ps), [c["ratio"] for c in r["per_curve"]], color=col, s=8, alpha=0.35)
    ax.axhline(1, color="k", lw=0.7)
    ax.set_xticks(range(3)); ax.set_xticklabels(["0", "0.1", "1"])
    ax.set_xlabel("GAE per native operation (§5a band; every emitted op priced)")
    ax.set_title(f"{arith}", fontsize=10)
axes[0].set_ylabel("walk cost, order 6 / order 24 (>1: order 24 cheaper)")
axes[0].legend(fontsize=7, frameon=False)
fig.suptitle("E34: walk cost ratios across the band (dots: the 8 curves, 16 walks each)", fontsize=10)
fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})): fig.savefig(here / f"band.{ext}", **kw)
pa = s["per_arm"]
arms = ["Order6Built", "Order6Cheap", "Order24Built", "Order24Cheap", "Order24CheapMulLean"]
kinds = ["cmp", "neg", "modn", "hash", "table"]
fig, axes = plt.subplots(1, 2, figsize=(10, 3.4))
p = 1069
muls = [pa[f"{p}|schoolbook|{a}"]["muls_per_step"] for a in arms]
axes[0].bar(range(5), muls, color=["#8c8c8c", "#bbbbbb", "#d9772b", "#4a7fb5", "#7fa7d0"])
axes[0].set_xticks(range(5)); axes[0].set_xticklabels([a.replace("Order", "o") for a in arms], fontsize=7)
axes[0].set_ylabel(f"F_p multiplications per step (p={p}, schoolbook)")
bottom = [0] * 5
cols = ["#4a7fb5", "#d9772b", "#8c8c8c", "#2ca02c", "#9467bd"]
for k, c in zip(kinds, cols):
    v = [pa[f"{p}|schoolbook|{a}"]["native_per_step_by_kind"][k] for a in arms]
    axes[1].bar(range(5), v, bottom=bottom, color=c, label=k)
    bottom = [b + x for b, x in zip(bottom, v)]
axes[1].set_xticks(range(5)); axes[1].set_xticklabels([a.replace("Order", "o") for a in arms], fontsize=7)
axes[1].set_ylabel("native operations per step"); axes[1].legend(fontsize=7, frameon=False)
fig.suptitle("E34: per-step cost, counted (left) and emitted native operations (right)", fontsize=10)
fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})): fig.savefig(here / f"per_step.{ext}", **kw)
