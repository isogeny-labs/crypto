#!/usr/bin/env python3
"""Second self-test: targets built as x(sum of random signed points with x in V)
(E or twist points, over F_{p^2}), so the zero set is non-empty; enum vs brute."""
import os, random, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pkmgen as G
from selftest_enum import brute

def main():
    rng = random.Random(11)
    p = 97
    A, B, n = G.prime_order_curve(p, rng)
    g = G.choose_g(p, A, B, rng)
    F = G.Fp2(p)
    bad = tot = 0
    for (m, t) in [(4, 2), (5, 1), (4, 1), (5, 2)]:
        z = G.root_of_unity(p, t, rng)
        V = [g * pow(z, k, p) % p for k in range(1 << t)]
        sp = G.signed_points(F, A, B, V)
        done = 0
        while done < 2:
            S = None
            for i in range(m):
                S = G.ec_add(F, A, S, sp[rng.randrange(len(V))][rng.randrange(2)][1])
            if S is None or S[0][1] != 0 or S[1][1] != 0:
                continue  # want R in E(F_p) with x_R in F_p
            xR, yR = S[0][0], S[1][0]
            ez, _ = G.enumerate_zeros(p, A, B, V, m, xR, yR, want_solutions=False)
            bz = brute(p, A, B, V, m, xR)
            ok = ez == bz
            bad += not ok; tot += 1; done += 1
            print(f"m={m} t={t} xR={xR}: enum {len(ez)} brute {len(bz)} {'OK' if ok else 'MISMATCH'}")
    print("selftest2 mismatches:", bad, "of", tot)

main()
