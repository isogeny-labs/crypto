#!/usr/bin/env python3
"""Deterministic prime-order curves over F_p for the prime medium-tier rho
calibration. For each bit size, the first (p, a, b) drawn from a seeded RNG
with #E(F_p) prime (cofactor 1, so r = #E) and j not in {0, 1728} (no extra
automorphisms: A = 2, floor sqrt(pi/4)). Orders by PARI SEA (ellcard);
each generator G is checked with independent pure-Python arithmetic:
G on the curve and [r]G = O. Writes curves.json."""
import json, math, random, sys
import cypari2

SEED = 20261115
BITS = (32, 36, 40, 44, 48)


def is_prime(n):
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def add(P, Q, a, p):
    if P is None:
        return Q
    if Q is None:
        return P
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2:
        if (y1 + y2) % p == 0:
            return None
        lam = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (lam * lam - x1 - x2) % p
    return (x3, (lam * (x1 - x3) - y1) % p)


def mul(k, P, a, p):
    R = None
    while k:
        if k & 1:
            R = add(R, P, a, p)
        P = add(P, P, a, p)
        k >>= 1
    return R


def main():
    pari = cypari2.Pari()
    rng = random.Random(SEED)
    out = []
    for bits in BITS:
        tries = 0
        while True:
            tries += 1
            p = rng.randrange(1 << (bits - 1), 1 << bits) | 1
            if not is_prime(p):
                continue
            a, b = rng.randrange(1, p), rng.randrange(1, p)
            if (4 * a ** 3 + 27 * b * b) % p == 0:
                continue
            N = int(pari(f"ellcard(ellinit([{a},{b}],{p}))"))
            if not is_prime(N) or N == p:
                continue
            # a generator: any affine point (prime order, cofactor 1)
            while True:
                x = rng.randrange(p)
                rhs = (x ** 3 + a * x + b) % p
                if pow(rhs, (p - 1) // 2, p) == 1:
                    y = int(pari(f"lift(sqrt(Mod({rhs},{p})))"))
                    break
            G = (x, y)
            assert (y * y - (x ** 3 + a * x + b)) % p == 0, "G not on curve"
            assert mul(N, G, a, p) is None, "[r]G != O"
            assert a % p != 0 and b % p != 0  # excludes j = 0 and j = 1728
            out.append({"kind": "prime_explicit", "p": p, "a": a, "b": b,
                        "group_order": N, "r": N, "gx": x, "gy": y,
                        "log2_r": round(math.log2(N), 3),
                        "search_tries": tries})
            print(f"{bits} bits: p={p} r={N} (tries {tries})", file=sys.stderr)
            break
    json.dump({"seed": SEED, "bits": list(BITS), "curves": out}, open("curves.json", "w"), indent=1)


if __name__ == "__main__":
    main()
