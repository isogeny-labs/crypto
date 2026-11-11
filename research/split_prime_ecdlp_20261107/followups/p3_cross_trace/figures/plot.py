#!/usr/bin/env python3
"""Plots values recorded in run-20261107T2220Z (analysis.json written by p3census analyze; rhoA/rhoB/cov
JSONL) and published-curve z_691 from ../../data/recount.json. Reads and draws only."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__)); R = os.path.join(H, "..", "runs", "run-20261107T2220Z")
L = lambda p: [json.loads(l) for l in open(p) if l.strip()]
fig, ax = plt.subplots(1, 3, figsize=(16, 5.2))
# (a) effects with CI
labels, mids, los, his, cols = [], [], [], [], []
for k, pr in enumerate(("p24", "p28")):
    a = json.load(open(os.path.join(R, pr, "analysis.json")))
    for t in a["tests"]:
        labels.append(f"{pr} {('rho' if t['outcome'].startswith('rho') else 'yield')} {t['covariate']}")
        mids.append(100 * t["slope_frac"]); los.append(100 * t["ci_lo"]); his.append(100 * t["ci_hi"]); cols.append("C0" if k == 0 else "C1")
y = range(len(labels))
for i in y: ax[0].plot([los[i], his[i]], [i, i], color=cols[i], lw=2); ax[0].plot(mids[i], i, "o", color=cols[i])
for x in (-1, 1): ax[0].axvline(x, color="r", ls=":", lw=1)
ax[0].axvline(0, color="grey", lw=0.8); ax[0].set_yticks(list(y)); ax[0].set_yticklabels(labels, fontsize=7); ax[0].invert_yaxis()
ax[0].set_xlabel("effect per covariate SD, % of outcome mean (95 % CI)"); ax[0].set_title("(a) 14 covariate tests: all null\nred: ±1 % decision line; 2,000 curves per prime")
# (b) A/A scatter
for pr, c in (("p24", "C0"), ("p28", "C1")):
    A = {r["idx"]: r["mean_over_sqrt_n"] for r in L(os.path.join(R, pr, "rhoA.jsonl")) if r.get("kind") == "rho"}
    B = {r["idx"]: r["mean_over_sqrt_n"] for r in L(os.path.join(R, pr, "rhoB.jsonl")) if r.get("kind") == "rho"}
    ks = sorted(set(A) & set(B)); r = json.load(open(os.path.join(R, pr, "analysis.json")))["aa_control"]["pearson_r"]
    ax[1].scatter([A[k] for k in ks], [B[k] for k in ks], s=3, alpha=0.4, color=c, label=f"{pr} (r = {r:.3f})")
ax[1].set_xlabel("run A: mean rho ops / √n per curve (256 targets)"); ax[1].set_ylabel("run B (disjoint seeds)")
ax[1].set_title("(b) A/A control: per-curve rho cost is bimodal and\nreproducible: an instrument defect, not a covariate effect"); ax[1].legend(fontsize=8, markerscale=4)
# (c) census z_691 with published curves
rec = json.load(open(os.path.join(H, "..", "..", "..", "data", "recount.json")))
for pr, c in (("p24", "C0"), ("p28", "C1")):
    zs = [r["z_691"] for r in L(os.path.join(R, pr, "cov.jsonl")) if "z_691" in r]
    ax[2].hist(zs, bins=60, histtype="step", color=c, label=f"census {pr} (2,000 random prime-order curves)")
for cv in rec["curves"]:
    z = next(x["z"] for x in cv["cutoffs"] if x["cutoff"] == 691); ax[2].axvline(z, color="k", lw=0.8, alpha=0.6)
    if z < -1.5: ax[2].text(z, ax[2].get_ylim()[1] * 0.9 if ax[2].get_ylim()[1] > 1 else 50, cv["name"].split(" ")[0], rotation=90, fontsize=7, va="top")
ax[2].set_xlabel("z_691 (split primes ℓ ≤ 691 vs conditioned GL2 model)"); ax[2].set_ylabel("curves")
ax[2].set_title("(c) Descriptive: published curves (vertical lines)\nagainst the census at toy primes"); ax[2].legend(fontsize=7)
fig.suptitle("P3 cross-trace covariate test, run-20261107T2220Z (CARD-p3-cross-trace-20261107-2): p24 = 16,777,213, p28 = 268,435,399", fontsize=10)
fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})): fig.savefig(os.path.join(H, "p3_summary." + ext), **kw)
print("ok")
