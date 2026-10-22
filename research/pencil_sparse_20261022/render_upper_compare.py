"""Lay out the two native 32-bit cap records without recalculating results."""

import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

ROOT = Path(__file__).resolve().parent
RUN = ROOT / "reports/run-20261022T0545Z"
RUN.mkdir(parents=True, exist_ok=True)
names = ["Online", "Degree peel"]
paths = [ROOT / "results/p186379-identity-t0.jsonl",
         ROOT / "results/p186379-peel-t0.jsonl"]
rows = [[json.loads(line) for line in path.read_text().splitlines()][1]
        for path in paths]
assert all(row["failure"] == "sparse_matrix_cap" and row["rho"]["verified"]
           and row["recovered"] is None for row in rows)

fig, axes = plt.subplots(1, 2, figsize=(10.6, 4.6))
axes[0].bar(names, [row["solver_stored_nonzero_coefficients"] for row in rows],
            color=["#b76b25", "#255a86"])
axes[0].axhline(10_000_000, color="#333333", linewidth=1, linestyle="--",
                label="Frozen cap")
axes[0].set_ylabel("Peak stored nonzero coefficients")
axes[0].legend(fontsize=8)
axes[1].bar(names, [row["solver_row_updates"] for row in rows],
            color=["#b76b25", "#255a86"])
axes[1].set_yscale("log")
axes[1].set_ylabel("Charged modular row updates, log scale")
for axis in axes:
    axis.grid(axis="y", alpha=0.25)
fig.suptitle("p=186,379, frozen target 0: two exact pencil solvers reached the cap")
fig.text(0.5, 0.015, "Same-process rho scalar checked in both runs; candidate scalars and cost ratios null",
         ha="center", fontsize=8)
fig.tight_layout(rect=[0, 0.055, 1, 0.91])
fig.savefig(RUN / "upper-two-caps.svg", bbox_inches="tight")
fig.savefig(RUN / "upper-two-caps.png", dpi=160, bbox_inches="tight")

cover = plt.figure(figsize=(8.5, 11))
cover.text(0.08, 0.93, "Pencil upper-size diagnostic: two exact solvers", fontsize=15,
           weight="bold")
cover.text(0.08, 0.89, "p=186,379 · frozen target 0 · 10,000,000-coefficient cap", fontsize=10)
table_rows = [[name, str(row["solver_rank"]),
               f"{row['solver_stored_nonzero_coefficients']:,}",
               f"{row['solver_row_updates']:,}",
               f"{row.get('solver_index_ops', 0):,}"]
              for name, row in zip(names, rows)]
table_axis = cover.add_axes([0.07, 0.61, 0.86, 0.21])
table_axis.axis("off")
table = table_axis.table(cellText=table_rows,
                         colLabels=["solver", "rank", "peak coeff.", "row updates", "index ops"],
                         loc="upper center", cellLoc="left", colLoc="left")
table.auto_set_font_size(False)
table.set_fontsize(8)
table.scale(1.0, 1.55)
lines = [
    "Each candidate stopped at the preregistered matrix cap.",
    "Both same-process rho controls recovered and verified the source scalar.",
    "Candidate scalar: null; candidate/rho cost ratio: null in both records.",
    "The row-update reduction is a stage count for unsuccessful attempts.",
    "The peel attempt also incurs the recorded logical index work.",
    "These two records do not represent a solved 32-bit curve slice.",
    "Wall time has L0 isolation because other jobs shared the host.",
]
for index, line in enumerate(lines):
    cover.text(0.08, 0.53 - index * 0.045, line, fontsize=9)
cover.text(0.08, 0.12, "Raw JSONL, frozen protocol, replay and price grid accompany this PDF.",
           fontsize=9)
with PdfPages(RUN / "RUN_REPORT.pdf",
             metadata={"Title": "Two exact pencil upper-size cap records"}) as pdf:
    pdf.savefig(cover)
    pdf.savefig(fig)
plt.close(cover)
plt.close(fig)
