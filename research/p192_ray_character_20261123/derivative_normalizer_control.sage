"""Check the kernel-derivative / EDS-normalizer identity.

Run: sage research/p192_ray_character_20261123/derivative_normalizer_control.sage
The P-192 markings below are known multiples used to check the identity.
"""

from functools import lru_cache


def division_values(E, P):
    x, two_y = P[0], 2 * P[1]
    third = E.division_polynomial(3)(x)
    fourth = E.division_polynomial(4)(x) / two_y
    K = x.parent()

    @lru_cache(None)
    def psi(j):
        if j == 0:
            return K(0)
        if j == 1:
            return K(1)
        if j == 2:
            return two_y
        if j == 3:
            return third
        if j == 4:
            return fourth
        k = j // 2
        if j % 2:
            return psi(k + 2) * psi(k) ** 3 - psi(k - 1) * psi(k + 1) ** 3
        return psi(k) * (
            psi(k + 2) * psi(k - 1) ** 2
            - psi(k - 2) * psi(k + 1) ** 2
        ) / two_y

    return psi


def check(E, G, p, r, scalars, direct_roots=False):
    assert r * G == E(0) and gcd(r, p - 1) == 1
    m = (r - 1) // 2
    roots = [(j * G)[0] for j in range(1, m + 1)] if direct_roots else None
    if roots is not None:
        assert len(set(roots)) == m
    root_exponent = inverse_mod(r * r, p - 1)
    for d in scalars:
        P = d * G
        psi = division_values(E, P)
        phi = (psi(p - 1) / psi(p - 1 + r)) ** root_exponent
        derivative = psi(m + 1) / (psi(2) * psi(m))
        assert derivative * psi(2) * phi ** r == -1
        assert kronecker(ZZ(derivative), p) == -kronecker(ZZ(phi / psi(2)), p)
        assert kronecker(ZZ(derivative), p) == -kronecker(ZZ(psi(r + 2)), p)
        if roots is not None:
            direct = E.base_field()(1)
            for u in roots:
                if u != P[0]:
                    direct *= P[0] - u
            assert derivative == direct
    print(f"order {r}: {len(scalars)} markings passed")


check(EllipticCurve(GF(79), [1, 6]), EllipticCurve(GF(79), [1, 6])(19, 66),
      79, 97, range(1, 49), direct_roots=True)

p = 6277101735386680763835789423207666416083908700390324961279
r = 6277101735386680763835789423176059013767194773182842284081
b = 0x64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1
E = EllipticCurve(GF(p), [-3, b])
G = E(0x188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012,
      0x07192B95FFC8DA78631011ED6B24CDD573F977A11E794811)
check(E, G, p, r, (1, 2, 3, 5, 7, 11, 17, 31))
