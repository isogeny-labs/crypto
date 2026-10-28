"""Render recorded sparse-pencil pilot values; research arithmetic stays native."""
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent
summary = json.loads((ROOT / "results/pencil-pilot/summary.json").read_text())
rows = sorted(summary["slices"], key=lambda row: row["p"])
assert len(rows) >= 2

fig, (ax, bx) = plt.subplots(1, 2, figsize=(12, 4.7))
ps = [row["p"] for row in rows]
ratios = [row["representative"]["pencil_over_rho"] for row in rows]
updates = [row["solver_row_updates"] / row["targets"] for row in rows]
ax.plot(ps, ratios, "o-", color="#245b92")
bx.plot(ps, updates, "s-", color="#a45a18")
for axis, values in ((ax, ratios), (bx, updates)):
    axis.set_xscale("log")
    axis.set_yscale("log")
    if max(ps) <= 5000:
        axis.set_xticks([739, 2000, 3000, 4000], ["739", "2k", "3k", "4k"])
        axis.set_xlim(620, 4600)
    else:
        axis.set_xticks([739, 2000, 4000, 8000, 12000], ["739", "2k", "4k", "8k", "12k"])
        axis.set_xlim(620, max(ps) * 1.2)
    axis.grid(alpha=0.25)
    for index in (0, len(ps) - 1):
        p, value = ps[index], values[index]
        axis.annotate(f"{value:.1f}" if axis is ax else f"{value:,.0f}",
                      (p, value), xytext=(0, 8), textcoords="offset points",
                      ha="center", fontsize=8)
ax.set(xlabel="Base field p", ylabel="Complete one-target pencil cost / source rho",
       title="Priced sparse pencil collector")
bx.set(xlabel="Base field p", ylabel="Modular row updates per target",
       title="Online elimination volume")
fig.suptitle("Point-driven pencil: completed pilot curves")
fig.text(0.5, 0.02,
         f"2026-10-21 · {sum(r['targets'] for r in rows)} source-verified candidate/rho pairs · M+0.5S+10I, 0.1 GAE/native operation",
         ha="center", fontsize=8)
fig.tight_layout(rect=[0, 0.07, 1, 0.93])
for suffix in ("svg", "png"):
    fig.savefig(ROOT / f"pencil-pilot-cost.{suffix}", dpi=150, bbox_inches="tight")
plt.close(fig)
svg = ROOT / "pencil-pilot-cost.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
