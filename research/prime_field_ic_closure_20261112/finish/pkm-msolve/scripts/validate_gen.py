#!/usr/bin/env python3
"""Instrument identity check: regenerate the repo's committed PKM systems with
pkmgen.build_system and compare polynomial sets exactly.

Usage: validate_gen.py SYS.ms [SYS.ms ...]
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pkmgen  # noqa: E402
import msparse  # noqa: E402


P_MOD = [0]


def canon(P):
    # up to a nonzero scalar: make the coefficient of the largest exponent tuple 1
    p = P_MOD[0]
    k0 = max(P)
    c = pow(P[k0], p - 2, p)
    return tuple(sorted((k, v * c % p) for k, v in P.items()))


def main():
    for path in sys.argv[1:]:
        names, p, polys = msparse.read_ms(path)
        P_MOD[0] = p
        m = 1 + max(int(n[1:].split("_")[0]) for n in names if n.startswith("y"))
        t = 1 + max(int(n.split("_")[1]) for n in names if n.startswith("y"))
        nv = len(names)
        zero = tuple([0] * nv)
        first = polys[0]
        A2 = first.get(zero, 0)
        u0 = names.index("u0")
        y00 = names.index("y0_0")
        e_u = [0] * nv
        e_u[u0] = 1
        c_u = first.get(tuple(e_u), 0)  # = -4B
        B = (-c_u) * pow(4, p - 2, p) % p
        e_g2 = [0] * nv
        e_g2[y00] = 2
        e_g2[u0] = 2
        g2 = first.get(tuple(e_g2), 0)
        A_c = pkmgen.sqrt_mod(A2, p)
        g_c = pkmgen.sqrt_mod(g2, p)
        xR = int(path.split("_x")[-1].split(".")[0])
        target = set(map(canon, polys))
        hit = None
        for A in (A_c, (-A_c) % p):
            for g in (g_c, (-g_c) % p):
                _, eqs, _ = pkmgen.build_system(p, A, B, g, m, t, xR)
                if set(map(canon, eqs)) == target:
                    hit = (A, B, g)
        print(os.path.basename(path), f"m={m} t={t} nv={nv} neq={len(polys)}",
              "MATCH" if hit else "NO MATCH", hit)


if __name__ == "__main__":
    main()
