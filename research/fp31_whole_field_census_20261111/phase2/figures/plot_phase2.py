#!/usr/bin/env python3
"""Lays out recorded values of phase2/analysis.json as SVG; computes nothing new
beyond the standard error sd/sqrt(n) of recorded per-class values for whiskers.
Usage: plot_phase2.py analysis.json outdir"""
import json, sys, math
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
A = json.load(open(sys.argv[1])); out = sys.argv[2]
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none"})
cols = {"S1": "#1f77b4", "S2": "#ff7f0e", "S3": "#2ca02c", "R": "#9467bd", "J0": "#d62728"}
C = A["classes"]

# 1. per-class mean c with SE whiskers
fig, ax = plt.subplots(figsize=(7.2, 3.4))
for i, c in enumerate(C):
    se = c["sd_c"] / math.sqrt(c["solves_isogenous"])
    ax.errorbar(i, c["mean_c"], yerr=2 * se, fmt="o", ms=3, color=cols[c["stratum"]], elinewidth=0.8)
ax.axhline(A["class_means"]["mean"], color="k", lw=0.6, ls="--")
for st, col in cols.items():
    ax.plot([], [], "o", color=col, label=st)
ax.set_xlabel("class (manifest order; S1, S2, S3: prime order by number of endomorphism rings; R: cofactor 4-64; J0: j = 0)")
ax.set_ylabel("mean iters / sqrt(pi r/2)  (+-2 SE)")
ax.legend(ncol=5, fontsize=7, loc="upper right")
ax.set_title(f"Rho cost per isogeny class, 57 classes of F_p, p = 2^31 - 1 (T1 KW p = {A['primary_tests'][0]['p']:.3f})", fontsize=9)
fig.tight_layout(); fig.savefig(f"{out}/class_means.svg"); plt.close(fig)

# 2. class mean c against class invariants (T2-T5)
main = [c for c in C if c["stratum"].startswith("S")]
fig, axs = plt.subplots(1, 2, figsize=(7.2, 3.0), sharey=True)
axs[0].scatter([math.log2(c["H"]) for c in main], [c["mean_c"] for c in main], c=[cols[c["stratum"]] for c in main], s=12)
axs[0].set_xlabel("log2 H (curves in class)")
axs[0].set_ylabel("class mean c")
axs[0].set_title(f"T2 Spearman {A['primary_tests'][1]['rho']:+.3f}, p {A['primary_tests'][1]['p_perm']:.2f}", fontsize=8)
axs[1].scatter([c["n_orders"] for c in main], [c["mean_c"] for c in main], c=[cols[c["stratum"]] for c in main], s=12)
axs[1].set_xlabel("number of endomorphism rings in class")
axs[1].set_title(f"T4 Spearman {A['primary_tests'][3]['rho']:+.3f}, p {A['primary_tests'][3]['p_perm']:.2f}", fontsize=8)
fig.tight_layout(); fig.savefig(f"{out}/invariants.svg"); plt.close(fig)

# 3. controls against their expectations
exp = {"aa": 1.0, "iso-control": 1.0, "plant-neg": 1 / math.sqrt(2), "plant-r4": None,
       "J0:aa": 1.0, "J0:iso-control": 1.0, "J0:plant-neg": 1 / math.sqrt(2), "J0:plant-r4": None,
       "J0:fold-aut": 1 / math.sqrt(6), "R r-scaling classes vs main classes (normalised c)": 1.0}
fig, ax = plt.subplots(figsize=(7.2, 3.2))
ctl = A["controls"]
for i, c in enumerate(ctl):
    r = c.get("ratio_to_root_v0", c.get("ratio"))
    ax.errorbar(r, i, xerr=2 * c["se"], fmt="o", color="k", ms=4)
    e = exp.get(c["arm"])
    if e: ax.plot(e, i, "|", color="r", ms=14, mew=2)
ax.set_yticks(range(len(ctl))); ax.set_yticklabels([c["arm"].replace(" (normalised c)", "") for c in ctl], fontsize=7)
ax.set_xlabel("ratio of pooled mean iterations to the root curve (+-2 SE); red: theoretical expectation")
ax.set_title("Controls: detector, A/A, isomorphic models, j = 0 fold, sqrt(r) scaling", fontsize=9)
fig.tight_layout(); fig.savefig(f"{out}/controls.svg"); plt.close(fig)

# 4. per-class KW p-values (T6, T7)
fig, axs = plt.subplots(1, 2, figsize=(7.2, 2.8), sharey=True)
for ax, key, t in [(axs[0], "within_kw_p", A["primary_tests"][5]), (axs[1], "generator_kw_p", A["primary_tests"][6])]:
    ps = [c[key] for c in main if c[key] is not None]
    ax.hist(ps, bins=10, range=(0, 1), color="#888", edgecolor="w")
    ax.axhline(len(ps) / 10, color="r", lw=0.8)
    ax.set_xlabel(f"per-class KW p ({'curves within class' if key == 'within_kw_p' else 'generators'})")
    ax.set_title(f"KS vs uniform: D {t['D']:.3f}, p {t['p']:.2f}, n {len(ps)}", fontsize=8)
axs[0].set_ylabel("classes")
fig.tight_layout(); fig.savefig(f"{out}/kw_pvalues.svg"); plt.close(fig)
print("ok")
