#!/usr/bin/env python3
"""CT228: independently census prime-field cubic-cover full signed factor bases.

Scope: a full-rank log solve with a variable for each rational point up to sign;
NOT a lower bound on all index-calculus or all transfer algorithms.
"""
import argparse
import json
from math import isqrt

A, B = 1, 46
PRIMES = (29, 41, 59, 101, 137, 149, 1019, 1427, 4871)


def prime(n):
    if type(n) is not int or n < 2:
        return False
    return all(n % d for d in range(2, isqrt(n) + 1))


def census(p):
    if not prime(p) or p % 3 != 2 or (4 * A**3 + 27 * B**2) % p == 0:
        raise ValueError('requires odd nonsingular prime p = 2 mod 3')
    # Check cubing permutation independently rather than assuming the theorem.
    cube = {pow(u, 3, p) for u in range(p)}
    if len(cube) != p:
        raise ValueError('cubic map unexpectedly not bijective')
    e_aff = c_aff = e_weier = c_weier = 0
    for x in range(p):
        rhs = (x*x*x + A*x + B) % p
        count = 1 if rhs == 0 else (2 if pow(rhs, (p-1)//2, p) == 1 else 0)
        e_aff += count
        e_weier += rhs == 0
    for u in range(p):
        rhs = (pow(u, 9, p) + pow(u, 3, p) + B) % p
        count = 1 if rhs == 0 else (2 if pow(rhs, (p-1)//2, p) == 1 else 0)
        c_aff += count
        c_weier += rhs == 0
    n = e_aff + 1
    w = e_weier
    full_base = (n - 1 + w) // 2
    if e_aff != c_aff or e_weier != c_weier:
        raise ValueError('rational cover census mismatch')
    if 2 * full_base != n - 1 + w:
        raise ValueError('signed factor-base count mismatch')
    if abs(n - (p + 1))**2 > 4*p:
        raise ValueError('Hasse failure')
    return dict(p=p, source_order=n, source_order_prime=prime(n),
                rational_weierstrass=w, signed_full_base=full_base,
                min_independent_full_rank_relations=full_base,
                cube_bijection=True)


def lower_bound(p):
    """Hasse-derived minimum signed rational factor-base cardinality."""
    if not prime(p) or p % 3 != 2:
        raise ValueError("Hasse bound is for an admissible prime")
    n_min = p + 1 - isqrt(4*p)
    return (n_min - 1 + 1) // 2


def self_test():
    rows = [census(p) for p in PRIMES]
    expected = [(29, 23, 11), (41, 47, 23), (59, 53, 26),
                (101, 109, 54), (137, 157, 78), (149, 163, 81),
                (1019, 1013, 506), (1427, 1433, 716), (4871, 4919, 2459)]
    if [(v['p'],v['source_order'],v['signed_full_base']) for v in rows] != expected:
        raise ValueError('frozen independent census mismatch')
    if not all(v['source_order_prime'] and not v['rational_weierstrass'] for v in rows):
        raise ValueError('prime-order/no-ramification precondition mismatch')
    for invalid in (4, 7, 13, 19, 31, 101.0, True):
        try:
            census(invalid)
        except ValueError:
            continue
        raise ValueError('bad input accepted: ' + str(invalid))
    if lower_bound(2147483579) != 1073695449:
        raise ValueError("31-bit Hasse-bound control mismatch")
    if any(v["signed_full_base"] < lower_bound(v["p"]) for v in rows):
        raise ValueError("Hasse-bound violation")
    if (2 * rows[3]['signed_full_base'], rows[3]['source_order']) != (108,109):
        raise ValueError('F101 controls wrong')
    return rows


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--self-test', action='store_true')
    args = parser.parse_args()
    rows = self_test()
    if args.self_test:
        print('CT228_SELFTEST_PASS', len(rows), rows[3]['signed_full_base'])
    else:
        print(json.dumps({'case': 'cubic_full_signed_base', 'scope': 'only-full-degree-one-basis',
                          'rows': rows}, sort_keys=True, separators=(',', ':')))


if __name__ == '__main__':
    main()
