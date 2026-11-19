#!/usr/bin/env python3
"""Plot measured mean coverage and E_2/X^2 against the paper's values.
Lays out recorded values from ../per_dilation.csv only (aggregates means/SD; no
new computation of the quantities themselves). See ../RESULT.md.
Run: <scratch-venv>/bin/python coverage.py
"""
import csv, statistics, os
import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "none"
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
rows = list(csv.DictReader(open(os.path.join(here, "..", "per_dilation.csv"))))
# 16-curve-averaged follow-up (per_curve.csv): one row per (p, curve).
mrows = list(csv.DictReader(open(os.path.join(here, "..", "per_curve.csv"))))
def magg(p):
    cov = [float(r["mean_coverage"]) for r in mrows if int(r["p"]) == p]
    en  = [float(r["E2overX2"]) for r in mrows if int(r["p"]) == p]
    return dict(mean=statistics.fmean(cov), sd=statistics.pstdev(cov),
                en=statistics.fmean(en))

def agg(p, arm):
    cov = [float(r["coverage"]) for r in rows if int(r["p"]) == p and r["arm"] == arm]
    en  = [float(r["E2overX2"]) for r in rows if int(r["p"]) == p and r["arm"] == arm]
    u1  = [float(r["coverage"]) for r in rows if int(r["p"]) == p and r["arm"] == arm and int(r["u"]) == 1]
    if not cov:
        return None
    return dict(mean=statistics.fmean(cov), sd=statistics.pstdev(cov),
                en=statistics.fmean(en), u1=(u1[0] if u1 else None))

primes = [1009, 4099, 10007]
gen = {p: agg(p, "generic") for p in primes}
j0  = {p: agg(p, "j0") for p in primes}
m16 = {p: magg(p) for p in primes}

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(9.4, 4.3))
xs = range(len(primes))

# Panel 1: coverage
ax1.axhspan(0.39, 0.42, color="#4b8fd1", alpha=0.18, zorder=0,
            label="paper band 0.39-0.42")
ax1.bar(list(xs), [gen[p]["mean"] for p in primes],
        yerr=[gen[p]["sd"] for p in primes], capsize=4, width=0.5,
        color="#caa43a", zorder=3, label="generic (mean +/- SD over dilations)")
ax1.plot(list(xs), [gen[p]["u1"] for p in primes], "D", color="#d1495b",
         ms=7, zorder=5, label="generic, plain box u=1")
# j0 where available
jx = [i for i, p in enumerate(primes) if j0[p]]
ax1.plot(jx, [j0[primes[i]]["mean"] for i in jx], "o", color="#2a9d55",
         ms=8, zorder=5, label="j=0 (a=0) mean")
ax1.errorbar([x + 0.16 for x in xs], [m16[p]["mean"] for p in primes],
             yerr=[m16[p]["sd"] for p in primes], fmt="s", color="black", ms=6,
             capsize=3, zorder=6, label="16-curve average (+/- SD)")
ax1.set_xticks(list(xs)); ax1.set_xticklabels([f"p={p}" for p in primes])
ax1.set_ylabel("2-sum coverage"); ax1.set_ylim(0, 0.7)
ax1.set_title("Mean coverage over dilations (m=2)")
ax1.grid(axis="y", ls=":", alpha=0.5); ax1.legend(fontsize=7.2, loc="upper left")

# Panel 2: energy
ax2.axhline(4.0, color="#4b8fd1", ls="--", lw=1.4, zorder=2, label="paper ~4")
ax2.bar(list(xs), [gen[p]["en"] for p in primes], width=0.5, color="#caa43a",
        zorder=3, label="generic mean E_2/X^2")
ax2.plot(jx, [j0[primes[i]]["en"] for i in jx], "o", color="#2a9d55", ms=8,
         zorder=5, label="j=0 mean E_2/X^2")
ax2.plot([x + 0.16 for x in xs], [m16[p]["en"] for p in primes], "s",
         color="black", ms=6, zorder=6, label="16-curve average")
ax2.set_xticks(list(xs)); ax2.set_xticklabels([f"p={p}" for p in primes])
ax2.set_ylabel("E_2 / X^2"); ax2.set_ylim(0, 5)
ax2.set_title("Mean additive energy over dilations")
ax2.grid(axis="y", ls=":", alpha=0.5); ax2.legend(fontsize=7.5, loc="lower right")

fig.suptitle("Kaarelson v2 Theorem 3 coverage, reproduced engine-free "
             "(exact counts; per_dilation.csv)", fontsize=10)
fig.tight_layout(rect=(0, 0, 1, 0.96))
fig.savefig(os.path.join(here, "coverage.svg"))
print("wrote coverage.svg; generic:",
      {p: (round(gen[p]["mean"], 4), round(gen[p]["en"], 3)) for p in primes})
