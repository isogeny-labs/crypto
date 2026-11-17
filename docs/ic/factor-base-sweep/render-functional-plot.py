#!/usr/bin/env python3
"""Render recorded native witness counts only. No cryptographic computation."""
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.ticker import MaxNLocator

root = Path(__file__).resolve().parent
data = json.loads((root / "functional-fixture-observations.json").read_text())
plt.rcParams.update({"font.size": 10})
fig, axes = plt.subplots(2, 1, figsize=(8.2, 9.3), gridspec_kw={"height_ratios": [3.3, 1.2]})
for ax, family in zip(axes, ["binary", "prime"]):
    cases = [row for row in data["candidates"] if row["panel"] == family]
    labels = []
    for pos, row in enumerate(cases):
        labels.append(row["label"])
        if row["outcome"] == "completed":
            ax.barh(pos - 0.16, row["train_verified"], height=0.30, color="#315f9a", label="Training witnesses" if pos == 0 else None)
            ax.barh(pos + 0.16, row["holdout_verified"], height=0.30, color="#d88a27", label="Holdout witnesses" if pos == 0 else None)
            ax.text(row["train_verified"] + 0.04, pos - 0.16, str(row["train_verified"]), va="center", fontsize=8)
            ax.text(row["holdout_verified"] + 0.04, pos + 0.16, str(row["holdout_verified"]), va="center", fontsize=8)
        else:
            ax.text(0.06, pos, row["outcome"], va="center", color="#666666", fontsize=9)
    ax.set_yticks(range(len(cases)), labels)
    ax.set_ylim(len(cases) - 0.5, -0.5)
    ax.set_xlim(0, data["train_requested"] + 0.4)
    ax.xaxis.set_major_locator(MaxNLocator(integer=True))
    ax.set_xlabel("Verified point witnesses (count)")
    ax.set_title(data["panel_titles"][family], fontsize=10, wrap=True)
    ax.grid(axis="x", alpha=0.2)
    ax.set_axisbelow(True)
    for spine in ["top", "right"]:
        ax.spines[spine].set_visible(False)
axes[0].legend(loc="lower right", fontsize=8)
fig.suptitle("CLI functional fixtures: verified witnesses", fontsize=13)
fig.text(0.02, 0.015,
         f'Run {data["run_id"]}; per completed candidate: {data["train_requested"]} training and {data["holdout_requested"]} holdout probes.\n'
         'Counts copied from native records; no uncertainty interval estimated. Missing/inapplicable results are not zero-yield observations.\n'
         'Base sizes and parameterizations differ. This is a functional check, not a speed comparison or certified regularity result.',
         fontsize=8)
fig.tight_layout(rect=(0, 0.105, 1, 0.95))
fig.savefig(root / "functional-witnesses.svg")
fig.savefig(root / "functional-witnesses.png", dpi=150)
svg = root / "functional-witnesses.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
