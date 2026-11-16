#!/usr/bin/env python3
"""Independent check of D9 (selector certificate, Kummer m = 3): d_F <= 3t + 5.

Stdlib only.  For each instance (prime p, curve a, b, tower generator g,
target x_R, tower height t, K = 2^t):

  1. at every grid point x in (g*mu_K)^3 the residual pair
         f1(u) = S3(x1, x2, u),   f2(u) = S3(u, x3, x_R)
     is coprime over F_p (extended Euclid), i.e. 1 is in the ideal of the
     system in R (D1), which is the hypothesis of D9;
  2. Bezout multipliers H1, H2 with deg_u <= 1 and H1 f1 + H2 f2 = 1;
  3. G_i := sum_x delta_x H_i^x is converted to the normal-form basis z^k u^e
     of R = F_p[z1,z2,z3]/(z_i^K - 1)[u] by an inverse DFT on mu_K^3;
  4. the identity G1*F1 + G2*F2 = 1 is checked by EXPLICIT multiplication in
     R (cyclic convolution in the z exponents, polynomial product in u),
     with F_i built symbolically from S3 -- not pointwise -- so the check does
     not rely on the evaluation isomorphism of D1;
  5. a tamper control: changing one coefficient of G1 must break the identity;
  6. the certificate degree max(popcount(k) + e) + deg F_i is reported and
     compared with the D9 bound 3t + 5.

Usage: python3 d9_check.py [--json out.json]
"""
import json
import random
import sys
import time


def inv(a, p):
    return pow(a % p, p - 2, p)


def popcount(k):
    return bin(k).count("1")


# ---------- univariate polynomials over F_p (lists, low degree first) ----------

def ptrim(f):
    while f and f[-1] == 0:
        f.pop()
    return f


def pmul(f, g, p):
    if not f or not g:
        return []
    r = [0] * (len(f) + len(g) - 1)
    for i, a in enumerate(f):
        if a:
            for j, b in enumerate(g):
                r[i + j] = (r[i + j] + a * b) % p
    return ptrim(r)


def padd(f, g, p):
    n = max(len(f), len(g))
    return ptrim([((f[i] if i < len(f) else 0) + (g[i] if i < len(g) else 0)) % p for i in range(n)])


def pdivmod(f, g, p):
    f = list(f)
    q = [0] * max(len(f) - len(g) + 1, 1)
    il = inv(g[-1], p)
    while len(f) >= len(g) and f:
        c = f[-1] * il % p
        d = len(f) - len(g)
        q[d] = c
        for i, b in enumerate(g):
            f[i + d] = (f[i + d] - c * b) % p
        ptrim(f)
    return ptrim(q), f


def bezout(f1, f2, p):
    """Return (H1, H2) with H1 f1 + H2 f2 = 1, or None if gcd is non-constant / both zero."""
    r0, r1 = ptrim(list(f1)), ptrim(list(f2))
    s0, s1, t0, t1 = [1], [], [], [1]
    while r1:
        q, r = pdivmod(r0, r1, p)
        r0, r1 = r1, r
        s0, s1 = s1, padd(s0, [(-c) % p for c in pmul(q, s1, p)], p)
        t0, t1 = t1, padd(t0, [(-c) % p for c in pmul(q, t1, p)], p)
    if len(r0) != 1:
        return None  # gcd zero or of positive degree: common root over the closure
    c = inv(r0[0], p)
    return [x * c % p for x in s0], [x * c % p for x in t0]


# ---------- Semaev S3 ----------

def s3_coeffs_in_x3(x1, x2, a, b, p):
    """S3(x1,x2,X) = A X^2 + B X + C as [C, B, A]."""
    A = (x1 - x2) ** 2 % p
    B = (-2 * ((x1 + x2) * (x1 * x2 + a) + 2 * b)) % p
    C = ((x1 * x2 - a) ** 2 - 4 * b * (x1 + x2)) % p
    return ptrim([C, B, A])


def s3_symbolic_F1(g, a, b, p):
    """F1 = S3(g z1, g z2, u) in R as {(k1,k2,k3,e): c}; S3 symmetric, coefficients from expansion.

    S3(x1,x2,x3) = x1^2 x3^2 - 2 x1 x2 x3^2 + x2^2 x3^2
                   - 2(x1^2 x2 + x1 x2^2 + a x1 + a x2 + 2b) x3
                   + x1^2 x2^2 - 2 a x1 x2 + a^2 - 4b x1 - 4b x2
    with x_i = g z_i (so x_i^2 = g^2 z_i^2), x3 = u.
    """
    terms = {}

    def add(k1, k2, e, c):
        key = (k1, k2, 0, e)
        terms[key] = (terms.get(key, 0) + c) % p

    g2 = g * g % p
    add(2, 0, 2, g2)
    add(1, 1, 2, -2 * g2)
    add(0, 2, 2, g2)
    add(2, 1, 1, -2 * g2 * g)
    add(1, 2, 1, -2 * g2 * g)
    add(1, 0, 1, -2 * a * g)
    add(0, 1, 1, -2 * a * g)
    add(0, 0, 1, -4 * b)
    add(2, 2, 0, g2 * g2)
    add(1, 1, 0, -2 * a * g2)
    add(0, 0, 0, a * a)
    add(1, 0, 0, -4 * b * g)
    add(0, 1, 0, -4 * b * g)
    return {k: v for k, v in terms.items() if v}


