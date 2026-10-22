"""Render recorded balanced-partition transition counts."""
import json
from pathlib import Path
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

B = Path(__file__).resolve().parent
s = json.loads((B / "results/summary.json").read_text())
by = s["by_bits"]
bits = [str(x["bits"]) for x in by]
fig, ax = plt.subplots(figsize=(10, 5), layout="constrained")
ax.bar(bits, [x["branch_0"] for x in by], color="#23738b", label="D → 2D")
ax.bar(bits, [x["branch_1"] for x in by], bottom=[x["branch_0"] for x in by],
       color="#d18b32", label="D → 2D+G")
ax.set_ylim(0, 560)
ax.set_ylabel("Verified quotient transitions")
ax.set_xlabel("Frozen target size (bits); 180 targets and 540 transitions per size")
ax.set_title("Fixed-difference Kummer walk: source-label transition gate")
ax.legend()
ax.grid(axis="y", alpha=0.2)
svg = B / "transition-cohort.svg"
fig.savefig(svg, bbox_inches="tight")
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
fig.savefig(B / "transition-cohort.png", dpi=160, bbox_inches="tight")
with PdfPages(B / "TRANSITION_GATE.pdf") as pdf:
    page = plt.figure(figsize=(8.27, 11.69))
    page.text(0.08, 0.94, "Fixed-difference fast-Kummer walk", fontsize=17, weight="bold")
    page.text(0.08, 0.91, "KUMMER-DIFFERENTIAL-WALK-20261022", fontsize=10)
    lines = [
        "Fixed pair: K(D), K(D+G); G is the signed source generator.",
        "Transitions D→2D or D→2D+G use three fast xADD calls.",
        "All 1,260 targets: 3,780/3,780 exact quotient and signed-label",
        "transitions, zero wrong or inapplicable cases.",
        "Canonical-ratio partition: branch 0 = 1,860; branch 1 = 1,920.",
        "It uses one field inversion per transition.",
        "Preliminary Legendre-product partition: 2 vs 3,778; raw retained.",
        "Signed relation extraction, collector and rho comparison: not run.",
        "Candidate cost, rho cost and ratio: null.",
        "Rows: results/transition-*.jsonl and legendre_diagnostic/.",
        "Primary formula source: Chung–Costello–Smith, §7.1:",
        "https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf",
    ]
    y = 0.85
    for line in lines:
        page.text(0.08, y, line, fontsize=9)
        y -= 0.037
    pdf.savefig(page)
    plt.close(page)
    pdf.savefig(fig, bbox_inches="tight")
plt.close(fig)
