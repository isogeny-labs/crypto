#!/usr/bin/env python3
"""Figures for research/ic_acct7_complete_counters_20261107. Plots recorded values only.

Reads coverage/items.tsv (per-item counts, whose `after` column is checked
against coverage/coverage-final.tsv by hand in README.md) and
breakdown-before.json / breakdown-after.json (written by
`examples/ic_acct6_archive.rs breakdown`). Computes no result.

    <venv>/bin/python research/ic_acct7_complete_counters_20261107/figures/plots.py
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
D = HERE.parent
GREY, LIGHT, BLUE, INK = "#b5b5b5", "#8fb8e8", "#2a78d6", "#333333"
plt.rcParams.update({"font.size": 9, "axes.spines.top": False, "axes.spines.right": False,
                     "axes.grid": True, "grid.color": "#e4e4e4", "grid.linewidth": 0.6,
                     "svg.fonttype": "none"})


def save(fig, name):
    fig.tight_layout()
    fig.savefig(HERE / f"{name}.svg", bbox_inches="tight")
    fig.savefig(HERE / f"{name}.png", dpi=150, bbox_inches="tight")
    plt.close(fig)


# 1. Configurations (1-90 of coverage/configs.jsonl) whose record still lists each item.
rows = [l.split("\t") for l in (D / "coverage" / "items.tsv").read_text().splitlines()
        if l and not l.startswith("#")]
head, body = rows[0], rows[1:]
assert head == ["item", "main", "intermediate", "after"]
items = [r for r in body if r[0] != "complete"]
fig, ax = plt.subplots(figsize=(6.8, 2.9))
cols = [("main", GREY, "main (shared_rank as item 6 recorded it)"),
        ("intermediate", LIGHT, "+ shared_rank instrumented"),
        ("after", BLUE, "+ Gaudry set-up markers (this branch)")]
h = 0.26
for j, (name, color, label) in enumerate(cols):
    k = head.index(name)
    for i, r in enumerate(items):
        y = len(items) - 1 - i + (1 - j) * h
        ax.barh(y, int(r[k]), height=h, color=color, label=label if i == 0 else None)
        ax.text(int(r[k]) + 0.08, y, r[k], va="center", fontsize=7, color=INK)
ax.set_yticks(range(len(items)))
ax.set_yticklabels([r[0] for r in reversed(items)], fontsize=8)
ax.set_xlabel("configurations whose record lists the item (of 90 diagnosed)")
ax.legend(loc="lower right", fontsize=7, frameon=False)
ax.set_title("Archive IC configurations still missing each counted item, on toy curves", fontsize=9)
save(fig, "items_by_config")

# 2. Archive totals: the committed records re-priced before and after.
b = json.loads((D / "breakdown-before.json").read_text())["totals"]
a = json.loads((D / "breakdown-after.json").read_text())["totals"]
cats = [("complete", "complete"), ("incomplete", "incomplete (lower bound)"), ("errors", "unpriceable")]
fig, ax = plt.subplots(figsize=(6.8, 2.3))
for i, (k, lab) in enumerate(cats):
    y = len(cats) - 1 - i
    ax.barh(y + 0.19, b[k], height=0.34, color=GREY, label="main rules" if i == 0 else None)
    ax.barh(y - 0.19, a[k], height=0.34, color=BLUE, label="this branch" if i == 0 else None)
    ax.text(max(b[k], a[k]) * 1.01 + 200, y, f"{b[k]:,} / {a[k]:,}", va="center", fontsize=7)
ax.set_yticks(range(len(cats)))
ax.set_yticklabels([c[1] for c in reversed(cats)])
ax.set_xlabel(f"verified non-warm-up records ({a['records']:,} in the archive)")
ax.set_xlim(0, max(a["complete"], b["complete"]) * 1.25)
ax.legend(loc="lower right", fontsize=7, frameon=False)
ax.set_title("Committed archive re-priced in bitop/v1: unchanged, as the rules predict", fontsize=9)
save(fig, "archive_totals")
