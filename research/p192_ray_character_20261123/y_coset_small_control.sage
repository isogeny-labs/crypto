"""Finite control for the proper-subgroup y-product and paired index grids.

Run with ``sage -python research/p192_ray_character_20261123/y_coset_small_control.sage``.
This checks every marked class on two curves of order 241 and exhausts
the 2-by-2 paired grids in their common 24-class scalar subgroup.
"""

from itertools import combinations

from sage.all import EllipticCurve, GF, ZZ, kronecker


r, m = 241, 24
half = (r - 1) // 2
generator = int(GF(r).multiplicative_generator())
subgroup_generator = pow(generator, half // m, r)


def class_rep(value):
    residue = value % r
    return min(residue, (-residue) % r)


classes = {class_rep(pow(subgroup_generator, j, r)) for j in range(m)}
assert len(classes) == m


def y_sign(point, p):
    product = GF(p)(1)
    for scalar in classes:
        product *= (scalar * point)[1]
    assert product != 0
    return int(kronecker(ZZ(product), p))


controls = (
    (223, 8, 7, 0, 26, set()),
    (271, 7, 3, 1, 108, {2, 3, 4}),
)
for p, a, b, gx, gy, expected_bad_cosets in controls:
    curve = EllipticCurve(GF(p), [a, b])
    assert curve.order() == r
    base = curve(gx, gy)
    assert base.order() == r
    base_sign = y_sign(base, p)
    bad = {
        scalar
        for scalar in range(1, half + 1)
        if y_sign(scalar * base, p) * base_sign
        != int(kronecker(scalar, r))
    }
    bad_cosets = {
        exponent
        for exponent in range(half // m)
        if min(pow(generator, exponent, r), r - pow(generator, exponent, r))
        in bad
    }
    assert bad_cosets == expected_bad_cosets, (p, bad_cosets)
    assert len(bad) == m * len(bad_cosets), (p, len(bad))
    print(
        f"p={p} r={r} m={m} checked_classes={half} "
        f"mismatches={len(bad)} bad_cosets={sorted(bad_cosets)}"
    )


# A paired grid has distinct classes i+j and i-j for every (i,j),
# and its full index list is disjoint. Replacing i or j by its negative
# preserves the two output classes, so 1..half exhausts both choices.
neighbors = {}
for center in range(1, half + 1):
    neighbors[center] = [
        offset
        for offset in range(1, half + 1)
        if class_rep(center + offset) in classes
        and class_rep(center - offset) in classes
        and class_rep(center + offset) != class_rep(center - offset)
    ]


best_single = 0
for center, offsets in neighbors.items():
    for size in range(len(offsets), best_single, -1):
        if any(
            len(
                {
                    class_rep(center + offset)
                    for offset in choice
                }
                | {
                    class_rep(center - offset)
                    for offset in choice
                }
            )
            == 2 * size
            for choice in combinations(offsets, size)
        ):
            best_single = size
            break


for first, second in combinations(range(1, half + 1), 2):
    common = set(neighbors[first]) & set(neighbors[second])
    for left, right in combinations(common, 2):
        output = {
            class_rep(center + sign * offset)
            for center in (first, second)
            for offset in (left, right)
            for sign in (-1, 1)
        }
        assert len(output) != 8, (first, second, left, right)


assert best_single == 4
print(
    f"r={r} m={m} max_single_center_offsets={best_single} "
    "disjoint_2x2_grids=0"
)
