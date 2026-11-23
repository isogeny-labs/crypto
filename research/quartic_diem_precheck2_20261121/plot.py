#!/usr/bin/env python3
"""Figure for pre-check 2 from the recorded runs and analysis.json. [pcc:unverified]

Usage: python3 plot.py   (reads raw/*.jsonl and analysis.json; writes fig_scaling.svg/.png)
Plots recorded values only.
"""
import collections
import glob
import json
import math

import matplotlib

matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "none"
import matplotlib.pyplot as plt

R = []
for path in sorted(glob.glob("raw/run*.jsonl")):
    for line in open(path):
        r = json.loads(line)
        if r.get("kind") == "run" and r.get("family") == "quartic" and r["certified"] and r["ok"]:
            R.append(r)
A = json.load(open("analysis.json"))


def fpm(r):
    return r["cantor_mul"] + 10 * r["cantor_inv"] + r["rel_mul"] + 9 * r["la_mul"]


def curve_means(arm, f):
    cur = collections.defaultdict(list)
    for r in R:
        if r["arm"] == arm:
            cur[(r["p"], r["curve_seed"])].append(math.log(f(r)))
    return [p for (p, _) in cur], [math.exp(sum(v) / len(v)) for v in cur.values()]


fig, (ax, bx) = plt.subplots(1, 2, figsize=(8.6, 3.7))
series = [
    ("Q_diem_2lp", fpm, "FPM mid, OLD (pre-check-1 instrument)", "#9a9a9a", "o"),
    ("Q_diem_2lp_new", fpm, "FPM mid, NEW (primary)", "#1f5fa8", "o"),
    ("Q_diem_2lp_new", lambda r: r["la_mul"], "la_mul, NEW", "#d1782a", "s"),
]
for arm, f, lab, col, mk in series:
    xs, ys = curve_means(arm, f)
    key = "fpm_mid" if f is fpm else "la_mul"
    e = A["arms"][arm]["exponents"][key]
    lo, hi = e["boot95"]
    ax.scatter(xs, ys, s=12, color=col, marker=mk, alpha=0.85,
               label=f"{lab}: {e['ols_curve_level']:.2f} [{lo:.2f}, {hi:.2f}]")
ps = sorted({r["p"] for r in R})
x0 = ps[0]
_, y0 = curve_means("Q_diem_2lp_new", fpm)
base = min(y0)
for slope, ls in [(1.0, ":"), (1.4, "--")]:
    ax.plot([ps[0], ps[-1]], [base * (p / x0) ** slope for p in (ps[0], ps[-1])], ls,
            color="k", lw=0.8, label=f"slope {slope}")
ax.set_xscale("log")
ax.set_yscale("log")
ax.set_xlabel("p")
ax.set_ylabel("counted ops per solve (curve geo-mean)")
ax.set_title("Diem 2LP, prime-order quartics: exponent [boot95]", fontsize=9)
ax.legend(fontsize=6.5, loc="upper left")

P = A["paired_new_over_old"]["per_prime"]
for key, lab, col, dx in [("fpm_mid", "FPM mid", "#1f5fa8", 0.97),
                          ("la_mul", "la_mul", "#d1782a", 1.0),
                          ("rel_mul", "rel_mul", "#5a9e4b", 1.03)]:
    xs = sorted(int(p) for p in P)
    g = [P[str(p)][key]["geo_mean"] if str(p) in P else P[p][key]["geo_mean"] for p in xs]
    lo = [P[str(p)][key]["boot95"][0] for p in xs]
    hi = [P[str(p)][key]["boot95"][1] for p in xs]
    bx.errorbar([p * dx for p in xs], g, yerr=[[a - b for a, b in zip(g, lo)], [b - a for a, b in zip(g, hi)]],
                fmt="o-", ms=3.5, lw=1, capsize=2, color=col, label=lab)
bx.axhline(1.0, color="k", lw=0.6)
bx.set_xscale("log")
bx.set_yscale("log")
bx.set_xlabel("p")
bx.set_ylabel("NEW / OLD, same curve and seed")
bx.set_title("paired ratio, geo-mean per prime [cluster boot95]", fontsize=9)
bx.legend(fontsize=7)
n = len([r for r in R if r["arm"] == "Q_diem_2lp_new"])
fig.suptitle(f"quartic_diem_precheck2_20261121: {n} certified NEW runs, counted F_p / F_r ops [pcc:unverified]",
             fontsize=8.5)
fig.tight_layout()
fig.savefig("fig_scaling.svg")
fig.savefig("fig_scaling.png", dpi=72)
# keep the PNG small: 64-colour palette
from PIL import Image

Image.open("fig_scaling.png").convert("P", palette=Image.ADAPTIVE, colors=64).save("fig_scaling.png", optimize=True)
