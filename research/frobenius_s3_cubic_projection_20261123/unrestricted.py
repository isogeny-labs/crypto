"""Split the unrestricted multiplier matrix by coefficient eigenvalue."""
import argparse
import json
from functools import cache
from math import comb
from pathlib import Path
from random import Random
import subprocess
from time import perf_counter

from profile import domains, factors, geometry, rank_f2, row_kernel, xor_selected


def supported_indices(mask):
    while mask:
        bit = mask & -mask
        mask ^= bit
        yield bit.bit_length() - 1


def root_powers(n, modulus):
    degree = (n - 1) // 2
    values = [1]
    for _ in range(n - 1):
        value = values[-1] << 1
        values.append(value ^ modulus if value >> degree else value)
    assert values[-1] << 1 == modulus ^ 1
    assert len(set(values)) == n
    return values


def lower_system(n, modulus, shift, degree=3):
    k = (n - 1) // 2
    weights = [pow(2, i, n) for i in range(k)]
    accepted = {0, *weights}
    terms = domains(k, degree)
    columns = {mask: j for j, mask in enumerate(terms)}
    powers = root_powers(n, modulus)

    def weight(mask):
        return sum(weights[j] for j in supported_indices(mask)) % n

    def extraneous(mask):
        return (weight(mask) + shift) % n not in accepted

    top_rows = [sum(1 << columns[mask ^ (1 << i)] for i in supported_indices(mask))
                for mask in domains(k, degree + 1)[len(terms):] if extraneous(mask)]
    top_kernel = row_kernel(top_rows, len(terms))
    lifts = [[] for _ in terms]
    for j, vector in enumerate(top_kernel):
        for i in supported_indices(vector):
            lifts[i].append(j)
    lower = {m: {} for m in terms if extraneous(m)}
    for alpha, m in enumerate(terms):
        if m in lower:
            lower[m][alpha] = powers[-(weight(m) + shift) % n]
        for j in range(k):
            if not (m >> j & 1):
                dest, phase = m | (1 << j), 0
            else:
                dest, phase = m, 0
                while dest >> j & 1:
                    dest ^= 1 << j
                    j = (j + 1) % k
                    phase += weights[j]
                dest |= 1 << j
            if dest in lower:
                row = lower[dest]
                row[alpha] = row.get(alpha, 0) ^ powers[phase % n]
    rows = []
    for original in lower.values():
        reduced = {}
        for alpha, coefficient in original.items():
            if coefficient:
                for j in lifts[alpha]:
                    reduced[j] = reduced.get(j, 0) ^ coefficient
        rows.append({i: v for i, v in reduced.items() if v})
    return rows, len(terms), len(top_rows), len(top_kernel)


def block_rank(binary, n, modulus, rows, columns, verify_arithmetic, timeout=120):
    payload = [f'{(n - 1) // 2} {modulus} {columns} {len(rows)}']
    payload.extend(' '.join([str(len(row)), *(f'{i} {v}' for i, v in row.items())]) for row in rows)
    data = '\n'.join(payload) + '\n'

    def run(*args):
        result = subprocess.run([binary, *args], input=data, capture_output=True,
                                text=True, check=True, timeout=timeout)
        return int(result.stdout)

    rank = run()
    if verify_arithmetic:
        assert rank == run('--polynomial')
    return rank


def trace_dual(field):
    n = field.n
    trace_mask = sum(field.tr(1 << i) << i for i in range(n))

    @cache
    def dual(value):
        return sum(((field.mul(value, 1 << j) & trace_mask).bit_count() & 1) << j
                   for j in range(n))
    return dual


def direct_constraints(field, basis, terms, dual):
    """Coefficient rows for annihilating the x-dependent part of S3."""
    for x in basis:
        square = dual(field.sq(x))
        products = [dual(field.mul(x, s)) for s in basis]
        rows = {}
        for j, monomial in enumerate(terms):
            offset = j * field.n
            rows[monomial] = rows.get(monomial, 0) ^ (square << offset)
            for i, coefficient in enumerate(products):
                support = monomial | (1 << i)
                rows[support] = rows.get(support, 0) ^ (coefficient << offset)
        yield from rows.values()


