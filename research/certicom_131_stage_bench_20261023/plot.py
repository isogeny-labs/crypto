"""Render summary.tsv only; no benchmark calculation or data filtering."""
import csv
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
with (here / "summary.tsv").open() as f:
    rows = list(csv.DictReader(f, delimiter="\t"))

targets = ["ECC2K-130", "ECC2-131", "ECCp-131"]
stages = [("rho_state_step", "Rho state updates"),
          ("s3_root_query", "S3 root queries")]
colors = ["#147d92", "#56aa92", "#da8b4a"]
fig, axes = plt.subplots(1, 2, figsize=(10.8, 4.2), layout="constrained")
for ax, (stage, title) in zip(axes, stages):
    cells = [next(r for r in rows if r["target"] == t and r["stage"] == stage)
             for t in targets]
    med = [float(r["median_per_s"]) / 1000 for r in cells]
    lo = [float(r["min_per_s"]) / 1000 for r in cells]
    hi = [float(r["max_per_s"]) / 1000 for r in cells]
    ax.bar(targets, med, color=colors, width=0.65)
    ax.errorbar(range(3), med,
                yerr=[[m - l for m, l in zip(med, lo)],
                      [h - m for h, m in zip(hi, med)]],
                fmt="none", ecolor="#1f2933", capsize=5, linewidth=1.4)
    ax.set_title(title)
    ax.set_ylabel("Thousands of operations/s")
    ax.set_ylim(0, max(hi) * 1.2)
    ax.grid(axis="y", alpha=0.22)
    ax.set_axisbelow(True)
    for tick in ax.get_xticklabels():
        tick.set_rotation(15)
        tick.set_ha("right")

fig.suptitle("Certicom 131-bit CPU stage diagnostic · 5 samples per cell")
fig.text(0.5, -0.04,
         "Full-field operations on one reserved physical core; relaxed admission, exploratory rates. "
         "Run: certicom_131_stage_bench_20261023",
         ha="center", fontsize=8)
fig.savefig(here / "stage-rates.svg", bbox_inches="tight")
fig.savefig(here / "stage-rates.png", dpi=150, bbox_inches="tight")
