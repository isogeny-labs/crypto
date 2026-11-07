"""Diagram: which swept sources bear on which knowledge-base entries and leads (layout only)."""
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch

here = Path(__file__).resolve().parent
sources = [
    ("arXiv 2611.00517 Dias-Lima: OpenAI ETH constants", "eth"),
    ("arXiv 2610.41622 Majdoddin: Anthropic 3-SAT crossovers", "eth"),
    ("arXiv 2611.02650 MIT Hardness Group: ETH false, SH plausible", "eth"),
    ("arXiv 2611.06813 Chan et al.: hyperclique, Max-3-SAT 2^0.99n", "eth"),
    ("arXiv 2610.41048 Kozma: #PPM from #3SAT", "eth"),
    ("arXiv 2610.18914 Lu: shared-leg separation, any char.", "omega"),
    ("arXiv 2611.04175 Li: exposition, char.-free (summary)", "omega"),
    ("arXiv 2610.13971 Sun: claims omega = 2 (unverified outlier)", "omega"),
    ("arXiv 2610.10494 Uznanski: rectangular, char. 0", "omega"),
    ("ePrint 2026/2622 Gao-Hu: sieving if omega < 9/4", "omega"),
    ("ePrint 2026/2625 Xiong-Wang: factoring n^1.5532 log q", "roots"),
    ("ePrint 2026/2560, 2621, 2253, 1969, 2470 (assessed 2026-10-31)", "prior"),
    ("ePrint 2026/1711 Budroni et al.: PKP/PEP (code-based)", "none"),
]
targets = {
    "eth": "KN-LIT-caa51d (ETH disproof)\nLEAD-ICB-decomposition-encoding-and-sat",
    "omega": "KN-LIT-58d42d / KN-LIT-eef882 (omega <= 9/4)\nLEAD-ICE-first-fall-degree-assumptions",
    "roots": "Decomposition root finding\n(cost model, LEAD-ICE engines)",
    "prior": "crypto research/ic_mega_pass_20261031\n(already assessed; leads named there)",
    "none": "No ECDLP bearing (discarded)",
}
effect = {
    "eth": "Galactic confirmed with constants;\nno change to closed_obstructed",
    "omega": "9/4 in char. 2: three routes reported;\nno number changes",
    "roots": "Asymptotic only; no row changes",
    "prior": "No new action",
    "none": "-",
}
order = ["eth", "omega", "roots", "prior", "none"]
fig, ax = plt.subplots(figsize=(13, 8))
ax.axis("off")
ys = {}
for i, (s, g) in enumerate(sources):
    y = 1 - (i + 0.5) / len(sources)
    ax.add_patch(FancyBboxPatch((0.0, y - 0.025), 0.36, 0.05, boxstyle="round,pad=0.003", fc="#edf2f7", ec="#4a5568", lw=0.6))
    ax.text(0.005, y, s, va="center", fontsize=7)
    ys.setdefault(g, []).append(y)
for j, g in enumerate(order):
    ty = sum(ys[g]) / len(ys[g])
    ax.add_patch(FancyBboxPatch((0.45, ty - 0.035), 0.26, 0.07, boxstyle="round,pad=0.003", fc="#fefcbf", ec="#744210", lw=0.8))
    ax.text(0.455, ty, targets[g], va="center", fontsize=7)
    ax.add_patch(FancyBboxPatch((0.76, ty - 0.035), 0.23, 0.07, boxstyle="round,pad=0.003", fc="#e6fffa", ec="#234e52", lw=0.8))
    ax.text(0.765, ty, effect[g], va="center", fontsize=7)
    ax.annotate("", xy=(0.76, ty), xytext=(0.71, ty), arrowprops=dict(arrowstyle="->", lw=0.8))
    for y in ys[g]:
        ax.annotate("", xy=(0.45, ty), xytext=(0.36, y), arrowprops=dict(arrowstyle="->", lw=0.5, color="#4a5568"))
ax.set_title("Literature sweep 2026-11-07: sources -> knowledge-base entries / leads -> effect (all source claims reported, not checked here)", fontsize=9)
ax.set_xlim(0, 1); ax.set_ylim(0, 1)
fig.tight_layout()
for ext in ("svg", "png", "pdf"):
    fig.savefig(here / f"source_map.{ext}", dpi=150)
print("wrote source_map.{svg,png,pdf}")
