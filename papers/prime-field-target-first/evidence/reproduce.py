#!/usr/bin/env python3
"""Reproduce exact four-position summation-polynomial sanity checks.

Run with Python 3.10+ and no third-party dependencies. The default experiment
checks every target against brute force using a target-first group-law split.
It also checks a smaller batch using an independent projective S3 and gcd
calculation and a CRT-component batch norm. None denotes the identity point
or its projective abscissa.
"""

from argparse import ArgumentParser
from collections import defaultdict
from dataclasses import dataclass
from itertools import product
from math import isqrt


Point = tuple[int, int] | None
Abscissa = int | None
Pair = tuple[int, int]
Witness = tuple[int, int, int, int]
HalfTable = dict[Point, list[Pair]]
HalfAbscissaTable = dict[Abscissa, list[Point]]
HalfMatches = set[tuple[Point, Point]]


@dataclass(frozen=True)
class Curve:
    p: int
    a: int
    b: int
    order: int
    generator: tuple[int, int]

    def add(self, left: Point, right: Point) -> Point:
        if left is None:
            return right
        if right is None:
            return left
        x1, y1 = left
        x2, y2 = right
        if x1 == x2:
            if (y1 + y2) % self.p == 0:
                return None
            slope = (3 * x1 * x1 + self.a) * pow(2 * y1, -1, self.p)
        else:
            slope = (y2 - y1) * pow(x2 - x1, -1, self.p)
        slope %= self.p
        x3 = (slope * slope - x1 - x2) % self.p
        y3 = (slope * (x1 - x3) - y1) % self.p
        return x3, y3

    def negate(self, point: Point) -> Point:
        return None if point is None else (point[0], -point[1] % self.p)

    def multiply_generator(self, scalar: int) -> Point:
        result = None
        point = self.generator
        while scalar:
            if scalar & 1:
                result = self.add(result, point)
            point = self.add(point, point)
            scalar >>= 1
        return result

    def generator_multiples(self) -> list[Point]:
        assert all(self.order % j for j in range(2, isqrt(self.order) + 1))
        assert self.order not in (self.p, self.p + 1)
        assert (4 * self.a**3 + 27 * self.b**2) % self.p
        assert (self.generator[1] ** 2 - self.generator[0] ** 3
                - self.a * self.generator[0] - self.b) % self.p == 0
        points = [None]
        for _ in range(1, self.order):
            points.append(self.add(points[-1], self.generator))
        assert len(set(points)) == self.order
        assert self.add(points[-1], self.generator) is None
        squares = {y * y % self.p for y in range(1, self.p)}
        rhs = ((x**3 + self.a * x + self.b) % self.p for x in range(self.p))
        assert 1 + sum(1 if z == 0 else 2 * (z in squares) for z in rhs) == self.order
        return points

    def s3_in_output(self, first: Abscissa, second: Abscissa) -> list[int]:
        """Return [c0,c1,c2] for c0 W² + c1 ZW + c2 Z² on [Z:W]."""
        if first is None and second is None:
            return [1, 0, 0]
        if first is None or second is None:
            finite = second if first is None else first
            return [finite * finite % self.p, -2 * finite % self.p, 1]
        sum_x = first + second
        product_x = first * second
        return [
            ((product_x - self.a) ** 2 - 4 * self.b * sum_x) % self.p,
            -2 * (sum_x * (product_x + self.a) + 2 * self.b) % self.p,
            (first - second) ** 2 % self.p,
        ]


def abscissa(point: Point) -> Abscissa:
    return None if point is None else point[0]


def trim(coefficients: list[int]) -> list[int]:
    while coefficients and coefficients[-1] == 0:
        coefficients.pop()
    return coefficients


def polynomial_gcd(left: list[int], right: list[int], p: int) -> list[int]:
    left, right = trim(left.copy()), trim(right.copy())
    while right:
        inverse_leading = pow(right[-1], -1, p)
        while len(left) >= len(right):
            shift = len(left) - len(right)
            scale = left[-1] * inverse_leading % p
            for index, coefficient in enumerate(right):
                left[shift + index] = (left[shift + index] - scale * coefficient) % p
            trim(left)
        left, right = right, left
    inverse_leading = pow(left[-1], -1, p)
    return [coefficient * inverse_leading % p for coefficient in left]


