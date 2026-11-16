#!/usr/bin/env python3
"""Plots the recorded values of R7.1 (no computation of results).

Reads analysis.json (written by `r7_1_complete_cost analyze`) and the raw
modpoly*.json rows; writes the SVG figures next to this file.
Run: python3 plot.py   (matplotlib only).
"""
import glob
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
A = json.load(open(os.path.join(HERE, "analysis.json")))
WEIGHTS = A["weights"]
COL = {
    "ladder (a)": "#1f77b4",
    "psi + factoring (b)": "#d62728",
    "Phi(j,Y) amortised (c)": "#2ca02c",
    "Phi(j,Y) cold (c)": "#ff7f0e",
}
MARK = {"ladder (a)": "o", "psi + factoring (b)": "s",
        "Phi(j,Y) amortised (c)": "^", "Phi(j,Y) cold (c)": "v"}
plt.rcParams.update({"font.size": 8, "axes.grid": True, "grid.alpha": 0.3})


def save(fig, name):
    fig.savefig(os.path.join(HERE, name), format="svg", bbox_inches="tight")
    png_dir = os.environ.get("PLOT_PNG_DIR")  # optional: PNG copies for inspection
    if png_dir:
        fig.savefig(os.path.join(png_dir, name.replace(".svg", ".png")), dpi=110, bbox_inches="tight")
    plt.close(fig)


# ---- fig1: measured costs per route and weight (descending edges).
fig, axs = plt.subplots(2, 2, figsize=(9, 6.5), sharex=True, sharey=True)
for ax, a in zip(axs.ravel(), WEIGHTS):
    for route in COL:
        pts = [(o["ell"], o["costs"][route]) for o in A["ordering"]
               if o["a"] == a and o["dir"] == "desc" and o["costs"][route] is not None]
        pts.sort()
        if pts:
            ax.plot([p[0] for p in pts], [p[1] for p in pts], marker=MARK[route],
                    ms=3.5, lw=1, color=COL[route], label=route)
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_title(f"addition weight a = {a} M", fontsize=9)
for ax in axs[1]:
    ax.set_xlabel("ell")
for ax in axs[:, 0]:
    ax.set_ylabel("cost, M-equivalents  (Meq + a(A+WA))")
axs[0, 0].legend(fontsize=7, loc="upper left")
fig.suptitle("R7.1 kernel-finding cost by route, descending edge (measured; 61-bit p; "
             "counted unit, one instance per ell)", fontsize=9)
save(fig, "fig1_costs_by_route.svg")

# ---- fig2: exponents vs a.
series = [
    ("projected ladder, auto", "l>=2^13", "projected ladder (auto), ell >= 2^13"),
    ("ladder find desc (measured)", "l>=256", "ladder measured, ell >= 256"),
    ("psi + CZ desc (measured)", "grid", "psi + CZ"),
    ("Phi amortised (eval+roots+Elkies) desc", "11..103", "Phi amortised"),
    ("Phi cold (build+eval+roots+Elkies) desc", "11..103", "Phi cold"),
]
fig, axs = plt.subplots(1, len(series), figsize=(11, 3.2))
for ax, (name, rng, label) in zip(axs, series):
    rows = [f for f in A["fits"] if f.get("series") == name and f.get("range") == rng]
    rows.sort(key=lambda f: f["a"])
    xs = list(range(len(rows)))
    ax.errorbar([x - 0.1 for x in xs], [r["exponent"] for r in rows],
                yerr=[[r["exponent"] - r["ols_ci95"][0] for r in rows],
                      [r["ols_ci95"][1] - r["exponent"] for r in rows]],
                fmt="o", ms=3, capsize=2, label="OLS 95%")
    ax.errorbar([x + 0.1 for x in xs], [r["exponent"] for r in rows],
                yerr=[[r["exponent"] - r["bootstrap_ci95"][0] for r in rows],
                      [r["bootstrap_ci95"][1] - r["exponent"] for r in rows]],
                fmt="s", ms=3, capsize=2, label="bootstrap 95%")
    ax.set_xticks(xs)
    ax.set_xticklabels([str(r["a"]) for r in rows])
    ax.set_xlabel("a (M per addition)")
    ax.set_title(label, fontsize=8)
    if name == "projected ladder, auto":
        ax.axhspan(1.5, 2.3, color="grey", alpha=0.15)
        ax.axhline(2.0, color="k", lw=0.6, ls=":")
axs[0].set_ylabel("fitted exponent of ell")
axs[0].legend(fontsize=6)
fig.suptitle("R7.1 fitted exponents by addition weight (shaded: falsifier band 1.5 to 2.3)", fontsize=9, y=1.08)
save(fig, "fig2_exponents_by_weight.svg")

# ---- fig3: crossover with rho at r = 2^128, g = 6.
rows128 = [c for c in A["crossover"] if c["g"] == 6.0 and c["log2_r"] == 128.0]
models = [r["model"] for r in rows128[0]["rows"]]
fig, ax = plt.subplots(figsize=(8.5, 4.2))
w = 0.2
for i, c in enumerate(sorted(rows128, key=lambda c: c["a"])):
    vals = [r["log2_ell_star"] for r in c["rows"]]
    ax.barh([j + (i - 1.5) * w for j in range(len(models))],
            [v if v is not None else 0 for v in vals], height=w, label=f"a = {c['a']}")
ax.set_yticks(range(len(models)))
ax.set_yticklabels(models, fontsize=6)
ax.invert_yaxis()
ax.set_xlabel("log2 ell* (extrapolated crossover with rho, r = 2^128, g = 6)")
ax.legend(fontsize=7)
ax.set_title("R7.1 crossover by route and addition weight (extrapolation of fits; "
             "ell_max = 2^65)", fontsize=9)
save(fig, "fig3_crossover_r128.svg")

# ---- fig4: Phi cost breakdown at a = 0 and a = 1 from the raw rows.
rows = []
for fn in sorted(glob.glob(os.path.join(HERE, "modpoly*.json"))):
    rows += json.load(open(fn))["rows"]
byl = {}
for r in rows:
    byl.setdefault(r["ell"], r)
rows = [byl[k] for k in sorted(byl)]
fig, axs = plt.subplots(1, 2, figsize=(9, 3.6), sharey=True)
for ax, a in zip(axs, (0.0, 1.0)):
    def c(t):
        return t["Meq"] + a * (t["A"] + t["WA"])
    ells = [r["ell"] for r in rows]
    ax.plot(ells, [c(r["build"]) for r in rows], "o-", ms=3, label="Phi_l construction (counted port)")
    ax.plot(ells, [c(r["desc"]["eval"]) for r in rows], "s-", ms=3, label="Phi(j,Y) evaluation")
    ax.plot(ells, [c(r["desc"]["roots"]) for r in rows], "^-", ms=3, label="root finding (desc)")
    ax.plot(ells, [c(r["desc"]["elkies"]) for r in rows], "v-", ms=3, label="Elkies kernel polynomial (desc)")
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel("ell")
    ax.set_title(f"a = {a} M", fontsize=9)
axs[0].set_ylabel("cost, M-equivalents")
axs[0].legend(fontsize=6)
fig.suptitle("R7.1 route (c) cost components (measured, counted unit)", fontsize=9, y=1.04)
save(fig, "fig4_phi_breakdown.svg")
print("wrote fig1..fig4")
