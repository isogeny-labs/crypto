#!/usr/bin/env python3
"""Figures for EXP-PKM-IDEAS-001 from results.json (no other input)."""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(HERE, "results.json")))
FIG = os.path.join(HERE, "figures")
os.makedirs(FIG, exist_ok=True)
C = {"tower": "#3b6ea8", "random": "#c8553d", "coset": "#7a9e4b", "fresh": "#3b6ea8", "replay": "#c8553d"}


def save(fig, name):
    for ext in ("svg", "png"):
        fig.savefig(os.path.join(FIG, f"{name}.{ext}"), dpi=160, bbox_inches="tight")
    plt.close(fig)


# idea 1: work vs S, tower (EXP-PKM-ODD-001) vs dense random / coset
fig, axes = plt.subplots(1, 2, figsize=(9, 3.6))
for ax, m in zip(axes, (2, 3)):
    cells = [c for c in R["idea1"]["cells"] if c["m"] == m and c["field_ops_max"]]
    tow = sorted({(c["S"], c["tower_field_ops"]) for c in cells if c["tower_field_ops"]})
    ax.plot([s for s, _ in tow], [f for _, f in tow], "o-", color=C["tower"], label="degree-2 tower (ODD-001)")
    for v in ("random", "coset"):
        pts = sorted((c["S"], c["field_ops_max"]) for c in cells if c["vkind"] == v)
        lab = "dense ∏(x−v), random V" if v == "random" else "dense, Kummer coset gμ_S"
        ax.plot([s for s, _ in pts], [f for _, f in pts], "s--" if v == "random" else "^:", color=C[v], label=lab)
    ax.set_xscale("log", base=2)
    ax.set_yscale("log")
    ax.set_xlabel("|V| = S")
    ax.set_ylabel("F4 field_ops (refutation system)")
    ax.set_title(f"m = {m}")
    ax.grid(alpha=0.3, which="both")
axes[0].legend(fontsize=7)
fig.suptitle("Idea 1: the smooth tower is not cheaper to solve; one dense polynomial is far cheaper", fontsize=9)
save(fig, "idea1_dense_vs_tower")

# idea 3/4: replay / fresh ratio
cells = R["idea3"]["cells"] + R["idea4"]["cells"]
fig, ax = plt.subplots(figsize=(8, 3.4))
names = [c["cell"].replace("replay-", "") for c in cells]
ratios = [c["ratio_max"] for c in cells]
cols = ["#3b6ea8" if c["cell"].startswith("replay") else "#7a9e4b" for c in cells]
ax.bar(range(len(cells)), ratios, color=cols)
ax.axhline(0.5, color="k", lw=0.8, ls="--")
ax.text(len(cells) - 0.5, 0.52, "frozen threshold 0.5", ha="right", fontsize=7)
ax.set_xticks(range(len(cells)))
ax.set_xticklabels(names, rotation=60, ha="right", fontsize=7)
ax.set_ylabel("replay ÷ fresh field_ops")
ax.set_ylim(0, 1)
ax.set_title("Ideas 3 and 4: trace replay (blue: same curve and prime; green: other curve / other prime)", fontsize=9)
save(fig, "idea3_replay_ratio")

# idea 5b: coverage
b = R["idea5"]["b"]
fig, ax = plt.subplots(figsize=(6, 3.4))
xs = range(len(b))
w = 0.2
for i, (k, lab, col) in enumerate((("covered_shared", "shared coset", "#3b6ea8"),
                                    ("covered_per_point", "per-point cosets", "#c8553d"),
                                    ("covered_random_shared", "random, shared", "#9aa5b1"),
                                    ("covered_random_pair", "random, per point", "#7a9e4b"))):
    ax.bar([x + (i - 1.5) * w for x in xs], [c[k] for c in b], w, label=lab, color=col)
ax.set_xticks(list(xs))
ax.set_xticklabels([f"S = {c['S']}" for c in b])
ax.set_yscale("log")
ax.set_ylabel("|{P1 + P2}| (m = 2, exact)")
ax.legend(fontsize=7)
ax.set_title("Idea 5b: coverage at p = 1001089", fontsize=9)
save(fig, "idea5_coverage")
print("figures written")
