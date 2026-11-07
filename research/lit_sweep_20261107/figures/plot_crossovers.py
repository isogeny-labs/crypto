"""Lay out the crossover points recorded in crossovers.json (no computation)."""
import json, sys
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
d = json.loads((here / "crossovers.json").read_text())
rows = sorted(d["rows"], key=lambda r: r["log10_n"])
labels = [f'{r["label"]}\n[{r["source"].split(",")[0]}; {r["stated"]}]' for r in rows]
vals = [r["log10_n"] for r in rows]
colors = ["#b5651d" if "Anthropic" in r["label"] else ("#7a7a7a" if "our estimate" in r["label"] else "#2b6cb0") for r in rows]

fig, ax = plt.subplots(figsize=(10, 6.2))
ax.barh(range(len(rows)), vals, color=colors, height=0.6)
band = d["reference_band"]
ax.axvspan(band["log10_lo"], band["log10_hi"], color="#2f855a", alpha=0.25, label=band["label"])
ax.set_xscale("log")
ax.set_xlim(1, 1e12)
ax.set_yticks(range(len(rows)))
ax.set_yticklabels(labels, fontsize=7)
ax.set_xlabel("log10 of crossover size (variables or clauses), log scale")
ax.set_title("Reported crossover points of subexponential #SAT algorithms\n(reported by the cited papers; not checked here; blue = OpenAI, brown = Anthropic, grey = our 2026-10-26 estimate)", fontsize=9)
for i, v in enumerate(vals):
    ax.text(v * 1.3, i, f"10^{v:,.0f}" if v >= 10 else f"10^{v}", va="center", fontsize=7)
ax.legend(loc="lower right", fontsize=7)
fig.tight_layout()
for ext in ("svg", "png", "pdf"):
    fig.savefig(here / f"crossovers.{ext}", dpi=150)
print("wrote crossovers.{svg,png,pdf}")
