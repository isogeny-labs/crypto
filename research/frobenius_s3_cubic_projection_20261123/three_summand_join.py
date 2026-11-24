"""Compare two resultant-based three-summand joins with signed point arithmetic."""

import argparse
import json
from collections import defaultdict
from random import Random
from time import perf_counter

from profile import factors, geometry, span
from quotient import half_trace, linear_map
from selector_elimination import normalized_rhs
from subspace_resultant import (
    evaluate, gcd_roots, image_coefficients, linearized_rhs, patch_zero, sparse_residual,
)


class Curve:
    """The toy binary curve y² + xy = x³ + x² + 1."""

    def __init__(self, field):
        self.field = field
        self.trace = linear_map(field, field.tr)
        self.half = linear_map(field, lambda value: half_trace(field, value))

    def lift(self, x):
        if x == 0:
            return ((0, 1),)
        f = self.field
        value = x ^ 1 ^ f.sq(f.inv(x))
        if self.trace(value):
            return ()
        point = (x, f.mul(x, self.half(value)))
        return point, negate(point)

    def add(self, p, q, inverse=None):
        if p is None:
            return q
        if q is None:
            return p
        f = self.field
        x, y = p
        a, b = q
        if x == a:
            if y != b or x == 0:
                return None
            slope = x ^ f.mul(y, f.inv(x))
            out_x = f.sq(slope) ^ slope ^ 1
            return out_x, f.sq(x) ^ f.mul(slope ^ 1, out_x)
        slope = f.mul(y ^ b, f.inv(x ^ a) if inverse is None else inverse)
        out_x = f.sq(slope) ^ slope ^ x ^ a ^ 1
        return out_x, f.mul(slope, x ^ out_x) ^ out_x ^ y

    def on_curve(self, point):
        if point is None:
            return True
        f = self.field
        x, y = point
        squared = f.sq(x)
        return f.sq(y) ^ f.mul(x, y) == f.mul(squared, x) ^ squared ^ 1

    def is_double(self, point):
        return point is None or bool(self.trace(point[0]))


def negate(point):
    if point is None:
        return None
    x, y = point
    return x, y ^ x


def inverses(field, values):
    prefixes = [1]
    for value in values:
        prefixes.append(field.mul(prefixes[-1], value or 1))
    inverse = field.inv(prefixes.pop())
    output = [None] * len(values)
    for index in range(len(values) - 1, -1, -1):
        if values[index]:
            output[index] = field.mul(inverse, prefixes[index])
            inverse = field.mul(inverse, values[index])
    return output


def triple(p, q, third):
    if p[0] != q[0] and third[0] not in (p[0], q[0]):
        return tuple(sorted((p, q, third)))
    return None


def direct_join(curve, points, membership, jobs):
    relations = set()
    checked = 0
    for intermediate, third in jobs:
        if intermediate is None:
            continue
        denominators = [intermediate[0] ^ point[0] for point in points]
        reciprocals = inverses(curve.field, denominators)
        for point, inverse in zip(points, reciprocals):
            complement = curve.add(intermediate, negate(point), inverse)
            checked += 1
            if complement in membership:
                relation = triple(point, complement, third)
                if relation:
                    relations.add(relation)
    return relations, checked


def scan(curve, sums, images, constant, linear):
    values = [0]
    for coefficient in linear:
        values += [value ^ coefficient for value in values]
    return {s for s, image, rhs in zip(sums, images, values)
            if s and not evaluate(curve.field, image, constant ^ rhs)}


def sparse_gcd(curve, subspace, target, constant):
    f = curve.field
    linear = linearized_rhs(f, subspace, target)
    terms = sparse_residual(f, subspace, constant, linear)
    patch_zero(terms, subspace, evaluate(f, subspace, constant))
    roots, costs = gcd_roots(f, subspace, terms)
    roots.discard(0)
    return roots, len(terms), costs["dense_coefficients"]


def recover(curve, by_x, s, rhs):
    f = curve.field
    w = f.mul(rhs, f.inv(f.sq(s)))
    assert not curve.trace(w)
    left = f.mul(s, curve.half(w)) ^ 1
    right = left ^ s
    return by_x.get(left, ()), by_x.get(right, ())


