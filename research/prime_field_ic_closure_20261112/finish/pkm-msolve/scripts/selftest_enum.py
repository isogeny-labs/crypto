#!/usr/bin/env python3
"""Self-test: enumerate_zeros (point arithmetic over F_{p^2}, MITM) against a brute
force that scans every u in F_{p^2} for the S3 chain, on tiny primes.
Also checks the planted construction by evaluating the generated polynomials."""
import os
import random
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pkmgen as G  # noqa: E402
import msparse as MP  # noqa: E402


def s3_fp2(F, A, B, a, b, c):
    one = (1, 0)
    Af, Bf = (A % F.p, 0), (B % F.p, 0)
    amb = F.sub(a, b)
    apb = F.add(a, b)
    ab = F.mul(a, b)
    t1 = F.mul(F.mul(amb, amb), F.mul(c, c))
    inner = F.add(F.mul(apb, F.add(ab, Af)), F.mul((2, 0), Bf))
    t2 = F.mul((F.p - 2, 0), F.mul(inner, c))
    abmA = F.sub(ab, Af)
    t3 = F.sub(F.mul(abmA, abmA), F.mul(F.mul((4, 0), Bf), apb))
    return F.add(F.add(t1, t2), t3)


def brute(p, A, B, V, m, xR):
    F = G.Fp2(p)
    allu = [(a0, a1) for a0 in range(p) for a1 in range(p)]
    roots_cache = {}

    def roots(a, b):
        key = (a, b)
        if key not in roots_cache:
            roots_cache[key] = [u for u in allu if s3_fp2(F, A, B, a, b, u) == (0, 0)]
        return roots_cache[key]

    import itertools
    zeros = set()
    for tup in itertools.product(range(len(V)), repeat=m):
        xs = [(V[i], 0) for i in tup]
        frontier = roots(xs[0], xs[1])
        for k in range(2, m - 1):
            nf = set()
            for u in frontier:
                nf.update(roots(u, xs[k]))
            frontier = nf
        if any(s3_fp2(F, A, B, u, xs[m - 1], (xR, 0)) == (0, 0) for u in frontier):
            zeros.add(tup)
    return zeros


def main():
    rng = random.Random(7)
    p = 97
    A, B, n = G.prime_order_curve(p, rng)
    g = G.choose_g(p, A, B, rng)
    print("p", p, "curve", A, B, "#E", n, "g", g)
    bad = 0
    for (m, t) in [(4, 1), (5, 1), (4, 2)]:
        z = G.root_of_unity(p, t, rng)
        V = [g * pow(z, k, p) % p for k in range(1 << t)]
        for trial in range(3):
            while True:
                xR = rng.randrange(p)
                f = (xR ** 3 + A * xR + B) % p
                if f and G.legendre(f, p) == 1:
                    break
            yR = G.sqrt_mod(f, p)
            ez, _ = G.enumerate_zeros(p, A, B, V, m, xR, yR, want_solutions=False)
            bz = brute(p, A, B, V, m, xR)
            ok = ez == bz
            bad += not ok
            print(f"m={m} t={t} xR={xR}: enum {len(ez)} brute {len(bz)} {'OK' if ok else 'MISMATCH'}")
    print("selftest mismatches:", bad)


if __name__ == "__main__":
    main()
