"""Compare two resultant-based three-summand joins with signed point arithmetic."""

import argparse
import json
from collections import defaultdict
from random import Random
from time import perf_counter

from compiled_projection import extract_small_fiber
from profile import factors, geometry, span
from quotient import half_trace, linear_map
from selector_elimination import normalized_rhs
from subspace_resultant import (
    batch_gcd_roots, evaluate, gcd_roots, image_coefficients, target_residual,
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
    terms = target_residual(f, subspace, target, constant)
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


def resultant_join(curve, basis, subspace, sums, images, by_x, jobs, method, compiled=None):
    grouped = defaultdict(list)
    for intermediate, third in jobs:
        if intermediate is not None:
            grouped[intermediate[0]].append((intermediate, third))
    sum_solutions = selector_visits = lift_failures = empty_intermediates = 0
    sparse_terms = dense_slots = internal_slots = fallbacks = false_positives = 0
    relations = set()
    assert all(curve.trace(t) and len(matches) <= 2 for t, matches in grouped.items())
    right_sides = {t: normalized_rhs(curve.field, basis, t) for t in grouped}
    if method == "batch":
        residuals = [target_residual(curve.field, subspace, t, c)
                     for t, (c, _) in right_sides.items()]
        isolated, internal_slots = batch_gcd_roots(curve.field, subspace, residuals)
        for roots in isolated:
            roots.discard(0)
        root_sets = dict(zip(right_sides, isolated))
        sparse_terms = sum(map(len, residuals))
        dense_slots = sum(max(terms, default=0) + 1 for terms in residuals)
    else:
        root_sets = {}
    for target, matches in grouped.items():
        constant, linear = right_sides[target]
        if method == "scan":
            roots = scan(curve, sums, images, constant, linear)
            selector_visits += len(sums) - 1
        elif method == "gcd":
            roots, terms, slots = sparse_gcd(curve, subspace, target, constant)
            sparse_terms += terms
            dense_slots += slots
        elif method == "compiled":
            plan, ranker = compiled
            rows = plan.low_rows(plan.specialized(constant, linear), 2)
            selectors, _ = extract_small_fiber(ranker, rows, len(basis), 3)
            if selectors is None:
                fallbacks += 1
                roots, terms, slots = sparse_gcd(curve, subspace, target, constant)
                sparse_terms += terms
                dense_slots += slots
            else:
                roots = {sums[selector] for selector in selectors if selector}
        else:
            roots = root_sets[target]
        verified_roots = set()
        for s in roots:
            weighted = curve.field.mul(target, curve.half(curve.field.sq(s)))
            rhs = constant ^ s ^ weighted
            if curve.trace(weighted):
                rhs ^= target
            if method == "compiled" and evaluate(
                curve.field, image_coefficients(curve.field, subspace, s), rhs
            ):
                false_positives += 1
                continue
            verified_roots.add(s)
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
        root_sets[target] = verified_roots
        sum_solutions += len(verified_roots)
        empty_intermediates += not verified_roots
    return relations, root_sets, dict(
        distinct_intermediate_abscissae=len(grouped), empty_intermediate_abscissae=empty_intermediates,
        selector_visits=selector_visits, recovered_sums=sum_solutions, lift_failures=lift_failures,
        sparse_terms=sparse_terms, dense_coefficient_slots=dense_slots,
        internal_coefficient_slots=internal_slots,
        compiled_fallbacks=fallbacks, compiled_false_positives=false_positives,
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


def run(degree, methods):
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
            expected_roots = None
            for method in methods:
                start = perf_counter()
                found, root_sets, counts = resultant_join(curve, basis, subspace, sums, images,
                                                         by_x, jobs, method)
                result[method + "_seconds"] = perf_counter() - start
                assert reference == found, (degree, g, target, reference ^ found)
                if expected_roots is None:
                    expected_roots = root_sets
                else:
                    assert root_sets == expected_roots
                result[method] = counts
            print(json.dumps(result), flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("fields", nargs="+", type=int, choices=(7, 23))
    parser.add_argument("--dense-gcd", action="store_true", help="also run dense FLINT GCD")
    parser.add_argument("--batch-gcd", action="store_true", help="also run the FLINT product tree")
    args = parser.parse_args()
    methods = ["scan"]
    if args.dense_gcd:
        methods.append("gcd")
    if args.batch_gcd:
        methods.append("batch")
    for n in args.fields:
        run(n, methods)
