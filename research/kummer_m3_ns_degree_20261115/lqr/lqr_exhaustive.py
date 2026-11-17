#!/usr/bin/env python3
"""Exhaustive: does lambda kill EVERY NS column of degree <= 2t+5 (f1: popcount+j+4, f2: popcount+j+3)?
And is the column (k1=k2=all-ones, k3=0, j=2) times f1 (degree 2t+6) NOT killed?"""
import json, random, sys
sys.path.insert(0, sys.argv[1])
from d9_check import s3_coeffs_in_x3, pdivmod, pmul, bezout, root_of_unity, popcount

def run(p, t, seed):
    rng = random.Random(seed); K = 1 << t; z = root_of_unity(K, p)
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b**2) % p: break
    g = rng.randrange(1, p); xR = rng.randrange(p)
    pts = [g*pow(z, i, p) % p for i in range(K)]
    F2 = [s3_coeffs_in_x3(pts[i], xR, a, b, p) for i in range(K)]
    if any(len(f) != 3 for f in F2): return {"skip": 1}
    # per point: c_j(x) = <u>( u^j * (target) * f1^{-1} mod f2 ), precompute for target f1 (-> u^j mod f2) and f2 (-> 0)
    # lambda(z^k u^j f1) = sum_x x1x2x3 z^k(x) <u>(u^j mod f2(x3));  lambda(z^k u^j f2) = 0 identically.
    JM = 2*t+6
    red = []
    for i3 in range(K):
        row = []
        for j in range(JM+1):
            _, r = pdivmod([0]*j + [1], F2[i3], p); row.append(r[1] if len(r) > 1 else 0)
        red.append(row)
    S1 = [sum(pts[i]*pow(z, k*i % K, p) for i in range(K)) % p for k in range(K)]   # sum x * z^k
    S3 = [[sum(pts[i]*pow(z, k*i % K, p)*red[i][j] for i in range(K)) % p for j in range(JM+1)] for k in range(K)]
    def lam_col(k1, k2, k3, j): return S1[k1]*S1[k2]*S3[k3][j] % p
    bad = 0
    for k1 in range(K):
        for k2 in range(K):
            for k3 in range(K):
                pc = popcount(k1)+popcount(k2)+popcount(k3)
                for j in range(0, 2*t+6 - pc):
                    if pc + j + 4 <= 2*t+5 and lam_col(k1, k2, k3, j): bad += 1
    witness = lam_col(K-1, K-1, 0, 2)
    # lambda(1) directly (needs f1^{-1}):
    l1 = 0
    for i1 in range(K):
        for i2 in range(K):
            f1 = s3_coeffs_in_x3(pts[i1], pts[i2], a, b, p)
            for i3 in range(K):
                bz = bezout(f1, F2[i3], p)
                if bz is None: return {"p": p, "t": t, "seed": seed, "skip": "consistent grid point"}
                _, r = pdivmod(bz[0], F2[i3], p)
                l1 += pts[i1]*pts[i2]*pts[i3]*(r[1] if len(r) > 1 else 0)
    l1 %= p
    return {"p": p, "t": t, "seed": seed, "unkilled_cols_deg_le_2t5": bad, "col_allones_j2_nonzero": witness != 0,
            "lambda1_nonzero": l1 != 0, "implied_dNS_ge": 2*t+6 if (bad == 0 and l1) else None}

rows = []
for p in (40961, 12289, 998244353):
    for t in (2, 3, 4, 5):
        for d in range(3):
            r = run(p, t, 6_000_000 + 100*t + d + p % 13); rows.append(r); print(json.dumps(r), flush=True)
ok = [r for r in rows if r.get("implied_dNS_ge")]
print("SUMMARY certified", len(ok), "/", len(rows))
