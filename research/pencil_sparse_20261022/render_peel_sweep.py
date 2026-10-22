"""Lay out native nine-curve peel comparison records as a graph and PDF."""

import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

ROOT = Path(__file__).resolve().parent
RUN = ROOT / "reports/run-20261022T0534Z"
RUN.mkdir(parents=True, exist_ok=True)
PS = [739, 1151, 1187, 3371, 3527, 3931, 11731, 11887, 12143]
rows = [json.loads((ROOT / f"results/p{p}-peel-vs-online.json").read_text()) for p in PS]

fig, (left, right) = plt.subplots(1, 2, figsize=(11.5, 4.8))
positions = list(range(len(PS)))
left.plot(positions,
          [row["representative"]["candidate_over_baseline"] for row in rows],
          "o-", color="#255a86", label="M+0.5S+10I, 0.1 logical")
for position, row in zip(positions, rows):
    left.vlines(position, row["min_candidate_over_baseline"],
                row["max_candidate_over_baseline"], color="#b76b25", linewidth=2)
left.plot([], [], color="#b76b25", label="36-cell observed range")
left.axhline(1, color="#222222", linewidth=0.8)
left.set_ylabel("Peel cold cost / online cold cost")
left.legend(fontsize=8)
right.plot(positions,
           [row["representative"]["candidate_over_rho"] for row in rows],
           "s-", color="#64884d")
right.axhline(1, color="#222222", linewidth=0.8)
right.set_yscale("log")
right.set_ylabel("Peel cold cost / source rho")
for axis in [left, right]:
    axis.set_xticks(positions, [f"{p:,}" for p in PS], rotation=45)
    axis.set_xlabel("Base field p; 60 original targets per curve")
    axis.grid(alpha=0.25)
fig.suptitle("Exact degree-peeling pencil collector: nine paired curves")
fig.text(0.5, 0.015,
         "run-20261022T0534Z · p=739 prior gate shown for context · full 36-cell "
         "band retained · wall time excluded", ha="center", fontsize=8)
fig.tight_layout(rect=[0, 0.05, 1, 0.92])
fig.savefig(RUN / "peel-nine-curves.svg", bbox_inches="tight")
fig.savefig(RUN / "peel-nine-curves.png", dpi=160, bbox_inches="tight")

cover = plt.figure(figsize=(8.5, 11))
cover.text(0.07, 0.94, "Exact degree-peeling pencil: nine paired curves",
           fontsize=15, weight="bold")
cover.text(0.07, 0.90, "run-20261022T0534Z | p=739 prior gate; eight new curves",
           fontsize=9)
cover.text(0.07, 0.86, "Every row: 60 source-verified candidate/rho solves and all 36 prices.",
           fontsize=9)
table_rows = [[str(row["p"]),
               f"{row['representative']['candidate_over_baseline']:.6f}",
               f"{row['representative']['candidate_over_rho']:.6f}",
               f"{row['cells_candidate_below_baseline']}/36",
               f"{row['cells_candidate_below_rho']}/36"] for row in rows]
table = cover.add_axes([0.07, 0.37, 0.86, 0.44])
table.axis("off")
rendered = table.table(cellText=table_rows,
                       colLabels=["p", "peel/online", "peel/rho", "below online", "below rho"],
                       loc="upper center", cellLoc="left", colLoc="left")
rendered.auto_set_font_size(False)
rendered.set_fontsize(8)
rendered.scale(1.0, 1.45)
cover.text(0.07, 0.31, "All values are complete cold operation prices, not wall-time ratios.",
           fontsize=9)
cover.text(0.07, 0.27, "Each field size retains both lower and higher pricing cells.", fontsize=9)
cover.text(0.07, 0.23, "The 28- and 32-bit pencil targets are separate, partial runs.", fontsize=9)
cover.text(0.07, 0.17, "Source: per-target JSONL, 36-cell price grid and native validator.",
           fontsize=9)
with PdfPages(RUN / "RUN_REPORT.pdf", metadata={"Title": "Nine-curve pencil degree-peeling comparison"}) as pdf:
    pdf.savefig(cover)
    pdf.savefig(fig)
plt.close(cover)
plt.close(fig)
