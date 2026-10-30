"""Lay out the recorded p=186379 diagnostic; no scalar or price arithmetic."""

import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

ROOT = Path(__file__).resolve().parent
RUN = ROOT / "reports/run-20261022T0522Z"
RUN.mkdir(parents=True, exist_ok=True)
raw = [json.loads(line) for line in (ROOT / "results/p186379-identity-t0.jsonl").read_text().splitlines()]
row = raw[1]

fig, (left, right) = plt.subplots(1, 2, figsize=(10.5, 4.4))
left.bar(["Stored coefficients", "Frozen cap"],
         [row["solver_stored_nonzero_coefficients"], 10_000_000],
         color=["#b76b25", "#255a86"])
left.set_ylabel("Sparse matrix coefficients")
left.grid(axis="y", alpha=0.25)
right.bar(["Pivot rank", "Factor-base nodes"],
          [row["solver_rank"], row["solver_nodes"]],
          color=["#b76b25", "#255a86"])
right.set_ylabel("Columns")
right.grid(axis="y", alpha=0.25)
fig.suptitle("p=186,379, frozen target 0: exact pencil collector stopped at matrix cap")
fig.text(0.5, 0.01,
         "run-20261022T0522Z · rho source scalar verified · candidate scalar absent · "
         "cost ratio null", ha="center", fontsize=8)
fig.tight_layout(rect=[0, 0.06, 1, 0.9])
fig.savefig(RUN / "matrix-cap.svg", bbox_inches="tight")
fig.savefig(RUN / "matrix-cap.png", dpi=160, bbox_inches="tight")

cover = plt.figure(figsize=(8.5, 11))
cover.text(0.08, 0.93, "Pencil upper-size diagnostic: p=186,379",
           fontsize=16, weight="bold")
lines = [
    "One original 32-bit public known-answer target; identity column order.",
    f"Candidate status: {row['failure']}; candidate recovered scalar: null.",
    f"Same-process rho source-verified: {row['rho']['verified']}.",
    f"Verified three-point relations: {row['verified_relations']:,}.",
    f"Pivot rank: {row['solver_rank']:,}; factor-base nodes: {row['solver_nodes']:,}.",
    f"Stored coefficients: {row['solver_stored_nonzero_coefficients']:,}.",
    f"Modular row updates: {row['solver_row_updates']:,}.",
    "Frozen matrix cap: 10,000,000 stored coefficients.",
    "Complete candidate cost and candidate/rho ratio: null.",
    "Rho counts and the unsuccessful candidate work remain in the raw JSONL.",
    "Wall time: L0 diagnostic; other independent workloads ran concurrently.",
]
for index, line in enumerate(lines):
    cover.text(0.08, 0.84 - index * 0.045, line, fontsize=9)
cover.text(0.08, 0.24, "Graph and raw JSONL identify the exact stop condition.", fontsize=9)
with PdfPages(RUN / "RUN_REPORT.pdf", metadata={"Title": "Pencil upper-size matrix cap"}) as pdf:
    pdf.savefig(cover)
    pdf.savefig(fig)
plt.close(cover)
plt.close(fig)
