"""Plot recorded cost ratios (summary.json, a1_m31.json). No computation beyond layout."""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
S = json.load(open(os.path.join(HERE, "summary.json")))
A1 = json.load(open(os.path.join(HERE, "a1_m31.json")))

ARMS = [
    ("b_saturated_sweep_F", "(b) saturated, sweep F", "#c0392b", "o"),
    ("c1_quotient_E_over_T", "(c1) quotient table on E/<T>", "#1f6fb2", "s"),
    ("c2_halfsweep_on_E", "(c2) sweep F/<T> on E", "#58a6d6", "D"),
    ("d_image_2E", "(d) image base in 2E", "#2e8b57", "^"),
]
PRED = {"b_saturated_sweep_F": {2: 1, 3: 2, 4: 4}, "c1_quotient_E_over_T": {2: 1, 3: 1, 4: 1},
        "c2_halfsweep_on_E": {2: 1, 3: 1, 4: 1}, "d_image_2E": {2: 0.5, 3: 0.5, 4: 0.5}}

groups = [g for g in S["groups"] if g["n"] >= 31]
fig, axes = plt.subplots(1, 3, figsize=(13, 4.4), sharey=True)
for ax, key, title in [
    (axes[0], "census_cost_ratio_D", "exact census: U·L_m·r/D_m vs (a)"),
    (axes[1], "oracle_cost_ratio_excl_phi", "library mitm oracle: U·W/H vs (a)\n(φ(R) charge excluded)"),
]:
    for gi, g in enumerate(sorted(groups, key=lambda g: (g["n"], g["m"]))):
        x0 = gi
        for ai, (name, label, col, mk) in enumerate(ARMS):
            arm = next(a for a in g["arms"] if a["arm"] == name)
            y = arm[key]
            ci = arm[key + "_ci95_seeds"] if key + "_ci95_seeds" in arm else None
            xs = x0 + (ai - 1.5) * 0.17
            if ci:
                ax.errorbar(xs, y, yerr=[[y - ci[0]], [ci[1] - y]], fmt=mk, color=col,
                            ms=5, capsize=2, label=label if gi == 0 else None)
            else:
                ax.plot(xs, y, mk, color=col, label=label if gi == 0 else None)
            p = PRED[name][g["m"]]
            ax.hlines(p, xs - 0.07, xs + 0.07, color=col, lw=2.5, alpha=0.45)
    labels = [f"n={g['n']}\nm={g['m']}\n|F|={g['meta']['base_points']}"
              for g in sorted(groups, key=lambda g: (g["n"], g["m"]))]
    ax.set_xticks(range(len(labels)))
    ax.set_xticklabels(labels, fontsize=7)
    ax.set_yscale("log", base=2)
    ax.set_title(title, fontsize=9)
    ax.grid(True, which="major", alpha=0.3)
axes[0].set_ylabel("total relation-collection cost / arm (a)  (log2)")
axes[0].legend(fontsize=7, loc="upper left")

# A1 panel: n = 31 ledger cell, subspace bases.
ax = axes[2]
names = [("x_proj", "x (plain)"), ("u_proj_sweep_F", "u, sweep F"), ("u_c1_quotient", "u, c1 quotient"),
         ("u_c2_halfsweep", "u, c2 half"), ("tz_proj", "trace-zero")]
for mi, r in enumerate(A1["results"]):
    m = r["m"]
    for ni, (nm, lab) in enumerate(names):
        arm = next(a for a in r["arms"] if a["arm"] == nm)
        p, lo, hi = arm["oracle"]["total_ratio_vs_ref_boot95"]
        xs = mi + (ni - 2) * 0.15
        ax.errorbar(xs, p, yerr=[[p - lo], [hi - p]], fmt="o", ms=5, capsize=2,
                    color=plt.cm.viridis(ni / 4.5), label=lab if mi == 0 else None)
ax.set_xticks([0, 1])
ax.set_xticklabels(["m=2", "m=3"])
ax.set_title("§7.A1-R cell icv1-f2m31-tm90707-c95f16f5\noracle U·W/H vs x (φ charge included)", fontsize=9)
ax.legend(fontsize=7)
ax.grid(True, alpha=0.3)
fig.suptitle("R5.1: 2-torsion saturation on Koblitz curves; short bars = Prop. 15.2 / Thm 7 prediction; "
             "error bars = 95% bootstrap over base seeds (panels 1-2) or targets (panel 3)", fontsize=8.5)
fig.tight_layout()
fig.savefig(os.path.join(HERE, "ratios.svg"))
fig.savefig(os.path.join(HERE, "ratios.png"), dpi=130)
