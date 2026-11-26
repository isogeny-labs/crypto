"""Certify the direct Boolean S3 solving degree on proper-subfield cosets."""

import argparse
import json
from pathlib import Path
from random import Random

from product_linearization import EvenCurve, SubfieldCoset, s3
from profile import xor_selected
from quotient import measures, satisfies
from selector_elimination import witness_coefficients


def direct_rows(coset, target):
    f = coset.field
    k = coset.k
    a2 = coset.shift_square
    coefficient_s2 = a2 ^ f.sq(target)
    coefficient_s = f.mul(target, coset.shift)

    def product_part(v):
        return f.sq(v) ^ f.mul(target, v)

    coefficients = {0: product_part(a2) ^ 1}
    for i, u in enumerate(coset.basis):
        coefficients[1 << i] = product_part(f.sq(u))
        coefficients[1 << (i + k)] = f.mul(coefficient_s2, f.sq(u)) ^ f.mul(coefficient_s, u)
        for j, s in enumerate(coset.basis):
            coefficients[(1 << i) | (1 << (j + k))] = product_part(f.mul(u, s))
    rows = []
    for bit in range(f.n):
        row = [mask for mask, value in coefficients.items() if value >> bit & 1]
        if row:
            rows.append(sorted(row))
    return rows


def oriented_points(coset, target):
    k = coset.k
    points = set()
    for left, right in coset.pairs(target)[0]:
        a = witness_coefficients(coset.basis, left ^ coset.shift)
        b = witness_coefficients(coset.basis, right ^ coset.shift)
        s = (a ^ b) << k
        points.update((s | a, s | b))
    return points


def panel(coset, count, rng):
    curve = EvenCurve(coset.field)
    selected = set()
    trial = 0
    while len(selected) < count:
        trial += 1
        if trial % 2 == 0:
            u, v = (xor_selected(coset.basis, rng.getrandbits(coset.k)) ^ coset.shift for _ in range(2))
            left, right = curve.lift(u), curve.lift(v)
            if not left or not right or (w := curve.add(left[0], right[0])) is None:
                continue
            target = w[0]
        else:
            target = rng.randrange(1 << coset.field.n)
            if not curve.lift(target):
                continue
        if target not in selected:
            selected.add(target)
            yield target


def run(ranker, count):
    for k, extension in ((3, 2), (3, 3), (5, 2), (4, 3), (5, 3), (6, 3), (11, 2)):
        coset = SubfieldCoset(k, extension)
        rng = Random(1109 * k + extension)
        for target in panel(coset, count, rng):
            rows = direct_rows(coset, target)
            points = oriented_points(coset, target)
            assert len(points) <= 8
            for _ in range(16):
                x, s = [rng.getrandbits(k) for _ in range(2)]
                a = coset.shift ^ xor_selected(coset.basis, x)
                b = a ^ xor_selected(coset.basis, s)
                assert satisfies(rows, x | (s << k)) == (s3(coset.field, a, b, target) == 0)
            result = measures(ranker, rows, points, k, k, max_degree=5)
            assert result["reduced_basis_degree"] <= 3
            print(json.dumps(dict(n=coset.field.n, k=k, extension=extension, target=target,
                                  solutions=len(points), **result)), flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ranker", type=Path, required=True)
    parser.add_argument("--samples", type=int, choices=range(1, 17), default=8)
    args = parser.parse_args()
    run(args.ranker.resolve(), args.samples)
