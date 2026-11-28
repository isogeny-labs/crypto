"""Compact rational-torsion x-root derivative and P-192 sign screen.

Run: sage -python research/p192_ray_character_20261123/rational_root_derivative_screen.sage

For odd prime order r and n=(r-1)/2, let M(T) be the monic product
of T-x([j]G), 1 <= j <= n. At any nonzero P, the roots are the same.
The division-polynomial difference identity gives
  x(P)-x([j]P) = psi_(j+1)(P) psi_(j-1)(P) / psi_j(P)^2.
Multiplying for 2 <= j <= n telescopes to
  M'(x(P)) = psi_(n+1)(P) / (psi_2(P) psi_n(P)).
"""

from functools import lru_cache

from sage.all import EllipticCurve, GF, ZZ, kronecker


def division_values(curve, point):
    x, two_y = point[0], 2 * point[1]
    seed = {0: x.parent()(0), 1: x.parent()(1), 2: two_y,
            3: curve.division_polynomial(3)(x),
            4: curve.division_polynomial(4)(x) / two_y}

    @lru_cache(None)
    def psi(j):
        if j in seed:
            return seed[j]
        q = j // 2
        if j & 1:
            return psi(q + 2) * psi(q)**3 - psi(q - 1) * psi(q + 1)**3
        return psi(q) * (psi(q + 2) * psi(q - 1)**2
                         - psi(q - 2) * psi(q + 1)**2) / two_y

    return psi


def derivative_value(curve, point, order):
    n = (order - 1) // 2
    psi = division_values(curve, point)
    return psi(n + 1) / (psi(2) * psi(n))


for p, a, b, gx, gy in ((79, 0, 3, 1, 2), (79, 1, 6, 1, 18)):
    curve = EllipticCurve(GF(p), [a, b])
    generator = curve(gx, gy)
    r = 97
    assert generator.order() == r
    n = (r - 1) // 2
    roots = [(j * generator)[0] for j in range(1, n + 1)]
    assert len(set(roots)) == n
    for d in range(1, n + 1):
        point = d * generator
        direct = GF(p)(1)
        for root in roots:
            if root != point[0]:
                direct *= point[0] - root
        assert direct == derivative_value(curve, point, r)
    print(f"finite_identity p={p} a={a} b={b} checked={n}")

p = 6277101735386680763835789423207666416083908700390324961279
r = 6277101735386680763835789423176059013767194773182842284081
curve = EllipticCurve(GF(p), [-3, 0x64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1])
generator = curve(
    0x188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012,
    0x07192B95FFC8DA78631011ED6B24CDD573F977A11E794811,
)
assert r * generator == curve(0)
assert derivative_value(curve, -generator, r) == derivative_value(curve, generator, r)
base_sign = int(kronecker(ZZ(derivative_value(curve, generator, r)), p))
for d in (1, 2, 3, 5, 7, 11, 13, 4, 10):
    value = derivative_value(curve, d * generator, r)
    observed = base_sign * int(kronecker(ZZ(value), p))
    expected = int(kronecker(d, r))
    print(f"p192 d={d} value={ZZ(value)} feature={observed} scalar_sign={expected}")
assert base_sign == 1
assert int(kronecker(ZZ(derivative_value(curve, 2 * generator, r)), p)) == -1
