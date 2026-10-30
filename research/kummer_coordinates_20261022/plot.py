"""Render recorded Kummer-coordinate cohort counts; no experiment here."""
import json
from pathlib import Path

import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

BASE = Path(__file__).resolve().parent
S = json.loads((BASE / "results/summary.json").read_text())
rows = S["by_bits"]
bits = [str(r["bits"]) for r in rows]
matches = [r["nonzero_matches"] for r in rows]
zeros = [r["zero_outputs"] for r in rows]
fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(10, 7), layout="constrained", sharex=True)
ax1.bar(bits, matches, color="#26718b")
ax1.set_ylim(1240, 1265)
ax1.set_ylabel("Matching nonzero xADD outputs")
ax1.set_title("Fast-Kummer coordinate identities: 1,260 public targets / 8,820 cases")
ax1.grid(axis="y", alpha=0.2)
ax2.bar(bits, zeros, color="#b36d34")
ax2.set_ylim(0, 5)
ax2.set_ylabel("Zero-vector exceptions")
ax2.set_xlabel("Frozen target size (bits); 180 targets and 1,260 labeled cases per size")
ax2.grid(axis="y", alpha=0.2)
fig.text(0.5, -0.015, "KUMMER-COORDINATES-20261022 · no fast signed collector or candidate/rho cost measured", ha="center", fontsize=8)
svg=BASE/"coordinate-cohort.svg"
fig.savefig(svg,bbox_inches="tight")
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines())+"\n")
fig.savefig(BASE/"coordinate-cohort.png",dpi=150,bbox_inches="tight")
with PdfPages(BASE/"COORDINATE_GATE.pdf") as pdf:
    page=plt.figure(figsize=(8.27,11.69))
    page.text(0.08,0.94,"Fast-Kummer coordinate gate",fontsize=18,weight="bold")
    page.text(0.08,0.91,"KUMMER-COORDINATES-20261022",fontsize=10)
    lines=[
        "21 genus-two models; 105 checked branch-point linear actions.",
        "Generic model cases: 84/84 doubles and 126/126 additions match.",
        "Exceptional model cases: 1,008 nonzero matches, 168 zero outputs,",
        "zero wrong nonzero outputs. All 175 non-pass rows are retained.",
        "Full public cohort: 1,260 targets, 8,820 labeled cases.",
        "xADD: 8,813 nonzero matches, seven zero outputs, zero wrong outputs.",
        "Source scalar, label and affine-map checks: all passed.",
        "Oracle sign disambiguation: 8,820 unique signed classes.",
        "Fast Recover and labeled DLP collector: unimplemented.",
        "Candidate/rho costs and ratio: null, not measured.",
        "Seven target zero outputs occur at 16 and 20 bits only.",
        "Raw: results/target-gate-*.jsonl and coordinate-exceptions.jsonl.",
        "Method: Chung, Costello, Smith, sections 7.1-7.4:",
        "https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf",
    ]
    y=0.85
    for line in lines:
        page.text(0.08,y,line,fontsize=10);y-=0.037
    pdf.savefig(page);plt.close(page)
    pdf.savefig(fig,bbox_inches="tight")
plt.close(fig)
