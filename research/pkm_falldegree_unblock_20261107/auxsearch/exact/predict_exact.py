#!/usr/bin/env python3
"""Aggregate `hurwitz predict p 8 8` per-trace lines (traces.*) into the exact
P_accept(t) for t = 8..16 at P1 (AMENDMENT_A1.md)."""
import glob, json, re
from fractions import Fraction
p = 2013265921
rows = {}
for f in sorted(glob.glob("traces.*")):
    for l in open(f):
        m = re.match(r"t=8 s=(-?\d+) 6H\(D\)=(\d+) 6H\(D/4\)=(\d+)", l)
        if m:
            s, h, h4 = map(int, m.groups())
            rows[s] = (h, h4)
r = 0
while (r + 1) ** 2 <= 4 * p: r += 1
expect = [s for s in range(-r, r + 1) if (p + 1 - s) % 256 == 0]
assert sorted(rows) == expect, (len(rows), len(expect))
out = []
for t in range(8, 17):
    M = 1 << t
    ss = [s for s in rows if (p + 1 - s) % M == 0]
    acc6 = sum(rows[s][0] - rows[s][1] for s in ss)
    P = Fraction(p - 1, 2) * Fraction(acc6, 6) / (p * p)
    out.append({"t": t, "traces": len(ss), "p_accept": float(P), "expected_curves_tried": float(1 / P),
                "ratio_to_2t": float(1 / P / M), "acc6": acc6})
json.dump({"p": p, "rows": out}, open("predict_exact.json", "w"), indent=1)
for o in out: print(o["t"], o["traces"], round(o["expected_curves_tried"], 1), round(o["ratio_to_2t"], 4))
