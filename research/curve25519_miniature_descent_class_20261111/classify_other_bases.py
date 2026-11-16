#!/usr/bin/env python3
"""See PROTOCOL_other_bases.md. Stdlib only. Run: python3 -I classify_other_bases.py > other_bases.out.txt"""
import json, random

p, A = 1048573, 926                       # the k = 20 miniature (rebuilt and checked in descent_class.py)
inv = lambda v: pow(v, p - 2, p)
leg = lambda v: 0 if v % p == 0 else (1 if pow(v % p, (p - 1) // 2, p) == 1 else -1)
assert leg(A * A - 4) == -1               # one rational 2-torsion point (0,0)
shift = A * inv(3) % p
rhs = lambda xm: (xm * xm * xm + A * xm * xm + xm) % p
def sqrt(v):
    v %= p
    if v == 0: return 0
    if leg(v) != 1: return None
    y = pow(v, (p + 3) // 8, p)
    return y if y * y % p == v else y * pow(2, (p - 1) // 4, p) % p
delta = lambda xm: leg(xm) if xm % p else 1          # class of (0,0) is B = 1

def report(name, xms):
    on = sorted({x % p for x in xms if leg(rhs(x % p)) >= 0})
    plus = sum(1 for x in on if delta(x) == 1); minus = len(on) - plus
    n = len(on); minority = min(plus, minus) / n if n else float("nan")
    verdict = ("too few (<8)" if n < 8 else "FIXED class" if minority == 0 else "mixed" if minority >= 0.10 else "skewed")
    print("%-52s on-curve %4d  +1:%4d  -1:%4d  minority %.2f  %s" % (name, n, plus, minus, minority, verdict))
    return on

# square-bits: x_W = w^2 + A/3  =>  x_M = w^2
for lo, t in ((0, 4), (0, 5), (0, 6), (984071, 4), (414271, 5)):
    report("square-bits lo=%d t=%d" % (lo, t), [((lo + k) % p) ** 2 % p for k in range(1 << t)])
# edwards-bits: x_M = (1+y)/(1-y)
for t in (4, 5, 6):
    report("edwards-bits t=%d" % t, [(1 + y) * inv(1 - y) % p for y in range(1 << t) if y != 1])
# t0-bits: roots z, 1/z of z^2 - w z + 1
for t in (4, 5, 6):
    xs = []
    for w in range(1 << t):
        disc = (w * w - 4) % p
        s = sqrt(disc)
        if s is None: continue
        z1 = (w + s) * inv(2) % p; z2 = (w - s) * inv(2) % p
        xs += [z1, z2]
    report("t0-bits lo=0 t=%d" % t, xs)
# digits: x_W = LO + sum c_j b_j
for lo, ds in ((0, [1, 2, 4, 8, 16]), (113971, [289589, 761384, 273167, 963514, 602728]),
               (51134, [293482, 1015139, 374525, 914144, 700453, 505371])):
    xs = []
    for mask in range(1 << len(ds)):
        xw = lo + sum(d for j, d in enumerate(ds) if mask >> j & 1)
        xs.append((xw - shift) % p)
    report("digits@%d:%s" % (lo, ";".join(map(str, ds))), xs)

# Supplementary (unregistered): square-bits t=9 lo=0, hits by lift class.
def add(P, Q):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        lam = (3 * x1 * x1 + 2 * A * x1 + 1) * inv(2 * y1) % p
    else: lam = (y2 - y1) * inv(x2 - x1) % p
    x3 = (lam * lam - A - x1 - x2) % p
    return (x3, (lam * (x1 - x3) - y1) % p)
def mul(P, k):
    R = None
    while k:
        if k & 1: R = add(R, P)
        P = add(P, P); k >>= 1
    return R
neg = lambda P: None if P is None else (P[0], (-P[1]) % p)
rnd = random.Random(20261111)
r = 131203
while True:                                # a generator of the order-r subgroup and a point of order 8
    x = rnd.randrange(1, p); y = sqrt(rhs(x))
    if y is not None and mul(mul((x, y), 8), r) is None and mul((x, y), 8) is not None: G = mul((x, y), 8); break
while True:
    x = rnd.randrange(1, p); y = sqrt(rhs(x))
    if y is not None:
        T = mul((x, y), r)
        if T and mul(T, 4) is not None and mul(T, 8) is None: break
E8 = [mul(T, i) for i in range(8)]
base = []
for k in range(1 << 9):
    xm = k * k % p
    y = sqrt(rhs(xm))
    if y is not None and y != 0: base += [(xm, y), (xm, p - y)]
pairs = {}
for i, a in enumerate(base):
    for b in base[i:]:
        pairs.setdefault(add(a, b), []).append((a, b))
hits = {1: 0, -1: 0}; viol = 0
for _ in range(200):
    R = mul(G, rnd.randrange(1, r))
    for K in E8:
        RK = add(R, K)
        c = delta(RK[0]) if RK else 1
        hits[c] += len(pairs.get(RK, []))
print("square-bits t=9 lo=0 (supplementary): base points %d; hits over 200 targets x 8 lifts: delta(R+K)=+1: %d, delta(R+K)=-1: %d"
      % (len(base), hits[1], hits[-1]))
