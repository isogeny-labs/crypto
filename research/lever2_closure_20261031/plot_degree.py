#!/usr/bin/env python3
"""Lay out the recorded F4 solving degrees of the PKM tower thread (evidence.json).

Document rendering only (AGENTS.md "Implementation language"): every number is read
from evidence.json, which copies committed values; nothing is computed or fitted here.
Usage: plot_degree.py [evidence.json] [out-stem]
"""
import json
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

src = sys.argv[1] if len(sys.argv) > 1 else "evidence.json"
stem = sys.argv[2] if len(sys.argv) > 2 else "degree_growth"
d = json.load(open(src))
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 9, "axes.spines.top": False,
                     "axes.spines.right": False, "svg.fonttype": "none"})

style = {
    "kummer-m2": dict(color="#1b6ca8", marker="o", ls="-", label="Kummer, m = 2"),
    "null-m2": dict(color="#222222", marker="x", ls="none", label="shape-matched null, m = 2 (= Kummer at N = 12-20)"),
    "isogeny-m2": dict(color="#c4602a", marker="s", ls="-", label="2-isogeny, m = 2 (= Kummer at N = 12-20)"),
    "kummer-m3": dict(color="#2e8b57", marker="^", ls="-", label="Kummer, m = 3"),
    "kummer-m4": dict(color="#8a4fa0", marker="D", ls="-", label="Kummer, m = 4"),
}

fig, (a, b) = plt.subplots(1, 2, figsize=(11.2, 5.0), gridspec_kw={"width_ratios": [1.45, 1]})
for s in d["series"]:
    xs, ys = zip(*s["points"])
    kw = dict(ms=5.5, lw=1.4, **style[s["id"]])
    if s["id"] == "null-m2":
        kw.update(ms=10, mew=1.6, zorder=6)
    a.plot(xs, ys, **kw)

pred = d["registered_prediction"]["points"]
px, py = zip(*pred)
a.plot(px, py, ls="--", color="#1b6ca8", lw=1.2, marker="o", mfc="white", ms=6.5,
       label="registered prediction (C-step), m = 2: not run")
a.annotate("t = 14 (N = 28): predicts 7\nthe decisive cell, not run", xy=(28, 7), xytext=(15.5, 8.15),
           arrowprops=dict(arrowstyle="->", color="#444"), fontsize=8.5, color="#222")
a.axvline(22.8, color="#bbb", lw=0.8)
a.text(23.1, 3.05, "no measurement\nbeyond N = 22 (m = 2)", fontsize=8, color="#666", va="bottom")
a.set_xlabel("N = m·t tower unknowns")
a.set_ylabel("F4 solving degree D")
a.set_ylim(2.8, 8.7)
a.set_xlim(0, 30)
a.set_yticks(range(3, 9))
a.set_title("Recorded solving degrees, PKM tower thread", loc="left", fontsize=10)
a.legend(frameon=False, fontsize=8, loc="upper center", bbox_to_anchor=(0.5, -0.16), ncol=2)

wx, wy = zip(*d["width_kummer_m2_round2"]["points"])
b.semilogy(wx, wy, color="#1b6ca8", marker="o", ms=5.5, lw=1.4)
for x, y in zip(wx, wy):
    b.annotate(f"{y:,}", (x, y), textcoords="offset points", xytext=(0, 7), ha="center", fontsize=7.5)
b.set_xlabel("N (Kummer, m = 2)")
b.set_ylabel("widest F4 matrix (columns)")
b.set_xlim(10.5, 23.5)
b.set_ylim(700, 120000)
b.set_title("Flat degree, growing matrix", loc="left", fontsize=10)

fig.text(0.01, 0.004, "Values copied from RESEARCH_PKM_TOWER_ORACLE.md and prime_fall_degree_20261027/PREREGISTRATION.md "
         "(evidence.json lists each source). Open markers are a registered prediction, not data.", fontsize=7.5, color="#555")
fig.tight_layout(rect=(0, 0.035, 1, 1))
fig.savefig(stem + ".svg")
fig.savefig(stem + ".png", dpi=170)
print("wrote", stem + ".svg", stem + ".png")
