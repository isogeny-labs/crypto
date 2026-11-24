"""Charge relation generation, rank, and target recovery on two toy groups."""

import argparse
import json
from math import isqrt
from pathlib import Path
from random import Random
from time import perf_counter

from sympy import isprime, sqrt_mod

from compiled_projection import Projection
from profile import factors, geometry, span
from three_summand_join import Curve, direct_join, negate, resultant_join


def multiply(curve, point, scalar):
    result = None
    while scalar:
        if scalar & 1:
            result = curve.add(result, point)
        point = curve.add(point, point)
        scalar >>= 1
    return result


def frobenius(curve, point):
    return tuple(curve.field.sq(coordinate) for coordinate in point)


def subgroup_order(n):
    previous, trace = 2, 1
    for _ in range(1, n):
        previous, trace = trace, trace - 2 * previous
    order, remainder = divmod((1 << n) + 1 - trace, 2)
    assert not remainder and isprime(order)
    return order


def orbit_columns(curve, by_x, generator, order):
    discriminants = sqrt_mod(-7, order, all_roots=True)
    candidates = [(1 + root) * pow(2, -1, order) % order for root in discriminants]
    eigenvalue = next(value for value in candidates
                      if multiply(curve, generator, value) == frobenius(curve, generator))
    labels = {}
    representatives = []
    for signs in by_x.values():
        rep = min(signs)
        if rep in labels:
            continue
        column = len(representatives)
        representatives.append(rep)
        point, weight, length = rep, 1, 0
        while point not in labels:
            labels[point] = (column, weight)
            labels[negate(point)] = (column, -weight % order)
            point = frobenius(curve, point)
            weight = weight * eigenvalue % order
            length += 1
        assert point == rep and weight == 1 and length == curve.field.n
    assert len(labels) == 2 * len(by_x)
    return representatives, labels, eigenvalue


def relation_row(triple, labels, order):
    row = {}
    for point in triple:
        column, weight = labels[point]
        total = (row.get(column, 0) + weight) % order
        if total:
            row[column] = total
        else:
            row.pop(column, None)
    return row


class RelationSystem:
    def __init__(self, order):
        self.order = order
        self.pivots = {}

    def append(self, row, rhs):
        while row:
            column = min(row)
            previous = self.pivots.get(column)
            if previous is None:
                inverse = pow(row[column], -1, self.order)
                self.pivots[column] = ({i: v * inverse % self.order for i, v in row.items()},
                                       rhs * inverse % self.order)
                return
            pivot, known = previous
            scale = row[column]
            for i, value in pivot.items():
                changed = (row.get(i, 0) - scale * value) % self.order
                if changed:
                    row[i] = changed
                elif i in row:
                    del row[i]
            rhs = (rhs - scale * known) % self.order
        assert rhs == 0

    def solve(self, width):
        assert len(self.pivots) == width
        solution = [0] * width
        for i in range(width - 1, -1, -1):
            row, rhs = self.pivots[i]
            solution[i] = (rhs - sum(value * solution[j] for j, value in row.items() if j != i)) % self.order
        return solution


def baby_step(curve, generator, challenge, order):
    step = isqrt(order - 1) + 1
    babies = {}
    point = None
    for i in range(step):
        babies[point] = i
        point = curve.add(point, generator)
    rewind = negate(point)
    point = challenge
    for i in range(step):
        if point in babies:
            return (step * i + babies[point]) % order
        point = curve.add(point, rewind)
    raise ValueError("Challenge is outside the toy subgroup")


