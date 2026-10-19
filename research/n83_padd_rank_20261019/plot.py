#!/usr/bin/env python3
"""Draw result.svg/result.png from analysis.json and points.json (recorded values only)."""
import json, statistics as st
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

H = Path(__file__).resolve().parent
a = json.loads((H / "analysis.json").read_text())
pts = json.loads((H / "points.json").read_text())["points"]
RUN = "Actions run 39296352594 (commit 4b2b3efa6), artifact 12223498464"
C_CUR, C_PADD, C_INK = "#5b6b7c", "#c2410c", "#1f2933"

fig, ax = plt.subplots(1, 3, figsize=(13.5, 4.6), constrained_layout=True)

# (a) paired cold saving per point
d = a["metrics"]["cold_s"]["per_point_saving"]
lo, hi = a["metrics"]["cold_s"]["paired_saving_boot95"]
m = a["metrics"]["cold_s"]["paired_saving_mean"]
ax[0].axhspan(lo, hi, color=C_PADD, alpha=0.18, label=f"95% bootstrap interval [{lo:.2f}, {hi:.2f}] s")
ax[0].axhline(m, color=C_PADD, lw=1.2, label=f"mean {m:.2f} s")
ax[0].plot(range(1, 21), d, "o", color=C_INK, ms=4.5)
ax[0].axvline(5.5, color="#9aa5b1", lw=0.8, ls=":")
ax[0].text(3, min(d) - 0.35, "5 sealed", ha="center", fontsize=8, color="#52606d")
ax[0].text(13, min(d) - 0.35, "15 new", ha="center", fontsize=8, color="#52606d")
ax[0].set_ylim(min(d) - 0.7, max(d) + 0.9)
ax[0].set_xlabel("point (1-20)"); ax[0].set_ylabel("cold saving, s (current mean of A/A minus padd)")
ax[0].set_title("(a) paired cold saving per point, n = 20", fontsize=10)
ax[0].legend(fontsize=7.5, loc="upper right")

# (b) phases
mt = a["metrics"]
phases = {}
for arm, key in (("current", "cur_mean"), ("padd", "padd_mean")):
    setup, rank, cold = mt["setup_s"][key], mt["rank_s"][key], mt["cold_s"][key]
    phases[arm] = (setup, rank, cold - setup - rank)
for i, (arm, (s_, r_, o_)) in enumerate(phases.items()):
    c = C_CUR if arm == "current" else C_PADD
    ax[1].bar(i, s_, color=c, alpha=0.45, edgecolor="white")
    ax[1].bar(i, r_, bottom=s_, color=c, alpha=0.85, edgecolor="white")
    ax[1].bar(i, o_, bottom=s_ + r_, color=c, alpha=0.25, edgecolor="white")
    for y0, v, lab in ((0, s_, "setup"), (s_, r_, "rank")):
        ax[1].text(i, y0 + v / 2, f"{lab} {v:.1f}", ha="center", va="center", fontsize=8, color="black")
    ax[1].text(i, s_ + r_ + o_ + 1.5, f"cold {s_ + r_ + o_:.1f} s (other {o_:.1f})", ha="center", fontsize=8.5)
ax[1].set_xticks([0, 1], ["current (A/A mean)", "padd"])
ax[1].set_ylim(0, 135); ax[1].set_ylabel("mean seconds over 20 points")
x = mt["rank_s"]["ratio_boot95"]
ax[1].set_title(f"(b) phases; rank stage {mt['rank_s']['ratio_cur_over_padd']:.4f}x [{x[0]:.4f}, {x[1]:.4f}]", fontsize=10)
ax[1].text(0.5, -0.17, "'other' = cold - setup - rank (derived): linear algebra, target, exit", ha="center",
           transform=ax[1].transAxes, fontsize=7.5, color="#52606d")

# (c) cold IC / rho
r = a["rho"]["ratios"]
for i, (lab, c) in enumerate((("cur", C_CUR), ("padd", C_PADD))):
    v = r[lab]["ic_over_rho_online"]; lo_, hi_ = r[lab]["boot95"]
    ax[2].errorbar([i], [v], yerr=[[v - lo_], [hi_ - v]], fmt="o", color=c, capsize=5, ms=7)
    ax[2].text(i + 0.08, v, f"{v:.1f}\n[{lo_:.1f}, {hi_:.1f}]", fontsize=8.5, va="center")
    s5 = r[lab]["sealed5_ic_over_rho_online"]
    ax[2].plot([i], [s5], "x", color=c, ms=7)
ax[2].axhline(55.4, color="#9aa5b1", ls="--", lw=1)
ax[2].text(1.45, 55.9, "earlier 5-draw figure ~55 (no interval)", fontsize=7.5, ha="right", color="#52606d")
ax[2].set_xticks([0, 1], ["current", "padd"]); ax[2].set_xlim(-0.5, 1.5)
ax[2].set_ylabel("cold IC / matched rho (wall, online)")
ax[2].set_ylim(30, 80)
ax[2].set_title(f"(c) cold IC/rho, {a['rho']['draws']} rho draws (x: 5 sealed points only)", fontsize=10)
ax[2].text(0.5, -0.17, f"rho mean {a['rho']['mean_online_s']:.3f} s, sd/mean {a['rho']['cv']:.2f}; IC wall (4 threads) over rho wall (1 thread)",
           ha="center", transform=ax[2].transAxes, fontsize=7.5, color="#52606d")

fig.suptitle("N83 K1120/root11: padd rank stage vs current producer, paired panel (GitHub ubuntu-24.04, 4 vCPU). " + RUN,
             fontsize=9.5)
fig.savefig(H / "result.svg")
fig.savefig(H / "result.png", dpi=150)
print("ok")
