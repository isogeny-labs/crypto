"""Measure the Boolean solving degrees after adjoining quadratic multiples of S3."""

import argparse
import json
from pathlib import Path
from random import Random

from profile import factors, geometry, xor_selected
from quotient import expected_points, half_trace, linear_map, measures, satisfies
from selector_elimination import normalized


def multiples(field, basis, constant, linear):
    k = len(basis)
    s_terms = {}
    x_terms = {}
    squared_basis = [field.sq(value) for value in basis]

    def add(terms, mask, value):
        terms[mask] = terms.get(mask, 0) ^ value

    for i, (v, v_squared) in enumerate(zip(basis, squared_basis)):
        x = 1 << i
        s = 1 << (i + k)
        weighted_constant = field.mul(v, constant)
        add(s_terms, s, weighted_constant)
        add(x_terms, x, weighted_constant)
        for j, (u, u_squared) in enumerate(zip(basis, squared_basis)):
            other_s = 1 << (j + k)
            sx_squared = field.mul(v_squared, u)
            weighted_linear = field.mul(v, linear[j])
            add(s_terms, s | other_s, weighted_linear)
            add(s_terms, x | other_s, sx_squared ^ field.mul(v, u_squared))
            add(x_terms, x | (1 << j), field.mul(v, u_squared))
            add(x_terms, x | other_s, sx_squared ^ weighted_linear)
    return s_terms, x_terms


def coordinates(field, terms):
    return [
        row for bit in range(field.n)
        if (row := sorted(mask for mask, value in terms.items() if value >> bit & 1))
    ]


def verify_sample(field, basis, constant, linear, s_terms, x_terms):
    k = len(basis)
    rng = Random(field.n)
    for _ in range(24):
        a, b = (rng.randrange(1 << k) for _ in range(2))
        assignment = a | (b << k)
        x, s = xor_selected(basis, a), xor_selected(basis, b)
        value = field.sq(x) ^ field.mul(x, s) ^ constant ^ xor_selected(linear, b)
        for factor, terms in ((s, s_terms), (x, x_terms)):
            evaluated = 0
            for mask, coefficient in terms.items():
                if mask & assignment == mask:
                    evaluated ^= coefficient
            assert evaluated == field.mul(factor, value)


def run(binary):
    for n in (7, 23):
        field, basis, _ = geometry(n, factors(n)[0])
        k = len(basis)
        trace = linear_map(field, field.tr)
        half = linear_map(field, lambda value: half_trace(field, value))
        targets = range(1, 1 << n) if n == 7 else (6277527, 5846863, 7373655, 5025355)
        for target in targets:
            if not trace(target) or trace(field.sq(field.inv(target))):
                continue
            original, constant, linear = normalized(field, basis, target)
            points = expected_points(field, basis, trace, half, constant, linear)
            if points is None:
                continue
            roots, _ = points
            s_terms, x_terms = multiples(field, basis, constant, linear)
            verify_sample(field, basis, constant, linear, s_terms, x_terms)
            with_s = original + coordinates(field, s_terms)
            with_both = with_s + coordinates(field, x_terms)
            for arm, equations in (("s_multiple", with_s), ("both_multiples", with_both)):
                assert all(satisfies(equations, point) for point in roots)
                system = [list(row) for row in dict.fromkeys(map(tuple, equations))]
                print(json.dumps(dict(
                    n=n, k=k, target=target, arm=arm, variables=2 * k,
                    roots=len(roots), equations=len(system),
                    **measures(binary, system, roots, k, k),
                )), flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ranker", type=Path, required=True)
    run(parser.parse_args().ranker)