def resultant_join(curve, basis, subspace, sums, images, by_x, jobs, method):
    grouped = defaultdict(list)
    for intermediate, third in jobs:
        if intermediate is not None:
            grouped[intermediate[0]].append((intermediate, third))
    sum_solutions = selector_visits = lift_failures = empty_intermediates = 0
    sparse_terms = dense_slots = 0
    relations = set()
    for target, matches in grouped.items():
        assert curve.trace(target)
        assert len(matches) <= 2
        constant, linear = normalized_rhs(curve.field, basis, target)
        if method == "scan":
            roots = scan(curve, sums, images, constant, linear)
            selector_visits += len(sums) - 1
        else:
            roots, terms, slots = sparse_gcd(curve, subspace, target, constant)
            sparse_terms += terms
            dense_slots += slots
        sum_solutions += len(roots)
        empty_intermediates += not roots
        for s in roots:
            weighted = curve.field.mul(target, curve.half(curve.field.sq(s)))
            rhs = constant ^ s ^ weighted
            if curve.trace(weighted):
                rhs ^= target
            left, right = recover(curve, by_x, s, rhs)
            if not left or not right:
                lift_failures += 1
            for p in left:
                for q in right:
                    total = curve.add(p, q)
                    for intermediate, third in matches:
                        if total == intermediate:
                            relation = triple(p, q, third)
                            if relation:
                                relations.add(relation)
    return relations, dict(
        distinct_intermediate_abscissae=len(grouped), empty_intermediate_abscissae=empty_intermediates,
        selector_visits=selector_visits, recovered_sums=sum_solutions, lift_failures=lift_failures,
        sparse_terms=sparse_terms, dense_coefficient_slots=dense_slots,
    )


def panel(curve, points, degree, g):
    if degree == 7:
        return [p for x in range(1 << degree) for p in curve.lift(x)], points
    rng = Random(g)
    thirds = rng.sample(points, 8)
    planted = curve.add(thirds[0], curve.add(thirds[1], thirds[2]))
    assert len({p[0] for p in thirds[:3]}) == 3 and planted is not None
    targets = [curve.lift(x)[0] for x in (6277527, 5846863, 7373655, 5025355)]
    return [planted, *targets], thirds


def run(degree, dense_gcd):
    for g in factors(degree):
        start_setup = perf_counter()
        field, basis, _ = geometry(degree, g)
        curve = Curve(field)
        sums = span(basis)
        subspace = [(g >> j) & 1 for j in range(len(basis) + 1)]
        by_x = {x: signs for x in (s ^ 1 for s in sums) if (signs := curve.lift(x))}
        points = [p for signs in by_x.values() for p in signs]
        membership = set(points)
        assert all(curve.on_curve(p) and curve.is_double(p) for p in points)
        built = perf_counter()
        images = [subspace] + [image_coefficients(field, subspace, s) for s in sums[1:]]
        cache_seconds = perf_counter() - built
        targets, thirds = panel(curve, points, degree, g)
        if degree == 7:
            assert {curve.add(p, p) for p in targets} == {
                None, *(p for p in targets if curve.is_double(p))
            }
        for target in targets:
            start = perf_counter()
            eligible = curve.is_double(target)
            jobs = [(curve.add(target, negate(p)), p) for p in thirds] if eligible else []
            grouped = perf_counter()
            assert all(curve.on_curve(w) and curve.is_double(w) for w, _ in jobs)
            if not eligible and degree == 7:
                # Independently verify every target rejected by the subgroup gate.
                control = [(curve.add(target, negate(p)), p) for p in thirds]
                assert not direct_join(curve, points, membership, control)[0]
            start_direct = perf_counter()
            reference, checks = direct_join(curve, points, membership, jobs)
            direct = perf_counter() - start_direct
            result = dict(
                n=degree, g=g, target=target, factor_base_abscissae=len(by_x),
                subgroup_rejected=not eligible,
                signed_thirds=len(thirds), intermediate_infinity=sum(w is None for w, _ in jobs),
                verified_distinct_signed_triples=len(reference), direct_checks=checks,
                common_setup_seconds=built - start_setup,
                intermediate_seconds=grouped - start, direct_seconds=direct,
                image_cache_seconds=cache_seconds,
            )
            for method in ("scan", "gcd") if dense_gcd else ("scan",):
                start = perf_counter()
                found, counts = resultant_join(curve, basis, subspace, sums, images, by_x, jobs, method)
                result[method + "_seconds"] = perf_counter() - start
                assert reference == found, (degree, g, target, reference ^ found)
                result[method] = counts
            print(json.dumps(result), flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("fields", nargs="+", type=int, choices=(7, 23))
    parser.add_argument("--dense-gcd", action="store_true", help="also run dense FLINT GCD")
    args = parser.parse_args()
    for n in args.fields:
        run(n, args.dense_gcd)