def solve(degree, g, method, max_attempts, ranker=None):
    start = perf_counter()
    field, basis, _ = geometry(degree, g)
    curve = Curve(field)
    sums = span(basis)
    subspace = [(g >> j) & 1 for j in range(len(basis) + 1)]
    by_x = {x: signs for x in (s ^ 1 for s in sums) if (signs := curve.lift(x))}
    points = [p for signs in by_x.values() for p in signs]
    membership = set(points)
    generator = points[0]
    order = subgroup_order(degree)
    assert multiply(curve, generator, order) is None
    reps, labels, eigenvalue = orbit_columns(curve, by_x, generator, order)
    system = RelationSystem(order)
    rng = Random(g ^ 0xA71)
    secret = rng.randrange(1, order)
    challenge = multiply(curve, generator, secret)
    thirds_per_trial = min(8, len(points)) if degree == 23 else len(points)
    compiled = (Projection(field, basis, g, "unrestricted", 3), ranker) if method == "compiled" else None
    setup = perf_counter() - start

    search_seconds = row_seconds = solve_seconds = 0.0
    verified = collection_triples = empty = descent_attempts = 0
    compiled_fallbacks = compiled_false_positives = 0
    trace = []
    result = None
    solved_logs = None
    for attempt in range(1, max_attempts + 1):
        collecting = solved_logs is None
        started = perf_counter()
        offset = rng.randrange(1, order)
        r = multiply(curve, generator, offset)
        target = r if collecting else curve.add(challenge, r)
        thirds = rng.sample(points, thirds_per_trial)
        jobs = [(curve.add(target, negate(p)), p) for p in thirds]
        if method == "direct":
            triples, _ = direct_join(curve, points, membership, jobs)
        else:
            triples, _, metrics = resultant_join(curve, basis, subspace, sums, [], by_x, jobs,
                                                 method, compiled)
            compiled_fallbacks += metrics["compiled_fallbacks"]
            compiled_false_positives += metrics["compiled_false_positives"]
        search_seconds += perf_counter() - started
        trace.append(dict(rank=len(system.pivots), triples=len(triples), collection=collecting))
        verified += len(triples)
        empty += not triples

        started = perf_counter()
        if collecting:
            collection_triples += len(triples)
            for triple in sorted(triples):
                system.append(relation_row(triple, labels, order), offset)
            row_seconds += perf_counter() - started
            if len(system.pivots) == len(reps):
                started = perf_counter()
                solved_logs = system.solve(len(reps))
                solve_seconds += perf_counter() - started
        else:
            descent_attempts += 1
            for triple in sorted(triples):
                row = relation_row(triple, labels, order)
                residual = (sum(solved_logs[i] * value for i, value in row.items()) - offset) % order
                if result is not None:
                    assert residual == result
                result = residual
            row_seconds += perf_counter() - started
            if result is not None:
                break

    started = perf_counter()
    if result is not None:
        assert multiply(curve, generator, result) == challenge and result == secret
        assert all(multiply(curve, generator, log) == point for point, log in zip(reps, solved_logs))
    verification_seconds = perf_counter() - started
    started = perf_counter()
    assert baby_step(curve, generator, challenge, order) == secret
    generic_seconds = perf_counter() - started
    return dict(
        n=degree, g=g, method=method, subgroup_order=order, frobenius_eigenvalue=eigenvalue,
        factor_base_abscissae=len(by_x), orbit_columns=len(reps), thirds_per_trial=thirds_per_trial,
        attempts=attempt, collection_attempts=attempt - descent_attempts, descent_attempts=descent_attempts,
        empty_attempts=empty, verified_triples=verified, collection_triples=collection_triples,
        rank=len(system.pivots), solved=result is not None, solution=result, attempts_trace=trace,
        compiled_fallbacks=compiled_fallbacks, compiled_false_positives=compiled_false_positives,
        setup_seconds=setup, search_seconds=search_seconds, row_seconds=row_seconds,
        solve_seconds=solve_seconds, verification_seconds=verification_seconds,
        generic_seconds=generic_seconds,
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("fields", nargs="+", type=int, choices=(7, 23))
    parser.add_argument("--method", choices=("direct", "batch", "compiled"), required=True)
    parser.add_argument("--ranker", type=Path)
    parser.add_argument("--max-attempts", type=int, choices=range(1, 513), default=128, metavar="COUNT")
    args = parser.parse_args()
    if args.method == "compiled" and (args.ranker is None or args.fields != [23]):
        parser.error("the compiled arm requires --ranker and the 23-bit field")
    for degree in args.fields:
        for factor in factors(degree):
            ranker = args.ranker.resolve() if args.ranker else None
            print(json.dumps(solve(degree, factor, args.method, args.max_attempts, ranker)), flush=True)
