"""Exhaustive finite control for the known-point modulo-eight identity.

Run: sage research/p192_ray_character_20261123/mod8_control.sage
This checks the algebraic identity on a 97-point curve, not P-192 cost.
"""

p = 79
r = 97
K = GF(p)
E = EllipticCurve(K, [1, 6])
G = E(K(19), K(66))
assert G.order() == r
m = (r - 1) // 2
assert m % 8 == 0 and p % 4 == 3

xs = [(j * G)[0] for j in range(1, m + 1)]
assert len(set(xs)) == m

for a, c in enumerate(xs, 1):
    # Direct full-group definition of S_a, including both zero terms.
    S = -sum(
        kronecker(j, r) * kronecker(ZZ((j * G)[0] - c), p)
        for j in range(1, r)
    )
    paired = -2 * sum(
        kronecker(j, r) * kronecker(ZZ(u - c), p)
        for j, u in enumerate(xs, 1)
    )
    assert S == paired and S % 4 == 2

    derivative = K(1)
    for u in xs:
        if u != c:
            derivative *= c - u
    recovered = kronecker(ZZ(derivative), p) * (-1)^((S + 2) // 4)
    assert recovered == kronecker(a, r)

print(f"modulo-eight identity: {m}/{m} distinct marked x-classes pass")
