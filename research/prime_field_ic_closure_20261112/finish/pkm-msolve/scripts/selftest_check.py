#!/usr/bin/env python3
"""check.enumerate_with_point agrees with pkmgen.enumerate_zeros (already brute-force validated)."""
import os, random, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pkmgen as G
import check as C
rng = random.Random(3)
bad = tot = 0
for p in (97, 769):
    A, B, n = G.prime_order_curve(p, rng)
    g = G.choose_g(p, A, B, rng)
    F = G.Fp2(p)
    for (m, t) in [(4, 1), (4, 2), (5, 2), (6, 1), (7, 1), (6, 2)]:
        z = G.root_of_unity(p, t, rng)
        V = [g * pow(z, k, p) % p for k in range(1 << t)]
        for _ in range(3):
            while True:
                x = rng.randrange(p); f = (x**3 + A*x + B) % p
                if f and G.legendre(f, p) == 1: break
            y = G.sqrt_mod(f, p)
            z1, _ = G.enumerate_zeros(p, A, B, V, m, x, y, want_solutions=False)
            z2, s2 = C.enumerate_with_point(p, A, B, V, m, x, (y, 0))
            tot += 1; bad += (z1 != z2)
            if z1 != z2: print("MISMATCH", p, m, t, x, len(z1), len(z2))
print("selftest_check mismatches", bad, "of", tot)
