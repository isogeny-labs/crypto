"""Exact signed-root orientation and direct-product cost gate.

Run: sage -python research/p192_ray_character_20261123/vandermonde_sign_gate.sage
The finite curves check the identity, not a P-192 evaluation algorithm.
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


def survivor_indices(n):
    """Parity of psi indices in the pair-difference expansion."""
    counts = [0] * (2 * n + 1)
    for i in range(1, n + 1):
        for j in range(i + 1, n + 1):
            counts[i + j] ^= 1
            counts[j - i] ^= 1
    return [k for k in range(1, 2 * n + 1) if counts[k]], counts


for p, a, b, gx, gy in ((79, 0, 3, 1, 2), (79, 1, 6, 1, 18)):
    curve = EllipticCurve(GF(p), [a, b])
    generator = curve(gx, gy)
    r, n = 97, 48
    assert generator.order() == r
    support, counts = survivor_indices(n)
    assert len(support) == n
    assert all(counts[k] + counts[r - k] == 1 for k in range(1, n + 1))

    base = None
    for d in range(1, n + 1):
        point = d * generator
        xs = [(j * point)[0] for j in range(1, n + 1)]
        vandermonde = GF(p)(1)
        for i in range(n):
            for j in range(i + 1, n):
                vandermonde *= xs[i] - xs[j]
        if base is None:
            base = vandermonde

        psi = division_values(curve, point)
        reduced_product = GF(p)(1)
        for k in support:
            reduced_product *= psi(k)
        assert int(kronecker(ZZ(vandermonde / reduced_product), p)) == 1
        assert vandermonde / base in (GF(p)(1), GF(p)(-1))
        assert int(kronecker(ZZ(vandermonde / base), p)) == int(kronecker(d, r))
    print(f"finite_identity p={p} a={a} b={b} checked={n} support={len(support)}")


p192_r = 6277101735386680763835789423176059013767194773182842284081
p192_n = (p192_r - 1) // 2
assert p192_r % 4 == 1 and p192_n % 2 == 0
print(f"p192_direct_product_factors={p192_n} bits={p192_n.bit_length()}")
