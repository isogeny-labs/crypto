#!/usr/bin/env python3
"""Plots values already derived by analysis/p2analyze from run-20261107T214118Z (no computation here
beyond reading TSV columns). Outputs p2_summary.svg/.png."""
import csv, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__)); D = os.path.join(H, "..", "analysis", "derived")
def rows(n): return list(csv.DictReader(open(os.path.join(D, n)), delimiter="\t"))
m1, m2, m3 = rows("m1_order_statistic.tsv"), rows("m2_coverage_law.tsv"), rows("m3_f4_cost.tsv")
fig, ax = plt.subplots(1, 3, figsize=(15, 4.8))
# (a) order statistic
for kind, mk in (("kummer", "o"), ("dickson", "s")):
    xs = [int(float(r["t"])) for r in m1 if r["kind"] == kind]
    ax[0].scatter([x + (-0.12 if kind == "kummer" else 0.12) for x in xs], [float(r["ratio_obs"]) for r in m1 if r["kind"] == kind], marker=mk, label=f"observed c_max/c_mean ({kind})", alpha=0.7)
ts = sorted({int(float(r["t"])) for r in m1})
ax[0].plot(ts, [sum(float(r["ratio_exact"]) for r in m1 if int(float(r["t"])) == t) / sum(1 for r in m1 if int(float(r["t"])) == t) for t in ts], "k-", lw=2, label="exact binomial order statistic (mean)")
ax[0].set_xticks(ts); ax[0].set_xlabel("tower depth t (base has 2^t elements)"); ax[0].set_ylabel("c_E of best coset / mean c_E")
ax[0].set_title("(a) M1: selection gain per column decays with t\n36 cells, C ≤ 10,000 candidates"); ax[0].legend(fontsize=7)
# (b) coverage law
pred = [float(r["ratio_pred"]) for r in m2]; obs = [float(r["ratio_obs"]) for r in m2]
ax[1].loglog(pred, obs, "o", alpha=0.7); lo, hi = min(pred) * 0.8, max(pred) * 1.2
ax[1].plot([lo, hi], [lo, hi], "k--", lw=1, label="obs = pred"); ax[1].plot([lo, hi], [lo * 1.2, hi * 1.2], "r:", lw=1, label="±20 % kill band"); ax[1].plot([lo, hi], [lo / 1.2, hi / 1.2], "r:", lw=1)
ax[1].set_xlabel("predicted (c_sel / c_rand)^2"); ax[1].set_ylabel("observed coverage(selected) / coverage(random), m = 2")
ax[1].set_title("(b) M2: exact coverage follows c_E^2\n36 cells, full enumeration"); ax[1].legend(fontsize=7)
# (c) F4 work per arm
cells = sorted({r["cell"] for r in m3}); x = range(len(cells)); w = 0.27
for i, arm in enumerate(("idx0", "idx1", "max")):
    ax[2].bar([k + (i - 1) * w for k in x], [float(next(r for r in m3 if r["cell"] == c and r["arm"] == arm)["mean_work"]) for c in cells], w, label={"idx0": "random coset", "idx1": "second random (A/A)", "max": "selected coset"}[arm])
ax[2].set_yscale("log"); ax[2].set_xticks(list(x)); ax[2].set_xticklabels([c.replace("dickson", "D").replace("kummer", "K") for c in cells], rotation=60, fontsize=7)
ax[2].set_ylabel("msolve F4 work (Σ rows × cols), mean of 4 targets")
ax[2].set_title("(c) M3: F4 cost does not depend on the coset\nmax |rel. diff| 0.4 % (A/A 0.45 %); D, widest identical"); ax[2].legend(fontsize=7)
fig.suptitle("P2 twist-pure coset selection, run-20261107T214118Z (CARD-p2-twist-coset-20261107-1): derived by p2analyze; toy primes 2^20, 2^24", fontsize=10)
fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})): fig.savefig(os.path.join(H, "p2_summary." + ext), **kw)
print("ok")
