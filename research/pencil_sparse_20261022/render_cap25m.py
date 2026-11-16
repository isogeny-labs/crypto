"""Lay out the three native p186379 cap records; do not reprice failed arms."""

import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

ROOT = Path(__file__).resolve().parent
RUN = ROOT / "reports/run-20261022T0647Z"
RUN.mkdir(parents=True, exist_ok=True)
names = ["Online\n10m", "Peel\n10m", "Peel\n25m"]
paths = [ROOT / "results/p186379-identity-t0.jsonl",
         ROOT / "results/p186379-peel-t0.jsonl",
         ROOT / "results/p186379-peel-cap25m-t0.jsonl"]
rows = [[json.loads(line) for line in path.read_text().splitlines()][1]
        for path in paths]
fig, axes = plt.subplots(1, 3, figsize=(12.3, 4.6))
colors = ["#b56a24", "#287097", "#21496d"]
series = [
    ("Peak stored coefficients", "solver_stored_nonzero_coefficients"),
    ("Modular row updates", "solver_row_updates"),
    ("Logical index operations", "solver_index_ops"),
]
for axis, (label, key) in zip(axes, series):
    axis.bar(names, [row.get(key, 0) for row in rows], color=colors)
    axis.set_title(label, fontsize=10)
    axis.tick_params(axis="x", labelsize=9)
    axis.grid(axis="y", alpha=0.25)
    axis.ticklabel_format(axis="y", style="sci", scilimits=(0, 0))
axes[0].scatter(names, [10_000_000, 10_000_000, 25_000_000],
                marker="_", s=200, color="#333333", label="Configured cap")
axes[0].legend(fontsize=8, loc="upper left")
fig.suptitle("p=186,379, frozen target 0: three exact pencil attempts reached their caps")
fig.text(0.5, 0.015,
         "All matched rho scalars verified; candidate scalars and candidate/rho ratios are null",
         ha="center", fontsize=8)
fig.tight_layout(rect=[0, 0.055, 1, 0.9])
fig.savefig(RUN / "three-caps.svg", bbox_inches="tight")
fig.savefig(RUN / "three-caps.png", dpi=170, bbox_inches="tight")

cover = plt.figure(figsize=(8.5, 11))
cover.text(0.08, 0.94, "Pencil upper-size diagnostic: three capped attempts",
           fontsize=14, weight="bold")
cover.text(0.08, 0.90, "p=186,379 · frozen target 0 · 32-bit model", fontsize=10)
table_rows = [[name.replace("\n", " "),
               f"{row.get('matrix_coefficient_cap', 10_000_000):,}",
               f"{row['solver_rank']:,}",
               f"{row['solver_stored_nonzero_coefficients']:,}",
               f"{row['solver_row_updates']:,}",
               f"{row.get('solver_index_ops', 0):,}"]
              for name, row in zip(names, rows)]
table_axis = cover.add_axes([0.035, 0.65, 0.93, 0.19])
table_axis.axis("off")
table = table_axis.table(cellText=table_rows,
                         colLabels=["solver", "cap", "rank", "peak coeff.",
                                    "row updates", "index ops"],
                         loc="upper center", cellLoc="left", colLoc="left")
table.auto_set_font_size(False)
table.set_fontsize(7)
table.scale(1.0, 1.65)
lines = [
    "Each attempt stopped at its preregistered stored-coefficient cap.",
    "All same-process rho controls recovered and verified the source scalar.",
    "Candidate scalar: null; candidate/rho cost ratio: null in every record.",
    "The changed row and index counts are unsuccessful-stage observations.",
    "The 25m extension kept the same target, collector, and source control.",
    "No 32-bit curve-slice success is inferred from these three attempts.",
    "Wall time has L0 isolation because other jobs shared the host.",
]
for index, line in enumerate(lines):
    cover.text(0.08, 0.57 - index * 0.047, line, fontsize=8.5)
cover.text(0.08, 0.13,
           "Raw JSONL, replay, protocol, process record, and validator accompany this PDF.",
           fontsize=8)
with PdfPages(RUN / "RUN_REPORT.pdf",
             metadata={"Title": "Three exact pencil upper-size cap records"}) as pdf:
    pdf.savefig(cover)
    pdf.savefig(fig)
plt.close(cover)
plt.close(fig)

(RUN / "README.md").write_text(
    "# Frozen p186379 target-0 cap diagnostic\n\n"
    "The PDF and graph show the original 10m online and 10m peeling cap failures "
    "beside the separately frozen 25m peeling cap failure. All candidate scalars "
    "and candidate/rho ratios are null; the matched rho scalars verified. "
    "The three raw JSONL files, process status, independent replay, protocol and "
    "validator are in the parent study. Wall time is L0 diagnostic.\n"
)
