"""Render recorded native summary values; no experimental calculation here."""
import json
from pathlib import Path

import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages


BASE = Path(__file__).resolve().parent
DATA = json.loads((BASE / "results/summary.json").read_text())
BITS = [str(row["bits"]) for row in DATA["per_bits"]]
TARGETS = [row["passed_targets"] for row in DATA["per_bits"]]
EXCEPTIONS = [row["exception_cases"] for row in DATA["per_bits"]]


fig, ax = plt.subplots(figsize=(10, 4.8), layout="constrained")
x = list(range(len(BITS)))
width = 0.37
ax.bar([v - width / 2 for v in x], TARGETS, width, label="Source-checked targets", color="#126783")
ax.bar([v + width / 2 for v in x], EXCEPTIONS, width, label="Exceptional input cases", color="#ad742e")
ax.set_xticks(x, BITS)
ax.set_ylim(0, 205)
ax.set_xlabel("Source E(Fp²) bit class")
ax.set_ylabel("Passed checks (count)")
ax.set_title("Differential quotient and signed-lift reference gate")
ax.legend(frameon=False)
ax.grid(axis="y", alpha=0.25)
ax.set_axisbelow(True)
fig.text(
    0.5, -0.02,
    "KUMMER-REF-20261022 · 21 models × 60 targets; 7 target cases each; 30 exceptional cases/model. "
    "Jacobian-oracle reference, no collector costs.",
    ha="center", fontsize=8,
)
fig.savefig(BASE / "kummer-reference-gate.svg", bbox_inches="tight")
fig.savefig(BASE / "kummer-reference-gate.png", dpi=150, bbox_inches="tight")
svg = BASE / "kummer-reference-gate.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")

with PdfPages(BASE / "REPORT.pdf") as pdf:
    page = plt.figure(figsize=(8.27, 11.69))
    page.text(0.08, 0.94, "Sign-safe Kummer differential walk", fontsize=18, weight="bold")
    page.text(0.08, 0.91, "Reference invariant gate · 2026-10-22 UTC", fontsize=10)
    paragraphs = [
        "Scope. 21 Scholten models and 1,260 frozen public targets (60/model).",
        f"Observed. {DATA['targets']} source scalars checked; {DATA['target_cases']} target",
        f"coefficient-pair cases and {DATA['exception_cases']} model exception cases passed.",
        f"Failures: {DATA['target_failures']} targets; {DATA['exception_failures']} exceptions.",
        "Compatible sign lifts: 1 in 1,260 cases; 2 in 7,560 cases.",
        "Every compatible set produced one quotient output; the signed anchor",
        "selected one Jacobian lift. Raw rows and a byte-identical rerun are retained.",
        "Method. K(D) is canonical min(D,-D). Differential outputs were",
        "enumerated from Jacobian lifts using K(A), K(B), K(A-B). A signed G",
        "and K(A+B+G) selected the unique signed output.",
        "Boundary. This uses Jacobian addition as the reference oracle. It does",
        "not implement coordinate-only Kummer arithmetic or a DLP collector.",
        "Candidate/rho ratios in all 36 price cells are null; 0 of 1,260",
        "requested Kummer full-collector targets were executed.",
        "A fixed-offset sidecar updated by one xADD per jump J needs S-J in S;",
        "this restricted design requires at least order(J) stored offsets.",
        "Source records: results/target-gate-*.jsonl, model-exceptions-*.jsonl,",
        "summary.json, price-grid.csv, manifest.json and SHA256SUMS.",
        "Primary arithmetic reference: Chung, Costello & Smith, ePrint 2015/983.",
        "https://eprint.iacr.org/2015/983.pdf",
        "Gaudry fast Kummer: https://eprint.iacr.org/2005/314.pdf",
    ]
    y = 0.85
    for line in paragraphs:
        page.text(0.08, y, line, fontsize=10)
        y -= 0.034
    pdf.savefig(page)
    plt.close(page)
    pdf.savefig(fig, bbox_inches="tight")

plt.close(fig)
