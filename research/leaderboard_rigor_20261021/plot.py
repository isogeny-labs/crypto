# Plots recorded values only: rho_lanes/sweep_summary.json, band_A/band_A.json, bisect/summary.jsonl + bisect.log lines.
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
root = os.path.dirname(os.path.abspath(__file__))
sw = json.load(open(f"{root}/rho_lanes/sweep_summary.json"))
band = json.load(open(f"{root}/band_A/band_A.json"))
fig, ax = plt.subplots(1, 3, figsize=(16, 4.4))
for n, c in ((47, "#4477aa"), (53, "#ee6677"), (61, "#228833")):
    rows = [r for r in sw if r["n"] == n]
    ax[0].plot([r["lanes"] for r in rows], [r["units_per_step_mean"] for r in rows], "o-", color=c, label=f"n = {n}")
ax[0].axhspan(4.6, 5.4, color="#cccccc", alpha=0.6, label="canonical step 4.6-5.4 (model)")
ax[0].set_xscale("log", base=2); ax[0].set_ylim(0, 14)
ax[0].set_xlabel("rho lanes"); ax[0].set_ylabel("rho cost per step (batched-addition units)")
ax[0].set_title("Rho step cost does not fall with more lanes"); ax[0].legend(fontsize=8)
for n, c in ((47, "#4477aa"), (53, "#ee6677"), (61, "#228833")):
    rows = [r for r in sw if r["n"] == n]
    ax[1].plot([r["lanes"] for r in rows], [r["ratio_mean"] for r in rows], "o-", color=c, label=f"n = {n}")
ax[1].set_xscale("log", base=2); ax[1].set_ylim(0, 20)
ax[1].set_xlabel("rho lanes"); ax[1].set_ylabel("cold IC / rho (same process)")
ax[1].set_title("x reference vs lanes (L0, 1-4 targets per point)"); ax[1].legend(fontsize=8)
xs = range(len(band))
for k, w in enumerate((0, 0.1, 1)):
    ax[2].plot(list(xs), [b["band"][k]["ratio_ic_over_rho"] for b in band], "o-", label=f"w = {w}")
ax[2].axhline(1, color="gray", lw=0.8); ax[2].set_xticks(list(xs)); ax[2].set_xticklabels([f"n={b['n']}" for b in band])
ax[2].set_ylabel("IC lower bound / rho, counted"); ax[2].set_title("Table A: counted lower-bound band (derived)"); ax[2].legend(fontsize=8)
fig.suptitle("IC leaderboard rigor study, 2026-10-21; values from rho_lanes/, band_A/ (sources beside this file)", fontsize=9)
fig.tight_layout()
fig.savefig(f"{root}/figures/rigor_summary.svg"); fig.savefig(f"{root}/figures/rigor_summary.png", dpi=150)
print("ok")
