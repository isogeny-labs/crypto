#!/usr/bin/env python3
"""Plot RECORDED values of a G6 run (verdicts.tsv, meta_*.txt, iota_audit.tsv). Computes nothing new: no statistics here.
Usage: plot_g6.py RUN_DIR   -> RUN_DIR/fig_g6.svg and fig_g6.png"""
import re, sys, glob, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

run = sys.argv[1]
curves = {(23, 0): ("n=23 a=0", "tab:blue", "o"), (19, 0): ("n=19 a=0", "tab:green", "s"), (19, 1): ("n=19 a=1", "tab:red", "^")}

def cellkey(c):
    m = re.match(r"n(\d+)a(\d+)m(\d+)", c)
    return int(m[1]), int(m[2]), int(m[3])

prim = {}  # (stat) -> list of (n,a,m, ratio, lo, hi, verdict)
for line in open(os.path.join(run, "verdicts.tsv")).read().splitlines()[1:]:
    f = line.split("\t")
    if f[1] != "PRIMARY_NI_vs_UP":
        continue
    n, a, m = cellkey(f[0])
    lo, hi = [float(x) for x in f[7].strip("[]").split(",")]
    prim.setdefault(f[2], []).append((n, a, m, float(f[6]), lo, hi, f[-1]))

feas = []  # (n,a,m, feasible_NI/draws, feasible_U/draws)
for p in glob.glob(os.path.join(run, "meta_*.txt")):
    kv = dict(l.split("\t", 1) for l in open(p).read().splitlines() if "\t" in l)
    d = float(kv["draws"])
    feas.append((int(kv["n"]), int(kv["a"]), int(kv["m"]), float(kv["feasible_NI"]) / d, float(kv["feasible_U"]) / d))

fig, ax = plt.subplots(2, 2, figsize=(11, 8), constrained_layout=True)
for k, stat in enumerate(["eff", "disp", "fano"]):
    a_ = ax[(k + 1) // 2][(k + 1) % 2]
    for (n, a), (lab, col, mk) in curves.items():
        pts = sorted([p for p in prim.get(stat, []) if p[0] == n and p[1] == a], key=lambda p: p[2])
        if not pts:
            continue
        x = [p[2] for p in pts]
        a_.errorbar(x, [p[3] for p in pts], yerr=[[p[3] - p[4] for p in pts], [p[5] - p[3] for p in pts]], fmt=mk + "-", color=col, label=lab, capsize=3)
    a_.axhspan(1 / 1.1, 1.1, color="grey", alpha=0.2, label="equivalence margin +-10%")
    a_.axhline(1.0, color="k", lw=0.6)
    a_.set_xlabel("m (number of slots)")
    a_.set_ylabel({"eff": "NI / UP  yield efficiency", "disp": "NI / UP  dispersion Var/mean", "fano": "NI / UP  orbit Fano factor"}[stat])
    a_.set_title("paired ratio, 90% interval (TOST)")
    a_.legend(fontsize=8)
a0 = ax[0][0]
for (n, a), (lab, col, mk) in curves.items():
    pts = sorted([f for f in feas if f[0] == n and f[1] == a], key=lambda f: f[2])
    if pts:
        a0.plot([p[2] for p in pts], [p[3] for p in pts], mk + "-", color=col, label=lab + " NI (Frobenius-interleaved)")
        a0.plot([p[2] for p in pts], [p[4] for p in pts], mk + "--", color=col, alpha=0.5, label=lab + " generic")
a0.set_xlabel("m (number of slots)")
a0.set_ylabel("fraction of draws with log2 mu in the window")
a0.set_title("can the design reach the target factor-base size?")
a0.legend(fontsize=7)
fig.suptitle("G6: Frobenius-structured vs profile-matched generic slot partitions (exact enumeration, toy n)")
fig.savefig(os.path.join(run, "fig_g6.svg"))
fig.savefig(os.path.join(run, "fig_g6.png"), dpi=130)
print("wrote", os.path.join(run, "fig_g6.svg"))
