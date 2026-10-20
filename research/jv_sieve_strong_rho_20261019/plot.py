#!/usr/bin/env python3
"""Plots recorded values of summary.json (computed by examples/jv_sieve_study.rs).
No result is computed here. Usage: plot.py <study dir>"""
import json, math, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

d = sys.argv[1]
sm = json.load(open(f"{d}/summary.json"))
all_rows = sm.get("sizes", [])

def eligible(e):
    # The analyzer must set this only when the required rows, paired seed
    # identities, correctness/cap checks, and frozen controls support a claim.
    # Older summaries without the field are descriptive-only.
    return e.get("comparison_status") == "eligible"

rows = [e for e in all_rows if eligible(e) and "rho" in e and "R3" in e["rho"]]
x = [e["log2_l"] for e in rows]

def withheld(ax, reason):
    ax.axis("off")
    ax.text(0.5, 0.5, reason, ha="center", va="center", wrap=True,
            transform=ax.transAxes, fontsize=10)

def sample_label(e, obj):
    n = obj.get("n") if isinstance(obj, dict) else None
    return str(n) if isinstance(n, int) else "unknown"

def save(fig, name):
    fig.savefig(f"{d}/figs/{name}.svg")
    fig.savefig(f"{d}/figs/{name}.png", dpi=150)
    plt.close(fig)

# 1. paired ratio sieve / strong rho against l; only validated rows are shown.
fig, ax = plt.subplots(figsize=(7.2, 4.4))
if rows:
    series = [("S, harness price", "S_harness", "C0", "o"), ("S, tuned rho price", "S_tuned", "C1", "s"),
              ("S⁺, harness price", "S_plus_harness", "C2", "^"), ("S⁺, tuned rho price", "S_plus_tuned", "C3", "v")]
    for i, (lab, k, c, m) in enumerate(series):
        vals = [e["rho"]["R3"]["ic_over_rho"][k] for e in rows]
        y = [v["ratio"] for v in vals]
        lo = [y[j] - vals[j]["ci95"][0] for j in range(len(rows))]
        hi = [vals[j]["ci95"][1] - y[j] for j in range(len(rows))]
        xs = [v + 0.12 * (i - 1.5) for v in x]
        ax.errorbar(xs, y, yerr=[lo, hi], fmt=m + "-", color=c, label=lab, capsize=3, lw=1)
    ax.axhline(1, color="k", ls="--", lw=1)
    ax.set_yscale("log")
    ax.set_xlabel("log₂ ℓ (subgroup order)")
    ax.set_ylabel("sieve / strong rho (paired by instance)")
    ns = sorted({sample_label(e, e["rho"]["R3"]["ic_over_rho"]["S_harness"]) for e in rows})
    ax.set_title(f"Joux–Vitse sieve vs strong rho (negation + batching + DPs), weak class\n"
                 f"validated sizes: {len(rows)}; paired n per ratio: {', '.join(ns)}; 95% intervals", fontsize=9)
    ax.legend(fontsize=8)
    ax.grid(alpha=0.3)
else:
    withheld(ax, "Comparative ratios withheld: no size has explicit comparison_status=eligible.")
save(fig, "ratio_vs_l")

# 2. exponent
fig, ax = plt.subplots(figsize=(7.2, 4.4))
fit_eligible = sm.get("exponent_status") == "eligible"
if fit_eligible and rows:
    ic = [math.log2(e["ic_muls"]["mean"]) for e in rows]
    rh = [math.log2(e["rho"]["R3"]["muls_harness"]["mean"]) for e in rows]
    rt = [math.log2(e["rho"]["R3"]["muls_tuned"]["mean"]) for e in rows]
    fx, fr, ft = sm["exponent_ic_muls_vs_l"], sm["exponent_rho_R3_harness_muls_vs_l"], sm["exponent_rho_R3_tuned_muls_vs_l"]
    ax.plot(x, ic, "o-", label=f"sieve: slope {fx['slope']:.3f} [{fx['ci95'][0]:.3f}, {fx['ci95'][1]:.3f}]")
    ax.plot(x, rh, "s-", label=f"strong rho, harness price: {fr['slope']:.3f} [{fr['ci95'][0]:.3f}, {fr['ci95'][1]:.3f}]")
    ax.plot(x, rt, "^-", label=f"strong rho, tuned price: {ft['slope']:.3f} [{ft['ci95'][0]:.3f}, {ft['ci95'][1]:.3f}]")
    ax.set_xlabel("log₂ ℓ")
    ax.set_ylabel("log₂ total F_p multiplications")
    ax.set_title(f"Cost scaling from {len(rows)} eligible size rows", fontsize=9)
    ax.legend(fontsize=8)
    ax.grid(alpha=0.3)
else:
    withheld(ax, "Exponent-fit plot withheld: analyzer did not mark the complete fit eligible.")
save(fig, "exponent")

# 3. rho factorial (negation x batching) at every size that ran it
fig, ax = plt.subplots(figsize=(7.2, 4.4))
names = {"R0": "R0 plain", "R1": "R1 batched", "R2": "R2 negation", "R3": "R3 negation+batched"}
sizes = [e for e in rows if all(a in e["rho"] for a in names)]
for k, e in enumerate(sizes):
    for j, a in enumerate(names):
        s = e["rho"][a]["S_harness"]
        ax.errorbar(k + 0.18 * (j - 1.5), s["mean"], yerr=[[s["mean"] - s["ci95"][0]], [s["ci95"][1] - s["mean"]]],
                    fmt="o", color=f"C{j}", capsize=3, label=names[a] if k == 0 else None)
ax.set_xticks(range(len(sizes)))
ax.set_xticklabels([f"p={e['p']}" for e in sizes])
if sizes:
    ax.set_ylabel("S (harness price), mean ± 95% CI")
    ax.set_title(f"Rho factorial comparison: {len(sizes)} eligible sizes", fontsize=9)
    ax.legend(fontsize=8)
    ax.grid(alpha=0.3)
else:
    withheld(ax, "Rho factorial comparisons withheld: no complete eligible rows for all four arms.")
save(fig, "rho_factorial")

# 4. dense vs sparse
fig, ax = plt.subplots(figsize=(7.2, 4.4))
ds = [e for e in rows if e.get("dense_vs_sparse")]
for lab, k, c in (("S", "S_dense_over_sparse", "C0"), ("linear-stage cost", "linear_stage_dense_over_sparse", "C1")):
    y = [e["dense_vs_sparse"][k]["ratio"] for e in ds]
    lo = [e["dense_vs_sparse"][k]["ratio"] - e["dense_vs_sparse"][k]["ci95"][0] for e in ds]
    hi = [e["dense_vs_sparse"][k]["ci95"][1] - e["dense_vs_sparse"][k]["ratio"] for e in ds]
    ax.errorbar([e["log2_l"] for e in ds], y, yerr=[lo, hi], fmt="o-", color=c, capsize=3, label=lab)
ax.axhline(1, color="k", ls="--", lw=1)
ax.set_xlabel("log₂ ℓ")
if ds:
    ax.set_ylabel("dense / sparse (validated seed-matched pairs)")
    ax.set_title(f"Dense vs sparse: {len(ds)} eligible size rows", fontsize=9)
    ax.legend(fontsize=8)
    ax.grid(alpha=0.3)
else:
    withheld(ax, "Dense/sparse comparisons withheld: no complete eligible size rows.")
save(fig, "dense_vs_sparse")
print("figures written")
