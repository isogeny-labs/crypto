"""Render recorded Rosenhain setup-gate values; no result computation."""
import json
from pathlib import Path

import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

BASE = Path(__file__).resolve().parent
ROWS = [json.loads(line) for line in (BASE / "results/rosenhain-models.jsonl").read_text().splitlines()]
SUMMARY = json.loads((BASE / "results/rosenhain-summary.json").read_text())
PRIMES = [str(row["p"]) for row in ROWS]
SQUARE = [row["square_pairs"] for row in ROWS]
TRIED = [row["pairs_tried"] for row in ROWS]

fig, ax = plt.subplots(figsize=(12, 4.8), layout="constrained")
ax.bar(PRIMES, SQUARE, color="#27758f", label="Fp square normalizations")
ax.plot(PRIMES, TRIED, marker="o", markersize=2.5, color="#a56828", label="Ordered pairs tried")
ax.set_ylim(0, 22)
ax.set_ylabel("Ordered root pairs (count)")
ax.set_xlabel("Scholten model prime p")
ax.set_title("Rosenhain normalization setup on 21 rational-root models")
ax.tick_params(axis="x", rotation=55, labelsize=8)
ax.grid(axis="y", alpha=0.25)
ax.set_axisbelow(True)
ax.legend(loc="center right", bbox_to_anchor=(0.99, 0.67), frameon=False)
fig.text(0.5, -0.025, "ROSENHAIN-SETUP-20261022 · coefficient identity checked for every model; no theta or Kummer coordinates tested.", ha="center", fontsize=8)
svg = BASE / "rosenhain-setup.svg"
fig.savefig(svg, bbox_inches="tight")
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
fig.savefig(BASE / "rosenhain-setup.png", dpi=150, bbox_inches="tight")

with PdfPages(BASE / "ROSENHAIN_GATE.pdf") as pdf:
    page = plt.figure(figsize=(8.27, 11.69))
    page.text(0.08, 0.94, "Rosenhain setup gate", fontsize=18, weight="bold")
    page.text(0.08, 0.91, "Separate addendum · ROSENHAIN-SETUP-20261022", fontsize=10)
    lines = [
        f"Models: {SUMMARY['models']}; exact coefficient checks passed: {SUMMARY['passed']}.",
        "Each model: 20 ordered finite-root pairs, 10 square scales.",
        "Every chosen Fp map gives y'^2=t(t-1)(t-lambda)(t-mu)(t-nu).",
        "First model p=739: r0=246, r1=369, scale=179, sqrt=87,",
        "(lambda,mu,nu)=(296,16,10). Raw rows give all 21 maps.",
        "Method: ordered root-pair search; square test; transformed quintic",
        "reconstructed coefficient by coefficient and checked exactly.",
        "The released binary was rerun byte-for-byte and a separate Rust",
        "validator checked roots, square roots and coefficient vectors.",
        "Boundary: theta constants, projection, xDBL/xADD, signed lifting,",
        "and every candidate/rho price cell remain unmeasured.",
        "Raw: results/rosenhain-models.jsonl and rosenhain-summary.json.",
        "Primary formula context: https://eprint.iacr.org/2005/314/",
        "Signed recovery context: https://eprint.iacr.org/2016/777/",
    ]
    y = 0.85
    for line in lines:
        page.text(0.08, y, line, fontsize=10)
        y -= 0.037
    pdf.savefig(page)
    plt.close(page)
    pdf.savefig(fig, bbox_inches="tight")
plt.close(fig)