def evaluate(coefficients: list[int], x: int, p: int) -> int:
    value = 0
    for coefficient in reversed(coefficients):
        value = (value * x + coefficient) % p
    return value


def root_polynomial(roots: list[int], p: int) -> list[int]:
    coefficients = [1]
    for x in roots:
        expanded = [0] + coefficients
        for index, coefficient in enumerate(coefficients):
            expanded[index] = (expanded[index] - x * coefficient) % p
        coefficients = expanded
    return coefficients


def choose_digits(radix: int, order: int) -> list[tuple[int, ...]]:
    assert 2 * radix < order
    digits = (*range(-radix, 0), *range(1, radix + 1))
    return [tuple((d * radix**position) % order for d in digits) for position in range(4)]


def half_tables(curve: Curve, digits: list[tuple[int, ...]]) -> tuple[HalfTable, HalfAbscissaTable]:
    digit_points = {digit: curve.multiply_generator(digit) for block in digits for digit in block}
    by_point: HalfTable = defaultdict(list)
    for first, second in product(*digits):
        point = curve.add(digit_points[first], digit_points[second])
        by_point[point].append((first, second))
    by_x: HalfAbscissaTable = defaultdict(list)
    for point in by_point:
        by_x[abscissa(point)].append(point)
    return dict(by_point), dict(by_x)


def brute_force(digits: list[tuple[int, ...]], order: int) -> dict[int, set[Witness]]:
    decompositions = defaultdict(set)
    for witness in product(*digits):
        decompositions[sum(witness) % order].add(witness)
    return decompositions


def extract_witnesses(left: HalfTable, right: HalfTable, matches: HalfMatches) -> set[Witness]:
    return {
        (*left_pair, *right_pair)
        for left_point, right_point in matches
        for left_pair, right_pair in product(left[left_point], right[right_point])
    }


def target_first_batch(curve: Curve, targets: list[Point], left: HalfTable,
                       right: HalfTable) -> list[HalfMatches]:
    """Impose L + H = R for each given R, without forming a full join."""
    rows = []
    for target in targets:
        matches = set()
        for left_point in left:
            right_point = curve.add(target, curve.negate(left_point))
            if right_point in right:
                matches.add((left_point, right_point))
        rows.append(matches)
    return rows


def projective_s3_matches(curve: Curve, target: Point, left_by_x: HalfAbscissaTable,
                          right_by_x: HalfAbscissaTable, right_roots: list[int]) -> HalfMatches:
    """Eliminate right abscissa after fixing R, then restore signed witnesses."""
    matches = set()
    quotient_pairs = set()
    for left_x, left_points in left_by_x.items():
        polynomial = curve.s3_in_output(left_x, abscissa(target))
        common = polynomial_gcd(right_roots, polynomial, curve.p)
        possible_right = {
            curve.add(target, curve.negate(left_point)) for left_point in left_points
        }
        finite_roots = {
            right_point[0] for right_point in possible_right
            if right_point is not None and evaluate(common, right_point[0], curve.p) == 0
        }
        assert len(finite_roots) == len(common) - 1
        infinite_root = polynomial[2] == 0 and None in right_by_x
        assert infinite_root == (None in possible_right and None in right_by_x)
        for right_x in finite_roots | ({None} if infinite_root else set()):
            assert right_x in right_by_x
            quotient_pairs.add((left_x, right_x))
            for left_point, right_point in product(left_points, right_by_x[right_x]):
                if curve.add(left_point, right_point) == target:
                    matches.add((left_point, right_point))
    assert quotient_pairs == {(abscissa(left), abscissa(right)) for left, right in matches}
    return matches



def check_batch_norm(curve: Curve, left_by_x: HalfAbscissaTable,
                     right_by_x: HalfAbscissaTable,
                     projections: dict[Abscissa, set[Abscissa]]) -> int:
    """Check the norm's zero set in the CRT evaluation basis of the batch."""
    finite_right = [x for x in right_by_x if x is not None]
    factor_evaluations = 0
    for target_x, expected_left in projections.items():
        matching_left = set()
        for left_x in left_by_x:
            if target_x is None:
                matches = left_x in right_by_x
            elif left_x is None:
                matches = target_x in right_by_x
            else:
                s3 = curve.s3_in_output(left_x, target_x)
                norm = 1
                for right_x in finite_right:
                    norm = norm * evaluate(s3, right_x, curve.p) % curve.p
                factor_evaluations += len(finite_right)
                matches = norm == 0 or (None in right_by_x and s3[2] == 0)
            if matches:
                matching_left.add(left_x)
        assert matching_left == expected_left
    return factor_evaluations


