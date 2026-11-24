"""Check sum/product linearization for subfield cosets and stable prime-degree bases."""

import argparse
import json
from collections import Counter, defaultdict
from random import Random
from time import perf_counter

from profile import GF, factors, geometry, span, xor_selected
from three_summand_join import Curve, direct_join, negate, triple


class BinaryMap:
    """An image echelon basis with preimages and a nullspace basis."""

    def __init__(self, columns):
        self.pivots = {}
        self.free = []
        for i, value in enumerate(columns):
            reduced, lift = self.reduce(value, 1 << i)
            if reduced:
                self.pivots[reduced.bit_length()] = reduced, lift
            else:
                self.free.append(lift)

    def reduce(self, value, lift=0):
        while value:
            row = self.pivots.get(value.bit_length())
            if row is None:
                break
            image, source = row
            value ^= image
            lift ^= source
        return value, lift

    def solve(self, value):
        remainder, lift = self.reduce(value)
        return None if remainder else lift


class EvenCurve(Curve):
    """Use the same curve formulas with a binary linear Artin–Schreier solver."""

    def __init__(self, field):
        self.field = field
        self.artin_schreier = BinaryMap(field.sq(1 << i) ^ (1 << i) for i in range(field.n))

    def lift(self, x):
        if x == 0:
            return ((0, 1),)
        f = self.field
        z = self.artin_schreier.solve(x ^ 1 ^ f.sq(f.inv(x)))
        if z is None:
            return ()
        point = (x, f.mul(x, z))
        return point, negate(point)


class SubfieldCoset:
    def __init__(self, k):
        self.field = GF(2*k)
        self.k = k
        f = self.field
        relative_trace = BinaryMap(f.pow(1 << i, 1 << k) ^ (1 << i) for i in range(2*k))
        self.basis = relative_trace.free
        self.shift = relative_trace.solve(1)
        assert self.shift is not None and len(self.basis) == k
        self.shift_square = f.sq(self.shift)

    def linear_system(self, t):
        f, basis, a2 = self.field, self.basis, self.shift_square
        t2 = f.sq(t)
        ta = f.mul(t, self.shift)
        columns = [f.mul(a2 ^ t2, f.sq(b)) ^ f.mul(ta, b) for b in basis]
        columns += [f.sq(b) ^ f.mul(t, b) for b in basis]
        return BinaryMap(columns), f.sq(a2) ^ f.mul(t, a2) ^ 1

    def parameter_fiber(self, t):
        linear, constant = self.linear_system(t)
        assert len(linear.free) <= 2
        particular = linear.solve(constant)
        rows = []
        if particular is not None:
            for offset in span(linear.free):
                state = particular ^ offset
                left = xor_selected(self.basis, state & ((1 << self.k) - 1))
                right = xor_selected(self.basis, state >> self.k)
                rows.append((left, right))
        return rows, len(linear.free)

    def pairs(self, t):
        rows, nullity = self.parameter_fiber(t)
        f, basis = self.field, self.basis
        pairs = set()
        for s, p in rows:
            quadratic = BinaryMap(f.sq(b) ^ f.mul(s, b) for b in basis)
            root = quadratic.solve(p)
            if root is None:
                continue
            u = xor_selected(basis, root)
            left, right = u ^ self.shift, u ^ s ^ self.shift
            assert s3(f, left, right, t) == 0
            pairs.add(tuple(sorted((left, right))))
        return pairs, len(rows), nullity


def s3(field, x, y, t):
    p, s = field.mul(x, y), x ^ y
    return field.sq(p) ^ field.mul(t, p) ^ field.sq(field.mul(t, s)) ^ 1


