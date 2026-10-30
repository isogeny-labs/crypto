"""Render only the recorded theta-gate counts and textual summary."""
import json
from pathlib import Path

import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

BASE = Path(__file__).resolve().parent
rows = [json.loads(line) for line in (BASE / "results/theta-models.jsonl").read_text().splitlines()]
summary = json.loads((BASE / "results/summary.json").read_text())
primes = [str(row["p"]) for row in rows]
counts = [row["theta_tuples"] for row in rows]

fig, ax = plt.subplots(figsize=(12, 4.8), layout="constrained")
ax.bar(primes, counts, color="#286b85")
ax.set_ylim(0, 128)
ax.set_ylabel("Verified theta tuples (fixed b=f=1 gauge)")
ax.set_xlabel("Scholten model prime p")
ax.set_title("Fp theta setup: 60 eligible affine Rosenhain orders per model")
ax.tick_params(axis="x", rotation=55, labelsize=8)
ax.grid(axis="y", alpha=0.25)
ax.set_axisbelow(True)
fig.text(0.5, -0.025, "KUMMER-THETA-AFFINE-20261022 · setup counts only; no fast Kummer DLP cost measured", ha="center", fontsize=8)
svg = BASE / "theta-setup.svg"
fig.savefig(svg, bbox_inches="tight")
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
fig.savefig(BASE / "theta-setup.png", dpi=150, bbox_inches="tight")

with PdfPages(BASE / "THETA_GATE.pdf") as pdf:
    page = plt.figure(figsize=(8.27, 11.69))
    page.text(0.08, 0.94, "Base-field theta setup gate", fontsize=18, weight="bold")
    page.text(0.08, 0.91, "KUMMER-THETA-AFFINE-20261022", fontsize=10)
    lines = [
        f"Models: {summary['model_count']}; with Fp theta tuple: {summary['models_with_fp_theta_tuple']}.",
        "Each model: 20 ordered finite-root pairs, 10 square pairs,",
        "60 eligible Rosenhain root orders; fixed gauges b=f=1.",
        f"Qualifying tuples: {summary['total_theta_tuples']}; counts per model: 48, 80 or 112.",
        "Independent lookup replay matches every model's count and checks",
        "each candidate against the original equations by cross product.",
        "First p=739 selection: (lambda,mu,nu)=(296,16,10);",
        "(a,b,c,d,e,f)=(369,1,255,605,719,1).",
        "Source: Chung, Costello, Smith, section 7.1:",
        "https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf",
        "Boundary: affine root orders and Fp constants only.",
        "No xADD/xDBL, signed DLP collector, operation cost or rho ratio measured.",
        "Raw: results/theta-models.jsonl; definition: DATA_DICTIONARY.md.",
    ]
    y = 0.85
    for line in lines:
        page.text(0.08, y, line, fontsize=10)
        y -= 0.037
    pdf.savefig(page)
    plt.close(page)
    pdf.savefig(fig, bbox_inches="tight")
plt.close(fig)
