#!/usr/bin/env python3
"""Numerical checks of the torus-tower identities behind PROTOCOL.md (derivation checks, not the
measurement).  Pure Python big integers; no external library.

Checks:
  1. Vélu 2-isogeny chain on the supersingular j=1728 curve y^2 = x^3 + x (p = 3 mod 4), kernel (0,0):
     x1 = (x^2+1)/x on E1: y^2 = x^3 - 4x ; x2 = (x1^2-4)/x1 on E2: y^2 = x^3 + 16x = E0 (x = 4u).
     Identity: x1 = 2*sigma(x), x2/4 = tau(x1/2), with sigma(a) = (a^2+1)/(2a), tau(a) = (a^2-1)/(2a).
     Points are mapped and checked on the codomains.
  2. sigma is doubling on G_m in the coordinate z = (a+1)/(a-1):  z(sigma(a)) = z(a)^2.
     tau is doubling on the norm-one torus T2 in the coordinate z = (a+i)/(a-i): z(tau(a)) = z(a)^2,
     and z(a)^(p+1) = 1 for every a in F_p.
  3. Fibre sizes: for 2^t | p+1 the set V = tau^{-t}(c) has exactly 2^t elements of F_p for c = tau^t(a0)
     whenever z(a0)^(2^(t+1)) != 1 (exhaustive over F_p at p = 8191, t = 1..6).
  4. At the NIST primes P-192, P-256, P-521: v2(p-1), v2(p+1); the coset V built by walking
     z0 * mu_{2^t} inside T2 (F_{p^2} arithmetic) gives 2^t distinct a-values in F_p and tau^t(a) = c
     on every one of them (the PKM "L(x) = c on all of V" check), t = 12.
Usage: torus_check.py [--out FILE.json]
"""
import json, random, sys, time

P192 = 2**192 - 2**64 - 1
P256 = 2**256 - 2**224 + 2**192 + 2**96 - 1
P521 = 2**521 - 1

def inv(a, p): return pow(a % p, p - 2, p)
def v2(n):
    k = 0
    while n % 2 == 0: n //= 2; k += 1
    return k

def sigma(a, p): return (a * a + 1) * inv(2 * a, p) % p
def tau(a, p): return (a * a - 1) * inv(2 * a, p) % p

# F_{p^2} = F_p[i]/(i^2+1), p = 3 mod 4; elements (re, im)
def f2mul(A, B, p): return ((A[0]*B[0] - A[1]*B[1]) % p, (A[0]*B[1] + A[1]*B[0]) % p)
def f2inv(A, p):
    n = inv((A[0]*A[0] + A[1]*A[1]) % p, p)
    return (A[0]*n % p, (-A[1])*n % p)
def f2pow(A, e, p):
    R = (1, 0)
    while e:
        if e & 1: R = f2mul(R, A, p)
        A = f2mul(A, A, p); e >>= 1
    return R
def z_of_a_T2(a, p):  # (a+i)/(a-i)
    return f2mul((a % p, 1), f2inv((a % p, p - 1), p), p)
def a_of_z_T2(z, p):  # a = i (z+1)/(z-1)
    num = f2mul((0, 1), ((z[0] + 1) % p, z[1]), p)
    return f2mul(num, f2inv(((z[0] - 1) % p, z[1]), p), p)