def power_of_two_report(k, samples):
    start = perf_counter()
    coset = SubfieldCoset(k)
    f = coset.field
    curve = EvenCurve(f)
    exhaustive = k <= 5
    by_target = defaultdict(set)
    group_targets = defaultdict(set)
    if exhaustive:
        xs = sorted(u ^ coset.shift for u in span(coset.basis))
        for i, x in enumerate(xs):
            for y in xs[i:]:
                pair = (x, y)
                # Eliminate t by independently solving its quadratic on K.
                q = BinaryMap(f.mul(f.sq(x ^ y), f.sq(1 << j)) ^
                              f.mul(f.mul(x, y), 1 << j) for j in range(f.n))
                value = q.solve(f.sq(f.mul(x, y)) ^ 1)
                if value is not None:
                    for z in span(q.free):
                        by_target[value ^ z].add(pair)
                for left in curve.lift(x):
                    for right in curve.lift(y):
                        w = curve.add(left, right)
                        if w is not None:
                            group_targets[w].add(pair)
        targets = range(1 << f.n)
    else:
        rng = Random(k ^ 0x5C05)
        targets = [rng.randrange(1 << f.n) for _ in range(samples)]
        while len(targets) < samples * 2:
            u, v = (xor_selected(coset.basis, rng.getrandbits(k)) ^ coset.shift for _ in range(2))
            left, right = curve.lift(u), curve.lift(v)
            if left and right and (w := curve.add(left[0], right[0])) is not None:
                targets.append(w[0])
                group_targets[w].add(tuple(sorted((u, v))))
    elapsed = perf_counter() - start
    start = perf_counter()
    solutions = Counter()
    nullities = Counter()
    linear_solutions = Counter()
    found = {}
    for t in targets:
        pairs, candidate_count, nullity = coset.pairs(t)
        if exhaustive:
            assert pairs == by_target[t], (k, t, pairs, by_target[t])
        solutions[len(pairs)] += 1
        nullities[nullity] += 1
        linear_solutions[candidate_count] += 1
        found[t] = pairs
    for target, expected in group_targets.items():
        recovered = set()
        for x, y in found[target[0]]:
            for left in curve.lift(x):
                for right in curve.lift(y):
                    if curve.add(left, right) == target:
                        recovered.add((x, y))
        assert recovered == expected if exhaustive else expected <= recovered
    print(json.dumps(dict(
        family="subfield_coset", n=2*k, k=k, targets=len(targets), exhaustive=exhaustive,
        nullities=nullities, candidate_counts=linear_solutions, abscissa_pair_counts=solutions,
        signed_targets_checked=len(group_targets), reference_seconds=elapsed,
        verify_and_extract_seconds=perf_counter()-start,
    )), flush=True)


def prime_degree_report(n):
    for g in factors(n):
        field, basis, _ = geometry(n, g)
        k = len(basis)
        products = BinaryMap(field.mul(x, y) for x in basis for y in basis)
        values = [value for value, _ in products.pivots.values()]
        assert len(values) == n - 1 and all(not field.tr(v) for v in values)
        rng = Random(g ^ 0x65A4)
        targets = []
        while len(targets) < 2:
            t = rng.randrange(1, 1 << n)
            if not field.tr(t) or field.tr(field.sq(field.inv(t))):
                continue
            t2 = field.sq(t)
            columns = [field.mul(1 ^ t2, field.sq(s)) ^ field.mul(t, s) for s in basis]
            columns += [field.sq(v) ^ field.mul(t, v) for v in values]
            linear = BinaryMap(columns)
            assert len(linear.free) == k and linear.solve(t) is not None
            targets.append(t)
        print(json.dumps(dict(family="prime_degree_frobenius", n=n, k=k, g=g,
                              product_span_rank=len(values), checked_targets=targets,
                              relaxed_selector_count=1 << k)), flush=True)


def join_report(k):
    start = perf_counter()
    coset = SubfieldCoset(k)
    curve = EvenCurve(coset.field)
    by_x = {x: signs for x in (coset.shift ^ u for u in span(coset.basis))
            if (signs := curve.lift(x))}
    points = [point for signs in by_x.values() for point in signs]
    membership = set(points)
    setup = perf_counter() - start
    rng = Random(k ^ 0x593F)
    original = joined = shared = 0.0
    checks = candidate_count = verified = intermediates = 0
    for _ in range(12):
        seed = rng.sample(list(by_x), 3)
        planted = [rng.choice(by_x[x]) for x in seed]
        target = curve.add(planted[0], curve.add(planted[1], planted[2]))
        thirds = points if k == 5 else rng.sample(points, 16) + planted[:1]
        thirds = list(dict.fromkeys(thirds))
        start = perf_counter()
        jobs = [(curve.add(target, negate(p)), p) for p in thirds]
        shared += perf_counter() - start
        start = perf_counter()
        expected, work = direct_join(curve, points, membership, jobs)
        original += perf_counter() - start
        checks += work
        start = perf_counter()
        pairs_by_t = {}
        found = set()
        for w, third in jobs:
            if w is None:
                continue
            t = w[0]
            if t not in pairs_by_t:
                pairs, candidates, _ = coset.pairs(t)
                pairs_by_t[t] = pairs
                intermediates += 1
                candidate_count += candidates
            for x, y in pairs_by_t[t]:
                for p in by_x.get(x, ()):
                    for q in by_x.get(y, ()):
                        relation = triple(p, q, third)
                        if relation and curve.add(p, q) == w:
                            found.add(relation)
        joined += perf_counter() - start
        assert found == expected and tuple(sorted(planted)) in found
        verified += len(found)
    print(json.dumps(dict(
        family="subfield_coset_join", n=2*k, k=k, targets=12, factor_base_abscissae=len(by_x),
        direct_pair_checks=checks, intermediate_abscissae=intermediates, parameter_candidates=candidate_count,
        verified_triples=verified, setup_seconds=setup, intermediate_seconds=shared,
        direct_seconds=original, linear_join_seconds=joined,
    )), flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--samples", type=int, default=24)
    args = parser.parse_args()
    for degree in (3, 5, 7, 11, 17):
        power_of_two_report(degree, args.samples)
    for degree in (5, 7, 11):
        join_report(degree)
    for n in (7, 23, 47, 71, 79):
        prime_degree_report(n)
