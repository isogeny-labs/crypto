"""Lay out native pencil-sparse records as a graph and PDF; no research arithmetic."""

import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

ROOT = Path(__file__).resolve().parent
RUN = ROOT / "reports/run-20261022T0516Z"
RUN.mkdir(parents=True, exist_ok=True)
summary = json.loads((ROOT / "results/small-summary.json").read_text())
validation = json.loads((ROOT / "results/small-validation.json").read_text())
rows = sorted(summary["slices"], key=lambda row: (row["p"], row["order"]))
labels = [f"p={row['p']}\n{row['order']}\nn={row['targets']}" for row in rows]
colors = {"identity": "#255a86", "reverse": "#b76b25", "affine": "#64884d"}
bars = [colors[row["order"]] for row in rows]

fig, (left, right) = plt.subplots(2, 1, figsize=(12.6, 8.1))
left.bar(range(len(rows)), [row["solver_row_updates"] for row in rows], color=bars)
left.set_yscale("log")
left.set_ylabel("Modular row updates, total for slice")
left.grid(axis="y", alpha=0.25)
right.bar(range(len(rows)),
          [row["representative"]["pencil_over_rho"] for row in rows], color=bars)
right.set_yscale("log")
right.set_ylabel("Cold pencil cost / matched source rho")
right.grid(axis="y", alpha=0.25)
right.axhline(1, color="#222222", linewidth=0.8)
right.set_xticks(range(len(rows)), labels, fontsize=8)
fig.suptitle("Exact sparse pencil pivot order: matched target slices")
fig.text(0.5, 0.01,
         "run-20261022T0516Z · M+0.5S+10I · 0.1 GAE/native logical op · "
         f"{validation['source_scalars_checked']} independent source-scalar checks · "
         "wall time excluded", ha="center", fontsize=8)
fig.tight_layout(rect=[0, 0.04, 1, 0.95])
fig.savefig(RUN / "matrix-order.svg", bbox_inches="tight")
fig.savefig(RUN / "matrix-order.png", dpi=160, bbox_inches="tight")

cover = plt.figure(figsize=(8.5, 11))
cover.text(0.07, 0.94, "Point-driven pencil: exact pivot-order continuation",
           fontsize=16, weight="bold")
cover.text(0.07, 0.90, "run-20261022T0516Z | source-verified matched targets",
           fontsize=10)
cover.text(0.07, 0.85, "Observed values from native price-sparse and validate-sparse records.",
           fontsize=9)
cover.text(0.07, 0.82,
           f"Target rows: {summary['target_rows']}; verified candidate/rho pairs: "
           f"{summary['source_verified_pairs']}; validated price cells: "
           f"{validation['price_cells_checked']}.", fontsize=9)
cover.text(0.07, 0.79,
           f"Non-wall replay rows: {validation['replay_rows_checked']}; "
           f"missing replays: {len(validation['missing_replays'])}.", fontsize=9)
cover.text(0.07, 0.75,
           "Cold cost includes setup and variable-order logical work. "
           "Wall times are L0 diagnostics.", fontsize=9)
table_rows = [[str(row["p"]), row["order"], str(row["targets"]),
               str(row["verified"]), f"{row['solver_row_updates']:,}",
               f"{row['representative']['pencil_over_rho']:.6f}"] for row in rows]
table = cover.add_axes([0.07, 0.28, 0.86, 0.44])
table.axis("off")
rendered = table.table(cellText=table_rows,
                       colLabels=["p", "order", "targets", "verified", "row updates", "pencil/rho"],
                       loc="upper center", cellLoc="left", colLoc="left")
rendered.auto_set_font_size(False)
rendered.set_fontsize(8)
rendered.scale(1.0, 1.45)
cover.text(0.07, 0.21,
           "Full requested pencil cohort: 1,260 distinct targets. "
           "This report identifies partial coverage explicitly.", fontsize=9)
cover.text(0.07, 0.18,
           "Raw JSONL, all 36-cell prices per target, manifest and native validator "
           "sit beside this report.", fontsize=9)
cover.text(0.07, 0.12,
           "No inference about unrun targets or upper field sizes is made from these slices.",
           fontsize=9)
with PdfPages(RUN / "RUN_REPORT.pdf", metadata={"Title": "Point-driven pencil pivot-order continuation"}) as pdf:
    pdf.savefig(cover)
    pdf.savefig(fig)
plt.close(cover)
plt.close(fig)
