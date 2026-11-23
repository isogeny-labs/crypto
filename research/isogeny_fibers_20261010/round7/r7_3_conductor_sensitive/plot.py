"""Plot recorded values of lead R7.3 (raw/summary.json and raw/r7_3_*.json).

Computes nothing new: every ratio and interval is read from summary.json
(written by `r7_3_conductor_sensitive summarise`, cluster bootstrap over
instances); the per-instance dots come from the instance files.
Usage: python plot.py <raw-dir> <out-dir>
"""
import glob
import json
import math
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

raw, out = sys.argv[1], sys.argv[2]
S = json.load(open(os.path.join(raw, "summary.json")))
A = [json.load(open(f)) for f in sorted(glob.glob(os.path.join(raw, "r7_3_expA_*.json")))]
B = [json.load(open(f)) for f in sorted(glob.glob(os.path.join(raw, "r7_3_expB_*.json")))]

C_CR, C_FL, C_K = "#e76f51", "#1b6ca8", "#444444"


def ci_bar(ax, y, d, color, label=None, marker="o"):
    lo, hi = d["ci95"]
    ax.errorbar([d["geo_mean"]], [y], xerr=[[d["geo_mean"] - lo], [hi - d["geo_mean"]]],
                fmt=marker, color=color, capsize=3, label=label)


# ---------------------------------------------------------------- Fig 1: Exp A
fig, axes = plt.subplots(1, 2, figsize=(11, 4.6), sharex=True)
for ax, dd in zip(axes, [-3, -4]):
    rows = [r for r in S["expA"] if r["D"] == dd and r["metric"] == "mean_iters"]
    groups = [r["group"] for r in rows]
    ys = list(range(len(groups)))
    for y, r in zip(ys, rows):
        ci_bar(ax, y + 0.22, r["aut_over_pm_on_crater"], C_CR, "crater: aut / pm (known Aut effect)" if y == 0 else None, "s")
        ci_bar(ax, y, r["crater_pm_over_floor_pm_conductor_effect"], C_K, "crater pm / floor pm (conductor effect)" if y == 0 else None)
        ci_bar(ax, y - 0.22, r["total_times_predicted_baseline_removed"], C_FL, "(crater aut / floor pm) x sqrt(|Aut|/2)" if y == 0 else None, "^")
    pred = rows[0]["predicted_aut_effect"]
    ax.axvline(1.0, color="k", lw=0.8, ls="--")
    ax.axvline(pred, color=C_CR, lw=0.8, ls=":")
    ax.set_yticks(ys)
    ax.set_yticklabels(groups)
    ax.set_xscale("log")
    ax.set_xlabel("ratio of mean rho iterations (geometric mean, 95% cluster-bootstrap CI)")
    n = len([a for a in A if a["D"] == dd])
    ax.set_title(f"D = {dd} (j = {'0, |Aut| = 6' if dd == -3 else '1728, |Aut| = 4'}), {n} instances; dotted = 1/sqrt(|Aut|/2)", fontsize=9)
    ax.grid(alpha=0.3)
axes[0].legend(fontsize=7, loc="lower left")
fig.suptitle("R7.3 Exp A: automorphism-class rho, crater vs floor (iterations; p in [2^20, 2^22), matched seeds)", fontsize=10)
fig.tight_layout()
fig.savefig(os.path.join(out, "fig1_expA_aut_baseline.svg"))
fig.savefig(os.path.join(out, "fig1_expA_aut_baseline.png"), dpi=140)

# ---------------------------------------------------------------- Fig 2: Exp B
fig, axes = plt.subplots(1, 2, figsize=(11, 4.6), sharex=True)
rowsB = S["expB"]
groups = [r["group"] for r in rowsB]
ys = list(range(len(groups)))
for ax, metric, title in zip(axes, ["mean_iters", "mean_mults"], ["rho iterations", "counted F_p multiplications"]):
    for y, r in zip(ys, rowsB):
        ci_bar(ax, y + 0.2, r[f"plain_{metric}_crater_over_floor"], C_K, "plain walk" if y == 0 else None)
        ci_bar(ax, y, r[f"eps_{metric}_crater_over_floor"], C_CR, "endomorphism-mixed walk (eps)" if y == 0 else None, "s")
        ci_bar(ax, y - 0.2, r[f"eps_over_plain_{metric}_crater"], C_FL, "crater: eps / plain" if y == 0 else None, "^")
    ax.axvline(1.0, color="k", lw=0.8, ls="--")
    ax.set_yticks(ys)
    ax.set_yticklabels(groups)
    ax.set_xscale("log")
    ax.set_xlabel(f"ratio of mean {title} (geo. mean, 95% CI)")
    ax.set_title(f"crater / floor ({title})", fontsize=9)
    ax.grid(alpha=0.3)
