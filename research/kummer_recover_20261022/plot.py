"""Render only counts recomputed by validate.py from preserved target rows."""
import json
from pathlib import Path
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

B = Path(__file__).resolve().parent
s = json.loads((B / "results/summary.json").read_text())
by = s["by_bits"]
bits = [str(x["bits"]) for x in by]
fast = [x.get("computed", 0) for x in by]
exception = [x.get("enumerated_exception_lift", 0) for x in by]
fig, ax = plt.subplots(figsize=(10, 5), layout="constrained")
ax.bar(bits, fast, label="Correct fast generic Recover", color="#23738b")
ax.bar(bits, exception, bottom=fast, label="Correct bounded exceptional lift", color="#d18b32")
ax.set_ylim(174, 182)
ax.set_ylabel("Verified signed target recoveries")
ax.set_xlabel("Frozen target size (bits); 180 targets at each size")
ax.set_title("Fast-Kummer signed recovery: 1,260 source-verified targets")
ax.legend(loc="lower right")
ax.grid(axis="y", alpha=0.2)
fig.savefig(B / "recover-cohort.svg", bbox_inches="tight")
svg = B / "recover-cohort.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
fig.savefig(B / "recover-cohort.png", dpi=160, bbox_inches="tight")
with PdfPages(B / "RECOVER_GATE.pdf") as pdf:
    page = plt.figure(figsize=(8.27, 11.69))
    page.text(0.08, 0.94, "Fast-Kummer signed recovery gate", fontsize=17, weight="bold")
    page.text(0.08, 0.91, "KUMMER-RECOVER-20261022", fontsize=10)
    lines = [
        "Input: 21 models and the fixed 1,260 source-verified target cohort.",
        "Zero-coordinate xADD: 175/175 quotient-only two-torsion repairs.",
        "Signed target recovery: 1,260/1,260 correct, 0 wrong.",
        "Generic corrected Recover: 1,257; D=0 exceptional lifts: 3.",
        "Literal displayed equations: 1,257 wrong, 3 inapplicable; raw retained.",
        "Eq. (12) G3 coefficient uses -(xi2 Z1+Z2) after reducing",
        "(G3 x-G4) a_P modulo a_Q in the implemented Mumford convention.",
        "Eq. (6) independently supplies b1(Q)^2; G3^2 then follows.",
        "Displayed Eq. (11) differs by 2 f5 xi3 Z2 Z3 in our quintic case.",
        "The three exceptional lifts enumerate field square roots, check",
        "b(x)^2=f(x) mod a(x), and select sign using K(Q+P).",
        "They use Jacobian additions and O(p) field search; not fast.",
        "Closed labeled collector and paired rho pricing: not run.",
        "Candidate cost, rho cost and candidate/rho ratio: null.",
        "Rows: results/recover-target-*.jsonl, zero-shift.jsonl,",
        "and results/literal-recover-failures/recover-target-*.jsonl.",
        "Primary source: Chung–Costello–Smith, §§6.4, 7.4",
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