def s3_symbolic_F2(g, a, b, xR, p):
    """F2 = S3(u, g z3, x_R) in R: expand in x1 = u, x2 = g z3, x3 = xR."""
    terms = {}

    def add(k3, e, c):
        key = (0, 0, k3, e)
        terms[key] = (terms.get(key, 0) + c) % p

    g2 = g * g % p
    r, r2 = xR % p, xR * xR % p
    # x1^2 x3^2 - 2 x1 x2 x3^2 + x2^2 x3^2
    add(0, 2, r2)
    add(1, 1, -2 * g * r2)
    add(2, 0, g2 * r2)
    # -2(x1^2 x2 + x1 x2^2 + a x1 + a x2 + 2b) x3
    add(1, 2, -2 * g * r)
    add(2, 1, -2 * g2 * r)
    add(0, 1, -2 * a * r)
    add(1, 0, -2 * a * g * r)
    add(0, 0, -4 * b * r)
    # x1^2 x2^2 - 2 a x1 x2 + a^2 - 4b x1 - 4b x2
    add(2, 2, g2)
    add(1, 1, -2 * a * g)
    add(0, 0, a * a)
    add(0, 1, -4 * b)
    add(1, 0, -4 * b * g)
    return {k: v for k, v in terms.items() if v}


def r_degree(F):
    return max(popcount(k1) + popcount(k2) + popcount(k3) + e for (k1, k2, k3, e) in F)


def r_mul_add(acc, G, F, K, p):
    for (a1, a2, a3, ea), ca in G.items():
        for (b1, b2, b3, eb), cb in F.items():
            key = ((a1 + b1) % K, (a2 + b2) % K, (a3 + b3) % K, ea + eb)
            acc[key] = (acc.get(key, 0) + ca * cb) % p


# ---------- DFT on mu_K^3 ----------