axes[0].legend(fontsize=7, loc="lower left")
fig.suptitle(f"R7.3 Exp B: endomorphism-mixed rho, {S['expB_instances']} instances, h = 1 craters", fontsize=10)
fig.tight_layout()
fig.savefig(os.path.join(out, "fig2_expB_endo_walk.svg"))
fig.savefig(os.path.join(out, "fig2_expB_endo_walk.png"), dpi=140)

# ---------------------------------------------------------------- Fig 3: eps cost + GLV
fig, axes = plt.subplots(1, 2, figsize=(11, 4.2))
ax = axes[0]
for r in B:
    ell = r["ell"]
    ax.plot([ell - 0.15], [r["crater"]["mults_per_eps_eval"]], "o", color=C_CR, alpha=0.6, ms=4)
    fl = [f["mults_per_eps_eval"] for f in r["floors"]]
    ax.plot([ell + 0.15] * len(fl), fl, "x", color=C_FL, alpha=0.5, ms=4)
ax.plot([], [], "o", color=C_CR, label="crater: omega (degree (1+|D|)/4)")
ax.plot([], [], "x", color=C_FL, label="floor: psi.omega.phi (degree l^2 (1+|D|)/4)")
ax.axhline(37, color="k", lw=0.8, ls=":", label="one affine addition (37 mults)")
ax.set_xticks([5, 7, 11, 13])
ax.set_xlabel("l")
ax.set_ylabel("F_p multiplications per endomorphism evaluation")
ax.legend(fontsize=7)
ax.grid(alpha=0.3)
ax.set_title("measured cost of the cheapest explicit endomorphism", fontsize=9)
ax = axes[1]
for r in B:
    c = r["crater"]["glv"]["mean_log2_max_k"]
    f = sum(x["glv"]["mean_log2_max_k"] for x in r["floors"]) / len(r["floors"])
    ax.plot([c], [f], "o", color=C_K, alpha=0.6, ms=4)
lo = min(min(r["crater"]["glv"]["mean_log2_max_k"] for r in B), 5)
hi = max(r["crater"]["glv"]["log2_sqrt_r"] for r in B)
ax.plot([lo, hi], [lo, hi], "k--", lw=0.8)
ax.set_xlabel("crater: mean log2 max(|k1|,|k2|) of the GLV split")
ax.set_ylabel("floor (mean over floors)")
ax.grid(alpha=0.3)
ax.set_title("GLV decomposition length, per instance (dashed: equal)", fontsize=9)
fig.suptitle("R7.3: primitive costs that depend on End(E) (known-scalar, not unknown-scalar, quantities)", fontsize=10)
fig.tight_layout()
fig.savefig(os.path.join(out, "fig3_eps_cost_glv.svg"))
fig.savefig(os.path.join(out, "fig3_eps_cost_glv.png"), dpi=140)

# ---------------------------------------------------------------- Fig 4: Exp H
rowsH = [r for r in S["expH"] if r["m"] == "c3" and r["size"] == 200]
if rowsH:
    fig, ax = plt.subplots(figsize=(7, 3.6))
    ys = list(range(len(rowsH)))
    for y, r in zip(ys, rowsH):
        d = r["crater_over_floor_class_factor"]
        if "ci95" in d:
            ci_bar(ax, y, d, C_K)
    ax.axvline(1.0, color="k", lw=0.8, ls="--")
    ax.set_yticks(ys)
    ax.set_yticklabels([f"{r['group']} (n={r['n_instances']}, h={sorted(r['h_D_values'])})" for r in rowsH], fontsize=8)
    ax.set_xlabel("crater / floor class factor c (m = 3, |F| = 200), geo. mean, 95% CI")
    ax.set_title("R7.3 Exp H: craters with h(D) > 1", fontsize=10)
    ax.grid(alpha=0.3)
    fig.tight_layout()
    fig.savefig(os.path.join(out, "fig4_expH_class_number.svg"))
    fig.savefig(os.path.join(out, "fig4_expH_class_number.png"), dpi=140)
print("figures written")
