#!/usr/bin/env python3
"""H109: torsion-local division-polynomial derivative character oracle.

The experiment is deliberately exhaustive on the frozen toy cohort.  It uses
memoized dual-number division-polynomial evaluation, checks the local torsion
identity, completely enumerates the constant-size character formula family,
and runs symmetry-matched label controls with the identical search budget.
"""

from __future__ import annotations

import argparse
from collections import defaultdict
from dataclasses import dataclass
import hashlib
import itertools
import json
import math
import platform
import shutil
import statistics
import subprocess
import sys
import time
from functools import lru_cache
from pathlib import Path
from typing import Iterable, Optional, Sequence


PREREGISTRATION_COMMIT = "4594f3c"
CANONICAL_PR = "https://github.com/isogeny-labs/crypto/pull/3102"
REFERENCE_PR = "https://github.com/isogeny-labs/cryptanalysis/pull/1342"
GENERATOR_MULTIPLIERS = (1, 2, 3, 5)
CONTROL_REPLICATES = 16
CONTROL_CURVES = ("E101", "E127")
SCRIPT_DIR = Path(__file__).resolve().parent
LEAN_SOURCE = SCRIPT_DIR / "formal/H109TorsionDerivative.lean"
RESULT_PATH = SCRIPT_DIR / "results/torsion-division-derivative.json"
BASELINE_DIR = (
    SCRIPT_DIR / "results"
    if (SCRIPT_DIR / "results/two-power-character-module.json").is_file()
    else SCRIPT_DIR / "baselines"
)


Point = Optional[tuple[int, int]]


@dataclass(frozen=True)
class Curve:
    p: int
    a: int
    b: int
    r: int
    name: str

    def on_curve(self, point: Point) -> bool:
        if point is None:
            return True
        x, y = point
        return (y * y - x * x * x - self.a * x - self.b) % self.p == 0

    def add(self, left: Point, right: Point) -> Point:
        if left is None:
            return right
        if right is None:
            return left
        x1, y1 = left
        x2, y2 = right
        if x1 == x2 and (y1 + y2) % self.p == 0:
            return None
        if left == right:
            slope = (3 * x1 * x1 + self.a) * pow(2 * y1, -1, self.p)
        else:
            slope = (y2 - y1) * pow(x2 - x1, -1, self.p)
        slope %= self.p
        x3 = (slope * slope - x1 - x2) % self.p
        y3 = (slope * (x1 - x3) - y1) % self.p
        result = (x3, y3)
        if not self.on_curve(result):
            raise AssertionError("group addition left the curve")
        return result

    def neg(self, point: Point) -> Point:
        return None if point is None else (point[0], (-point[1]) % self.p)

    def sub(self, left: Point, right: Point) -> Point:
        return self.add(left, self.neg(right))

    def mul(self, scalar: int, point: Point) -> Point:
        if scalar < 0:
            return self.mul(-scalar, self.neg(point))
        output = None
        addend = point
        while scalar:
            if scalar & 1:
                output = self.add(output, addend)
            addend = self.add(addend, addend)
            scalar >>= 1
        return output

    def first_point(self) -> Point:
        for x in range(self.p):
            rhs = (x * x * x + self.a * x + self.b) % self.p
            for y in range(1, self.p):
                if y * y % self.p == rhs:
                    point = (x, y)
                    if self.mul(self.r, point) is None:
                        return point
        raise RuntimeError(f"no point of order {self.r} on {self.name}")


CURVES = (
    Curve(101, 6, 9, 83, "E101"),
    Curve(127, 1, 7, 109, "E127"),
    Curve(149, 2, 8, 139, "E149"),
    Curve(211, 1, 1, 223, "E211"),
    Curve(283, 2, 7, 281, "E283"),
    Curve(503, 3, 8, 499, "E503"),
    Curve(907, 1, 1, 947, "E907"),
)


def prime_factors(value: int) -> tuple[int, ...]:
    factors = []
    divisor = 2
    remaining = value
    while divisor * divisor <= remaining:
        if remaining % divisor == 0:
            factors.append(divisor)
            while remaining % divisor == 0:
                remaining //= divisor
        divisor += 1 if divisor == 2 else 2
    if remaining > 1:
        factors.append(remaining)
    return tuple(factors)


