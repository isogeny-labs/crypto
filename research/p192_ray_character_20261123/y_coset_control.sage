"""Screen small P-192 y-coordinate coset products.

Run: sage research/p192_ray_character_20261123/y_coset_control.sage
"""


p = 6277101735386680763835789423207666416083908700390324961279
r = 6277101735386680763835789423176059013767194773182842284081
n = (r - 1) // 2
E = EllipticCurve(
    GF(p), [-3, 0x64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1]
)
G = E(
    0x188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012,
    0x07192B95FFC8DA78631011ED6B24CDD573F977A11E794811,
)
assert r * G == E(0)


def y_character(P, reps):
    value = E.base_field()(1)
    for j in reps:
        value *= (j * P)[1]
    return int(kronecker(ZZ(value), p))


for m in (8, 40):
    t = n // m
    assert t % 2 == 1
    h = pow(3, t, r)
    assert pow(h, m, r) == r - 1
    reps = sorted({min(pow(h, j, r), r - pow(h, j, r)) for j in range(m)})
    assert len(reps) == m
    base = y_character(G, reps)
    assert y_character(h * G, reps) * base == -1
    for d in (1, 2, 3, 5):
        observed = y_character(d * G, reps) * base
        expected = int(kronecker(d, r))
        if d == 2:
            assert observed == -1 and expected == 1
        print(f"m={m}, d={d}: y-coset ratio={observed}, ray ratio={expected}")
