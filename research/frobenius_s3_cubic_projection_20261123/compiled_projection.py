"""Specialize low-degree trace consequences without constructing quartic equations."""

import argparse
import json
from pathlib import Path
from random import Random
from time import perf_counter

from profile import FrobeniusRing, domains, factors, geometry, quotient_coordinates, xor_selected
from quotient import expected_points, half_trace, insert, linear_map, measures, satisfies
from selector_elimination import normalized, run_ranker, witness_coefficients
from unrestricted import direct_constraints, supported_indices, trace_dual


def physical_monomials(field, basis, terms):
    powers = [basis]
    for _ in range(1, len(basis)):
        powers.append([field.sq(value) for value in powers[-1]])
    translation = {0: {0: 1}}
    for mask in terms[1:]:
        bit = mask & -mask
        expanded = {}
        for support, word in translation[mask ^ bit].items():
            for i, coefficient in enumerate(powers[bit.bit_length() - 1]):
                product = support | (1 << i)
                expanded[product] = expanded.get(product, 0) ^ field.mul(word, coefficient)
        translation[mask] = expanded
    return translation


def extract_small_fiber(ranker, rows, width, degree):
    """Check all points in the affine slice derived by bounded closure."""
    closure = run_ranker(ranker, rows, 0, degree, 0, 45, True, right=width)
    if closure["timeout"]:
        return None, closure
    if closure["rejected"]:
        return set(), closure
    pivots = {}
    for row in closure["selector_rows"]:
        if all(m.bit_count() <= 1 for m in row):
            insert(sum((m << 1) or 1 for m in row), pivots)
    free = [1 << j for j in range(1, width + 1) if j not in pivots]
    if len(free) > 3:
        return None, closure
    assignments = [1]
    for bit in free:
        assignments += [point | bit for point in assignments]
    output = set()
    for point in assignments:
        for position, row in sorted(pivots.items()):
            if (point & row).bit_count() & 1:
                point |= 1 << position
        if satisfies(rows, point >> 1):
            output.add(point >> 1)
    return output, closure


def recover_orientations(field, basis, half, constant, linear, sums):
    k = len(basis)
    result = set()
    for selector in sums:
        s = xor_selected(basis, selector)
        assert s
        value = constant ^ xor_selected(linear, selector)
        w = field.mul(value, field.inv(field.sq(s)))
        x = field.mul(s, half(w))
        assert field.sq(x) ^ field.mul(x, s) == value
        left = witness_coefficients(basis, x)
        right = selector << k
        result.update((left | right, (left ^ selector) | right))
    return result


def targets(field, trace, g, extra):
    # The last fixed target has two distinct sums in the specified geometry.
    double_hit = {2787: 5187531, 3189: 7248487}
    known = [6277527, 5846863, 7373655, 5025355, double_hit[g]]
    rng = Random(g ^ 0x3C11)
    while len(known) < 5 + extra:
        candidate = rng.randrange(1, 1 << field.n)
        if candidate not in known and trace(candidate) and not trace(field.sq(field.inv(candidate))):
            known.append(candidate)
    return known


class Projection:
    def __init__(self, field, basis, g, family, multiplier_degree):
        n = field.n
        self.k = len(basis)
        self.dual = trace_dual(field)
        supports = domains(self.k, multiplier_degree)
        self.monomials = domains(self.k, multiplier_degree + 1)
        if family == "unrestricted":
            constraints = direct_constraints(field, basis, supports, self.dual)
            columns, self.dimension = quotient_coordinates(constraints, n * len(supports))
            self.columns = {mask: columns[i*n:(i+1)*n] for i, mask in enumerate(supports)}
        else:
            ring = FrobeniusRing(g, multiplier_degree)
            multipliers = ring.family()
            self.dimension = len(multipliers)
            translations = physical_monomials(field, basis, supports)
            self.columns = {mask: [0] * n for mask in supports}
            for i, multiplier in enumerate(multipliers):
                expanded = {}
                for j in supported_indices(multiplier):
                    for mask, value in translations[supports[j]].items():
                        expanded[mask] = expanded.get(mask, 0) ^ value
                for mask, value in expanded.items():
                    for bit in supported_indices(value):
                        self.columns[mask][bit] |= 1 << i

    def specialized(self, constant, linear):
        dual = list(map(self.dual, [constant, *linear]))
        columns = []
        for m in self.monomials:
            word = dual[0]
            for j in supported_indices(m):
                word ^= dual[j + 1]
            apply = self.columns.get(m)
            coefficient = xor_selected(apply, word) if apply is not None else 0
            for j in supported_indices(m):
                coefficient ^= xor_selected(self.columns[m ^ (1 << j)], dual[j + 1])
            columns.append(coefficient)
        return columns

    def low_rows(self, columns, degree):
        kept = sum(m.bit_count() <= degree for m in self.monomials)
        quotient, dimension = quotient_coordinates(columns[kept:], self.dimension)
        polynomials = [0] * dimension
        for i, coefficient in enumerate(columns[:kept]):
            for position in supported_indices(xor_selected(quotient, coefficient)):
                polynomials[position] |= 1 << i
        pivots = {}
        for polynomial in polynomials:
            insert(polynomial, pivots)
        return [[self.monomials[i] for i in supported_indices(row)] for row in pivots.values()]


