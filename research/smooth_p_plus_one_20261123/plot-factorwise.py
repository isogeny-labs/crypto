#!/usr/bin/env python3
"""Plot recorded audit values only; no research computation or solver calls."""
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
rows = json.loads((root / "evidence/factorwise_v1/summary.json").read_text())
plt.rcParams.update({"font.size": 10, "svg.fonttype": "none"})
fig, ax = plt.subplots(figsize=(9, 3.2), constrained_layout=True)
positions = list(range(len(rows)))
old = [100 * r["old_rank_two"] / r["rr5_bindings_checked"] for r in rows]
ax.bar([x - .17 for x in positions], old, width=.32, color="#ad6443", label="Original pushforward")
ax.scatter([x + .17 for x in positions], [0] * len(rows), s=55, marker="s", color="#217e6b", label="Factorwise divisibility", zorder=3)
for x, r, value in zip(positions, rows, old):
    ax.text(x - .17, value + .22, f'{r["old_rank_two"]:,}/{r["rr5_bindings_checked"]:,}', ha="center", va="bottom")
    ax.text(x + .17, .22, f'0/{r["rr5_bindings_checked"]:,}', ha="center", va="bottom", color="#18614f")
ax.set_xticks(positions, [f'F{r["prime"]}\ndegree {2 ** r["depth"]}' for r in rows])
ax.set_ylim(-.45, 8.7)
ax.set_ylabel("Nonzero tangent dimension (%)")
ax.set_title("The repeated-image rank defect is absent on the audited open chart")
ax.legend(loc="upper left", frameon=False)
ax.spines[["top", "right"]].set_visible(False)
fig.savefig(root / "factorwise-ranks.svg")
svg = root / "factorwise-ranks.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
fig.savefig(root / "factorwise-ranks.pdf")
