"""Presentation only: plots values read from the ecbench outputs in analysis/
and the parents' committed bound records. Computes no result."""
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
D = "research/prime_medium_rho_cpc_20261109"
def c(p):
    k = json.load(open(p))["constant"]["ratio_to_floor"]; return k["value"], k["ci95"]
rows = [("parents 11-07 (480 runs)", c("docs/bounds/records/prime-medium-rho-neg.json")),
        ("this study (432 runs)", c(f"{D}/analysis/bound-rho-neg.json")),
        ("pooled (912 runs)", c(f"{D}/analysis/bound-rho-neg-pooled.json"))]
g = json.load(open(f"{D}/analysis/gates.json"))["G3c"]
fig, (a1, a2) = plt.subplots(1, 2, figsize=(9.6, 3.8), gridspec_kw={"width_ratios": [3, 2]})
for i, (n, (v, ci)) in enumerate(rows):
    a1.errorbar([v], [i], xerr=[[v - ci[0]], [ci[1] - v]], fmt="o", color="#2b6cb0", capsize=3)
a1.axvline(1.0, color="#444", lw=0.8, ls=":", label="ideal (floor)")
a1.axvline(1.10, color="#2f855a", lw=1, ls="--", label="setup_excess: M <= 1.10")
a1.axvline(1.30, color="#c05621", lw=1, ls="--", label="walk_shortfall: M >= 1.30")
a1.set_yticks(range(len(rows))); a1.set_yticklabels([r[0] for r in rows])
a1.set_xlabel("rho.negation constant / floor sqrt(pi/4), w = 0, 95% CI")
a1.set_title("Medium tier, log2 r 32.4-49.0"); a1.legend(fontsize=7, loc="lower right")
pts = [("11-08 PC (designed 1.5)", 1.4639, [1.4438, 1.4863]), ("this PC", g["ratio"], g["ci95"])]
for i, (n, v, ci) in enumerate(pts):
    a2.errorbar([v], [i], xerr=[[v - ci[0]], [ci[1] - v]], fmt="s", color="#6b46c1", capsize=3)
a2.axvline(1.5, color="#444", lw=0.8, ls=":", label="unconditional design 1.5")
a2.axvspan(1.473, 1.531, color="#e9d8fd", alpha=0.6, label="design 2.5-97.5 %")
a2.set_yticks(range(len(pts))); a2.set_yticklabels([p[0] for p in pts])
a2.set_xlabel("bsgs.textbook / bsgs.negation, 95% CI"); a2.set_title(f"Positive control: {g['exact']}/{g['records']} exact")
a2.legend(fontsize=7, loc="center left"); fig.tight_layout()
fig.savefig(f"{D}/figures/result.svg"); fig.savefig(f"{D}/figures/result.png", dpi=160)
with PdfPages(f"{D}/REPORT.pdf") as pdf:
    pdf.savefig(fig)
    t, a = plt.subplots(figsize=(8.5, 3.6)); a.axis("off")
    v = json.load(open(f"{D}/analysis/verdict.json"))
    cells = [["M (this study, w = 0)", f"{v['M']:.3f}", f"[{v['ci95'][0]:.3f}, {v['ci95'][1]:.3f}]"],
             ["verdict (PROTOCOL.md section 5)", v["verdict"], "all gates pass" if g["pass"] else "gate failed"],
             ["pooled with parents", f"{rows[2][1][0]:.3f}", f"[{rows[2][1][1][0]:.3f}, {rows[2][1][1][1]:.3f}]"],
             ["positive control", f"{g['ratio']:.3f}", f"[{g['ci95'][0]:.3f}, {g['ci95'][1]:.3f}]"]]
    tab = a.table(cellText=cells, colLabels=["quantity", "value", "95% CI / note"], loc="center")
    tab.auto_set_font_size(False); tab.set_fontsize(8); tab.scale(1, 1.5)
    a.set_title("isolab L2 (pods exit 42: counts admitted, wall time not); not validated", fontsize=8)
    pdf.savefig(t)
