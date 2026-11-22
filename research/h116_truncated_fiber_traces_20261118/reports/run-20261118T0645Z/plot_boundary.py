#!/usr/bin/env python3
"""Render the retained Frobenius-boundary coefficient data."""

import csv
from pathlib import Path

import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
with (HERE / "boundary_data.csv").open(newline="", encoding="utf-8") as handle:
    rows = list(csv.DictReader(handle))

indices = [int(row["index"]) for row in rows]
phi = [int(row["phi"]) for row in rows]
scalar = [int(row["scalar"]) for row in rows]
delta = [int(row["delta_mod_31"]) for row in rows]

figure, (top, bottom) = plt.subplots(
    2, 1, figsize=(8.4, 5.8), sharex=True,
    gridspec_kw={"height_ratios": [2.1, 1.0]},
)
top.plot(indices, phi, "o-", label="retained normalized V-1 map", linewidth=1.5)
top.plot(indices, scalar, "s--", label="independent scalar [-6] map", linewidth=1.2)
top.axvline(15, color="#dc2626", linestyle=":", label="preregistered boundary 15")
top.set_ylabel("coefficient in F31")
top.set_title("H116-FROBENIUS-BOUNDARY-001 · frozen p=31 cell")
top.grid(alpha=0.25)
top.legend(fontsize=8, ncol=2)

colors = ["#dc2626" if value else "#94a3b8" for value in delta]
bottom.bar(indices, delta, color=colors, width=0.72)
bottom.axvline(15, color="#dc2626", linestyle=":")
bottom.annotate("first delta = 10", xy=(15, 10), xytext=(11.3, 16),
                arrowprops={"arrowstyle": "->", "color": "#991b1b"},
                color="#991b1b", fontsize=9)
bottom.set_xlabel("normalized w-coefficient index")
bottom.set_ylabel("V-1 − scalar\n(mod 31)")
bottom.set_xticks(indices)
bottom.set_ylim(0, 25)
bottom.grid(axis="y", alpha=0.25)

figure.tight_layout()
figure.savefig(HERE / "boundary.svg")
# The SVG is the resolution-independent archival figure. Keep the PNG below
# the repository's 64 KiB inline-artifact threshold for PR rendering.
figure.savefig(HERE / "boundary.png", dpi=80)