def experiment(curve: Curve, radix: int, all_gcd_targets: bool) -> None:
    points = curve.generator_multiples()
    digits = choose_digits(radix, curve.order)
    left, left_by_x = half_tables(curve, digits[:2])
    right, right_by_x = half_tables(curve, digits[2:])
    expected = brute_force(digits, curve.order)
    right_roots = root_polynomial(sorted(x for x in right_by_x if x is not None), curve.p)
    matches_by_target = target_first_batch(curve, points, left, right)
    quotient_lengths = []
    witness_counts = []
    for scalar, matches in enumerate(matches_by_target):
        witnesses = extract_witnesses(left, right, matches)
        assert witnesses == expected[scalar]
        witness_counts.append(len(witnesses))
        quotient_lengths.append(len({(abscissa(l), abscissa(r)) for l, r in matches}))

    sampled = {0, 1, 2, curve.order - 1, curve.order - 2, curve.order // 2, curve.order // 3}
    first_miss = next((index for index, count in enumerate(witness_counts) if not count), None)
    if first_miss is not None:
        sampled.add(first_miss)
    gcd_targets = range(curve.order) if all_gcd_targets else sorted(sampled)
    gcd_quotients = {}
    batch_projections = {}
    for scalar in gcd_targets:
        target = points[scalar]
        matches = projective_s3_matches(curve, target, left_by_x, right_by_x, right_roots)
        assert matches == matches_by_target[scalar]
        x_target = abscissa(target)
        quotient_length = quotient_lengths[scalar]
        assert gcd_quotients.get(x_target, quotient_length) == quotient_length
        gcd_quotients[x_target] = quotient_length
        projection = {abscissa(left_point) for left_point, _ in matches_by_target[scalar]}
        assert batch_projections.get(x_target, projection) == projection
        batch_projections[x_target] = projection
    norm_factors = check_batch_norm(curve, left_by_x, right_by_x, batch_projections)

    target_support = sum(count > 0 for count in witness_counts[1:])
    assert target_support % 2 == 0
    print(
        f"p={curve.p}, N={curve.order}, radix={radix}, "
        f"y²=x³+{curve.a}x+{curve.b}, P={curve.generator}"
    )
    print(f"  batched CRT norm checked the projection with {norm_factors} finite factor evaluations")
    print(f"  signed tuples in brute force = {(2 * radix)**4}")
    print(f"  distinct signed half sums = {len(left)}, {len(right)}")
    print(f"  distinct projective half abscissas = {len(left_by_x)}, {len(right_by_x)}")
    print(f"  nonidentity abscissa support = {target_support // 2}/{(curve.order - 1) // 2}")
    print(f"  max distinct signed half-pair matches per target = {max(map(len, matches_by_target))}")
    print(f"  max ordered signed tuple witnesses per target = {max(witness_counts)}")
    print(f"  max reduced specialized half-abscissa quotient dimension = {max(quotient_lengths)}")
    print(f"  target-first batch checked all {curve.order} points with {curve.order * len(left)} residual lookups")
    print(
        f"  projective S3/gcd checked {len(gcd_targets)} points with {len(gcd_quotients)} distinct abscissas; "
        f"reduced batch quotient dimension = {sum(gcd_quotients.values())}"
    )


CASES = (
    (Curve(101, 6, 9, 83, (0, 3)), 2),
    (Curve(101, 6, 9, 83, (0, 3)), 4),
    (Curve(509, 2, 10, 479, (3, 68)), 5),
    (Curve(1009, 4, 10, 1069, (0, 162)), 6),
    (Curve(5003, 2, 13, 5011, (1, 4)), 9),
)


def main() -> None:
    parser = ArgumentParser(description=__doc__)
    parser.add_argument("--field", type=int, choices=sorted({curve.p for curve, _ in CASES}))
    parser.add_argument(
        "--all-gcd-targets", action="store_true",
        help="also run the slower fixed-target polynomial verification for every group point",
    )
    options = parser.parse_args()
    for curve, radix in CASES:
        if options.field is None or options.field == curve.p:
            experiment(curve, radix, options.all_gcd_targets)


if __name__ == "__main__":
    main()
