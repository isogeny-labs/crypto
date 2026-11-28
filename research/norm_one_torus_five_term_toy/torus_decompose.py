"""Toy five-term elliptic decomposition with a norm-one torus factor base."""

import argparse
from dataclasses import dataclass
from itertools import combinations_with_replacement
from math import isqrt
from random import Random
from time import perf_counter

Point = tuple[int, int] | None
Witness = tuple[Point, Point]
Decomposition = tuple[Point, Point, Point, Point, Point]


@dataclass(frozen=True)
class Curve:
    p: int
    a: int
    b: int

    def rhs(self, x: int) -> int:
        return (x**3 + self.a*x + self.b) % self.p

    def neg(self, point: Point) -> Point:
        return None if point is None else (point[0], -point[1] % self.p)

    def add(self, left: Point, right: Point) -> Point:
        if left is None:
            return right
        if right is None:
            return left
        x, y = left
        u, v = right
        if x == u:
            if (y + v) % self.p == 0:
                return None
            numerator, denominator = 3*x*x + self.a, 2*y
        else:
            numerator, denominator = v-y, u-x
        slope = numerator * pow(denominator, -1, self.p) % self.p
        result_x = (slope*slope - x - u) % self.p
        return result_x, (slope*(x-result_x) - y) % self.p

    def sum(self, points) -> Point:
        total = None
        for point in points:
            total = self.add(total, point)
        return total


def multiply(left, right, p, d):
    a, b = left
    c, e = right
    return (a*c + d*b*e) % p, (a*e + b*c) % p


def power(value, exponent, p, d):
    result = (1, 0)
    while exponent:
        if exponent & 1:
            result = multiply(result, value, p, d)
        value = multiply(value, value, p, d)
        exponent //= 2
    return result


def prime_divisors(number: int) -> tuple[int, ...]:
    divisors = []
    candidate = 2
    while candidate <= isqrt(number):
        if number % candidate == 0:
            divisors.append(candidate)
            while number % candidate == 0:
                number //= candidate
        candidate += 1 if candidate == 2 else 2
    if number > 1:
        divisors.append(number)
    return tuple(divisors)


def torus_parameter(x: int, p: int, d: int) -> tuple[int, int]:
    dx2 = d*x*x
    inverse = pow((1-dx2) % p, -1, p)
    return (1+dx2)*inverse % p, 2*x*inverse % p


