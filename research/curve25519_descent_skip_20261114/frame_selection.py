#!/usr/bin/env python3
"""Part B of PROTOCOL.md: does the one-bit descent class rank frames as well as the full coset label?
Stdlib only. Run: python3 -I frame_selection.py > frame_selection.out.txt"""
import json, random

p, A, r, T_BITS, NFRAMES = 1048573, 926, 131203, 5, 2000
inv = lambda v: pow(v, p - 2, p)
def leg(v):
    v %= p
    return 0 if v == 0 else (1 if pow(v, (p - 1) // 2, p) == 1 else -1)
rhs = lambda x: (x * x * x + A * x * x + x) % p
def sqrt(v):
    v %= p
    if v == 0: return 0
    if leg(v) != 1: return None
    y = pow(v, (p + 3) // 8, p)
    return y if y * y % p == v else y * pow(2, (p - 1) // 4, p) % p
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
rnd = random.Random(20261114)
while True:                                   # a point of exact order 8 in E[8]
    x = rnd.randrange(1, p); y = sqrt(rhs(x))
    if y is not None:
        T = mul((x, y), r)
        if T and mul(T, 4) is not None and mul(T, 8) is None: break
E8 = {}
for i in range(8): E8[mul(T, i)] = i
shift = A * inv(3) % p
def label(P):                                 # coset label in Z/8: [r]P = [l]T
    return E8[mul(P, r)]

def ranks(v):
    o = sorted(range(len(v)), key=lambda i: v[i]); rk = [0.0] * len(v); i = 0
    while i < len(o):
        j = i
        while j + 1 < len(o) and v[o[j + 1]] == v[o[i]]: j += 1
        for k in range(i, j + 1): rk[o[k]] = (i + j) / 2
        i = j + 1
    return rk
def spearman(a, b):
    ra, rb = ranks(a), ranks(b); n = len(a); ma = sum(ra) / n; mb = sum(rb) / n
    num = sum((x - ma) * (y - mb) for x, y in zip(ra, rb))
    den = (sum((x - ma) ** 2 for x in ra) * sum((y - mb) ** 2 for y in rb)) ** 0.5
    return num / den if den else float("nan")

C, S3, S1, NP = [], [], [], []
for f in range(NFRAMES):
    lo = rnd.randrange(p)
    pts = []                                  # signed points with labels
    for k in range(1 << T_BITS):
        xm = (lo + k - shift) % p; y = sqrt(rhs(xm))
        if y is None or y == 0: continue
        P = (xm, y); l = label(P)
        pts += [(P, l), ((xm, p - y), (-l) % 8)]
    s3 = s1 = 0; hit = set()
    for i, (a, la) in enumerate(pts):
        for b, lb in pts[i:]:
            if a[0] == b[0] and a[1] != b[1]: continue        # P + (-P): excluded
            if (la + lb) % 2 == 0:
                s1 += 1
                if (la + lb) % 8 == 0:
                    s3 += 1
                    S = add(a, b)
                    if S is not None: hit.add(S[0])
    C.append(len(hit)); S3.append(s3); S1.append(s1); NP.append(len(pts))

print("frames %d, t=%d, mean base signed points %.1f" % (NFRAMES, T_BITS, sum(NP) / NFRAMES))
print("mean C %.2f  mean S3 %.2f  mean S1 %.2f" % (sum(C) / NFRAMES, sum(S3) / NFRAMES, sum(S1) / NFRAMES))
print("Spearman rho(S3, C) = %.4f   rho(S1, C) = %.4f" % (spearman(S3, C), spearman(S1, C)))
pool, draws = 100, 2000
tot = {"random": 0.0, "S1": 0.0, "S3": 0.0, "C": 0.0}
for _ in range(draws):
    idx = rnd.sample(range(NFRAMES), pool)
    tot["random"] += C[idx[0]]
    tot["S1"] += C[max(idx, key=lambda i: (S1[i], -i))]
    tot["S3"] += C[max(idx, key=lambda i: (S3[i], -i))]
    tot["C"] += max(C[i] for i in idx)
m = {k: v / draws for k, v in tot.items()}
print("mean coverage of the frame chosen from a pool of %d: %s" % (pool, json.dumps({k: round(v, 2) for k, v in m.items()})))
print("S1 choice / S3 choice = %.3f ; S1 choice / random = %.3f ; S3 choice / C choice = %.3f" % (m["S1"] / m["S3"], m["S1"] / m["random"], m["S3"] / m["C"]))
