"""Check the EDS coset-product parity law on a finite control curve.

Run: sage research/p192_ray_character_20261123/even_coset_parity_control.sage
"""

from functools import lru_cache


p, r = 79, 97
n = (r - 1) // 2
E = EllipticCurve(GF(p), [1, 6])
G = E(19, 66)
assert E.order() == r and G.order() == r


def division_values(P):
    x, two_y = P[0], 2 * P[1]
    field = E.base_field()
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


def odd_rep(j):
    return j if j % 2 else r - j


def coset_character(P, reps):
    psi = division_values(P)
    value = E.base_field()(1)
    for j in reps:
        value *= psi(j)
    return int(kronecker(ZZ(value), p))


for m in (4, 16):
    t = n // m
    h = pow(5, t, r)
    reps = sorted({odd_rep(pow(h, j, r)) for j in range(m)})
    assert len(reps) == m and all(j % 2 for j in reps)
    assert pow(h, m, r) == r - 1
    observed = coset_character(h * G, reps) * coset_character(G, reps)
    expected = int(kronecker(h, r))
    assert observed == -1 and expected == (-1) ** t
    print(f"m={m}, index={t}: coset ratio={observed}, ray ratio={expected}")
