#!/usr/bin/env python3
"""KM3-LQR-NS check: lambda(g) = sum_x x1 x2 x3 <u>(g * f1^{-1} mod f2(x3,u)).
(1) lambda(1) != 0  -> d_NS(m=3) >= 2t+5 (given the orthogonality argument);
(2) numeric sanity: lambda(mu*f2)=0 and lambda(z^k u^j f1)=0 for random columns with k1 or k2 not all-ones."""
import json, random, sys
sys.path.insert(0, sys.argv[1])
from d9_check import (s3_coeffs_in_x3, pdivmod, pmul, padd, bezout, root_of_unity, inv)

def lam_point(gpoly, f1, f2, p):
    # gpoly in F_p[u]; return u-coeff of gpoly * f1^{-1} mod f2
    h1, h2 = bezout(f1, f2, p)          # h1 f1 + h2 f2 = 1 -> f1^{-1} = h1 mod f2
    _, r = pdivmod(pmul(gpoly, h1, p), f2, p)
    return r[1] if len(r) > 1 else 0

def run(p, t, seed):
    rng = random.Random(seed); K = 1 << t; z = root_of_unity(K, p)
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b**2) % p: break
    g = rng.randrange(1, p); xR = rng.randrange(p)
    pts = [g*pow(z, i, p) % p for i in range(K)]
    F2 = [s3_coeffs_in_x3(pts[i], xR, a, b, p) for i in range(K)]
    if any(len(f) != 3 for f in F2): return {"skip": "deg f2 < 2"}
    def lam(fn):  # fn(i1,i2,i3) -> poly in u (value of g at grid point)
        s = 0
        for i1 in range(K):
            for i2 in range(K):
                f1 = s3_coeffs_in_x3(pts[i1], pts[i2], a, b, p)
                for i3 in range(K):
                    gp = fn(i1, i2, i3, f1)
                    if gp:
                        s += pts[i1]*pts[i2]*pts[i3] * lam_point(gp, f1, F2[i3], p)
        return s % p
    l1 = lam(lambda i1, i2, i3, f1: [1])
    # sanity: random columns that must be killed
    kill_ok = True
    for _ in range(6):
        k = [rng.randrange(K) for _ in range(3)]; j = rng.randrange(3)
        if k[0] == K-1 and k[1] == K-1: k[0] = 0
        mono = lambda i1, i2, i3: pow(z, (k[0]*i1 + k[1]*i2 + k[2]*i3) % K, p)
        v1 = lam(lambda i1, i2, i3, f1: pmul([0]*j + [mono(i1, i2, i3)], f1, p))
        v2 = lam(lambda i1, i2, i3, f1: pmul([0]*j + [mono(i1, i2, i3)], F2[i3], p))
        kill_ok &= (v1 == 0 and v2 == 0)
    # control: the all-ones column with j=1 should generically NOT be killed
    allones = lam(lambda i1, i2, i3, f1: pmul([0, pow(z, ((K-1)*i1 + (K-1)*i2) % K, p)], f1, p))
    return {"p": p, "t": t, "seed": seed, "lambda1": l1, "lambda1_nonzero": l1 != 0,
            "kills_random_columns": kill_ok, "allones_u_column_nonzero": allones != 0,
            "implied_dNS_lower": 2*t+5 if l1 else None}

rows = []
for p in (40961, 12289, 998244353):
    for t in (2, 3, 4):
        for d in range(3):
            r = run(p, t, 5_000_000 + 100*t + d + p % 11); rows.append(r); print(json.dumps(r), flush=True)
print("SUMMARY lambda1_nonzero", sum(bool(r.get("lambda1_nonzero")) for r in rows), "/", len(rows),
      "kills_ok", all(r.get("kills_random_columns", True) for r in rows))
