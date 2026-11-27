"""Falsify exact ray-sign products from nontrivial odd-order index cosets.

Run: sage research/p192_ray_character_20261123/odd_coset_falsifier.sage
Only public known multiples are used as counterexample markings.
"""

from functools import lru_cache


p = 6277101735386680763835789423207666416083908700390324961279
r = 6277101735386680763835789423176059013767194773182842284081
n = (r - 1) // 2
odd_prime_factors = (
    5,
    2389,
    9564682313913860059195669,
    3433859179316188682119986911,
)
assert n == 8 * prod(odd_prime_factors)
assert is_prime(r) and all(is_prime(q) for q in odd_prime_factors)

E = EllipticCurve(GF(p), [-3, 0x64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1])
G = E(
    0x188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012,
    0x07192B95FFC8DA78631011ED6B24CDD573F977A11E794811,
)
assert r * G == E(0)


def division_values(P):
    x, two_y = P[0], 2 * P[1]
    field = P.curve().base_field()
    seed = {
        0: field(0),
        1: field(1),
        2: two_y,
        3: E.division_polynomial(3)(x),
        4: E.division_polynomial(4)(x) / two_y,
    }

    @lru_cache(None)
    def psi(j):
        if j in seed:
            return seed[j]
        k = j // 2
        if j & 1:
            return psi(k + 2) * psi(k) ** 3 - psi(k - 1) * psi(k + 1) ** 3
        return psi(k) * (
            psi(k + 2) * psi(k - 1) ** 2
            - psi(k - 2) * psi(k + 1) ** 2
        ) / two_y

    return psi


representatives = []
for q in odd_prime_factors:
    h = pow(3, n // q, r)
    if h % 2 == 0:
        h = r - h
    assert h % 2 == 1 and h not in (1, r - 1)
    assert pow(h, q, r) in (1, r - 1)
    assert kronecker(h, r) == 1
    representatives.append(h)

markings = (1, 2, 3, 5)
rows = []
for d in markings:
    psi = division_values(d * G)
    if d == 1:
        for j in (5, 7, 11):
            assert psi(j) == E.division_polynomial(j)(G[0])
    rows.append(tuple(int(kronecker(ZZ(psi(h)), p)) for h in representatives))

for col, q in enumerate(odd_prime_factors):
    signs = tuple(row[col] for row in rows)
    assert set(signs) == {-1, 1}, (q, signs)
    print(f"q={q}: signs at d={markings}: {signs}")
