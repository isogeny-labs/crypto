#!/usr/bin/env python3
"""LEAD-ICB-tau-horner-chain toy: Horner chain tau^2 P1 + tau P2 + P3 = R vs plain P1+P2+P3 = R
on a Koblitz curve y^2+xy=x^3+x^2+1 over GF(2^n), n odd prime. stdlib only."""
import sys, json, random

def make_field(n, poly):
    exp = [0] * (2 * (1 << n)); log = [0] * (1 << n)
    x = 1
    for i in range((1 << n) - 1):
        exp[i] = x; log[x] = i
        x <<= 1
        if x >> n: x ^= poly
    assert x == 1, "poly not primitive"
    for i in range((1 << n) - 1, 2 * (1 << n)): exp[i] = exp[i - ((1 << n) - 1)]
    return exp, log

def build(n, poly):
    exp, log = make_field(n, poly); q1 = (1 << n) - 1
    mul = lambda a, b: 0 if a == 0 or b == 0 else exp[log[a] + log[b]]
    inv = lambda a: exp[q1 - log[a]]
    sq = lambda a: 0 if a == 0 else exp[(2 * log[a]) % q1]
    def tr(c):
        t = 0
        for _ in range(n): t ^= c; c = sq(c)
        return t
    def hT(c):
        r = 0
        for _ in range((n + 1) // 2): r ^= c; c = sq(sq(c))
        return r
    pts = [(0, 1)]
    for x in range(1, 1 << n):
        c = x ^ 1 ^ sq(inv(x))
        if tr(c) == 0:
            z = hT(c); assert sq(z) ^ z == c
            pts.append((x, mul(x, z))); pts.append((x, mul(x, z ^ 1)))
    def add(P, Q):
        if P is None: return Q
        if Q is None: return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if y1 != y2 or x1 == 0: return None if (y1 ^ y2) == x2 or y1 == y2 and x1 == 0 else None
            l = x1 ^ mul(y1, inv(x1))
        else:
            l = mul(y1 ^ y2, inv(x1 ^ x2))
        x3 = sq(l) ^ l ^ x1 ^ x2 ^ 1
        return (x3, mul(l, x1 ^ x3) ^ x3 ^ y1)
    def neg(P): return None if P is None else (P[0], P[1] ^ P[0])
    tau = lambda P: None if P is None else (sq(P[0]), sq(P[1]))
    return pts, add, neg, tau, sq, mul

def ok(P, sq, mul):
    if P is None: return True
    x, y = P
    return sq(y) ^ mul(x, y) == mul(sq(x), x) ^ sq(x) ^ 1

def sub(add, neg, A, B): return add(A, neg(B))

def arm(n, poly, kind, norb, targets, seed):
    pts, add, neg, tau, sq, mul = build(n, poly)
    rng = random.Random(seed)
    aff = [p for p in pts]
    for p in rng.sample(aff, 10): assert ok(p, sq, mul) and ok(tau(p), sq, mul)
    F = []; S = set()
    if kind == "closed":
        while len(F) < norb * n:
            P = rng.choice(aff)
            if P in S: continue
            Q = P
            while Q not in S:
                S.add(Q); F.append(Q); Q = tau(Q)
    else:  # random set of same size, not tau-closed
        for P in rng.sample(aff, norb * n): S.add(P); F.append(P)
    idx = {P: i for i, P in enumerate(F)}
    t1 = [tau(P) for P in F]; t2 = [tau(P) for P in t1]
    Dp = Dh = Hraw = Hexp = adds_p = adds_h = 0
    tg = [rng.choice(aff) for _ in range(targets)]
    per = []
    for R in tg:
        rp = set(); rh = set(); raw = 0
        for i, A in enumerate(F):
            RA = sub(add, neg, R, A)
            for j in range(i, len(F)):
                C = sub(add, neg, RA, F[j]); adds_p += 2
                k = idx.get(C, -1)
                if k >= j: rp.add((i, j, k))
        for i in range(len(F)):
            RA = sub(add, neg, R, t2[i])
            for j in range(len(F)):
                C = sub(add, neg, RA, t1[j]); adds_h += 2
                if C in idx:
                    raw += 1
                    a, b = idx.get(t2[i], -1), idx.get(t1[j], -1)
                    if a >= 0 and b >= 0: rh.add(tuple(sorted((a, b, idx[C]))))
        Dp += len(rp); Dh += len(rh); Hraw += raw; Hexp += len(rh)
        per.append((len(rp), len(rh)))
    return dict(kind=kind, N=len(pts), fb=len(F), D_plain=Dp, D_horner_expressible=Dh,
                horner_raw_tuples=Hraw, ratio=(Dh / Dp if Dp else None),
                adds_per_distinct_plain=(adds_p / Dp if Dp else None),
                adds_per_distinct_horner=(adds_h / Dh if Dh else None), per_target=per)

def main():
    smoke = "--smoke" in sys.argv
    n, poly, norb, T = (7, 0x83, 2, 6) if smoke else (13, 0x201B, 6, 40)
    seed = 20261110
    closed = arm(n, poly, "closed", norb, T, seed)
    aa = arm(n, poly, "closed", norb, T, seed + 1)       # A/A: independent seed, same arm
    ctrl = arm(n, poly, "random", norb, T, seed + 2)     # negative control: not tau-closed
    # positive control: plain-vs-plain with ordered enumeration cannot differ; here the
    # detector's sensitivity is shown by ctrl (ratio must fall well below 1 when closure fails).
    enough = closed["D_plain"] >= (5 if smoke else 50) and aa["D_plain"] >= (5 if smoke else 50)
    ident = closed["ratio"] == 1.0 and aa["ratio"] == 1.0
    ctrl_ok = ctrl["D_plain"] > 0 and ctrl["ratio"] is not None and ctrl["ratio"] < 0.5
    if not enough or not ctrl_ok: v = "inconclusive"
    elif ident: v = "consistent_with_prediction"
    else: v = "falsified"
    print(json.dumps(dict(lead="LEAD-ICB-tau-horner-chain", smoke=smoke,
        params=dict(n=n, poly=hex(poly), m=3, orbits=norb, targets=T, seeds=[seed, seed + 1, seed + 2]),
        statistic="D_horner_expressible / D_plain over targets (closed FB); threshold ==1 exactly; control <0.5",
        closed=closed, aa=aa, negative_control=ctrl, verdict=v)))
main()
