#!/usr/bin/env python3
"""Lays out recorded values only (data/census_*.jsonl, data/summary_*.json)
as SVG; computes no result. Usage: plot.py <census.jsonl> <summary.json> <outdir>"""
import json, sys, math
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

rows_path, summ_path, out = sys.argv[1:4]
S = json.load(open(summ_path))
T, H, LR, COST = [], [], [], []
for line in open(rows_path):
    r = json.loads(line)
    T.append(r["t"]); H.append(r["H_num"] / r["H_den"]); LR.append(r["log2_r"]); COST.append(r["rho_cost_ph"])
p = S["p"]
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none"})

# 1. class size against trace, coloured by log2 r
fig, ax = plt.subplots(figsize=(7.2, 3.6))
sc = ax.scatter(T, H, c=LR, s=0.3, cmap="viridis", rasterized=True, linewidths=0)
cb = fig.colorbar(sc, ax=ax); cb.set_label("log2 r (largest prime of #E)")
ax.set_xlabel("Frobenius trace t (one isogeny class each)")
ax.set_ylabel("curves in class, H(4p - t^2)")
ax.set_title(f"All {S['classes']:,} isogeny classes over F_p, p = 2^31 - 1", fontsize=9)
fig.tight_layout(); fig.savefig(f"{out}/class_size_vs_trace.svg", dpi=200); plt.close(fig)

# 2. log2 r histogram, curve-weighted vs class-weighted (recorded bins)
rows = S["log2_r_hist_quarter_bits"]["rows"]
lo = [x[0] for x in rows]; cls = [x[1] / S["classes"] for x in rows]; crv = [x[2] for x in rows]
fig, ax = plt.subplots(figsize=(7.2, 3.2))
ax.step(lo, cls, where="post", label="share of isogeny classes (random trace)")
ax.step(lo, crv, where="post", label="share of curves (random (a,b), weighted by H)")
ax.set_yscale("log"); ax.set_xlabel("log2 r, quarter-bit bins"); ax.set_ylabel("fraction")
ax.legend(fontsize=8); ax.set_title("Largest prime factor of the group order, whole field", fontsize=9)
fig.tight_layout(); fig.savefig(f"{out}/log2r_hist.svg"); plt.close(fig)

# 3. Lenstra divisibility: recorded fractions against prediction and 1/l
L = S["lenstra_divisibility"]["rows"]
fig, ax = plt.subplots(figsize=(7.2, 3.2))
ls = [x[0] for x in L]
ax.plot(ls, [x[2] * x[0] for x in L], "o", label="curves with l | #E  (x l)")
ax.plot(ls, [x[4] * x[0] for x in L], "k_", ms=12, label="prediction l/(l^2-1) or 1/(l-1)  (x l)")
ax.plot(ls, [x[3] * x[0] for x in L], "s", ms=4, label="classes with l | N  (x l)")
for x in L:
    if x[1]:
        ax.annotate("p=1 mod l", (x[0], x[2] * x[0]), fontsize=6, xytext=(3, 3), textcoords="offset points")
ax.axhline(1, color="grey", lw=0.5)
ax.set_xlabel("prime l"); ax.set_ylabel("fraction x l"); ax.legend(fontsize=8)
ax.set_title("Per-curve vs per-class divisibility of #E by small primes", fontsize=9)
fig.tight_layout(); fig.savefig(f"{out}/lenstra.svg"); plt.close(fig)

# 4. CDF of expected generic (Pohlig-Hellman + rho) cost, curve-weighted
pairs = sorted(zip(COST, H)); tot = sum(H); acc = 0; xs = []; ys = []
for c, h in pairs:
    acc += h; xs.append(math.log2(c)); ys.append(acc / tot)
pk = sorted(COST); n = len(pk)
fig, ax = plt.subplots(figsize=(7.2, 3.2))
ax.plot(xs, ys, label="share of curves")
ax.plot([math.log2(c) for c in pk], [(i + 1) / n for i in range(n)], "--", label="share of classes")
ax.axvline(S["log2_rho_cost_prime_order_reference"], color="k", lw=0.6)
ax.text(S["log2_rho_cost_prime_order_reference"] - 0.2, 0.05, "prime order\nsqrt(pi p/2)", ha="right", fontsize=7)
ax.set_xlabel("log2 expected group operations, sum over q^e || #E of e*sqrt(pi q/2)")
ax.set_ylabel("cumulative fraction"); ax.legend(fontsize=8, loc="upper left")
ax.set_title("Expected generic ECDLP cost over every curve of F_p (model, not measured)", fontsize=9)
fig.tight_layout(); fig.savefig(f"{out}/generic_cost_cdf.svg"); plt.close(fig)
print("ok")
