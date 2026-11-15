#!/usr/bin/env python3
"""LEAD-ICE-gpu-rootfind-oracle toy: Semaev-S3 root-finding 2-decomposition oracle vs direct group arithmetic. stdlib only."""
import sys, json, math, random

def prime_3mod4_below(n):
    n -= 1
    while True:
        if n % 4 == 3 and all(n % d for d in range(2, int(n**.5) + 1)):
            return n
        n -= 1

def run(logp, T, seed, Bdiv):
    p = prime_3mod4_below(1 << logp)
    rng = random.Random(seed)
    while True:
        a, b = rng.randrange(1, p), rng.randrange(1, p)
        if (4*a**3 + 27*b*b) % p: break
    rhs = lambda x: (x*x*x + a*x + b) % p
    sq = lambda v: pow(v, (p+1)//4, p)
    isqr = lambda v: pow(v, (p-1)//2, p) == 1
    N = 1 + sum(1 + (0 if rhs(x) == 0 else (1 if isqr(rhs(x)) else -1)) for x in range(p))
    B = max(8, p // Bdiv)
    fb = []
    for x in range(B):
        r = rhs(x)
        if r == 0: fb.append((x, 0))
        elif isqr(r):
            y = sq(r); fb += [(x, y), (x, p-y)]
    fbset = set(fb); byx = {}
    for P in fb: byx.setdefault(P[0], P)
    fbx = sorted({P[0] for P in fb})
    def add(P, Q):
        if P is None: return Q
        if Q is None: return P
        if P[0] == Q[0]:
            if (P[1] + Q[1]) % p == 0: return None
            l = (3*P[0]*P[0] + a) * pow(2*P[1], -1, p) % p
        else:
            l = (Q[1]-P[1]) * pow(Q[0]-P[0], -1, p) % p
        x = (l*l - P[0] - Q[0]) % p
        return (x, (l*(P[0]-x) - P[1]) % p)
    neg = lambda P: None if P is None else (P[0], (-P[1]) % p)
    def randpt():
        while True:
            x = rng.randrange(p); r = rhs(x)
            if r and isqr(r): return (x, sq(r))
    def oracle_roots(x1, xR, bb):
        c2 = (xR-x1)**2 % p
        if c2 == 0: return None
        c1 = (-2*x1*xR*xR - 2*xR*(x1*x1+a) - 2*a*x1 - 4*bb) % p
        c0 = (xR*xR*x1*x1 - 2*xR*(a*x1+2*bb) + a*a - 4*bb*x1) % p
        d = (c1*c1 - 4*c2*c0) % p
        if d and not isqr(d): return set()
        s = sq(d); inv = pow(2*c2, -1, p)
        return {(-c1+s)*inv % p, (-c1-s)*inv % p}
    def one(R, bb=b):
        mism = 0; direct = 0; xhits = 0
        for x1 in fbx:
            if x1 == R[0]: continue
            ro = oracle_roots(x1, R[0], bb)
            dset = set()
            for P1 in [byx[x1]]:
                for S in (P1, neg(P1)):
                    Q = add(R, neg(S))
                    if Q is not None: dset.add(Q[0])
            if ro != dset: mism += 1
            xhits += sum(1 for x in ro if x < B and rhs(x) and isqr(rhs(x)) or (x < B and rhs(x) == 0))
        for P1 in fb:
            Q = add(R, neg(P1))
            if Q is not None and Q in fbset and P1[0] != R[0]: direct += 1
        return mism, direct
    lam = len(fb)**2 / N
    mm = 0; cnt = []
    pos_ok = 0; npos = max(3, T // 10)
    for _ in range(T):
        m, d = one(randpt()); mm += m; cnt.append(d)
    for _ in range(npos):  # positive control: planted decomposition must be found by oracle
        P1, P2 = rng.choice(fb), rng.choice(fb); R = add(P1, P2)
        if R is None or R[0] == P1[0]: npos -= 1; continue
        ro = oracle_roots(P1[0], R[0], b)
        pos_ok += (ro is not None and P2[0] in ro)
    nm = 0  # negative control: oracle with wrong curve constant must disagree with direct
    for _ in range(max(3, T // 10)):
        m, _d = one(randpt(), (b + 1) % p); nm += m
    h = T // 2
    s1, s2 = sum(cnt[:h]), sum(cnt[h:])
    z_aa = (s1 - s2) / math.sqrt(max(1, s1 + s2))
    tot = sum(cnt); z = (tot - lam*T) / math.sqrt(2 * lam * T)  # x2 var inflation for ordered pairs
    ok = mm == 0 and pos_ok == npos and nm > 0 and abs(z) < 4 and abs(z_aa) < 4
    bad = mm > 0 or pos_ok < npos or abs(z) >= 4
    return {"lead": "LEAD-ICE-gpu-rootfind-oracle", "seed": seed, "p": p, "N": N, "B": B, "fb_points": len(fb),
            "targets": T, "expected_per_target": lam, "observed_per_target": tot / T,
            "z_yield": z, "z_AA_split_half": z_aa, "oracle_vs_direct_mismatches": mm,
            "positive_control_found": [pos_ok, npos], "negative_control_mismatches": nm,
            "verdict": "consistent_with_prediction" if ok else ("falsified" if bad else "inconclusive")}

if __name__ == "__main__":
    if "--smoke" in sys.argv: r = run(10, 6, 1, 8)
    else: r = run(16, 600, 20261110, 64)
    print(json.dumps(r)); sys.exit(0)
