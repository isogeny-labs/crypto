"""Plots recorded values only: matched.json (icprog rule matched) and history.json.
Computes nothing new; the fit lines are the slopes and intercepts the read-out printed."""
import json, math, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = sys.argv[1]
m = json.load(open(f"{here}/matched.json"))
h = json.load(open(f"{here}/history.json"))
hist = {e["name"]: e for e in h["entries"]}
sl = m["slope_of_ln_cold_ratio_on_ln_r"]

fig, ax = plt.subplots(figsize=(7.2, 4.6))
def pts(sizes, f):
    return [(s["log2_r"], f(s)) for s in sorted(sizes, key=lambda s: s["log2_r"])]
for name, col, mk in [("v3", "#999999", "s"), ("v7", "#4c78a8", "^")]:
    p = pts(hist[name]["sizes"], lambda s: s["cold_ratio"])
    ax.plot(*zip(*p), mk + "--", color=col, label=f"rule {name} (rho as then)", ms=5, lw=1)
for arm, key, col, mk, lab in [("control", "cold_ratio_control", "#f58518", "o", "v8c control: rho as at v7, same binary"),
                               ("matched", "cold_ratio_matched", "#e45756", "D", "v8 matched rho")]:
    sz = sorted(m["sizes"], key=lambda s: s["log2_r"])
    x = [s["log2_r"] for s in sz]; y = [s[key]["value"] for s in sz]
    lo = [s[key]["value"] - s[key]["ci95"][0] for s in sz]; hi = [s[key]["ci95"][1] - s[key]["value"] for s in sz]
    ax.errorbar(x, y, yerr=[lo, hi], fmt=mk + "-", color=col, label=lab, ms=5, lw=1.2, capsize=2)
    e = sl[arm]["extrapolation"]
    ax.plot([x[-1], 50, 60], [y[-1], e[0]["point"], e[1]["point"]], ":", color=col, lw=1)
ax.set_xscale("linear"); ax.set_yscale("log")
ax.axvline(50, color="#bbbbbb", lw=0.8); ax.text(50.2, 2.6, "50 bits: no\nsingle-word\ncurve", fontsize=7, color="#777777")
ax.axvline(47.21, color="#dddddd", lw=0.6)
ax.set_xlabel("log2 r (prime subgroup order)"); ax.set_ylabel("cold time, index calculus / rho  (above 1: rho faster)")
ax.set_title("Cold IC/rho by size; dotted = fit extrapolation (a prediction)", fontsize=9)
ax.grid(True, which="both", alpha=0.25); ax.legend(fontsize=7, loc="upper left")
fig.text(0.01, 0.005, "rule v8, 64 targets a size, 95% bootstrap over targets; Emerald Rapids container; source: matched.json, history.json", fontsize=6)
fig.tight_layout(rect=(0, 0.02, 1, 1))
fig.savefig(f"{here}/report/cold-ratio.svg"); fig.savefig(f"{here}/report/cold-ratio.png", dpi=150)

fig, ax = plt.subplots(figsize=(7.2, 3.6))
sz = sorted(m["sizes"], key=lambda s: s["log2_r"])
x = list(range(len(sz)))
for off, key, col, lab in [(-0.2, "rho_online_control_over_matched", "#54a24b", "rho online time, control / matched"),
                            (0.2, "cold_matched_over_control", "#e45756", "cold IC/rho, matched / control")]:
    y = [s[key]["value"] for s in sz]
    lo = [s[key]["value"] - s[key]["ci95"][0] for s in sz]; hi = [s[key]["ci95"][1] - s[key]["value"] for s in sz]
    ax.bar([i + off for i in x], y, 0.4, yerr=[lo, hi], color=col, label=lab, capsize=2)
ic = [s["ic_online_matched_over_control"]["value"] for s in sz]
ax.plot(x, ic, "k_", ms=14, label="IC online time, v8 / v8c (identical code)")
ax.axhline(1, color="k", lw=0.6)
ax.set_xticks(x); ax.set_xticklabels([f'{s["log2_r"]}\nn={s["n"]}' for s in sz], fontsize=8)
ax.set_ylabel("ratio"); ax.legend(fontsize=7, loc="upper left"); ax.set_ylim(0, 2.2)
ax.set_title("Same binary, same 64 targets: what matching rho changed", fontsize=9)
fig.tight_layout(); fig.savefig(f"{here}/report/paired.svg"); fig.savefig(f"{here}/report/paired.png", dpi=150)
