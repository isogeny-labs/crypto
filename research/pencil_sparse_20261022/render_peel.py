"""Lay out the native paired peel/online price grid as a graph and PDF."""

import csv
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

ROOT = Path(__file__).resolve().parent
RUN = ROOT / "reports/run-20261022T0529Z"
RUN.mkdir(parents=True, exist_ok=True)
paired = json.loads((ROOT / "results/p739-peel-vs-online.json").read_text())
with (ROOT / "results/p739-peel-vs-online.csv").open(newline="") as handle:
    prices = list(csv.DictReader(handle))

fig, axes = plt.subplots(1, 3, figsize=(12.6, 4.6), sharey=True)
palette = {0.0: "#255a86", 0.5: "#b76b25", 1.0: "#64884d"}
for ax, native in zip(axes, [0.0, 0.1, 1.0]):
    for square in [0.0, 0.5, 1.0]:
        rows = [row for row in prices
                if float(row["native_GAE_per_logical"]) == native
                and float(row["S_per_M"]) == square]
        ax.plot([float(row["I_per_M"]) for row in rows],
                [float(row["candidate_over_baseline"]) for row in rows],
                "o-", color=palette[square], label=f"S/M={square}")
    ax.axhline(1, color="#222222", linewidth=0.8)
    ax.set_xscale("log")
    ax.set_xticks([1, 10, 100, 1000], ["1", "10", "100", "1000"])
    ax.set_xlabel("I/M")
    ax.set_title(f"Native price: {native} GAE/op")
    ax.grid(alpha=0.25)
axes[0].set_ylabel("Peel cold cost / online cold cost")
axes[2].legend(loc="upper right", fontsize=8)
fig.suptitle("p=739, 60 paired public targets: exact degree peeling vs online elimination")
fig.text(0.5, 0.015,
         "run-20261022T0529Z · 36 native-priced cells · source-verified candidate, "
         "baseline and rho · wall time excluded", ha="center", fontsize=8)
fig.tight_layout(rect=[0, 0.05, 1, 0.92])
fig.savefig(RUN / "peel-price-grid.svg", bbox_inches="tight")
fig.savefig(RUN / "peel-price-grid.png", dpi=160, bbox_inches="tight")

representative = paired["representative"]
cover = plt.figure(figsize=(8.5, 11))
cover.text(0.08, 0.93, "Exact degree-peeling pencil collector", fontsize=16, weight="bold")
cover.text(0.08, 0.89, "run-20261022T0529Z | p=739 | 60 paired targets", fontsize=10)
lines = [
    "All 60 candidate, online-control and rho scalars source verified.",
    "A fresh archive-input rerun matched all non-wall fields.",
    "Full cold pricing: M + 0.5S + 10I; 0.1 GAE/native operation.",
    f"Peel candidate: {representative['candidate_GAE']:,.2f} GAE.",
    f"Online control: {representative['baseline_GAE']:,.2f} GAE.",
    f"Same-process rho: {representative['rho_GAE']:,.2f} GAE.",
    f"Peel / online: {representative['candidate_over_baseline']:.6f}.",
    f"Peel / rho: {representative['candidate_over_rho']:.6f}.",
    f"Cells with peel below online: {paired['cells_candidate_below_baseline']}/36.",
    f"Cells with peel below rho: {paired['cells_candidate_below_rho']}/36.",
    "All 36 cells and every one-target cost are in the native records.",
    "The larger frozen target is a separate pending run.",
]
for index, line in enumerate(lines):
    cover.text(0.08, 0.83 - index * 0.043, line, fontsize=9)
cover.text(0.08, 0.22,
           "The graph shows measured price ratios; it does not extrapolate to other curves.",
           fontsize=9)
with PdfPages(RUN / "RUN_REPORT.pdf", metadata={"Title": "Pencil degree-peeling paired gate"}) as pdf:
    pdf.savefig(cover)
    pdf.savefig(fig)
plt.close(cover)
plt.close(fig)
