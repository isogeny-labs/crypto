"""Plot recorded values of lead R6.11 (summary.json, r6_11_ell*_i*.json).

Computes nothing new: means/sd/intervals are read from summary.json
(written by `r6_11_conductor_yield summarise`), per-curve c values from the
instance files.  Usage: python plot.py <dir>
"""
import json, sys, glob, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

d = sys.argv[1]
S = json.load(open(os.path.join(d, "summary.json")))
ells = S["ells"]
sizes = S["sizes"]
rows = S["rows"]

def row(ell, rule, size, m):
    return next(r for r in rows if r["ell"] == ell and r["rule"] == rule and r["size"] == size and r["m"] == m)

# Fig 1: crater/floor class-factor ratio vs ℓ, mean ± 2 sd across instances.
fig, axes = plt.subplots(1, 2, figsize=(10, 4), sharey=True)
colors = {60: "#1b6ca8", 80: "#2a9d8f", 100: "#e76f51", 200: "#7f7f7f"}
for ax, m in zip(axes, ["m2", "m3"]):
    for k, size in enumerate(sizes):
        xs = [e + (k - 1.5) * 0.18 for e in ells]
        mu = [row(e, "smallx", size, m)["ratio0_mean"] for e in ells]
        lo = [mu[i] - row(e, "smallx", size, m)["interval_2sd"][0] for i, e in enumerate(ells)]
        hi = [row(e, "smallx", size, m)["interval_2sd"][1] - mu[i] for i, e in enumerate(ells)]
        ax.errorbar(xs, mu, yerr=[lo, hi], fmt="o" if size <= 100 else "s", color=colors[size],
                    capsize=3, label=f"|F| = {size}" + (" (suppl.)" if size > 100 else ""))
        # individual instances
        for i, e in enumerate(ells):
            pts = [q["ratio_crater_floor0"] for q in row(e, "smallx", size, m)["instances"]]
            ax.plot([xs[i]] * len(pts), pts, ".", color=colors[size], alpha=0.35, ms=4)
    ax.axhline(1.0, color="k", lw=0.8, ls="--")
    ax.set_xticks(ells)
    ax.set_xlabel("ℓ (prime conductor gap)")
    ax.set_title(f"{'m = 2' if m == 'm2' else 'm = 3'}: c_crater / c_floor (smallx base)")
    ax.grid(alpha=0.3)
axes[0].set_ylabel("class-factor ratio crater / floor₀")
axes[0].legend(fontsize=8)
n_inst = S["n_instances"]
fig.suptitle(f"R6.11: relation-yield class factor across a prime conductor gap; "
             f"{n_inst} instances (p ∈ [2^20, 2^22), h = 1), exact census; bars = mean ± 2 sd over instances, dots = instances", fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(d, "fig1_ratio_vs_ell.svg"))
fig.savefig(os.path.join(d, "fig1_ratio_vs_ell.png"), dpi=150)

# Fig 2: c / c_birthday per curve vs |F|, crater vs floor, all ℓ pooled.
files = sorted(glob.glob(os.path.join(d, "r6_11_ell*_i*.json")))
fig, axes = plt.subplots(1, 2, figsize=(10, 4))
for ax, m in zip(axes, ["m2", "m3"]):
    for lvl, col, mk in [("crater", "#e76f51", "o"), ("floor", "#1b6ca8", "x")]:
        xs, ys = [], []
        for fpath in files:
            R = json.load(open(fpath))
            curves = [R["crater"]] if lvl == "crater" else R["floors"]
            for c in curves:
                for cen in c["census"]:
                    if cen["rule"] != "smallx":
                        continue
                    xs.append(cen["size"] + (2 if lvl == "crater" else -2))
                    ys.append(cen[m]["c"] / cen[m]["c_birthday_derived"])
        ax.plot(xs, ys, mk, color=col, alpha=0.5, ms=4, label=f"{lvl} ({len(ys)} curve×size)")
    ax.axhline(1.0, color="k", lw=0.8, ls="--")
    ax.set_xlabel("|F| (signed base points)")
    ax.set_ylabel("c / c* (c* = birthday reference, derived)")
    ax.set_title(f"{'m = 2' if m == 'm2' else 'm = 3'}: measured class factor over birthday reference")
    ax.grid(alpha=0.3)
    ax.legend(fontsize=8)
fig.suptitle("R6.11: every curve of every instance (smallx rule); crater and floor interleave", fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(d, "fig2_c_over_birthday.svg"))
fig.savefig(os.path.join(d, "fig2_c_over_birthday.png"), dpi=150)
print("wrote fig1_ratio_vs_ell.svg fig2_c_over_birthday.svg")
