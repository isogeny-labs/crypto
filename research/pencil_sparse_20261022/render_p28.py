"""Render the native first-target 28-bit pencil price records."""

import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

ROOT = Path(__file__).resolve().parent
RUN = ROOT / "reports/run-20261022T0550Z"
RUN.mkdir(parents=True, exist_ok=True)
summary = json.loads((ROOT / "results/summary.json").read_text())
slices = summary["slices"]
selected = [("48,383 I", 48383, "identity", "online"),
            ("48,383 R", 48383, "reverse", "online"),
            ("48,383 A", 48383, "affine", "online"),
            ("48,383 P", 48383, "identity", "peel"),
            ("48,823 I", 48823, "identity", "online"),
            ("49,451 I", 49451, "identity", "online")]
rows = []
for label, p, order, mode in selected:
    matches = [row for row in slices if row["p"] == p and row["order"] == order
               and row["solver_mode"] == mode]
    assert len(matches) == 1 and matches[0]["targets"] == 1
    rows.append((label, matches[0]))
paired = json.loads((ROOT / "results/p48383-peel-vs-online.json").read_text())

fig, (left, right) = plt.subplots(1, 2, figsize=(11.5, 4.8))
left.bar(range(len(rows)), [row["representative"]["pencil_over_rho"] for _, row in rows],
         color=["#255a86", "#64884d", "#b76b25", "#884c78", "#255a86", "#255a86"])
left.set_yscale("log")
left.axhline(1, color="#222222", linewidth=0.8)
left.set_ylabel("Complete cold pencil cost / source rho")
left.set_xticks(range(len(rows)), [label for label, _ in rows], rotation=50)
left.grid(axis="y", alpha=0.25)
right.bar(["Identity", "Reverse", "Affine", "Peel"],
          [1.0,
           json.loads((ROOT / "results/p48383-reverse-vs-identity.json").read_text())["representative"]["candidate_over_baseline"],
           json.loads((ROOT / "results/p48383-affine-vs-identity.json").read_text())["representative"]["candidate_over_baseline"],
           paired["representative"]["candidate_over_baseline"]],
          color=["#255a86", "#64884d", "#b76b25", "#884c78"])
right.axhline(1, color="#222222", linewidth=0.8)
right.set_ylabel("p=48,383 cold cost / identity online cost")
right.grid(axis="y", alpha=0.25)
fig.suptitle("28-bit pencil first-target records: complete operation prices")
fig.text(0.5, 0.015, "I/R/A are fixed column orders; P is degree peel · one frozen target per curve",
         ha="center", fontsize=8)
fig.tight_layout(rect=[0, 0.045, 1, 0.92])
fig.savefig(RUN / "p28-first-targets.svg", bbox_inches="tight")
fig.savefig(RUN / "p28-first-targets.png", dpi=160, bbox_inches="tight")

cover = plt.figure(figsize=(8.5, 11))
cover.text(0.07, 0.94, "Pencil 28-bit first-target records", fontsize=15, weight="bold")
cover.text(0.07, 0.90, "Six source-verified candidate/rho pairs; one target per arm.", fontsize=9)
table_rows = [[label,
               f"{row['representative']['pencil_over_rho']:,.3f}",
               f"{row['solver_row_updates']:,}",
               f"{row['solver_index_ops']:,}"] for label, row in rows]
table_axis = cover.add_axes([0.07, 0.47, 0.86, 0.36])
table_axis.axis("off")
table = table_axis.table(cellText=table_rows,
                         colLabels=["p and arm", "pencil/rho", "row updates", "index ops"],
                         loc="upper center", cellLoc="left", colLoc="left")
table.auto_set_font_size(False)
table.set_fontsize(8)
table.scale(1, 1.5)
cover.text(0.07, 0.42, "Representative price: M+0.5S+10I and 0.1 GAE/logical.", fontsize=9)
cover.text(0.07, 0.37, f"At p=48,383, peel/online identity: {paired['representative']['candidate_over_baseline']:.6f}.",
           fontsize=9)
cover.text(0.07, 0.32, "Peel/online band: "
           f"{paired['min_candidate_over_baseline']:.6f}–{paired['max_candidate_over_baseline']:.6f}; "
           f"{paired['cells_candidate_below_baseline']}/36 lower.", fontsize=9)
cover.text(0.07, 0.27, "All 36 candidate/rho cells exceed one for each of these records.", fontsize=9)
cover.text(0.07, 0.22, "The 28-bit slices contain one target each; no size-wide estimate follows.",
           fontsize=9)
cover.text(0.07, 0.14, "Source: raw JSONL, 36-cell grid, exact replay and native validator.", fontsize=9)
with PdfPages(RUN / "RUN_REPORT.pdf",
             metadata={"Title": "Pencil 28-bit first-target operation prices"}) as pdf:
    pdf.savefig(cover)
    pdf.savefig(fig)
plt.close(cover)
plt.close(fig)
