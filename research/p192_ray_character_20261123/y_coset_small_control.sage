"""Finite control for the proper-subgroup y-product and paired index grids.

Run with ``sage -python research/p192_ray_character_20261123/y_coset_small_control.sage``.
This checks every marked class on one 241-point curve and exhausts the
2-by-2 paired grids in its 24-class scalar subgroup.
"""

from itertools import combinations

from sage.all import EllipticCurve, GF, ZZ, kronecker


p, r, m = 223, 241, 24
curve = EllipticCurve(GF(p), [8, 7])
assert curve.order() == r
base = curve(0, 26)
assert base.order() == r
half = (r - 1) // 2
generator = int(GF(r).multiplicative_generator())
subgroup_generator = pow(generator, half // m, r)


def class_rep(value):
    residue = value % r
    return min(residue, (-residue) % r)


classes = {class_rep(pow(subgroup_generator, j, r)) for j in range(m)}
assert len(classes) == m


def y_sign(point):
    product = GF(p)(1)
    for scalar in classes:
        product *= (scalar * point)[1]
    assert product != 0
    return int(kronecker(ZZ(product), p))


base_sign = y_sign(base)
for scalar in range(1, half + 1):
    observed = y_sign(scalar * base) * base_sign
    expected = int(kronecker(scalar, r))
    assert observed == expected, (scalar, observed, expected)


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
    f"p={p} r={r} m={m} checked_classes={half} "
    f"exact_y_sign=true max_single_center_offsets={best_single} "
    "disjoint_2x2_grids=0"
)
