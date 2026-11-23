#!/usr/bin/env python3
"""Amendment A1, arm U3: shear the eliminating variable of the u-ring system.

    shear_w.py <uring.ms> <uring.planted> <seed> <out.ms> <out.planted>

In the u-ring system the quadric s_m - w^2 has leading monomial w^2 in grevlex, so every
standard monomial has w-degree <= 1: the BNS22 chains have length <= 2 and hnfsolve's
stability check fails (results/U1). This applies the linear change w := w' - c*s1 with
c = 1 + (seed-derived value mod p-1) and renames w' back to w, so the quadric becomes
s_m - (w + ... )^2 with leading monomial s1^2 and w is no longer killed at degree 2. The ideal
is the image of the same points under an invertible linear map (same D). The planted point
maps to w' = w + c*s1. Exits 1 unless the mapped planted point satisfies every equation.
"""
import hashlib
import re
import sys
from math import comb

src, spl, seed, out, opl = sys.argv[1:6]
lines = open(src).read().split("\n", 2)
vs = lines[0].strip().split(",")
p = int(lines[1].strip())
m = len(vs) - 1
assert vs == [f"s{i}" for i in range(1, m + 1)] + ["w"], vs
c = 1 + int.from_bytes(hashlib.sha256(f"shear_w:{seed}:{p}".encode()).digest()[:8], "big") % (p - 1)
polys = [q.strip() for q in lines[2].replace("\n", "").split(",") if q.strip()]
assert len(polys) == m + 1, len(polys)
TERM = re.compile(r"([+-]?)([^+-]+)")
n = len(vs)


def parse(poly):
    d = {}
    for sign, body in TERM.findall(poly):
        co = 1
        e = [0] * n
        for x in body.split("*"):
            if re.fullmatch(r"\d+", x):
                co = co * int(x) % p
            else:
                v, _, k = x.partition("^")
                e[vs.index(v)] += int(k) if k else 1
        if sign == "-":
            co = -co % p
        k = e[-1]
        for j in range(k + 1):  # (w - c s1)^k = sum_j C(k,j) w^(k-j) (-c s1)^j
            ne = list(e)
            ne[-1] = k - j
            ne[0] += j
            ne = tuple(ne)
            d[ne] = (d.get(ne, 0) + co * comb(k, j) * pow(-c % p, j, p)) % p
    return {k: v for k, v in d.items() if v}


def fmt(d):
    ts = []
    for e, co in sorted(d.items(), key=lambda t: (-sum(t[0]), [-x for x in t[0]])):
        mon = "*".join(v if k == 1 else f"{v}^{k}" for v, k in zip(vs, e) if k)
        ts.append(f"{co}*{mon}" if mon else f"{co}")
    return "+".join(ts)


new = [parse(q) for q in polys]
pl = [int(x) for x in open(spl).read().split("\n")[1].split(",")]
pt = pl[:m] + [(pl[m] + c * pl[0]) % p]


def ev(d):
    t = 0
    for e, co in d.items():
        for x, k in zip(pt, e):
            co = co * pow(x, k, p) % p
        t += co
    return t % p


ok = all(ev(d) == 0 for d in new)
with open(out, "w") as f:
    f.write(",".join(vs) + "\n" + str(p) + "\n" + ",\n".join(fmt(d) for d in new) + "\n")
with open(opl, "w") as f:
    f.write(",".join(vs) + "\n" + ",".join(map(str, pt)) + "\n")
print({"p": p, "c": c, "terms": [len(d) for d in new], "max_total_degree": [max(map(sum, d)) for d in new],
       "planted_satisfies": ok})
sys.exit(0 if ok else 1)
