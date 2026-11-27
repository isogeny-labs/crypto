#!/usr/bin/env python3
"""Figure: fully charged cost per arm on the frozen B677 workload, log scale, with rho references."""
import json, sys, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
d = json.load(open(sys.argv[1])); out = sys.argv[2]
arms = [("baseline", "frozen #5097\nsource"), ("exact", "exact\n(same stream)"), ("s26", "sampled\nf=26/256"),
        ("s13", "sampled\nf=13/256"), ("rho", "frozen rho\n(1 seed)"), ("rhob", "equal-eng. rho\n(40-seed mean)")]
def val(a, k):
    v = d["arms"][a]
    return v["mean"][k] if a == "rhob" else v[k]["median"]
fig, axes = plt.subplots(1, 3, figsize=(13, 4.2))
cols = ["#4c6ef5"] * 4 + ["#868e96", "#212529"]
for ax, (k, title) in zip(axes, [("M", "field multiplications\n(inversions expanded)"), ("pts", "points computed"), ("wall", "cold wall (ms), 4 vCPU cloud VM")]):
    ys = [val(a, k) for a, _ in arms]
    ax.bar(range(len(arms)), ys, color=cols)
    ax.set_yscale("log"); ax.set_title(title, fontsize=10)
    ax.set_xticks(range(len(arms))); ax.set_xticklabels([l for _, l in arms], fontsize=7)
    for i, y in enumerate(ys):
        ax.text(i, y * 1.15, f"{y:,.0f}" if y >= 100 else f"{y:.1f}", ha="center", fontsize=7)
    ax.grid(axis="y", alpha=0.3)
fig.suptitle("B677 M31 generic four-sum relation source, fully charged to target recovery (scalar 987654321 every run)", fontsize=10)
fig.tight_layout()
fig.savefig(out + ".svg"); fig.savefig(out + ".pdf"); fig.savefig(out + ".png", dpi=150)
