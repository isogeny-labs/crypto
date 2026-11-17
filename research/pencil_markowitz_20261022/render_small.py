"""Render native p739 paired prices and counters; no research arithmetic."""

import csv
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

ROOT = Path(__file__).resolve().parent
RUN = ROOT / "reports/run-20261022T0732Z"
RUN.mkdir(parents=True, exist_ok=True)
pair = json.loads((ROOT / "results/p739-markowitz-vs-peel.json").read_text())
summary = json.loads((ROOT / "results/summary.json").read_text())
slices = summary["slices"]
with (ROOT / "results/p739-markowitz-vs-peel.csv").open(newline="") as stream:
    grid = list(csv.DictReader(stream))

fig, (left, right) = plt.subplots(1, 2, figsize=(11.3, 4.7))
for inversion, color in [("1", "#557f74"), ("10", "#245a88"),
                         ("100", "#b6752a"), ("1000", "#8c5277")]:
    cells = [next(row for row in grid if row["S_per_M"] == "0.5"
                  and row["I_per_M"] == inversion
                  and row["native_GAE_per_logical"] == native)
             for native in ("0", "0.1", "1")]
    left.plot([0, 1, 2], [float(row["candidate_over_baseline"]) for row in cells],
              marker="o", color=color, label=f"I/M = {inversion}")
left.axhline(1, color="#222222", linewidth=0.8)
left.set_xticks([0, 1, 2], ["0", "0.1", "1"])
left.set_xlabel("GAE per logical operation")
left.set_ylabel("Markowitz / peel cold cost")
left.grid(alpha=0.25)
left.legend(fontsize=8)

names = ["Peel", "Markowitz"]
updates = [row["solver_row_updates"] for row in slices]
index_ops = [row["solver_index_ops"] for row in slices]
positions = [0, 1]
right.bar([x - 0.18 for x in positions], updates, width=0.36,
          color="#b6752a", label="Modular row updates")
right.bar([x + 0.18 for x in positions], index_ops, width=0.36,
          color="#245a88", label="Logical index operations")
right.set_xticks(positions, names)
right.set_yscale("log")
right.set_ylabel("Recorded operations, log scale")
right.grid(axis="y", alpha=0.25)
right.legend(fontsize=8)
fig.suptitle("p=739: fixed exact pivot rules on the same 60 source targets")
fig.tight_layout(rect=[0, 0, 1, 0.93])
fig.savefig(RUN / "p739-markowitz-paired.svg", bbox_inches="tight")
fig.savefig(RUN / "p739-markowitz-paired.png", dpi=170, bbox_inches="tight")

cover = plt.figure(figsize=(8.5, 11))
cover.text(0.08, 0.93, "Exact Markowitz pivot priority: p=739 gate",
           fontsize=15, weight="bold")
cover.text(0.08, 0.89, "60 frozen targets · source and rho scalars verified · fresh replay",
           fontsize=10)
table_axis = cover.add_axes([0.07, 0.66, 0.86, 0.17])
table_axis.axis("off")
table = table_axis.table(
    cellText=[
        ["Peel", f"{updates[0]:,}", f"{index_ops[0]:,}",
         f"{pair['representative']['baseline_over_rho']:.6f}"],
        ["Markowitz", f"{updates[1]:,}", f"{index_ops[1]:,}",
         f"{pair['representative']['candidate_over_rho']:.6f}"],
    ],
    colLabels=["solver", "row updates", "index ops", "cost / rho"],
    loc="upper center", cellLoc="left", colLoc="left")
table.auto_set_font_size(False)
table.set_fontsize(9)
table.scale(1.0, 1.6)
lines = [
    f"Representative Markowitz/peel cold cost: {pair['representative']['candidate_over_baseline']:.6f}.",
    f"Across 36 pricing cells: {pair['cells_candidate_below_baseline']} below peel; "
    f"{pair['cells_candidate_below_rho']} below matched source rho.",
    f"Markowitz/peel range: {pair['min_candidate_over_baseline']:.6f}–"
    f"{pair['max_candidate_over_baseline']:.6f}.",
    "The full CSV retains all cells, including equality and higher prices.",
    "The graph shows the S/M=0.5 slice for all four inversion prices.",
    "The p=186,379 target-0 resource diagnostic is recorded separately.",
]
for index, line in enumerate(lines):
    cover.text(0.08, 0.57 - index * 0.055, line, fontsize=9)
cover.text(0.08, 0.13,
           "Native raw JSONL, paired grid, replay, source checks, and frozen protocol accompany this PDF.",
           fontsize=8.2)
with PdfPages(RUN / "RUN_REPORT.pdf",
             metadata={"Title": "Exact Markowitz pivot p739 paired gate"}) as pdf:
    pdf.savefig(cover)
    pdf.savefig(fig)
plt.close(cover)
plt.close(fig)
(RUN / "README.md").write_text(
    "# p=739 paired exact-pivot gate\n\n"
    "The PDF and graph lay out the native 60-target Markowitz/peel comparison. "
    "All 36 paired pricing cells and raw/replay source checks are retained in "
    "the parent study results. Wall time is L0 diagnostic.\n"
)
