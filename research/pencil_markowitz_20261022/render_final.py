"""Lay out native Markowitz records as a PDF and graph; no research computation."""

import csv
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

ROOT = Path(__file__).resolve().parent
RUN = ROOT / "reports/final-20261022"
RUN.mkdir(parents=True, exist_ok=True)
status = json.loads((ROOT / "results/status.json").read_text())
pair = json.loads((ROOT / "results/p739-markowitz-vs-peel.json").read_text())
with (ROOT / "results/p739-markowitz-vs-peel.csv").open(newline="") as stream:
    grid = list(csv.DictReader(stream))
upper_raw = [json.loads(line) for line in
             (ROOT / "results/p186379-markowitz-t0.jsonl").read_text().splitlines()]
upper = upper_raw[1] if len(upper_raw) > 1 else None
upper_process = status["p186379_target0_process"]

fig, (left, right) = plt.subplots(1, 2, figsize=(11.2, 4.6))
for inversion, color in [("1", "#557f74"), ("10", "#245a88"),
                         ("100", "#b6752a"), ("1000", "#8c5277")]:
    cells = [next(row for row in grid if row["S_per_M"] == "0.5"
                  and row["I_per_M"] == inversion
                  and row["native_GAE_per_logical"] == native)
             for native in ("0", "0.1", "1")]
    left.plot([0, 1, 2], [float(row["candidate_over_baseline"]) for row in cells],
              marker="o", color=color, label=f"I/M={inversion}")
left.axhline(1, linewidth=0.8, color="#222222")
left.set_xticks([0, 1, 2], ["0", "0.1", "1"])
left.set_xlabel("GAE per logical operation")
left.set_ylabel("Markowitz / peel cold cost")
left.set_title("p=739, same 60 targets; S/M=0.5")
left.grid(alpha=0.25)
left.legend(fontsize=8)

controls = [
    ("Peel 10m", ROOT.parent / "pencil_sparse_20261022/results/p186379-peel-t0.jsonl"),
    ("Peel 25m", ROOT.parent / "pencil_sparse_20261022/results/p186379-peel-cap25m-t0.jsonl"),
]
names = []
row_updates = []
index_ops = []
for label, path in controls:
    row = json.loads(path.read_text().splitlines()[-1])
    names.append(label)
    row_updates.append(row["solver_row_updates"])
    index_ops.append(row["solver_index_ops"])
if upper is not None:
    names.append("Markowitz 25m")
    row_updates.append(upper["solver_row_updates"])
    index_ops.append(upper["solver_index_ops"])
positions = list(range(len(names)))
right.bar([x - 0.17 for x in positions], row_updates, width=0.34,
          color="#b6752a", label="Modular row updates")
right.bar([x + 0.17 for x in positions], index_ops, width=0.34,
          color="#245a88", label="Logical index operations")
right.set_xticks(positions, names)
right.set_yscale("log")
right.set_ylabel("Recorded operations, log scale")
right.set_title("p=186379, target 0; fixed source workload")
right.grid(axis="y", alpha=0.25)
right.legend(fontsize=8)
for tick in right.get_xticklabels():
    tick.set_rotation(12)
if upper is None:
    right.text(0.5, 0.90, "Markowitz: no target row emitted",
               transform=right.transAxes, ha="center", fontsize=9)
fig.suptitle("Exact sparse pivot-priority follow-up · measured records")
fig.tight_layout(rect=[0, 0, 1, 0.94])
fig.savefig(RUN / "markowitz-pricing-and-upper-guard.svg", bbox_inches="tight")
fig.savefig(RUN / "markowitz-pricing-and-upper-guard.png", dpi=170, bbox_inches="tight")

cover = plt.figure(figsize=(8.5, 11))
cover.text(0.08, 0.94, "Exact sparse pivot-priority follow-up", fontsize=15, weight="bold")
cover.text(0.08, 0.90, "Frozen source targets · native solver and validator · matched source rho",
           fontsize=9.5)
lines = [
    "p=739 repeated 60-target gate: all candidate and rho scalars source verified; replay matched.",
    f"Representative cold cost: peel/rho {pair['representative']['baseline_over_rho']:.6f}; "
    f"Markowitz/rho {pair['representative']['candidate_over_rho']:.6f}.",
    f"Markowitz/peel across 36 pricing cells: "
    f"{pair['cells_candidate_below_baseline']} lower (see full CSV for equal and higher).",
    f"Paired range: {pair['min_candidate_over_baseline']:.6f}–"
    f"{pair['max_candidate_over_baseline']:.6f}; "
    f"{pair['cells_candidate_below_rho']}/36 below rho.",
    "",
    f"p=186379 target 0: process {upper_process['status']}, "
    f"exit {upper_process['exit_code']}, failure {json.dumps(upper_process['failure'])}.",
    (f"Upper recorded work: {upper['solver_row_updates']:,} row updates; "
     f"{upper['solver_index_ops']:,} index ops." if upper else
     "Upper recorded work: no target row emitted."),
    (f"Upper peak stored coefficients: {upper['solver_stored_nonzero_coefficients']:,}; "
     f"rank {upper['solver_rank']:,}." if upper else
     "Upper peak stored coefficients and rank: unavailable."),
    f"Candidate scalar verified: {json.dumps(upper_process['candidate_verified'])}; "
    f"same-process rho verified: {json.dumps(upper_process['rho_verified'])}.",
    f"Candidate/rho cost ratio: {json.dumps(status['p186379_target0_candidate_ratio'])}.",
    f"Original cohort: {status['source_verified_distinct_targets']}/1260 distinct solves; "
    f"{status['distinct_targets_still_open']} targets open.",
    "",
    "This packet retains the raw failure, process exit, independent replay, all paired prices,",
    "source checks, native validator, provenance manifest, and frozen protocol.",
    "A repeated p=739 solve does not add a distinct original cohort target.",
    "Wall time is an L0 diagnostic on a shared host and is not a priced field cost.",
]
for index, line in enumerate(lines):
    cover.text(0.08, 0.83 - index * 0.042, line, fontsize=8.7)
cover.text(0.08, 0.08,
           "See DATA_DICTIONARY.md and results/price-grid.csv for every raw price and null.",
           fontsize=8)
with PdfPages(RUN / "RUN_REPORT.pdf", metadata={"Title": "Exact sparse Markowitz pivot follow-up"}) as pdf:
    pdf.savefig(cover)
    pdf.savefig(fig)
plt.close(cover)
plt.close(fig)
(RUN / "README.md").write_text(
    "# Exact sparse pivot-priority follow-up\n\n"
    "The PDF and graph lay out native matched prices and the upper-size process outcome. "
    "Raw JSONL, replay, all 36 paired cells, validator, manifest, and frozen protocol "
    "are in the parent study. The upper-size candidate/rho price is null unless "
    "a source-verified candidate scalar was recovered.\n"
)
