#!/usr/bin/env python3
"""(2) Discriminant identity, symbolically over Z[x1, x2, A], plus a brute-force (no discriminant)
root census of S3 at small primes over ALL A with A^2 != 4.  Usage: v_identity.py OUT.json"""
import json, sys, time
import numpy as np

# ---- tiny multivariate polynomial ring over Z: dict {(e_x1, e_x2, e_A): coeff}
def P(d):
    return {k: v for k, v in d.items() if v}

def add(p, q, s=1):
    r = dict(p)
    for k, v in q.items():
        r[k] = r.get(k, 0) + s * v
    return P(r)

def mul(p, q):
    r = {}
    for k1, v1 in p.items():
        for k2, v2 in q.items():
            k = tuple(a + b for a, b in zip(k1, k2))
            r[k] = r.get(k, 0) + v1 * v2
    return P(r)

def const(c):
    return P({(0, 0, 0): c})

X1, X2, AA = {(1, 0, 0): 1}, {(0, 1, 0): 1}, {(0, 0, 1): 1}
one = const(1)
a = mul(add(X1, X2, -1), add(X1, X2, -1))                 # (x1 - x2)^2
m = mul(X1, X2)
c = mul(add(m, one, -1), add(m, one, -1))                 # (x1 x2 - 1)^2
B2 = add(mul(add(X1, X2), add(m, one)), mul(const(2), mul(AA, m)))
def f(x):
    return add(add(mul(x, mul(x, x)), mul(AA, mul(x, x))), x)
lhs = add(mul(B2, B2), mul(a, c), -1)                     # disc/4 of a X^2 - 2 B2 X + c
rhs = mul(const(4), mul(f(X1), f(X2)))
diff = add(lhs, rhs, -1)
out = {"symbolic": {"B2^2 - a c - 4 f(x1) f(x2) over Z[x1,x2,A]": "zero polynomial" if not diff else diff,
                    "n_monomials_lhs": len(lhs), "identity_holds": not diff,
                    "given_variant_same_disc": True if not diff else None,
                    "note": "a c is symmetric in (a, c), so the reversed ('given') polynomial has the same disc/4"}}
print(out)

# ---- brute force at small primes: roots by evaluating S3 at every X in F_p (no discriminant used)
def census(p):
    t0 = time.process_time()
    x = np.arange(p, dtype=np.int64)
    sq = np.zeros(p, dtype=bool)
    sq[(x * x) % p] = True
    tot = {"A_values": 0, "mixed_pairs": 0, "mixed_with_Fp_root": 0,
           "same_nondiag_nonZ_pairs": 0, "same_not_2_roots": 0, "same_root_on_other_curve": 0,
           "same_root_in_Z": 0, "identity_failures": 0, "A_with_full_2torsion": 0}
    for A in range(p):
        if (A * A - 4) % p == 0:
            continue
        tot["A_values"] += 1
        f = x * ((x * x + A * x + 1) % p) % p
        cls = np.where(f == 0, 0, np.where(sq[f], 1, 2))
        if sq[(A * A - 4) % p]:
            tot["A_with_full_2torsion"] += 1
        a1 = x[:, None, None]; a2 = x[None, :, None]; XX = x[None, None, :]
        mm = a1 * a2 % p
        B2 = ((a1 + a2) * (mm + 1) + 2 * A * mm) % p
        aa = (a1 - a2) ** 2 % p
        cc = (mm - 1) ** 2 % p
        val = (aa * (XX * XX % p) - 2 * B2 * XX + cc) % p      # p x p x p
        isroot = val == 0
        nroots = isroot.sum(axis=2)                              # distinct roots in F_p
        # identity check on every pair
        d = (B2[:, :, 0] ** 2 - aa[:, :, 0] * cc[:, :, 0]) % p
        tot["identity_failures"] += int((d != 4 * f[:, None] % p * f[None, :] % p).sum())
        c1 = cls[:, None]; c2 = cls[None, :]
        mixed = ((c1 == 1) & (c2 == 2)) | ((c1 == 2) & (c2 == 1))
        tot["mixed_pairs"] += int(mixed.sum())
        tot["mixed_with_Fp_root"] += int((mixed & (nroots > 0)).sum())
        same = (c1 == c2) & (c1 != 0) & (x[:, None] != x[None, :])
        tot["same_nondiag_nonZ_pairs"] += int(same.sum())
        tot["same_not_2_roots"] += int((same & (nroots != 2)).sum())
        rcls = cls[None, None, :]
        other = np.where(c1 == 1, 2, 1)[:, :, None]
        tot["same_root_on_other_curve"] += int((same[:, :, None] & isroot & (rcls == other)).sum())
        tot["same_root_in_Z"] += int((same[:, :, None] & isroot & (rcls == 0)).sum())
    tot["cpu_s"] = round(time.process_time() - t0, 1)
    return tot

out["bruteforce"] = {}
for p in (101, 103):
    out["bruteforce"][p] = census(p)
    print(p, out["bruteforce"][p])
json.dump(out, open(sys.argv[1], "w"), indent=1)
