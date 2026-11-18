"""Stacked S by phase for rho-neg, C = 1 and C = 2 at k = 32, with each arm's collision floor (reads summary.json)."""
import json, pathlib
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
h = pathlib.Path(__file__).resolve().parent
s = json.loads((h / "summary.json").read_text())
arms = [("rho-neg", s["rho_neg"]["setup"], s["rho_neg"]["search"], s["floors"]["rho_neg"]),
        ("C = 1 (wwalk)", s["c1"]["oracle_setup"] + s["c1"]["factor_base"], s["c1"]["relations"], s["floors"]["c1_F1"]),
        ("C = 2 (inc-c2)", s["c2"]["oracle_setup"], s["c2"]["relations"], s["floors"]["c2_F2"])]
fig, ax = plt.subplots(figsize=(7, 4))
for i, (n, a, b, f) in enumerate(arms):
    ax.bar(i, a, color="#9fb7d6", label="setup / table build" if i == 0 else None)
    ax.bar(i, b, bottom=a, color="#1f6fb2", label="walk / search" if i == 0 else None)
    ax.hlines(f, i - 0.4, i + 0.4, colors="#c0504d", linestyles="--", label="generic collision floor" if i == 0 else None)
    ax.text(i, a + b + 0.03, f"S = {a+b:.3f}", ha="center", fontsize=8)
ax.set_xticks(range(3)); ax.set_xticklabels([a[0] for a in arms])
ax.set_ylabel("S = GAE / sqrt(r)"); ax.legend(fontsize=8, loc="upper left")
ax.set_title("k = 32, five fresh sibling curves, 120 runs per arm (rerun of the frozen C = 1 spec)", fontsize=9)
fig.tight_layout()
for e in ("svg", "png"): fig.savefig(h / f"c1_phases.{e}", dpi=150)
with PdfPages(h / "c1_phases.pdf") as p: p.savefig(fig)
