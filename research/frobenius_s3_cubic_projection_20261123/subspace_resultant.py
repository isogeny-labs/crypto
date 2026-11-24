"""Check a compact exact resultant for the normalized two-coordinate equation."""

import argparse
import json
from pathlib import Path
from random import Random
from time import perf_counter

from profile import factors, geometry, rank_f2, span, xor_selected
from quotient import expected_points, half_trace, linear_map
from selector_elimination import normalized


def image_coefficients(field, subspace, s):
    """Return ascending linearized coefficients for the image at s != 0."""
    twists = [s]
    for _ in range(1, len(subspace) - 1):
        twists.append(field.sq(twists[-1]))
    leading = 1
    image = [leading]
    for j in range(len(subspace) - 2, 0, -1):
        leading = field.mul(leading, twists[j]) ^ subspace[j]
        image.append(leading)
    return image[::-1]


def evaluate(field, coefficients, value):
    result = 0
    for i, coefficient in enumerate(coefficients):
        if i:
            value = field.sq(value)
        if coefficient:
            result ^= field.mul(coefficient, value)
    return result


def frobenius(field, subspace, coefficients):
    squared = [field.sq(value) for value in coefficients]
    overflow = squared.pop()
    return [value ^ (overflow if c else 0)
            for value, c in zip([0, *squared], subspace)]


def linearized_rhs(field, subspace, target):
    """Return the linear part of p in powers S, S², ..., S^(2^(k-1))."""
    k = len(subspace) - 1
    power = [1] + [0] * (k - 1)
    half = [0] * k
    for j in range(field.n):
        power = frobenius(field, subspace, power)
        if not j % 2:
            half = [a ^ b for a, b in zip(half, power)]
    weighted = [field.mul(target, value) for value in half]
    conjugate = weighted
    trace = weighted.copy()
    for _ in range(1, field.n):
        conjugate = frobenius(field, subspace, conjugate)
        trace = [a ^ b for a, b in zip(trace, conjugate)]
    linear = [value ^ field.mul(target, bit) for value, bit in zip(weighted, trace)]
    linear[0] ^= 1
    return linear


def sparse_residual(field, subspace, constant, linear):
    """Expand the resultant factor in the univariate basis on U."""
    k = len(linear)
    modulus_degree = 1 << k
    low_powers = [1 << j for j, c in enumerate(subspace[:-1]) if c]
    terms = {}

    def add(degree, value):
        terms[degree] = terms.get(degree, 0) ^ value

    for i in range(k):
        values = [(0, constant)] + [(1 << j, value) for j, value in enumerate(linear)]
        offset = 0
        for j in range(i + 1, k + 1):
            if subspace[j]:
                for power, value in values:
                    degree = offset + power
                    if degree < modulus_degree:
                        add(degree, value)
                    else:
                        for reduced in low_powers:
                            add(degree - modulus_degree + reduced, value)
            offset += 1 << j
        constant = field.sq(constant)
        linear = frobenius(field, subspace, linear)
    return {degree: value for degree, value in terms.items() if value}


def sparse_evaluate(field, terms, value):
    k = max(terms, default=0).bit_length()
    powers = [value]
    for _ in range(1, k):
        powers.append(field.sq(powers[-1]))
    result = 0
    for exponent, coefficient in terms.items():
        while exponent:
            bit = exponent & -exponent
            coefficient = field.mul(coefficient, powers[bit.bit_length() - 1])
            exponent ^= bit
        result ^= coefficient
    return result


def patch_zero(terms, subspace, desired):
    # L_U(S)/S is the indicator of zero on U because c_0 = 1.
    change = desired ^ terms.get(0, 0)
    for j, coefficient in enumerate(subspace):
        if coefficient and change:
            degree = (1 << j) - 1
            result = terms.get(degree, 0) ^ change
            if result:
                terms[degree] = result
            else:
                terms.pop(degree, None)


def anf(values):
    result = values.copy()
    bit = 1
    while bit < len(result):
        for mask in range(len(result)):
            if mask & bit:
                result[mask] ^= result[mask ^ bit]
        bit <<= 1
    return result


def rank_by_degree(terms, width):
    full_rank = rank_f2(terms.values())
    # These high-degree columns give the constraints on linear combinations
    # of the coordinate equations that have degree at most the specified cap.
    return [
        full_rank - rank_f2(value for mask, value in terms.items() if mask.bit_count() > degree)
        for degree in range(width + 1)
    ]


def witness_entry(degree, g):
    path = Path(__file__).with_name("unrestricted-ranks.jsonl")
    for entry in map(json.loads, path.read_text().splitlines()):
        if entry["n"] == degree and entry["g"] == g:
            return entry["false_positive"]
    raise ValueError(f"No cubic witness for {degree}, {g}")


