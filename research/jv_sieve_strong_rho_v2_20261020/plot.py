#!/usr/bin/env python3
"""Plots recorded values of summary.json. No result is computed here. Usage: plot.py <dir>"""
import json, math, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
d = sys.argv[1]
sm = json.load(open(f"{d}/summary.json"))
rows = [e for e in sm["sizes"] if "R3" in e.get("rho", {})]
x = [e["log2_l"] for e in rows]
import os
os.makedirs(f"{d}/figs", exist_ok=True)
def save(fig, name):
    fig.savefig(f"{d}/figs/{name}.svg"); fig.savefig(f"{d}/figs/{name}.png", dpi=150); plt.close(fig)
fig, ax = plt.subplots(figsize=(7.2, 4.4))
for i, (lab, k, c, m) in enumerate([("S, harness price", "S_harness", "C0", "o"), ("S, tuned rho price", "S_tuned", "C1", "s"),
                                     ("S⁺, harness price", "S_plus_harness", "C2", "^"), ("S⁺, tuned rho price", "S_plus_tuned", "C3", "v")]):
    y = [e["rho"]["R3"]["ic_over_rho"][k]["ratio"] for e in rows]
    lo = [y[j] - e["rho"]["R3"]["ic_over_rho"][k]["ci95"][0] for j, e in enumerate(rows)]
    hi = [e["rho"]["R3"]["ic_over_rho"][k]["ci95"][1] - y[j] for j, e in enumerate(rows)]
    ax.errorbar([v + 0.12 * (i - 1.5) for v in x], y, yerr=[lo, hi], fmt=m + "-", color=c, label=lab, capsize=3, lw=1)
ax.axhline(1, color="k", ls="--", lw=1); ax.set_yscale("log")
ax.set_xlabel("log₂ ℓ (subgroup order)"); ax.set_ylabel("sieve / strong rho (paired by instance seed)")
ax.set_title("V2 replication, seeds 17–32: sieve vs strong rho, weak class\n95% bootstrap intervals; 1 = parity", fontsize=9)
ax.legend(fontsize=8); ax.grid(alpha=0.3); save(fig, "ratio_vs_l")
fig, ax = plt.subplots(figsize=(7.2, 4.4))
ic = [math.log2(e["ic_muls"]["mean"]) for e in rows]
rh = [math.log2(e["rho"]["R3"]["muls_harness"]["mean"]) for e in rows]
f1, f2, fr = sm["exponent_F1_all_sizes_sieve"], sm["exponent_F2_p_ge_1009_sieve"], sm["exponent_rho_R3_harness"]
ax.plot(x, ic, "o-", label=f"sieve: F1 {f1['slope']:.3f} [{f1['ci95'][0]:.3f}, {f1['ci95'][1]:.3f}]; F2 (p >= 1009) {f2['slope']:.3f} [{f2['ci95'][0]:.3f}, {f2['ci95'][1]:.3f}]")
ax.plot(x, rh, "s-", label=f"strong rho, harness price: {fr['slope']:.3f} [{fr['ci95'][0]:.3f}, {fr['ci95'][1]:.3f}]")
ax.set_xlabel("log₂ ℓ"); ax.set_ylabel("log₂ total F_p multiplications (mean of 16)")
ax.set_title("Cost against ℓ (theory: ℓ^{1/3} sieve, ℓ^{1/2} rho)", fontsize=9); ax.legend(fontsize=7); ax.grid(alpha=0.3); save(fig, "exponent")
fig, ax = plt.subplots(figsize=(7.2, 4.4))
names = {"R0": "R0 plain", "R1": "R1 batched", "R2": "R2 negation", "R3": "R3 negation+batched"}
sizes = [e for e in sm["sizes"] if all(a in e.get("rho", {}) for a in names)]
for k, e in enumerate(sizes):
    for j, a in enumerate(names):
        s = e["rho"][a]["S_harness"]
        ax.errorbar(k + 0.18 * (j - 1.5), s["mean"], yerr=[[s["mean"] - s["ci95"][0]], [s["ci95"][1] - s["mean"]]], fmt="o", color=f"C{j}", capsize=3, label=names[a] if k == 0 else None)
ax.set_xticks(range(len(sizes))); ax.set_xticklabels([f"p={e['p']}" for e in sizes])
ax.set_ylabel("S (harness price), mean ± 95% CI, n = 16"); ax.set_title("Rho in this tower: negation and batched inversion", fontsize=9)
ax.legend(fontsize=8, ncol=4, loc="upper center", bbox_to_anchor=(0.5, -0.08)); fig.tight_layout(); save(fig, "rho_factorial")
fig, ax = plt.subplots(figsize=(7.2, 4.4))
ds = [e for e in sm["sizes"] if e.get("dense_vs_sparse")]
for lab, k, c in (("S", "S_dense_over_sparse", "C0"), ("descent linear stage", "linear_stage_dense_over_sparse", "C1")):
    y = [e["dense_vs_sparse"][k]["ratio"] for e in ds]
    lo = [e["dense_vs_sparse"][k]["ratio"] - e["dense_vs_sparse"][k]["ci95"][0] for e in ds]
    hi = [e["dense_vs_sparse"][k]["ci95"][1] - e["dense_vs_sparse"][k]["ratio"] for e in ds]
    ax.errorbar([e["log2_l"] for e in ds], y, yerr=[lo, hi], fmt="o-", color=c, capsize=3, label=lab)
ax.axhline(1, color="k", ls="--", lw=1); ax.set_xlabel("log₂ ℓ"); ax.set_ylabel("dense / sparse (paired)")
ax.set_title("Sparse FGLM in the sieve's descent", fontsize=9); ax.legend(fontsize=8); ax.grid(alpha=0.3); save(fig, "dense_vs_sparse")
print("figures written")