def direct_dimension(n, modulus):
    """Independent rank calculation in physical-field coordinates for small n."""
    field, basis, _ = geometry(n, modulus)
    terms = domains(len(basis), 3)
    return n * len(terms) - rank_f2(direct_constraints(field, basis, terms, trace_dual(field)))


def false_positive(n, modulus, degree=3):
    """Find an S3 target with a sum surviving the telescoping multipliers."""
    field, basis, square = geometry(n, modulus)
    rng = Random(n + 0x5A39)

    def half_trace(value):
        result = value
        for _ in range((n - 1) // 2):
            result = square(square(result)) ^ value
        return result

    for _ in range(512):
        target = rng.randrange(1, 1 << n)
        if not field.tr(target):
            continue
        inverse_square = square(field.inv(target))
        if field.tr(inverse_square):
            continue
        s = xor_selected(basis, rng.randrange(1, 1 << len(basis)))
        conjugates = [s]
        for _ in range(n - 1):
            conjugates.append(square(conjugates[-1]))
        multipliers = [1]
        for r in range(1, degree + 1):
            alpha = 0
            for j in range(r):
                term = conjugates[-j]
                for i in range(1, r - j):
                    term = field.mul(term, conjugates[i])
                alpha ^= term
            multipliers.append(alpha)
        product = field.mul(target, half_trace(inverse_square ^ square(s)))
        if not field.tr(product):
            product ^= target
        rhs = product ^ s ^ 1
        checks = [field.tr(field.mul(alpha, rhs)) for alpha in multipliers]
        if any(checks):
            continue
        columns = [square(x) ^ field.mul(x, s) for x in basis]
        rank = rank_f2(columns)
        augmented = rank_f2([*columns, rhs])
        if rank == augmented:
            continue
        assert rank == len(basis) - 1 and augmented == len(basis)
        assert (square(product) ^ field.mul(target, product)
                ^ field.mul(square(target), square(s)) ^ 1) == 0
        y = field.mul(target, half_trace(target ^ 1 ^ inverse_square))
        assert (square(y) ^ field.mul(target, y)) == field.mul(square(target), target) ^ square(target) ^ 1
        return dict(field_modulus=field.poly, target_x=target, target_y=y, sum_x=s,
                    normalized_rhs=rhs, trace_checks=checks, image_rank=rank,
                    augmented_rank=augmented)
    raise AssertionError('no false positive found in the deterministic target stream')


def measure(binary, n, verify=False, degree=3, timeout=120):
    k = (n - 1) // 2
    for modulus in factors(n):
        dimensions = []
        details = []
        for shift in (0, 1, n - 1):
            start = perf_counter()
            rows, initial, top_rows, top_kernel = lower_system(n, modulus, shift, degree)
            if degree == 4:
                rows.sort(key=len)
            rank = block_rank(binary, n, modulus, rows, top_kernel,
                              verify and k <= 23 and degree == 3, timeout)
            dimensions.append(top_kernel - rank)
            details.append(dict(shift=shift, original_columns=initial, top_rows=top_rows,
                                after_top=top_kernel, lower_rows=len(rows), lower_rank=rank,
                                dimension=top_kernel - rank, seconds=perf_counter() - start))
        scalar_span = (sum(comb(k, j) * (degree + 1 - j) for j in range(degree + 1))
                       if k > degree else None)
        dimension = dimensions[0] + k * sum(dimensions[1:])
        direct = direct_dimension(n, modulus) if verify and n <= 23 and degree == 3 else None
        if direct is not None:
            assert direct == dimension
        witness = false_positive(n, modulus, degree) if dimension == scalar_span else None
        print(json.dumps(dict(n=n, g=modulus, k=k, multiplier_degree=degree, components=details,
                              false_positive=witness,
                              direct_dimension=direct,
                              unrestricted_dimension=dimension, scalar_multiple_dimension=scalar_span,
                              span_certificate=dimension == scalar_span)), flush=True)


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('n', type=int, nargs='+', choices=(7, 23, 47, 71, 79))
    parser.add_argument('--ranker', type=Path, required=True)
    parser.add_argument('--verify', action='store_true')
    parser.add_argument('--degree', choices=(3, 4), type=int, default=3)
    parser.add_argument('--rank-timeout', type=int, default=120)
    args = parser.parse_args()
    for n in args.n:
        measure(args.ranker.resolve(), n, args.verify, args.degree, args.rank_timeout)
