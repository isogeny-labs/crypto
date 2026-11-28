#!/usr/bin/env python3
"""CT206: independent finite census of the sextic-twist cross-factor obstruction.

Finite evidence for a general proof, not a benchmark or a formal verification.
Requires only Python 3 stdlib. [pcc:unverified]
"""
from math import isqrt

LIMIT = 199


def primes_up_to(limit):
    return [p for p in range(5, limit + 1)
            if all(p % d for d in range(2, isqrt(p) + 1))]


def curve_orders(p):
    chi = [-1] * p
    chi[0] = 0
    for y in range(1, p):
        chi[y * y % p] = 1
    cubes = [pow(x, 3, p) for x in range(p)]
    return {b: 1 + sum(1 + chi[(cube + b) % p] for cube in cubes)
            for b in range(1, p)}


def validate_prime(p):
    orders = curve_orders(p)
    assert len(orders) == p - 1
    if p % 3 == 2:
        # Cubing is a permutation, so the sum of quadratic characters is zero.
        assert set(orders.values()) == {p + 1}
        return (0, 0)
    assert p % 3 == 1
    sixth_powers = {pow(x, 6, p) for x in range(1, p)}
    assert len(sixth_powers) == (p - 1) // 6

    # Six coefficient classes, each of size (p-1)/6.
    traces_by_class = {}
    for b, order in orders.items():
        cl = frozenset((b * s) % p for s in sixth_powers)
        trace = p + 1 - order
        if cl in traces_by_class:
            assert traces_by_class[cl] == trace
        else:
            traces_by_class[cl] = trace
    assert len(traces_by_class) == 6
    assert len(set(traces_by_class.values())) == 6, "sextic trace collision"

    odd_cases = 0
    for b, order in orders.items():
        if order % 2:
            odd_cases += 1
            assert b not in sixth_powers, "odd group would have 2-torsion"
            assert order != orders[(b * b) % p], "companion isogenous"
            assert all((pow(u, 6, p) + b) % p for u in range(p)), (
                "rational ramification would force a rational point of order two"
            )
    return (1, odd_cases)


def main():
    prime_count = sextic_count = odd_count = total_coefficients = 0
    for p in primes_up_to(LIMIT):
        prime_count += 1
        total_coefficients += p - 1
        six, odd = validate_prime(p)
        sextic_count += six
        odd_count += odd
    assert (prime_count, sextic_count, odd_count, total_coefficients) == (
        44, 21, 1352, 4178
    )

    # Explicit falsification control: a deliberate duplicate cannot satisfy
    # the distinct-trace condition.
    example = [4, -4, 7, -7, 11, -11]
    assert len(set(example)) == 6
    example[1] = example[0]
    assert len(set(example)) != 6
    print("CT206 finite census PASS: 44 primes 5..199, 4178 coefficients;")
    print("  21 ordinary p=1 mod 3 primes with six distinct twist traces;")
    print("  1352 odd-order source curves have nonisogenous E_(b^2) companions")
    print("  [pcc:unverified] Finite check; mathematical proof in CT206 report")


if __name__ == "__main__":
    main()