def check_velu(p, trials, rng):
    """Vélu chain on y^2 = x^3 + x with kernel (0,0)."""
    ok = 0
    for _ in range(trials):
        while True:
            x = rng.randrange(1, p)
            r = (x**3 + x) % p
            if pow(r, (p - 1) // 2, p) == 1: break
        y = pow(r, (p + 1) // 4, p)            # p = 3 mod 4
        assert y * y % p == r
        # phi_1: (x,y) -> ((x^2+1)/x, y (x^2-1)/x^2) onto y^2 = x^3 - 4x
        x1 = (x*x + 1) * inv(x, p) % p
        y1 = y * (x*x - 1) % p * inv(x*x % p, p) % p
        assert y1*y1 % p == (x1**3 - 4*x1) % p, "codomain 1"
        # phi_2 with kernel (0,0) on y^2 = x^3 - 4x: (x,y) -> ((x^2-4)/x, y (x^2+4)/x^2) onto y^2 = x^3 + 16x
        x2 = (x1*x1 - 4) * inv(x1, p) % p
        y2 = y1 * (x1*x1 + 4) % p * inv(x1*x1 % p, p) % p
        assert y2*y2 % p == (x2**3 + 16*x2) % p, "codomain 2"
        u, v = x2 * inv(4, p) % p, y2 * inv(8, p) % p
        assert v*v % p == (u**3 + u) % p, "back on E0"
        assert x1 == 2 * sigma(x, p) % p, "x1 = 2 sigma(x)"
        assert u == tau(x1 * inv(2, p) % p, p), "x2/4 = tau(x1/2)"
        ok += 1
    return ok

def check_doubling(p, trials, rng):
    for _ in range(trials):
        a = rng.randrange(2, p - 1)
        # G_m: z = (a+1)/(a-1)
        z = (a + 1) * inv(a - 1, p) % p
        s = sigma(a, p)
        zs = (s + 1) * inv(s - 1, p) % p
        assert zs == z * z % p, "sigma doubling"
        # T2: z = (a+i)/(a-i)
        zt = z_of_a_T2(a, p)
        assert f2pow(zt, p + 1, p) == (1, 0), "norm one"
        tt = tau(a, p)
        assert z_of_a_T2(tt, p) == f2mul(zt, zt, p), "tau doubling"
    return trials

def fibre_sizes(p, tmax, rng, draws=20):
    out = {}
    assert (p + 1) % (1 << tmax) == 0
    for t in range(1, tmax + 1):
        sizes = []
        for _ in range(draws):
            while True:
                a0 = rng.randrange(1, p)
                z0 = z_of_a_T2(a0, p)
                if f2pow(z0, 1 << (t + 1), p) != (1, 0): break
            c = a0
            for _ in range(t): c = tau(c, p)
            # exhaustive preimage over F_p (a = 0 maps to infinity: excluded)
            V = []
            for a in range(1, p):
                b = a; bad = False
                for _ in range(t):
                    if b == 0: bad = True; break
                    b = tau(b, p)
                if not bad and b == c: V.append(a)
            sizes.append(len(V))
        out[t] = {"expected": 1 << t, "min": min(sizes), "max": max(sizes), "draws": draws}
    return out

def nist_towers(t, rng):
    res = {}
    for name, p in (("P-192", P192), ("P-256", P256), ("P-521", P521)):
        assert p % 4 == 3
        rec = {"v2_p_minus_1": v2(p - 1), "v2_p_plus_1": v2(p + 1), "t": t}
        # generator of mu_{2^t} in T2
        while True:
            w = z_of_a_T2(rng.randrange(1, p), p)
            zeta = f2pow(w, (p + 1) >> t, p)
            if f2pow(zeta, 1 << (t - 1), p) != (1, 0): break
        while True:
            z0 = z_of_a_T2(rng.randrange(1, p), p)
            if f2pow(z0, 1 << (t + 1), p) != (1, 0): break
        c = a_of_z_T2(f2pow(z0, 1 << t, p), p)
        assert c[1] == 0
        c = c[0]
        t0 = time.time()
        V = set(); z = z0; allc = True
        for _ in range(1 << t):
            a = a_of_z_T2(z, p)
            assert a[1] == 0, "a not in F_p"
            a = a[0]
            b = a
            for _ in range(t): b = tau(b, p)
            if b != c: allc = False
            V.add(a)
            z = f2mul(z, zeta, p)
        rec.update({"distinct_a": len(V), "expected": 1 << t, "tower_hits_c_on_all": allc,
                    "seconds": round(time.time() - t0, 2)})
        res[name] = rec
    return res

def main():
    out_path = sys.argv[sys.argv.index("--out") + 1] if "--out" in sys.argv else None
    rng = random.Random(20261114)
    report = {}
    def next_prime_3mod4(n):
        while True:
            n += 1
            if n % 4 == 3 and all(n % q for q in range(2, int(n**0.5) + 1)): return n
    for p in (8191, next_prime_3mod4(40960), next_prime_3mod4(65500)):  # 8191 = 2^13-1
        assert p % 4 == 3 and all(p % q for q in range(2, int(p**0.5) + 1))
        report.setdefault("toy_primes", []).append(p)
        report[f"velu_identity_p{p}"] = check_velu(p, 200, rng)
        report[f"doubling_identity_p{p}"] = check_doubling(p, 200, rng)
    report["velu_identity_P192"] = check_velu(P192, 20, rng)
    report["doubling_identity_P256"] = check_doubling(P256, 20, rng)
    report["fibre_sizes_p8191"] = fibre_sizes(8191, 6, rng)
    report["nist"] = nist_towers(12, rng)
    s = json.dumps(report, indent=1)
    print(s)
    if out_path:
        open(out_path, "w").write(s + "\n")

if __name__ == "__main__":
    main()