def gcd_roots(field, subspace, terms):
    from flint import fmpz_mod_poly_ctx, fq_default_ctx, fq_default_poly_ctx

    def bits(word, width):
        return [(word >> i) & 1 for i in range(width)]

    start = perf_counter()
    modulus = fmpz_mod_poly_ctx(2)(bits(field.poly, field.n + 1))
    coeffs = fq_default_ctx(modulus=modulus, fq_type="FQ_NMOD")
    ring = fq_default_poly_ctx(coeffs)
    dense = [coeffs(0)] * (max(terms, default=0) + 1)
    for degree, value in terms.items():
        dense[degree] = coeffs(bits(value, field.n))
    expression = ring(dense)
    x = ring.gen()
    domain = sum((x ** (1 << j) for j, value in enumerate(subspace) if value), ring.zero())
    built = perf_counter()
    common = expression.gcd(domain)
    isolated = perf_counter()
    roots = {sum(int(bit) << i for i, bit in enumerate(root.to_list()))
             for root, _ in common.roots()}
    return roots, dict(
        gcd_degree=common.degree(), dense_coefficients=len(dense),
        dense_build_seconds=built - start, gcd_seconds=isolated - built,
        root_isolation_seconds=perf_counter() - isolated,
    )


def verify_sample(field, basis, subspace):
    points = span(basis[:min(len(basis), 7)])
    rng = Random(field.n)
    for s in points[1:]:
        image = image_coefficients(field, subspace, s)
        assert field.mul(s, image[0]) == subspace[0]
        for x in basis:
            assert evaluate(field, image, field.sq(x) ^ field.mul(s, x)) == 0
        if len(basis) <= 7:
            for _ in range(3):
                value = rng.randrange(1 << field.n)
                product = 1
                for x in points:
                    product = field.mul(product, field.sq(x) ^ field.mul(s, x) ^ value)
                assert product == field.sq(evaluate(field, image, value))


def run(n, dense_gcd):
    targets = (6277527, 5846863, 7373655, 5025355)
    for g in factors(n):
        field, basis, _ = geometry(n, g)
        k = len(basis)
        subspace = [(g >> j) & 1 for j in range(k + 1)]
        verify_sample(field, basis, subspace)
        if n == 7:
            panel = range(1, 1 << n)
        elif n == 23:
            panel = targets
        else:
            witness = witness_entry(n, g)
            panel = [witness["target_x"]]
        if n <= 23:
            trace = linear_map(field, field.tr)
            half = linear_map(field, lambda value: half_trace(field, value))
            sums = span(basis)
        for target in panel:
            if not field.tr(target) or field.tr(field.sq(field.inv(target))):
                continue
            _, constant, linear = normalized(field, basis, target)
            start = perf_counter()
            power_coefficients = linearized_rhs(field, subspace, target)
            terms = sparse_residual(field, subspace, constant, power_coefficients)
            patch_zero(terms, subspace, evaluate(field, subspace, constant))
            setup = perf_counter() - start
            linear_terms = {1 << j: value for j, value in enumerate(power_coefficients)}
            assert all(sparse_evaluate(field, linear_terms, s) == c for s, c in zip(basis, linear))
            result = dict(
                n=n, g=g, target=target,
                resultant_boolean_degree=max((m.bit_count() for m in terms), default=0),
                resultant_univariate_degree=max(terms, default=0),
                resultant_univariate_terms=len(terms),
                sparse_build_seconds=setup,
            )
            if n > 23:
                s = witness["sum_x"]
                value = evaluate(field, image_coefficients(field, subspace, s), witness["normalized_rhs"])
                assert value and value == sparse_evaluate(field, terms, s)
                result.update(checked_nonzero_sums=127, cubic_witness_residual=value)
                if n == 47:
                    path = Path(__file__).with_name("unrestricted-quartic-ranks.jsonl")
                    quartic = next(entry["false_positive"] for entry in map(json.loads, path.read_text().splitlines())
                                   if entry["g"] == g)
                    image = image_coefficients(field, subspace, quartic["sum_x"])
                    value = evaluate(field, image, quartic["normalized_rhs"])
                    assert value
                    result["quartic_witness_residual"] = value
            else:
                roots = expected_points(field, basis, trace, half, constant, linear)
                if roots is None:
                    continue
                start = perf_counter()
                values = []
                for mask, s in enumerate(sums):
                    row = image_coefficients(field, subspace, s) if s else subspace
                    values.append(evaluate(field, row, constant ^ xor_selected(linear, mask)))
                elapsed = perf_counter() - start
                assert all(values[i] == sparse_evaluate(field, terms, sums[i])
                           for i in range(0, len(sums), 17 if n == 23 else 1))
                observed = {mask for mask, value in enumerate(values) if value == 0}
                assert observed == {point >> k for point in roots[0]}
                coefficients = anf(values)
                boolean_terms = {i: value for i, value in enumerate(coefficients) if value}
                assert result["resultant_boolean_degree"] == max(
                    (m.bit_count() for m in boolean_terms), default=0)
                result.update(selector_candidates=len(sums), matching_sums=len(observed),
                              oriented_roots=len(roots[0]), scan_seconds=elapsed,
                              resultant_boolean_terms=len(boolean_terms),
                              coordinate_span_by_degree=rank_by_degree(boolean_terms, k))
                if dense_gcd:
                    isolated, timings = gcd_roots(field, subspace, terms)
                    assert isolated == {sums[mask] for mask in observed}
                    result.update(timings)
            print(json.dumps(result), flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("fields", nargs="+", type=int, choices=(7, 23, 47, 71, 79))
    parser.add_argument("--dense-gcd", action="store_true", help="compare dense FLINT GCD through 23 bits")
    args = parser.parse_args()
    for degree in args.fields:
        run(degree, args.dense_gcd)
