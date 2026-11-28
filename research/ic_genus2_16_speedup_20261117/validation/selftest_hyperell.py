#!/usr/bin/env python3
"""Self-test of the validator's own hyperelliptic code (hyperell.py) on tiny
curves where #J(F_p) can be counted by brute force as the number of reduced
Mumford pairs (u monic, deg u <= g, deg v < deg u, u | v^2 - f).

Checks, per random curve:
  1. brute-force #J == L(1) from own N_1..N_g (Newton identities);
  2. Hasse-Witt charpoly mod p == L-polynomial coefficients mod p;
  3. [#J] D == 0 for random D (Cantor law), associativity/commutativity spot checks;
  4. optional: PARI hyperellcharpoly(1) == brute-force #J (if cypari2 importable).
"""
import itertools
import random
import sys

sys.path.insert(0, __import__("os").path.dirname(__file__))
from hyperell import (HCurve, charpoly_mod_p, count_points_ext, hasse_witt,
                      lpoly_from_counts, pmod, pmul, psub, trim, pgcd)

try:
    import cypari2
    PARI = cypari2.Pari()
except Exception:  # noqa: BLE001
    PARI = None


def brute_jac(f, p, g):
    n = 0
    for d in range(0, g + 1):
        for ulow in itertools.product(range(p), repeat=d):
            u = list(ulow) + [1]
            if d == 0:
                n += 1
                continue
            for vv in itertools.product(range(p), repeat=d):
                v = trim(list(vv))
                if pmod(psub(pmul(v, v, p), f, p), u, p) == []:
                    n += 1
    return n


def squarefree(f, p):
    df = trim([(i * c) % p for i, c in enumerate(f)][1:])
    return len(pgcd(f, df, p)) == 1


def main():
    rng = random.Random(2026)
    cases = [(2, 5), (2, 7), (2, 11), (2, 13), (3, 5), (3, 7), (4, 5)]
    fails = 0
    for g, p in cases:
        for _ in range(3):
            while True:
                f = [rng.randrange(p) for _ in range(2 * g + 1)] + [1]
                if squarefree(f, p):
                    break
            C = HCurve(f, p)
            nj = brute_jac(f, p, g)
            N = [count_points_ext(f, p, k) for k in range(1, g + 1)]
            a, full = lpoly_from_counts(N, p, g)
            L1 = sum(full)
            hw = charpoly_mod_p(hasse_witt(f, p, g), p)  # det(tI - A), low first
            # a_i == coeff of t^{g-i} in det(tI - A) mod p
            hw_ok = all((a[i] - hw[g - i]) % p == 0 for i in range(1, g + 1))
            ok_law = True
            for _ in range(4):
                D = C.random_element(rng)
                E = C.random_element(rng)
                F = C.random_element(rng)
                if not C.is_zero(C.mul(nj, D)):
                    ok_law = False
                if C.add(C.add(D, E), F) != C.add(D, C.add(E, F)) or C.add(D, E) != C.add(E, D):
                    ok_law = False
                if not C.is_zero(C.add(D, C.neg(D))):
                    ok_law = False
            pari = None
            if PARI is not None:
                fx = " + ".join(f"{c}*x^{i}" for i, c in enumerate(f))
                pari = int(PARI(f"subst(lift(hyperellcharpoly(Mod(1,{p})*({fx}))), x, 1)"))
            ok = nj == L1 and hw_ok and ok_law and (pari is None or pari == nj)
            fails += not ok
            print(f"g={g} p={p} f={f} bruteJ={nj} L(1)={L1} HWmodp={'ok' if hw_ok else 'BAD'} "
                  f"law={'ok' if ok_law else 'BAD'} pari={pari} -> {'PASS' if ok else 'FAIL'}")
    print("SELFTEST", "PASS" if fails == 0 else f"FAIL ({fails})")
    return fails


if __name__ == "__main__":
    sys.exit(1 if main() else 0)
