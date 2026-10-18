"""Plots recorded values only (figures/cells.json from raw/*.stdout.log; plan records
from raw/card-04-plan-*.stdout.log; incumbent values cited from
research/dreg_closure_scale_20261016/RESULTS.md). No new computation."""
import json, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
rows = json.load(open("cells.json"))
col = {"refuted": "#1b7837", "saturated": "#c51b7d", "timelimit": "#e08214"}
lab = {"refuted": "refuted (D=4)", "saturated": "saturated, no 1 (D>4)", "timelimit": "time limit (unmeasured)"}
fig, ax = plt.subplots(1, 2, figsize=(11, 4.6))
seen = set()
for r in rows:
    if r["cell"].startswith(("sat-", "ctl-19")):
        mk = "s"
    else:
        mk = "o" if r["m"] == 4 else "^"
    l = lab[r["status"]] if r["status"] not in seen else None
    seen.add(r["status"])
    ax[0].scatter(r["N"], r["secs"], c=col[r["status"]], marker=mk, label=l, s=38, alpha=.85)
    ax[1].scatter(r["N"], r["rss_mb"]/1024, c=col[r["status"]], marker=mk, s=38, alpha=.85)
ax[0].scatter([42], [244], facecolors="none", edgecolors="k", s=70, label="committed dense incumbent (13,4,4), cited")
ax[1].scatter([42], [7.1], facecolors="none", edgecolors="k", s=70)
plan = {46: 2.207, 50: 4.216, 54: 7.705, 62: 22.970}  # resident_upper_bytes / 1e9 (plan records)
ax[1].plot(list(plan), list(plan.values()), "k:", marker="x", label="plan upper bound (resident, rank = columns)")
ax[1].axhline(6.92, color="r", lw=1); ax[1].text(37.2, 7.3, "instrument cap 6.92 GB", color="r", fontsize=8)
ax[1].set_ylim(0, 12); ax[1].set_xlim(36, 64)
ax[0].axhline(460, color="gray", lw=.8, ls="--"); ax[0].text(37, 520, "card time limit 450-460 s", fontsize=8, color="gray")
ax[0].set_yscale("log"); ax[0].set_xlabel("unknowns N"); ax[0].set_ylabel("wall seconds (shared host, L0)")
ax[1].set_xlabel("unknowns N"); ax[1].set_ylabel("peak RSS, GB (measured)")
ax[0].set_title("Time per draw", fontsize=10); ax[1].set_title("Memory per draw vs plan bound", fontsize=10)
h0, l0 = ax[0].get_legend_handles_labels(); h1, l1 = ax[1].get_legend_handles_labels()
ax[0].set_ylim(10, 2500); ax[0].legend(h0, l0, fontsize=7, loc="lower right"); ax[1].legend(h1, l1, fontsize=7, loc="upper left")
fig.suptitle("BLINDSPOT-07 blocked closure ladder: degree-4 closure per draw (circle m=4, triangle m=3, square control)", fontsize=10)
fig.text(.01, .005, "Source: raw/card-02..11 logs and receipts (per-card commits in the receipts). Cards 1 and 7 incomplete (no receipt).", fontsize=7)
fig.tight_layout(rect=(0, .02, 1, .96))
fig.savefig("ladder.svg"); fig.savefig("ladder.png", dpi=150)