def least_primitive_root(prime: int) -> int:
    factors = prime_factors(prime - 1)
    for candidate in range(2, prime):
        if all(pow(candidate, (prime - 1) // factor, prime) != 1 for factor in factors):
            return candidate
    raise ValueError(f"no primitive root modulo {prime}")


def orbit_points(curve: Curve, generator: Point) -> list[Point]:
    points = [None]
    current = None
    for _ in range(1, curve.r):
        current = curve.add(current, generator)
        points.append(current)
    if curve.add(points[-1], generator) is not None or len(set(points)) != curve.r:
        raise AssertionError("generator orbit failed its order/injectivity check")
    return points


def deterministic_seed(*parts: object) -> int:
    payload = "|".join(map(str, parts)).encode()
    return int.from_bytes(hashlib.sha256(payload).digest()[:16], "big")


def division_polynomial_values(curve: Curve, point: Point, maximum: int) -> list[int]:
    """Independent ordinary-value recurrence used as a full frozen control."""
    if point is None:
        raise ValueError("division-polynomial evaluation requires affine point")
    x, y = point
    p, a, b = curve.p, curve.a % curve.p, curve.b % curve.p
    values = [0] * (maximum + 1)
    if maximum >= 1:
        values[1] = 1
    if maximum >= 2:
        values[2] = 2 * y % p
    if maximum >= 3:
        values[3] = (3 * pow(x, 4, p) + 6 * a * x * x + 12 * b * x - a * a) % p
    if maximum >= 4:
        bracket = (
            pow(x, 6, p) + 5 * a * pow(x, 4, p) + 20 * b * pow(x, 3, p)
            - 5 * a * a * x * x - 4 * a * b * x - 8 * b * b - a * a * a
        ) % p
        values[4] = 4 * y * bracket % p
    inverse_psi_two = pow(values[2], -1, p) if maximum >= 5 else 0
    for index in range(5, maximum + 1):
        if index & 1:
            half = (index - 1) // 2
            values[index] = (
                values[half + 2] * pow(values[half], 3, p)
                - values[half - 1] * pow(values[half + 1], 3, p)
            ) % p
        else:
            half = index // 2
            values[index] = values[half] * inverse_psi_two * (
                values[half + 2] * pow(values[half - 1], 2, p)
                - values[half - 2] * pow(values[half + 1], 2, p)
            ) % p
    return values


def character_phase_context(prime: int) -> dict:
    modulus = (prime - 1) & -(prime - 1)
    primitive_root = least_primitive_root(prime)
    root = pow(primitive_root, (prime - 1) // modulus, prime)
    phase_table = {pow(root, exponent, prime): exponent for exponent in range(modulus)}
    if len(phase_table) != modulus:
        raise AssertionError("phase root does not have declared order")
    return {
        "modulus": modulus,
        "half_phase": modulus // 2,
        "primitive_root": primitive_root,
        "phase_root": root,
        "phase_table": phase_table,
    }


def character_phase(value: int, prime: int, context: dict) -> int:
    value %= prime
    if value == 0:
        raise ValueError("character phase is undefined at zero")
    residue = pow(value, (prime - 1) // context["modulus"], prime)
    return context["phase_table"][residue]


def parity_target_phases(order: int, modulus: int) -> list[int]:
    half = modulus // 2
    return [half if scalar & 1 else 0 for scalar in range(1, order)]


def matched_negation_control(order: int, seed: int) -> int:
    import random

    rng = random.Random(seed)
    word = 0
    for scalar in range(1, (order + 1) // 2):
        bit = rng.randrange(2)
        if bit:
            word |= 1 << (scalar - 1)
        if bit ^ 1:
            word |= 1 << (order - scalar - 1)
    return word


def matched_phase_control(order: int, modulus: int, seed: int) -> list[int]:
    bits = matched_negation_control(order, seed)
    half = modulus // 2
    return [
        half * ((bits >> (scalar - 1)) & 1)
        for scalar in range(1, order)
    ]


@dataclass(frozen=True)
class Dual:
    value: int
    derivative: int


class CountedDualField:
    """First-order dual arithmetic over F_p with explicit operation counts."""

    def __init__(self, prime: int) -> None:
        self.p = prime
        self.cost: defaultdict[str, int] = defaultdict(int)

    def const(self, value: int) -> Dual:
        return Dual(value % self.p, 0)

    def add(self, left: Dual, right: Dual) -> Dual:
        self.cost["field_additions"] += 2
        return Dual(
            (left.value + right.value) % self.p,
            (left.derivative + right.derivative) % self.p,
        )

    def sub(self, left: Dual, right: Dual) -> Dual:
        self.cost["field_additions"] += 2
        return Dual(
            (left.value - right.value) % self.p,
            (left.derivative - right.derivative) % self.p,
        )

    def neg(self, value: Dual) -> Dual:
        self.cost["field_negations"] += 2
        return Dual((-value.value) % self.p, (-value.derivative) % self.p)

    def mul(self, left: Dual, right: Dual) -> Dual:
        self.cost["field_multiplications"] += 3
        self.cost["field_additions"] += 1
        return Dual(
            left.value * right.value % self.p,
            (left.derivative * right.value + left.value * right.derivative)
            % self.p,
        )

    def scale(self, value: Dual, scalar: int) -> Dual:
        self.cost["field_multiplications"] += 2
        scalar %= self.p
        return Dual(
            value.value * scalar % self.p,
            value.derivative * scalar % self.p,
        )

    def inv(self, value: Dual) -> Dual:
        if value.value == 0:
            raise ZeroDivisionError("dual inverse of zero")
        inverse = pow(value.value, -1, self.p)
        self.cost["field_inversions"] += 1
        self.cost["field_multiplications"] += 2
        self.cost["field_negations"] += 1
        return Dual(
            inverse,
            (-value.derivative * inverse * inverse) % self.p,
        )

    def div(self, numerator: Dual, denominator: Dual) -> Dual:
        return self.mul(numerator, self.inv(denominator))

    def pow(self, value: Dual, exponent: int) -> Dual:
        if exponent < 0:
            return self.pow(self.inv(value), -exponent)
        output = self.const(1)
        base = value
        remaining = exponent
        while remaining:
            if remaining & 1:
                output = self.mul(output, base)
            remaining >>= 1
            if remaining:
                base = self.mul(base, base)
        return output


class DivisionPolynomialDualEvaluator:
    """Memoized O(polylog n)-node evaluator for psi_n and D psi_n."""

    def __init__(self, curve: Curve, point: Point) -> None:
        if point is None:
            raise ValueError("division-polynomial derivative needs affine point")
        self.curve = curve
        self.point = point
        self.field = CountedDualField(curve.p)
        self.cache_hits = 0
        self.recurrence_nodes = 0
        self._cache: dict[int, Dual] = {}
        self._initialize_base()

    def _initialize_base(self) -> None:
        p, a, b = self.curve.p, self.curve.a, self.curve.b
        x_value, y_value = self.point  # type: ignore[misc]
        # Coordinate seeds implement D(x)=2y and D(y)=3x^2+a.
        self.field.cost["field_multiplications"] += 3
        self.field.cost["field_additions"] += 1
        x = Dual(x_value, 2 * y_value % p)
        y = Dual(y_value, (3 * x_value * x_value + a) % p)
        x2 = self.field.pow(x, 2)
        x3 = self.field.mul(x2, x)
        x4 = self.field.mul(x2, x2)
        x6 = self.field.mul(x3, x3)

        psi3 = self.field.add(
            self.field.add(
                self.field.scale(x4, 3),
                self.field.scale(x2, 6 * a),
            ),
            self.field.add(
                self.field.scale(x, 12 * b),
                self.field.const(-(a * a)),
            ),
        )
        bracket = self.field.add(
            self.field.add(
                self.field.add(x6, self.field.scale(x4, 5 * a)),
                self.field.scale(x3, 20 * b),
            ),
            self.field.add(
                self.field.add(
                    self.field.scale(x2, -5 * a * a),
                    self.field.scale(x, -4 * a * b),
                ),
                self.field.const(-8 * b * b - a * a * a),
            ),
        )
        self._cache = {
            0: self.field.const(0),
            1: self.field.const(1),
            2: self.field.scale(y, 2),
            3: psi3,
            4: self.field.mul(self.field.scale(y, 4), bracket),
        }

    def psi(self, index: int) -> Dual:
        if index < 0:
            raise ValueError("division-polynomial index must be nonnegative")
        if index in self._cache:
            self.cache_hits += 1
            return self._cache[index]
        self.recurrence_nodes += 1
        f = self.field
        if index & 1:
            half = (index - 1) // 2
            value = f.sub(
                f.mul(self.psi(half + 2), f.pow(self.psi(half), 3)),
                f.mul(self.psi(half - 1), f.pow(self.psi(half + 1), 3)),
            )
        else:
            half = index // 2
            bracket = f.sub(
                f.mul(self.psi(half + 2), f.pow(self.psi(half - 1), 2)),
                f.mul(self.psi(half - 2), f.pow(self.psi(half + 1), 2)),
            )
            value = f.div(f.mul(self.psi(half), bracket), self.psi(2))
        self._cache[index] = value
        return value

    def cost_snapshot(self) -> dict[str, int]:
        return {
            **dict(self.field.cost),
            "recurrence_nodes": self.recurrence_nodes,
            "cache_hits": self.cache_hits,
            "cached_duals": len(self._cache),
        }


@lru_cache(maxsize=1)
def _symbolic_small_expressions() -> tuple[tuple[object, ...], str]:
    """Construct independent symbolic psi_0..psi_8 and their D derivatives."""
    import sympy as sp

    x, y, a, b = sp.symbols("x y a b")
    values: list[object] = [sp.Integer(0)] * 9
    values[1] = sp.Integer(1)
    values[2] = 2 * y
    values[3] = 3 * x**4 + 6 * a * x**2 + 12 * b * x - a**2
    values[4] = 4 * y * (
        x**6
        + 5 * a * x**4
        + 20 * b * x**3
        - 5 * a**2 * x**2
        - 4 * a * b * x
        - 8 * b**2
        - a**3
    )
    for index in range(5, 9):
        if index & 1:
            half = (index - 1) // 2
            expression = (
                values[half + 2] * values[half] ** 3
                - values[half - 1] * values[half + 1] ** 3
            )
        else:
            half = index // 2
            expression = values[half] * (
                values[half + 2] * values[half - 1] ** 2
                - values[half - 2] * values[half + 1] ** 2
            ) / values[2]
        values[index] = sp.cancel(expression)
        numerator, denominator = sp.fraction(values[index])
        if denominator != 1:
            raise AssertionError(f"psi_{index} did not cancel to a polynomial")
        values[index] = sp.expand(numerator)
    pairs = []
    for expression in values:
        derivative = sp.expand(
            sp.diff(expression, x) * (2 * y)
            + sp.diff(expression, y) * (3 * x**2 + a)
        )
        pairs.append((expression, derivative))
    serial = "\n".join(sp.srepr(item) for pair in pairs for item in pair)
    return tuple(pairs), hashlib.sha256(serial.encode()).hexdigest()


@lru_cache(maxsize=None)
def _small_polynomial_terms(
    prime: int, curve_a: int, curve_b: int
) -> tuple[tuple[tuple[tuple[int, int, int], ...], ...], str]:
    import sympy as sp

    x, y, a, b = sp.symbols("x y a b")
    pairs, digest = _symbolic_small_expressions()
    compiled = []
    for pair in pairs:
        compiled_pair = []
        for expression in pair:
            specialized = sp.Poly(
                sp.expand(expression.subs({a: curve_a, b: curve_b})), x, y
            )
            terms = tuple(
                (int(powers[0]), int(powers[1]), int(coefficient) % prime)
                for powers, coefficient in specialized.terms()
            )
            compiled_pair.append(terms)
        compiled.append(tuple(compiled_pair))
    return tuple(compiled), digest


def _evaluate_terms(terms: Sequence[tuple[int, int, int]], point: Point, p: int) -> int:
    if point is None:
        raise ValueError("polynomial control needs affine point")
    x, y = point
    return sum(
        coefficient * pow(x, x_power, p) * pow(y, y_power, p)
        for x_power, y_power, coefficient in terms
    ) % p


def direct_symbolic_small_values(curve: Curve, point: Point) -> tuple[list[Dual], str]:
    compiled, digest = _small_polynomial_terms(curve.p, curve.a, curve.b)
    return [
        Dual(
            _evaluate_terms(pair[0], point, curve.p),
            _evaluate_terms(pair[1], point, curve.p),
        )
        for pair in compiled
    ], digest


def formula_search(
    feature_phases: Sequence[Sequence[int]],
    target: Sequence[int],
    modulus: int,
) -> dict:
    if len(feature_phases) != len(target):
        raise ValueError("feature and target row counts differ")
    if any(len(row) != 5 for row in feature_phases):
        raise ValueError("H109 formulas require exactly five coefficients")
    best_matches = -1
    best: list[tuple[int, ...]] = []
    exact: list[tuple[int, ...]] = []
    started = time.perf_counter_ns()
    candidates = 0
    for coefficients in itertools.product(range(modulus), repeat=5):
        candidates += 1
        matches = sum(
            sum(coefficient * phase for coefficient, phase in zip(coefficients, row))
            % modulus
            == expected
            for row, expected in zip(feature_phases, target)
        )
        if matches > best_matches:
            best_matches = matches
            best = [coefficients]
        elif matches == best_matches:
            best.append(coefficients)
        if matches == len(target):
            exact.append(coefficients)
    elapsed = time.perf_counter_ns() - started
    return {
        "formula_candidates": candidates,
        "coefficient_modulus": modulus,
        "best_matches": best_matches,
        "targets": len(target),
        "best_match_rate": best_matches / len(target) if target else 1.0,
        "best_coefficients": [list(row) for row in best],
        "exact_coefficients": [list(row) for row in exact],
        "exact_formula_count": len(exact),
        "search_modular_multiply_adds": candidates * len(target) * 5,
        "search_elapsed_ns": elapsed,
    }


def independently_verify_formulas(
    curve: Curve,
    input_rows: Sequence[Sequence[int]],
    coefficients: Sequence[Sequence[int]],
    target: Sequence[int],
    context: dict,
) -> dict:
    failures = []
    observed_hashes = []
    for formula_index, formula in enumerate(coefficients):
        observed = []
        for values in input_rows:
            product = pow(context["primitive_root"], formula[0], curve.p)
            for value, exponent in zip(values, formula[1:]):
                product = product * pow(value, exponent, curve.p) % curve.p
            observed.append(character_phase(product, curve.p, context))
        if observed != list(target):
            failures.append(formula_index)
        observed_hashes.append(hashlib.sha256(bytes(observed)).hexdigest())
    return {
        "formulas_checked": len(coefficients),
        "failures": failures,
        "all_exact": not failures,
        "observed_phase_sha256": observed_hashes,
    }


def _merge_cost(target: dict[str, int], source: dict[str, int]) -> None:
    for key, value in source.items():
        target[key] = target.get(key, 0) + value


def evaluate_oracle_inputs(
    curve: Curve, point: Point
) -> tuple[list[int], list[int], dict, dict]:
    evaluator = DivisionPolynomialDualEvaluator(curve, point)
    minus = evaluator.psi(curve.r - 1)
    central = evaluator.psi(curve.r)
    plus = evaluator.psi(curve.r + 1)
    if point is None:
        raise ValueError("oracle input requires affine point")
    _, y = point
    identity_residue = (
        central.derivative * central.derivative
        + (curve.r * curve.r % curve.p) * minus.value * plus.value
    ) % curve.p
    raw = [central.derivative, minus.value, plus.value, y]
    checks = {
        "psi_r_zero": central.value == 0,
        "local_identity": identity_residue == 0,
        "local_identity_residue": identity_residue,
        "nonzero_inputs": all(value % curve.p for value in raw),
    }
    return raw, [minus.value, central.value, plus.value], evaluator.cost_snapshot(), checks


def decode_all_targets(
    curve: Curve,
    generator: Point,
    coefficients: Sequence[int],
    context: dict,
) -> dict:
    points = orbit_points(curve, generator)
    inverse_two = pow(2, -1, curve.r)
    failures = []
    total_cost: dict[str, int] = {}
    bit_steps = 0
    started = time.perf_counter_ns()
    for declared in range(1, curve.r):
        current = points[declared]
        scalar = declared
        reconstructed = 0
        place = 1
        while current is not None:
            raw, _, cost, checks = evaluate_oracle_inputs(curve, current)
            _merge_cost(total_cost, cost)
            if not all((checks["psi_r_zero"], checks["local_identity"], checks["nonzero_inputs"])):
                failures.append([declared, scalar, "input-check"])
                break
            product = pow(context["primitive_root"], coefficients[0], curve.p)
            for value, exponent in zip(raw, coefficients[1:]):
                product = product * pow(value, exponent, curve.p) % curve.p
            phase = character_phase(product, curve.p, context)
            if phase not in (0, context["half_phase"]):
                failures.append([declared, scalar, "nonbinary-phase"])
                break
            bit = int(phase == context["half_phase"])
            if bit != (scalar & 1):
                failures.append([declared, scalar, "wrong-bit"])
                break
            if bit:
                current = curve.sub(current, generator)
            current = curve.mul(inverse_two, current)
            scalar = (scalar - bit) // 2
            reconstructed += bit * place
            place *= 2
            bit_steps += 1
        if (
            scalar != 0
            or reconstructed != declared
            or curve.mul(reconstructed, generator) != points[declared]
        ):
            failures.append([declared, scalar, "reconstruction"])
    return {
        "targets": curve.r - 1,
        "bit_steps": bit_steps,
        "failures": failures,
        "all_recovered": not failures,
        "cost": total_cost,
        "elapsed_ns": time.perf_counter_ns() - started,
    }


def analyze_generator(
    curve: Curve, canonical_generator: Point, multiplier: int
) -> tuple[dict, list[list[int]]]:
    generator = curve.mul(multiplier, canonical_generator)
    points = orbit_points(curve, generator)
    context = character_phase_context(curve.p)
    phase_rows: list[list[int]] = []
    input_rows: list[list[int]] = []
    target_records = []
    aggregate_cost: dict[str, int] = {}
    max_cached = 0
    psi_zero_failures = []
    identity_failures = []
    nonzero_failures = []
    sequential_value_failures = []
    symbolic_failures = []
    symbolic_digest = None
    candidate_ns = 0
    control_ns = 0

    for scalar in range(1, curve.r):
        point = points[scalar]
        started = time.perf_counter_ns()
        raw, adjacent, cost, checks = evaluate_oracle_inputs(curve, point)
        candidate_ns += time.perf_counter_ns() - started
        _merge_cost(aggregate_cost, cost)
        max_cached = max(max_cached, cost["cached_duals"])
        if not checks["psi_r_zero"]:
            psi_zero_failures.append(scalar)
        if not checks["local_identity"]:
            identity_failures.append([scalar, checks["local_identity_residue"]])
        if not checks["nonzero_inputs"]:
            nonzero_failures.append(scalar)

        started = time.perf_counter_ns()
        sequential = division_polynomial_values(curve, point, curve.r + 1)
        if adjacent != sequential[curve.r - 1 : curve.r + 2]:
            sequential_value_failures.append(scalar)
        direct_small, digest = direct_symbolic_small_values(curve, point)
        symbolic_digest = digest
        small_evaluator = DivisionPolynomialDualEvaluator(curve, point)
        for index in range(2, 9):
            if small_evaluator.psi(index) != direct_small[index]:
                symbolic_failures.append([scalar, index])
        control_ns += time.perf_counter_ns() - started

        if checks["nonzero_inputs"]:
            phases = [1] + [
                character_phase(value, curve.p, context) for value in raw
            ]
        else:
            phases = [0] * 5
        phase_rows.append(phases)
        input_rows.append(raw)
        target_records.append(
            [scalar, *raw, *phases[1:], checks["local_identity_residue"]]
        )

    target = parity_target_phases(curve.r, context["modulus"])
    search = formula_search(phase_rows, target, context["modulus"])
    verification = independently_verify_formulas(
        curve,
        input_rows,
        search["exact_coefficients"],
        target,
        context,
    )
    decoder = None
    if search["exact_coefficients"]:
        decoder = decode_all_targets(
            curve, generator, search["exact_coefficients"][0], context
        )
    target_payload = json.dumps(target_records, separators=(",", ":")).encode()
    target_count = curve.r - 1
    row = {
        "generator_multiplier": multiplier,
        "generator": list(generator),
        "character": {
            key: value for key, value in context.items() if key != "phase_table"
        },
        "checks": {
            "psi_r_zero_failures": psi_zero_failures,
            "local_identity_failures": identity_failures,
            "nonzero_input_failures": nonzero_failures,
            "sequential_value_control_failures": sequential_value_failures,
            "symbolic_small_derivative_failures": symbolic_failures,
            "symbolic_expression_sha256": symbolic_digest,
        },
        "cost": {
            "candidate_evaluation": aggregate_cost,
            "mean_recurrence_nodes_per_target":
                aggregate_cost.get("recurrence_nodes", 0) / target_count,
            "mean_field_multiplications_per_target":
                aggregate_cost.get("field_multiplications", 0) / target_count,
            "mean_field_inversions_per_target":
                aggregate_cost.get("field_inversions", 0) / target_count,
            "mean_candidate_evaluation_ns_per_target": candidate_ns / target_count,
            "mean_recurrence_nodes_per_log2_r":
                aggregate_cost.get("recurrence_nodes", 0)
                / target_count
                / math.log2(curve.r),
            "candidate_elapsed_ns": candidate_ns,
            "independent_controls_elapsed_ns": control_ns,
            "maximum_cached_duals_per_target": max_cached,
            "stored_input_field_elements": (curve.r - 1) * 4,
            "stored_phase_elements": (curve.r - 1) * 5,
            "advice_coefficients": 5,
            "generic_bsgs_group_operations_reference": math.ceil(math.sqrt(curve.r)),
            "generic_rho_ideal_group_operations_reference": math.sqrt(
                math.pi * curve.r / 4
            ),
        },
        "search": search,
        "independent_exact_formula_verification": verification,
        "decoder": decoder,
        "target_record_schema": [
            "scalar",
            "delta_r",
            "psi_r_minus_1",
            "psi_r_plus_1",
            "y",
            "phase_delta_r",
            "phase_psi_r_minus_1",
            "phase_psi_r_plus_1",
            "phase_y",
            "local_identity_residue",
        ],
        "target_records": target_records,
        "target_records_sha256": hashlib.sha256(target_payload).hexdigest(),
    }
    return row, phase_rows


def analyze_controls(
    curve: Curve,
    feature_rows: Sequence[Sequence[int]],
    replicates: int = CONTROL_REPLICATES,
) -> dict:
    context = character_phase_context(curve.p)
    rows = []
    started = time.perf_counter_ns()
    for replicate in range(replicates):
        seed = deterministic_seed(
            "H109", curve.name, "symmetry-matched-phase-control", replicate
        )
        target = matched_phase_control(
            curve.r, context["modulus"], seed
        )
        search = formula_search(feature_rows, target, context["modulus"])
        rows.append(
            {
                "replicate": replicate,
                "seed": seed,
                "label_sha256": hashlib.sha256(bytes(target)).hexdigest(),
                "best_matches": search["best_matches"],
                "best_match_rate": search["best_match_rate"],
                "exact_formula_count": search["exact_formula_count"],
                "best_coefficients": search["best_coefficients"],
                "formula_candidates": search["formula_candidates"],
            }
        )
    return {
        "curve": curve.name,
        "replicates": replicates,
        "character_modulus": context["modulus"],
        "same_formula_budget_as_candidate": True,
        "rows": rows,
        "elapsed_ns": time.perf_counter_ns() - started,
    }


def verify_lean(check: bool) -> dict:
    payload = LEAN_SOURCE.read_bytes()
    digest = hashlib.sha256(payload).hexdigest()
    lowered = payload.lower()
    clean = b"sorry" not in lowered and b"admit" not in lowered
    if not check:
        return {
            "checked": False,
            "passed": False,
            "reason": "Lean check disabled",
            "source_sha256": digest,
            "sorry_free_source_scan": clean,
        }
    lake = shutil.which("lake")
    if lake is None:
        return {
            "checked": True,
            "passed": False,
            "reason": "lake executable not found",
            "source_sha256": digest,
            "sorry_free_source_scan": clean,
        }
    process = subprocess.run(
        [lake, "build", "H109TorsionDerivative"],
        text=True,
        capture_output=True,
        check=False,
    )
    return {
        "checked": True,
        "passed": process.returncode == 0 and clean,
        "exit_code": process.returncode,
        "stdout": process.stdout,
        "stderr": process.stderr,
        "source_sha256": digest,
        "sorry_free_source_scan": clean,
    }


def _load_baseline(path: Path) -> dict:
    if not path.is_file():
        return {"path": path.name, "available": False}
    payload = path.read_bytes()
    parsed = json.loads(payload)
    return {
        "path": path.name,
        "available": True,
        "sha256": hashlib.sha256(payload).hexdigest(),
        "summary": parsed.get("summary"),
    }


def scientific_payload_sha256(result: dict) -> str:
    """Hash deterministic scientific content while excluding diagnostic clocks."""

    def scrub(value):
        if isinstance(value, dict):
            return {
                key: scrub(item)
                for key, item in value.items()
                if not key.endswith("_ns")
                and "_ns_" not in key
                and key not in {"scientific_payload_sha256", "environment"}
            }
        if isinstance(value, list):
            return [scrub(item) for item in value]
        return value

    payload = json.dumps(
        scrub(result), sort_keys=True, separators=(",", ":")
    ).encode()
    return hashlib.sha256(payload).hexdigest()


def summarize(curves: Sequence[dict], controls: Sequence[dict], lean: dict) -> dict:
    rows = [row for curve in curves for row in curve["generators"]]
    exact_rows = [row for row in rows if row["search"]["exact_formula_count"]]
    common_by_curve = {}
    for curve in curves:
        sets = [
            {tuple(value) for value in row["search"]["exact_coefficients"]}
            for row in curve["generators"]
        ]
        common = set.intersection(*sets) if sets else set()
        common_by_curve[curve["curve"]] = [list(value) for value in sorted(common)]
    control_exact = sum(
        row["exact_formula_count"]
        for control in controls
        for row in control["rows"]
    )
    all_checks = all(
        not any(
            row["checks"][key]
            for key in (
                "psi_r_zero_failures",
                "local_identity_failures",
                "nonzero_input_failures",
                "sequential_value_control_failures",
                "symbolic_small_derivative_failures",
            )
        )
        for row in rows
    )
    all_decoders = bool(exact_rows) and all(
        row["decoder"] is not None and row["decoder"]["all_recovered"]
        for row in exact_rows
    )
    generator_stable_curves = sum(bool(values) for values in common_by_curve.values())
    candidate_rates = [row["search"]["best_match_rate"] for row in rows]
    control_rates = [
        row["best_match_rate"] for control in controls for row in control["rows"]
    ]
    control_calibration = {}
    for control in controls:
        curve_row = next(
            curve for curve in curves if curve["curve"] == control["curve"]
        )
        primary = next(
            row
            for row in curve_row["generators"]
            if row["generator_multiplier"] == 1
        )
        rates = [row["best_match_rate"] for row in control["rows"]]
        candidate_rate = primary["search"]["best_match_rate"]
        at_least_candidate = sum(rate >= candidate_rate for rate in rates)
        control_calibration[control["curve"]] = {
            "candidate_primary_best_match_rate": candidate_rate,
            "control_mean_best_match_rate": statistics.mean(rates),
            "control_sample_standard_deviation": (
                statistics.stdev(rates) if len(rates) > 1 else 0.0
            ),
            "controls_at_least_candidate": at_least_candidate,
            "replicates": len(rates),
            "plus_one_empirical_tail_probability":
                (at_least_candidate + 1) / (len(rates) + 1),
        }
    total_candidate_cost: dict[str, int] = {}
    for row in rows:
        _merge_cost(total_candidate_cost, row["cost"]["candidate_evaluation"])
    result = {
        "curves": len(curves),
        "generator_rows": len(rows),
        "targets": sum(curve["parameters"]["r"] - 1 for curve in curves)
        * (len(curves[0]["generators"]) if curves else 0),
        "formula_candidates_total": sum(
            row["search"]["formula_candidates"] for row in rows
        ),
        "exact_rows": len(exact_rows),
        "exact_formula_count_total": sum(
            row["search"]["exact_formula_count"] for row in rows
        ),
        "generator_stable_curves": generator_stable_curves,
        "common_exact_coefficients_by_curve": common_by_curve,
        "matched_control_replicates": sum(control["replicates"] for control in controls),
        "matched_control_exact_formula_count": control_exact,
        "candidate_best_match_rate": {
            "minimum": min(candidate_rates) if candidate_rates else None,
            "median": statistics.median(candidate_rates) if candidate_rates else None,
            "maximum": max(candidate_rates) if candidate_rates else None,
            "rows": candidate_rates,
        },
        "control_best_match_rate": {
            "minimum": min(control_rates) if control_rates else None,
            "median": statistics.median(control_rates) if control_rates else None,
            "maximum": max(control_rates) if control_rates else None,
            "rows": control_rates,
        },
        "primary_candidate_vs_matched_controls": control_calibration,
        "total_candidate_evaluation_cost": total_candidate_cost,
        "total_candidate_evaluation_ns": sum(
            row["cost"]["candidate_elapsed_ns"] for row in rows
        ),
        "total_formula_search_ns": sum(
            row["search"]["search_elapsed_ns"] for row in rows
        ),
        "total_independent_control_ns": sum(
            row["cost"]["independent_controls_elapsed_ns"] for row in rows
        ),
        "all_algebraic_and_independent_checks_passed": all_checks,
        "decoder_rows_executed": sum(row["decoder"] is not None for row in rows),
        "all_executed_decoders_recovered": all_decoders,
        "lean_certificate_gate_passed": lean["passed"],
        "generic_exact_generator_stable_gate_passed": generator_stable_curves == len(curves),
        "control_separation_gate_passed": bool(exact_rows) and control_exact == 0,
        "subexponential_runtime_gate_passed": False,
        "novelty_established": False,
        "all_requested_gates_passed": False,
    }
    return result


def run_experiment(
    curves: Sequence[Curve] = CURVES,
    generator_multipliers: Iterable[int] = GENERATOR_MULTIPLIERS,
    control_replicates: int = CONTROL_REPLICATES,
    check_lean: bool = True,
) -> dict:
    multipliers = tuple(generator_multipliers)
    started = time.perf_counter_ns()
    lean = verify_lean(check_lean)
    curve_rows = []
    controls = []
    for curve in curves:
        canonical = curve.first_point()
        generators = []
        primary_phases = None
        for multiplier in multipliers:
            row, phases = analyze_generator(curve, canonical, multiplier)
            generators.append(row)
            if multiplier == 1:
                primary_phases = phases
        curve_rows.append(
            {
                "curve": curve.name,
                "parameters": {
                    "p": curve.p,
                    "a": curve.a,
                    "b": curve.b,
                    "r": curve.r,
                },
                "canonical_generator": list(canonical),
                "generators": generators,
            }
        )
        if curve.name in CONTROL_CURVES and primary_phases is not None:
            controls.append(
                analyze_controls(curve, primary_phases, control_replicates)
            )
    summary = summarize(curve_rows, controls, lean)
    result = {
        "experiment": "H109-torsion-division-derivative",
        "date": "2026-10-31",
        "preregistration_commit": PREREGISTRATION_COMMIT,
        "pull_requests": {"canonical": CANONICAL_PR, "reference": REFERENCE_PR},
        "requested_target": "classical L_r(1/2,O(1)) or better prime-field ECDLP",
        "protocol": {
            "generator_multipliers": list(multipliers),
            "control_curves": list(CONTROL_CURVES),
            "control_replicates_per_curve": control_replicates,
            "coefficient_family": "(Z/MZ)^5, completely enumerated",
            "feature_order": [
                "primitive-root constant",
                "D psi_r",
                "psi_(r-1)",
                "psi_(r+1)",
                "y",
            ],
            "pollard_rho_used_as_candidate": False,
            "index_calculus_used": False,
        },
        "environment": {
            "python": sys.version,
            "platform": platform.platform(),
            "machine": platform.machine(),
        },
        "paired_baselines": {
            "H107": _load_baseline(BASELINE_DIR / "two-power-character-module.json"),
            "H108": _load_baseline(BASELINE_DIR / "extension-consecutive-eds-phase.json"),
        },
        "lean_certificate": lean,
        "curves": curve_rows,
        "matched_controls": controls,
        "summary": summary,
        "total_elapsed_ns": time.perf_counter_ns() - started,
    }
    result["scientific_payload_sha256"] = scientific_payload_sha256(result)
    return result


def write_result(result: dict, path: Path = RESULT_PATH) -> str:
    payload = (json.dumps(result, indent=2, sort_keys=True) + "\n").encode()
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(payload)
    return hashlib.sha256(payload).hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=RESULT_PATH)
    parser.add_argument("--quick", action="store_true")
    parser.add_argument("--skip-lean", action="store_true")
    args = parser.parse_args()
    result = run_experiment(
        curves=CURVES[:1] if args.quick else CURVES,
        generator_multipliers=(1,) if args.quick else GENERATOR_MULTIPLIERS,
        check_lean=not args.skip_lean,
    )
    digest = write_result(result, args.output)
    print(json.dumps(result["summary"], indent=2, sort_keys=True))
    print(f"sha256={digest}")


if __name__ == "__main__":
    main()
