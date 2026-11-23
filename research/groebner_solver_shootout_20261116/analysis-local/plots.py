#!/usr/bin/env python3
"""Layout only: draws figures from analysis-local/out (values computed by analyze_amend1.rs
or recorded in rows.jsonl). No statistic is computed here except log2 of recorded per-target
CPU pairs for display. Run with a matplotlib venv: plots.py <analysis-local dir>."""
import csv, json, math, sys
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

d = Path(sys.argv[1]); out = d / "out"; fig = d / "figures"
s = json.loads((out / "summary.unblinded.json").read_text())
P = ["inherited-f4", "msolve-f4", "cms-xor"]
COL = {"inherited-f4": "#2a6f97", "msolve-f4": "#c0392b", "cms-xor": "#7d8c2a"}
CAP = "GB-SOLVER-SHOOTOUT-20261116 amendment-1, run-local (2026-11-17T07:23Z), K17-m3-plain-l5-a1;\nLOCAL DIAGNOSTIC, non-admitted, [pcc:unverified]"
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none"})

def save(f, name):
    f.savefig(fig / f"{name}.svg"); f.savefig(fig / f"{name}.png", dpi=150); plt.close(f)

# 1. stratified relations per CPU-second with 95% bootstrap CI
f, ax = plt.subplots(figsize=(6.4, 2.8))
for i, p in enumerate(P):
    r = s["pipelines"][p]["rate"]; lo, hi = r["ci95"]
    ax.errorbar(r["rel_per_cpu_s"], i, xerr=[[r["rel_per_cpu_s"] - lo], [hi - r["rel_per_cpu_s"]]], fmt="o", color=COL[p], capsize=4)
    ax.annotate(f'{r["rel_per_cpu_s"]:.3g} [{lo:.3g}, {hi:.3g}]', (r["rel_per_cpu_s"], i), xytext=(6, 6), textcoords="offset points")
ax.set_yticks(range(3), P); ax.set_xscale("log"); ax.set_ylim(-0.6, 2.8)
ax.set_xlabel("verified relations per charged CPU-s (stratified; 95% bootstrap CI, 20000 resamples)")
ax.set_title("Primary endpoint, n = 80 SAT + 80 UNSAT natural targets per arm", fontsize=9)
f.text(0.01, 0.005, CAP, fontsize=6.5); f.tight_layout(rect=(0, 0.08, 1, 1)); save(f, "fig1_rel_per_cpu")

# 2. per-target charged CPU (mean of passes), by stratum
rows = list(csv.DictReader((out / "per_target.tsv").open(), delimiter="\t"))
f, ax = plt.subplots(figsize=(6.4, 3.4))
for i, p in enumerate(P):
    for j, stv in enumerate(["sat", "unsat"]):
        ys = [(float(r["cpu_pass_a"]) + float(r["cpu_pass_b"])) / 2 for r in rows if r["pipeline"] == p and r["stratum"] == stv]
        xs = [i * 3 + j + (k % 20 - 10) * 0.025 for k in range(len(ys))]
        ax.scatter(xs, ys, s=5, color=COL[p], alpha=0.6 if stv == "sat" else 0.3, marker="o" if stv == "sat" else "^")
ax.set_xticks([i * 3 + j for i in range(3) for j in range(2)], [f"{p}\n{stv}" for p in P for stv in ("SAT", "UNSAT")], fontsize=7)
ax.set_yscale("log"); ax.set_ylabel("charged CPU per target (s), mean of passes")
ax.axhline(60, ls="--", color="grey", lw=0.8); ax.text(0, 62, "60 s cap (no call reached it)", fontsize=7)
ax.set_title("Per-target charged CPU, 160 natural targets per arm", fontsize=9)
f.text(0.01, 0.005, CAP, fontsize=6.5); f.tight_layout(rect=(0, 0.08, 1, 1)); save(f, "fig2_per_target_cpu")

# 3. rho band distances (log2) per arm
f, ax = plt.subplots(figsize=(6.4, 3.0))
keys = [("w=0.1", "w=0.1"), ("w=1", "w=1"), ("clock", "clock")]
for i, p in enumerate(P):
    rho = s["pipelines"][p]["rho"]
    for j, (k, lab) in enumerate(keys):
        v = rho[k]["log2_distance"]
        ax.bar(i * 4 + j, v, color=COL[p], alpha=[0.45, 0.7, 1.0][j])
        ax.text(i * 4 + j, v + 0.4, f"{v:.1f}", ha="center", fontsize=7)
    ax.text(i * 4 - 0.9, 0.5, "w=0:\nratio 0", fontsize=6, color="grey")
ax.set_xticks([i * 4 + j for i in range(3) for j in range(3)], [lab for p in P for _, lab in keys], fontsize=7)
for i, p in enumerate(P): ax.text(i * 4 + 1, -6.5, p, ha="center", fontsize=8, color=COL[p])
ax.axhline(20, ls=":", color="black", lw=0.8); ax.text(9.6, 20.4, "M8 abandon line 2^20 (clock)", fontsize=6, ha="right")
ax.set_ylabel("log2(24-relation phase / rho)")
ax.set_title("Distance remaining to matched rho (IC lower bound; w = 0 gives 0 for every solver)", fontsize=8.5)
f.text(0.01, 0.005, CAP, fontsize=6.5); f.tight_layout(rect=(0, 0.12, 1, 1)); save(f, "fig3_rho_band")

# 4. A/A per-target log2(pass A / pass B)
f, axs = plt.subplots(1, 3, figsize=(6.4, 2.6), sharey=True)
for ax, p in zip(axs, P):
    v = [math.log2(float(r["cpu_pass_a"]) / float(r["cpu_pass_b"])) for r in rows if r["pipeline"] == p]
    ax.hist(v, bins=30, color=COL[p]); fl = s["aa"][p]["aa_floor_log2"]
    ax.axvline(fl, ls="--", color="k", lw=0.7); ax.axvline(-fl, ls="--", color="k", lw=0.7)
    ax.set_title(f"{p}\nA/A floor {fl:.3f}", fontsize=8); ax.set_xlabel("log2(CPU A / CPU B)", fontsize=7)
axs[0].set_ylabel("targets (of 160)")
f.text(0.01, 0.005, CAP, fontsize=6.5); f.tight_layout(rect=(0, 0.1, 1, 1)); save(f, "fig4_aa_noise")
print("figures written to", fig)