def run(ranker, extra):
    for g in factors(23):
        field, basis, _ = geometry(23, g)
        k = len(basis)
        trace = linear_map(field, field.tr)
        half = linear_map(field, lambda w: half_trace(field, w))
        panel = []
        for target in targets(field, trace, g, extra):
            start = perf_counter()
            original, constant, linear = normalized(field, basis, target)
            normalization = perf_counter() - start
            solutions = expected_points(field, basis, trace, half, constant, linear)
            assert solutions is not None
            oriented = solutions[0]
            panel.append((target, original, constant, linear, oriented, normalization))
            start = perf_counter()
            baseline = measures(ranker, original, oriented, k, k)
            verification = perf_counter() - start
            start = perf_counter()
            found, closure = extract_small_fiber(ranker, original, 2*k, 4)
            extraction = perf_counter() - start
            assert found == oriented
            print(json.dumps(dict(n=23, g=g, target=target, arm="original",
                                  solutions=len(oriented), variables=2*k,
                                  normalization_seconds=normalization,
                                  verification_seconds=verification,
                                  extraction_seconds=extraction,
                                  extraction_row_xors=closure["row_xors"], **baseline)), flush=True)
        for family, multiplier_degree, dimension in (
            ("equivariant", 3, 35), ("unrestricted", 2, 80), ("unrestricted", 3, 761),
        ):
            start = perf_counter()
            plan = Projection(field, basis, g, family, multiplier_degree)
            build = perf_counter() - start
            assert plan.dimension == dimension
            for target, original, constant, linear, oriented, normalization in panel:
                sums = {point >> k for point in oriented}
                start = perf_counter()
                columns = plan.specialized(constant, linear)
                specialize = perf_counter() - start
                for cap in (2, 3):
                    start = perf_counter()
                    rows = plan.low_rows(columns, cap)
                    project = perf_counter() - start
                    assert all(satisfies(rows, point) for point in sums)
                    for arm in ("selector", "augmented"):
                        if arm == "augmented":
                            equations = original + [[m << k for m in row] for row in rows]
                            points, left = oriented, k
                        else:
                            equations, points, left = rows, sums, 0
                        start = perf_counter()
                        result = measures(ranker, equations, points, k, left)
                        elapsed = perf_counter() - start
                        extraction = {}
                        if family == "unrestricted" and multiplier_degree == 3 and arm == "selector" and cap == 2:
                            start = perf_counter()
                            found, closure = extract_small_fiber(ranker, rows, k, 3)
                            recovered = None
                            if found is not None:
                                recovered = recover_orientations(field, basis, half, constant, linear, found)
                            extraction = dict(
                                extracted=found is not None,
                                extraction_seconds=perf_counter() - start,
                                extraction_row_xors=closure["row_xors"],
                            )
                            if found is not None:
                                assert found == points
                                assert recovered == oriented
                            if len(points) <= 1 and result["solving_degree"] in range(4):
                                assert found is not None
                        print(json.dumps(dict(
                            n=23, g=g, target=target, family=family, arm=arm, input_cap=cap,
                            solutions=len(points), variables=k + left, projected_rows=len(rows),
                            multiplier_degree=multiplier_degree, multiplier_dimension=plan.dimension,
                            coefficient_columns=len(plan.monomials), setup_seconds=build,
                            specialize_seconds=specialize, project_seconds=project,
                            normalization_seconds=normalization, verification_seconds=elapsed,
                            **result, **extraction,
                        )), flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ranker", type=Path, required=True)
    parser.add_argument("--extra", type=int, choices=range(33), default=8, metavar="COUNT")
    args = parser.parse_args()
    run(args.ranker.resolve(), args.extra)