def field_sqrt(value: int, p: int, nonsquare: int) -> int | None:
    if value == 0:
        return 0
    if pow(value, (p-1)//2, p) != 1:
        return None
    odd = p-1
    level = 0
    while odd % 2 == 0:
        odd //= 2
        level += 1
    generator = pow(nonsquare, odd, p)
    root, residue = pow(value, (odd+1)//2, p), pow(value, odd, p)
    while residue != 1:
        squared, index = residue, 0
        while squared != 1:
            squared = squared*squared % p
            index += 1
        adjustment = pow(generator, 1 << (level-index-1), p)
        generator = adjustment*adjustment % p
        root = root*adjustment % p
        residue = residue*generator % p
        level = index
    return root


def coset_factor_base(curve: Curve, order: int, seed: int) -> tuple[Point, ...]:
    """Walk one torus coset using a generator of its order-M subgroup."""
    p = curve.p
    if p <= 3 or prime_divisors(p) != (p,):
        raise ValueError("expected an odd prime p greater than 3")
    if (4*curve.a**3 + 27*curve.b**2) % p == 0:
        raise ValueError("expected a nonsingular short Weierstrass curve")
    if not 0 < order < p+1 or (p+1) % order:
        raise ValueError("coset order must be a positive proper divisor of p+1")
    if not 0 <= seed < p:
        raise ValueError("coset seed must be an affine coordinate in [0, p)")
    d = next(i for i in range(2, p) if pow(i, (p-1)//2, p) == p-1)
    z = torus_parameter(seed, p, d)
    if power(z, order, p, d) == (1 if order % 2 == 0 else p-1, 0):
        raise ValueError("coset contains infinity, choose a different seed")
    divisors = prime_divisors(order)
    for x in range(p):
        candidate = power(torus_parameter(x, p, d), (p+1)//order, p, d)
        if all(power(candidate, order//q, p, d) != (1, 0) for q in divisors):
            step = candidate
            break
    else:
        raise AssertionError("could not find a generator of the torus subgroup")

    points = []
    for _ in range(order):
        real, imag = z
        x = imag * pow((real+1) % p, -1, p) % p
        y = field_sqrt(curve.rhs(x), p, d)
        if y is not None:
            points.append((x, y))
            if y:
                points.append((x, -y % p))
        z = multiply(z, step, p, d)
    return tuple(sorted(points))


@dataclass
class Counts:
    translated_queries: int = 0
    candidate_probes: int = 0
    cache_hits: int = 0
    polynomial_gcds: int = 0


class PairSumSolver:
    """Store one witness per pair sum, then query A intersect (W - A)."""

    def __init__(self, curve: Curve, factor_base: tuple[Point, ...], *, cache=False):
        self.curve = curve
        if cache and set(factor_base) != {curve.neg(point) for point in factor_base}:
            raise ValueError("sign-symmetric caching requires a negation-closed factor base")
        self.factor_base = factor_base
        self.pairs = {
            curve.add(left, right): (left, right)
            for left, right in combinations_with_replacement(factor_base, 2)
        }
        self.cache = {} if cache else None
        self.counts = Counts()

    def scan(self, target: Point) -> tuple[Witness, Witness] | None:
        for point, witness in self.pairs.items():
            self.counts.candidate_probes += 1
            complement = self.curve.add(target, self.curve.neg(point))
            partner = self.pairs.get(complement)
            if partner is not None:
                return witness, partner
        return None

    def four_terms(self, target: Point) -> tuple[Witness, Witness] | None:
        self.counts.translated_queries += 1
        if self.cache is None:
            return self.scan(target)
        inverse = self.curve.neg(target)
        negate = target is not None and inverse is not None and inverse < target
        key = inverse if negate else target
        if key in self.cache:
            self.counts.cache_hits += 1
        else:
            self.cache[key] = self.scan(key)
        result = self.cache[key]
        if result is not None and negate:
            left, right = result
            return ((self.curve.neg(left[0]), self.curve.neg(left[1])),
                    (self.curve.neg(right[0]), self.curve.neg(right[1])))
        return result

    def solve(self, target: Point) -> Decomposition | None:
        for point in self.factor_base:
            result = self.four_terms(self.curve.add(target, self.curve.neg(point)))
            if result is not None:
                left, right = result
                return (point, *left, *right)
        return None


class EliminationSolver(PairSumSolver):
    """Explicit polynomial reference for the translated intersection.

    The pair-sum support is materialized. SymPy is needed only for this solver.
    """

    def __init__(self, curve: Curve, factor_base: tuple[Point, ...], *, block_size=None):
        from sympy import Poly, Symbol

        super().__init__(curve, factor_base, cache=True)
        if block_size is not None and block_size < 1:
            raise ValueError("block size must be positive")
        self.block_size = block_size
        self.x = Symbol("x")
        self.poly = lambda expr: Poly(expr, self.x, modulus=curve.p)
        self.support = {}
        for point in self.pairs:
            if point is not None:
                self.support.setdefault(point[0], []).append(point)
        self.h = self.poly(1)
        for x in self.support:
            self.h *= self.poly(self.x - x)

    def scan(self, target: Point) -> tuple[Witness, Witness] | None:
        if not self.pairs:
            return None
        if target is None:
            point, witness = next(iter(self.pairs.items()))
            return witness, self.pairs[self.curve.neg(point)]

        curve = self.curve
        w, y = target

        def match(point):
            self.counts.candidate_probes += 1
            partner = self.pairs.get(curve.add(target, curve.neg(point)))
            return None if partner is None else (self.pairs[point], partner)

        # Check infinity and x = w directly. The rational formula has a pole there.
        for point in (None, *self.support.get(w, [])):
            if point in self.pairs:
                result = match(point)
                if result is not None:
                    return result
        if self.h.degree() < 1:
            return None

        x, poly, h = self.x, self.poly, self.h
        if w in self.support:
            h = h.exquo(poly(x-w))
        if h.degree() < 1:
            return None
        d = poly((x-w)**2).rem(h)
        n = poly(x*x*w + x*w*w + curve.a*(x+w) + 2*curve.b).rem(h)
        k = poly(4*y*y*(x**3+curve.a*x+curve.b)).rem(h)

        for norm in self.norms(h, n, d, k):
            self.counts.polynomial_gcds += 1
            roots = h.gcd(norm)
            if roots.degree() > 0:
                for coordinate, points in self.support.items():
                    if roots.eval(coordinate) == 0:
                        for point in points:
                            result = match(point)
                            if result is not None:
                                return result
        return None

    def norms(self, h, n, d, k):
        if self.block_size is not None:
            # Norms multiply. One proposed output coordinate r contributes the
            # quartic (N - rD)^2 - k. Check blocks so a hit can stop the search.
            sizes = range(0, len(self.support), self.block_size)
            coordinates = tuple(self.support)
            for start in sizes:
                norm = self.poly(1)
                for coordinate in coordinates[start:start+self.block_size]:
                    difference = n - d.mul_ground(coordinate)
                    norm = (norm*(difference*difference-k)).rem(h)
                yield norm
            return

        # Evaluate D^deg(H) H((N+Y)/D) = even + odd*Y, with Y² = k.
        even, odd, d_power = self.poly(1), self.poly(0), self.poly(1)
        for coefficient in self.h.all_coeffs()[1:]:
            d_power = (d_power*d).rem(h)
            even, odd = ((n*even + k*odd + d_power.mul_ground(coefficient)).rem(h),
                         (even + n*odd).rem(h))
        yield (even*even - k*odd*odd).rem(h)


def experiment(args):
    curve = Curve(args.prime, args.a, args.b)
    base = coset_factor_base(curve, args.order, args.seed)
    if not base:
        raise ValueError("this coset has no affine points on the chosen curve")
    ys = {y*y % curve.p: y for y in range(curve.p)}
    points = [None]
    for x in range(curve.p):
        y = ys.get(curve.rhs(x))
        if y is not None:
            points.append((x, y))
            if y:
                points.append((x, -y % curve.p))
    rng = Random(args.random_seed)
    if args.trials < 1:
        raise ValueError("trials must be positive")
    if args.distinct_targets and args.trials > len(points):
        raise ValueError("distinct trials cannot exceed the number of curve points")
    sample = rng.sample if args.distinct_targets else rng.choices
    targets = sample(points, k=args.trials)
    factories = [
        ("hash", lambda: PairSumSolver(curve, base)),
        ("cached", lambda: PairSumSolver(curve, base, cache=True)),
    ]
    if args.elimination:
        factories.extend([
            ("norm-horner", lambda: EliminationSolver(curve, base)),
            ("norm-blocks", lambda: EliminationSolver(curve, base, block_size=args.block_size)),
        ])
    expected = None
    print(f"p={curve.p}  |E|={len(points)}  |F|={len(base)}  trials={len(targets)}")
    for name, construct in factories:
        start = perf_counter()
        solver = construct()
        setup = perf_counter() - start
        start = perf_counter()
        results = [solver.solve(target) for target in targets]
        elapsed = perf_counter() - start
        for target, result in zip(targets, results):
            if result is not None and curve.sum(result) != target:
                raise AssertionError(f"{name} returned an invalid witness for {target}")
        coverage = [result is not None for result in results]
        if expected is not None and coverage != expected:
            raise AssertionError(f"{name} disagrees with the baseline")
        expected = coverage
        print(f"{name:12} |A|={len(solver.pairs):5}  found={sum(coverage):5}  "
              f"setup={setup:.4f}s  search={elapsed:.4f}s  "
              f"queries={solver.counts.translated_queries}  "
              f"probes={solver.counts.candidate_probes}  hits={solver.counts.cache_hits}  "
              f"gcds={solver.counts.polynomial_gcds}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--prime", type=int, default=1019)
    parser.add_argument("--a", type=int, default=2)
    parser.add_argument("--b", type=int, default=18)
    parser.add_argument("--order", type=int, default=6)
    parser.add_argument("--seed", type=int, default=1, help="affine x in the selected coset")
    parser.add_argument("--trials", type=int, default=1000)
    parser.add_argument("--distinct-targets", action="store_true", help="sample targets without replacement")
    parser.add_argument("--random-seed", type=int, default=42)
    parser.add_argument("--elimination", action="store_true", help="include slow SymPy reference")
    parser.add_argument("--block-size", type=int, default=8)
    experiment(parser.parse_args())
