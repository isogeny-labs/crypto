"""Plot E7b's recorded yield and F4 solver work per relation (tables_e7.json, from examples/isogeny_gap_round5_e7_tables.rs)."""
import json, os
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt
here = os.path.dirname(os.path.abspath(__file__))
t = json.load(open(os.path.join(here, "../../tables_e7.json")))["e7b_k0"]["curves"]
curves = sorted(t)
fig, (a1, a2) = plt.subplots(1, 2, figsize=(8, 3.6))
w = 0.38
for k, (arm, col) in enumerate((("f4-blind", "#c0632b"), ("f4-tracekernel", "#3b6ea8"))):
    xs = [i + (k - 0.5) * w for i in range(len(curves))]
    a1.bar(xs, [t[c][arm]["yield"] for c in curves], w, color=col, label=arm)
    a2.bar(xs, [t[c][arm]["solver_ops_per_relation"] for c in curves], w, color=col, label=arm)
for ax, lab in ((a1, "relations per trial (yield)"), (a2, "F4 solver ops per relation")):
    ax.set_xticks(range(len(curves))); ax.set_xticklabels(curves, fontsize=7, rotation=10)
    ax.set_ylabel(lab); ax.grid(True, axis="y", alpha=0.3); ax.legend(fontsize=7)
a1.set_yscale("log")
fig.suptitle("e7b_k0 (ECBS1heff1e81e9e36): default vs trace-kernel subspace base, dimension 9, 8 runs per arm", fontsize=8)
fig.tight_layout(); fig.savefig(os.path.join(here, "e7b_yield_ops.svg")); print("ok")