def root_of_unity(K, p):
    for h in range(2, p):
        z = pow(h, (p - 1) // K, p)
        if pow(z, K // 2, p) != 1:
            return z
    raise ValueError("no root")


def inverse_dft3(vals, K, zeta, p):
    """vals[(a1,a2,a3)] = phi(zeta^a) -> coefficients c[(k1,k2,k3)] of sum c_k z^k."""
    zi = inv(zeta, p)
    W = [[pow(zi, a * k % K, p) for a in range(K)] for k in range(K)]
    arr = vals
    for axis in range(3):
        new = {}
        for idx, v in arr.items():
            if not v:
                continue
            a = idx[axis]
            for k in range(K):
                key = list(idx)
                key[axis] = k
                key = tuple(key)
                new[key] = (new.get(key, 0) + v * W[k][a]) % p
        arr = new
    s = inv(pow(K, 3, p), p)
    return {k: v * s % p for k, v in arr.items() if v}


def run_instance(p, t, seed):
    rng = random.Random(seed)
    K = 1 << t
    zeta = root_of_unity(K, p)
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a ** 3 + 27 * b ** 2) % p:
            break
    g = rng.randrange(1, p)
    xR = rng.randrange(p)
    pts = [g * pow(zeta, i, p) % p for i in range(K)]
    t0 = time.time()
    # Steps 1-2: pointwise Bezout.
    H = {}  # (a1,a2,a3) -> (H1, H2)
    maxdegH = 0
    for i1 in range(K):
        for i2 in range(K):
            f1 = s3_coeffs_in_x3(pts[i1], pts[i2], a, b, p)
            for i3 in range(K):
                # f2(u) = S3(u, x3, xR) = S3(x3, xR, u) by symmetry
                f2 = s3_coeffs_in_x3(pts[i3], xR, a, b, p)
                bz = bezout(f1, f2, p)
                if bz is None:
                    return {"p": p, "t": t, "seed": seed, "consistent_point": [i1, i2, i3], "ok": False}
                h1, h2 = bz
                if padd(pmul(h1, f1, p), pmul(h2, f2, p), p) != [1]:
                    return {"p": p, "t": t, "seed": seed, "bezout_fail": True, "ok": False}
                maxdegH = max(maxdegH, len(h1) - 1, len(h2) - 1)
                H[(i1, i2, i3)] = (h1, h2)
    # Step 3: G_i in normal form.
    G = [{}, {}]
    for i in range(2):
        for e in range(2):
            vals = {pt: (hh[i][e] if e < len(hh[i]) else 0) for pt, hh in H.items()}
            for k, c in inverse_dft3(vals, K, zeta, p).items():
                G[i][(k[0], k[1], k[2], e)] = c
    F1 = s3_symbolic_F1(g, a, b, p)
    F2 = s3_symbolic_F2(g, a, b, xR, p)
    # Step 4: explicit ring identity.
    acc = {}
    r_mul_add(acc, G[0], F1, K, p)
    r_mul_add(acc, G[1], F2, K, p)
    acc = {k: v for k, v in acc.items() if v}
    identity_ok = acc == {(0, 0, 0, 0): 1}
    # Step 5: tamper control.
    key0 = next(iter(G[0]))
    Gt = dict(G[0])
    Gt[key0] = (Gt[key0] + 1) % p
    acc2 = {}
    r_mul_add(acc2, Gt, F1, K, p)
    r_mul_add(acc2, G[1], F2, K, p)
    acc2 = {k: v for k, v in acc2.items() if v}
    tamper_breaks = acc2 != {(0, 0, 0, 0): 1}
    # Step 6: degrees.
    dF1, dF2 = r_degree(F1), r_degree(F2)
    cert = max(max(popcount(k1) + popcount(k2) + popcount(k3) + e for (k1, k2, k3, e) in G[0]) + dF1,
               max(popcount(k1) + popcount(k2) + popcount(k3) + e for (k1, k2, k3, e) in G[1]) + dF2)
    return {
        "p": p, "t": t, "K": K, "seed": seed, "a": a, "b": b, "g": g, "xR": xR,
        "grid_points": K ** 3, "all_points_coprime": True, "max_deg_u_H": maxdegH,
        "deg_F1": dF1, "deg_F2": dF2, "identity_in_R": identity_ok, "tamper_breaks": tamper_breaks,
        "certificate_degree": cert, "bound_3t_plus_5": 3 * t + 5,
        "ok": identity_ok and tamper_breaks and cert <= 3 * t + 5 and maxdegH <= 1,
        "seconds": round(time.time() - t0, 2),
    }


def sqrt_mod(n, p):
    n %= p
    if n == 0:
        return 0
    if pow(n, (p - 1) // 2, p) != 1:
        return None
    # Tonelli-Shanks
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, tt, r = s, pow(z, q, p), pow(n, q, p), pow(n, (q + 1) // 2, p)
    while tt != 1:
        i, t2 = 0, tt
        while t2 != 1:
            t2 = t2 * t2 % p
            i += 1
        bb = pow(c, 1 << (m - i - 1), p)
        m, c, tt, r = i, bb * bb % p, tt * bb * bb % p, r * bb % p
    return r


def quad_root(f, p):
    """An F_p root of f = [C, B, A] (A != 0), or None."""
    C, B, A = f
    d = sqrt_mod(B * B - 4 * A * C, p)
    if d is None:
        return None
    return (-B + d) * inv(2 * A, p) % p


def negative_control(p, t, seed):
    """Plant a common root at one grid point; the checker must reject the instance."""
    rng = random.Random(seed)
    K = 1 << t
    zeta = root_of_unity(K, p)
    for _ in range(1000):
        a, b = rng.randrange(p), rng.randrange(p)
        g = rng.randrange(1, p)
        pts = [g * pow(zeta, i, p) % p for i in range(K)]
        i1, i2, i3 = rng.randrange(K), rng.randrange(K), rng.randrange(K)
        f1 = s3_coeffs_in_x3(pts[i1], pts[i2], a, b, p)
        if len(f1) != 3:
            continue
        u0 = quad_root(f1, p)
        if u0 is None:
            continue
        h = s3_coeffs_in_x3(u0, pts[i3], a, b, p)  # S3(u0, x3, X) in X
        if len(h) != 3:
            continue
        xR = quad_root(h, p)
        if xR is None:
            continue
        f2 = s3_coeffs_in_x3(pts[i3], xR, a, b, p)
        rejected = bezout(f1, f2, p) is None
        return {"p": p, "t": t, "seed": seed, "planted_point": [i1, i2, i3], "rejected": rejected}
    return {"p": p, "t": t, "seed": seed, "rejected": None}


def main():
    if "--control" in sys.argv:
        res = [negative_control(p, t, 7_000_000 + t) for p in (998244353, 2013265921) for t in (2, 3, 4)]
        for r in res:
            print(json.dumps(r))
        ok = all(r["rejected"] for r in res)
        print(f"CONTROL rejected={sum(bool(r['rejected']) for r in res)}/{len(res)}")
        sys.exit(0 if ok else 1)
    out = None
    if "--json" in sys.argv:
        out = sys.argv[sys.argv.index("--json") + 1]
    primes = [998244353, 2013265921]  # 119*2^23+1, 15*2^27+1
    rows = []
    for p in primes:
        for t in range(2, 6):
            for draw in range(3):
                seed = 9_000_000 + 1000 * t + 10 * draw + (p % 7)
                r = run_instance(p, t, seed)
                rows.append(r)
                print(json.dumps(r), flush=True)
    n_ok = sum(r["ok"] for r in rows)
    print(f"SUMMARY ok={n_ok}/{len(rows)}")
    if out:
        with open(out, "w") as fh:
            json.dump({"rows": rows, "ok": n_ok, "total": len(rows)}, fh, indent=1)
    sys.exit(0 if n_ok == len(rows) else 1)


if __name__ == "__main__":
    main()
