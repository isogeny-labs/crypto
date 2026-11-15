#!/usr/bin/env python3
"""Figure for EXP-PKM-DENSE-DEC-001 from results.json."""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(HERE, "results.json")))
cells = [c for c in R["cells"] if c["p"] == 1399680001]
fig, axes = plt.subplots(1, 2, figsize=(9, 3.7))
for ax, m in zip(axes, (2, 3)):
    cs = [c for c in cells if c["m"] == m]
    def ser(key, vk):
        pts = sorted((c["S"], c[key]) for c in cs if c["vkind"] == vk and c.get(key))
        return [s for s, _ in pts], [v for _, v in pts]
    for key, vk, lab, sty, col in (("tower_planted_ops_median", "coset", "tower oracle", "o-", "#3b6ea8"),
                                   ("dense_planted_ops_median", "coset", "dense oracle, coset V", "^:", "#7a9e4b"),
                                   ("dense_planted_ops_median", "random", "dense oracle, random V", "s--", "#c8553d"),
                                   ("enum_ops", "coset", "enumeration over V", "d-", "#555555")):
        x, y = ser(key, vk)
        ax.plot(x, y, sty, color=col, label=lab)
    ax.set_xscale("log", base=2)
    ax.set_yscale("log")
    ax.set_xlabel("|V| = S")
    ax.set_ylabel("field_ops to find all relations (planted, median)")
    ax.set_title(f"m = {m}")
    ax.grid(alpha=0.3, which="both")
axes[0].legend(fontsize=7)
fig.suptitle("Finding decompositions: dense membership beats the tower; enumeration beats both", fontsize=9)
for ext in ("svg", "png"):
    fig.savefig(os.path.join(HERE, "figures", f"find_cost.{ext}"), dpi=160, bbox_inches="tight")
print("ok")
