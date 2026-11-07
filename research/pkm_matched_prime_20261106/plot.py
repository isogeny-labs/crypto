"""Plots raw/t*.log: the F4 solving degree Dmax of the m = 2 Kummer tower system
and of its random-coefficient null, on primes matched to the tower depth t, with
the fixed-prime staircase (p = 2013265921, research/ic_next10_20261014/03-pkm-degree)
for comparison.  python3 plot.py"""
import re
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
pat = re.compile(r"kummer\s+m=2 (tower|null)\s+t=\s*(\d+) .*?Dmax=\s*(\d+)")
P = {5: 2081, 6: 8513, 7: 33409, 8: 134401, 9: 524801}
d = {"tower": {}, "null": {}}
for t in P:
    for line in open(f"raw/t{t}.log"):
        m = pat.search(line)
        if m and int(m.group(2)) == t:
            d[m.group(1)].setdefault(t, set()).add(int(m.group(3)))
fixed = {1: 3, 2: 4, 3: 4, 4: 4, 5: 4, 6: 5, 7: 5, 8: 5, 9: 5, 10: 6}
fig, ax = plt.subplots(figsize=(7.5, 4.2))
ax.step(list(fixed), list(fixed.values()), where="mid", color="#999999", label="fixed p = 2013265921 (committed)")
ts = sorted(P)
ax.plot(ts, [max(d["tower"][t]) for t in ts], "o", ms=9, color="#2a6fbb", label="PKM tower, matched p (this run)")
ax.plot(ts, [max(d["null"][t]) for t in ts], "x", ms=9, color="#d9822b", label="random-coefficient null, matched p")
for t in ts:
    ax.annotate(f"p={P[t]}", (t, max(d['tower'][t])), textcoords="offset points", xytext=(0, 10), ha="center", fontsize=7)
ax.set_xlabel("tower depth t  (matched prime: p = k·2^t + 1, about 2t+2 bits)")
ax.set_ylabel("F4 solving degree")
ax.set_ylim(2.5, 6.5)
ax.set_title("m = 2 Kummer tower: the degree follows t, not p, and equals the null", fontsize=9)
ax.legend(fontsize=8)
fig.tight_layout()
fig.savefig("fig_matched.svg"); fig.savefig("fig_matched.png", dpi=120)
print({k: {t: sorted(v) for t, v in d[k].items()} for k in d})
