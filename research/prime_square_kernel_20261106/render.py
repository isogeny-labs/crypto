"""Layout only: plot exact group counts already computed by analyze.cpp."""
import csv
import gzip
import json
import pathlib

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

root = pathlib.Path(__file__).parent
report = root / "reports/local-20261107"
with (report / "per-level.csv").open() as stream:
    rows = list(csv.DictReader(stream))
summary = json.loads((report / "summary.json").read_text())
fig, ax = plt.subplots(figsize=(11, 9))
y = list(range(len(rows)))
left = [0] * len(rows)
for field, label, color in [
    ("fewer", "Candidate: fewer tuples", "#20854e"),
    ("equal", "Equal", "#999999"),
    ("more", "Candidate: more tuples", "#bd3232"),
]:
    values = [int(row[field]) for row in rows]
    ax.barh(y, values, left=left, color=color, label=label, height=0.75)
    left = [a + b for a, b in zip(left, values)]  # bar placement only
ax.set_yticks(y, [f"L={r['L']}, i={r['level']}, {r['method']}" for r in rows], fontsize=8)
ax.invert_yaxis()
ax.set_xlabel("Matched finite cells (exact counts)")
ax.grid(axis="x", alpha=0.2)
ax.set_axisbelow(True)
ax.legend(loc="lower right", fontsize=8)
fig.suptitle("Normalized-square kernel: all finite-cell comparisons", fontsize=15)
ax.set_title("Same precision; polynomial degree, recovery and basis costs remain separate", fontsize=10)
fig.tight_layout()
fig.savefig(report / "comparisons.svg")
with (report / "comparisons.svgz").open("wb") as stream:
    with gzip.GzipFile(filename="", fileobj=stream, mode="wb", mtime=0) as archive:
        archive.write((report / "comparisons.svg").read_bytes())
fig.savefig(report / "comparisons.pdf")
with PdfPages(report / "report.pdf") as pdf:
    textfig = plt.figure(figsize=(8.3, 11.7))
    textfig.text(0.09, 0.93, "Normalized-square kernel: exact panel", fontsize=17)
    lines = [
        "Frozen input panel: L=4,8,16,32; all active levels; zeta=-1,1,3; q=8,12,16,24.",
        f"Input cells: {summary['input_cells']}; method rows per invocation: {summary['rows']}.",
        "Two local invocations completed; replay is byte-identical.",
        f"Independent tuple recomputations: {summary['tuple_recomputations']}.",
        "",
    ]
    for method, values in summary["methods"].items():
        lines.append(f"{method}: fewer={values['fewer']}, equal={values['equal']}, more={values['more']}.")
    lines += [
        "",
        "Witness: L=32, t=-31, zeta=-1, q=16, Smith: original16, candidate33.",
        "The candidate raises entry valuations but introduces more surviving modes.",
        "",
        "Scope: finite component tuple counts, with no timing or speedup claim.",
        "Candidate degree4L-2 versus original2L-1 still needs recovery-cost accounting.",
        "",
        "Protocol record issue: version1 misdescribes controls/instance granularity.",
        "A retrospective export-layout draft is preserved. The maintained checker",
        "refuses freezing this post-data amendment; no gate was weakened.",
        "Raw computation/replay checks pass; full protocol-export validation does not.",
        "",
        "Producer evidence only; independent signed verification is pending.",
        "Details: RESULTS_local_20261107.md and reports/local-20261107/.",
    ]
    textfig.text(0.09, 0.87, "\n".join(lines), fontsize=10, va="top", linespacing=1.6)
    pdf.savefig(textfig)
    pdf.savefig(fig)
    plt.close(textfig)
plt.close(fig)
