"""Plot only the four recorded numeric diagnostics in stderr.txt."""

from pathlib import Path
import re

import matplotlib.pyplot as plt


root = Path(__file__).parent
raw = (root / "stderr.txt").read_text()
match = re.search(
    r"old trials=(\d+) target_rows=(\d+) rank=(\d+) setup_adds=(\d+) total_gae=(\d+); "
    r"harvest trials=(\d+) target_rows=(\d+) rank=(\d+) setup_adds=(\d+) total_gae=(\d+) seed_rank=(\d+)",
    raw,
)
if match is None:
    raise SystemExit("missing recorded diagnostic line")
old_trials, old_rows, _, old_adds, old_gae, new_trials, new_rows, _, new_adds, new_gae, _ = map(
    int, match.groups()
)

fig, axes = plt.subplots(2, 2, figsize=(8.5, 5.4), layout="constrained")
panels = [
    ("Target trials", "count", old_trials, new_trials),
    ("Target-bearing relations", "count", old_rows, new_rows),
    ("Setup group additions", "additions", old_adds, new_adds),
    ("Total modeled work", "GAE", old_gae, new_gae),
]
for ax, (title, unit, old, new) in zip(axes.flat, panels):
    bars = ax.bar(
        ["Existing", "Harvest"],
        [old, new],
        color=["#526781", "#bb6e43"],
        width=0.6,
    )
    ax.bar_label(bars, padding=3)
    ax.set_title(title)
    ax.set_ylabel(unit)
    ax.set_ylim(0, max(old, new) * 1.22)
    ax.spines[["top", "right"]].set_visible(False)
fig.suptitle("K13/d5, scalar 641, seed 811 — one local correctness fixture", fontsize=12)
fig.text(
    0.5,
    -0.025,
    "Source: integration-check-20261116T0734Z/stderr.txt; diagnostic only, no arm timing",
    ha="center",
    fontsize=8,
)
for suffix in ("svg", "png"):
    fig.savefig(root / f"integration-diagnostics.{suffix}", dpi=180, bbox_inches="tight")
svg = root / "integration-diagnostics.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
