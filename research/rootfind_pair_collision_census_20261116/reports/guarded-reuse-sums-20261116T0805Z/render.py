"""Plot recorded operation counts from the original and guarded test stderr."""

from pathlib import Path
import re

import matplotlib.pyplot as plt


root = Path(__file__).parent
prior = root.parent / "integration-check-20261116T0734Z" / "stderr.txt"
current = root / "stderr.txt"
pattern = re.compile(
    r"harvest trials=(\d+) target_rows=(\d+) rank=(\d+) "
    r"setup_adds=(\d+) total_gae=(\d+) seed_rank=(\d+)"
)
values = []
for path in (prior, current):
    found = pattern.search(path.read_text())
    if found is None:
        raise SystemExit(f"missing harvest diagnostic in {path}")
    values.append(tuple(map(int, found.groups())))

fig, axes = plt.subplots(1, 2, figsize=(8, 3.4), layout="constrained")
for ax, title, unit, index in (
    (axes[0], "Setup group additions", "additions", 3),
    (axes[1], "Total modeled work", "GAE", 4),
):
    bars = ax.bar(
        ["Five-add check", "Guarded reuse"],
        [values[0][index], values[1][index]],
        color=["#526781", "#bb6e43"],
        width=0.62,
    )
    ax.bar_label(bars, padding=3)
    ax.set_title(title)
    ax.set_ylabel(unit)
    ax.set_ylim(0, max(values[0][index], values[1][index]) * 1.18)
    ax.spines[["top", "right"]].set_visible(False)
fig.suptitle("K13/d5, scalar 641, seed 811 — producer operation diagnostics")
fig.text(
    0.5,
    -0.06,
    "Sources: integration-check-20261116T0734Z and guarded-reuse-sums-20261116T0805Z stderr.txt; no arm timing",
    ha="center",
    fontsize=8,
)
fig.savefig(root / "guarded-reuse.svg", bbox_inches="tight")
svg = root / "guarded-reuse.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
