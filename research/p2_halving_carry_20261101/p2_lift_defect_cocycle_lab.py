#!/usr/bin/env python3
"""H110: canonical p^2 Hensel-section defect experiment.

The candidate is the first formal-layer coordinate of [r] applied to a
coordinate-fixed Hensel lift.  The experiment measures the complete scalar
interpolation degree, runs the frozen transfer/null controls, and only opens
the root-and-verify decoder when the preregistered polylogarithmic degree gate
is met.
"""

from __future__ import annotations

import argparse
from collections import Counter
from dataclasses import asdict, dataclass
import hashlib
import json
import math
import platform
import random
import statistics
import time
from pathlib import Path
from typing import Optional, Sequence


PREREGISTRATION_COMMIT = "83658ad"
CANONICAL_PR = "https://github.com/isogeny-labs/crypto/pull/3111"
REFERENCE_PR = "https://github.com/isogeny-labs/cryptanalysis/pull/1346"
GENERATOR_MULTIPLIERS = (1, 2, 3, 5)
CONTROL_CURVES = ("E101", "E127")
SECTION_CONTROLS = 8
PERMUTATION_CONTROLS = 8
SCRIPT_DIR = Path(__file__).resolve().parent
RESULT_PATH = SCRIPT_DIR / "results/p2-lift-defect-cocycle.json"


Point = Optional[tuple[int, int]]
Jacobian = tuple[int, int, int]


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
        out = (x3, y3)
        if not self.on_curve(out):
            raise AssertionError("base-field group addition left the curve")
        return out

    def neg(self, point: Point) -> Point:
        return None if point is None else (point[0], (-point[1]) % self.p)

    def mul(self, scalar: int, point: Point) -> Point:
        if scalar < 0:
            return self.mul(-scalar, self.neg(point))
        out = None
        addend = point
        while scalar:
            if scalar & 1:
                out = self.add(out, addend)
            addend = self.add(addend, addend)
            scalar >>= 1
        return out

    def first_point(self) -> tuple[int, int]:
        for x in range(self.p):
            rhs = (x * x * x + self.a * x + self.b) % self.p
            for y in range(1, self.p):
                if y * y % self.p == rhs:
                    point = (x, y)
                    if self.mul(self.r, point) is None:
                        return point
        raise RuntimeError(f"no order-{self.r} point on {self.name}")


CURVES = (
    Curve(101, 6, 9, 83, "E101"),
    Curve(127, 1, 7, 109, "E127"),
    Curve(149, 2, 8, 139, "E149"),
    Curve(211, 1, 1, 223, "E211"),
    Curve(283, 2, 7, 281, "E283"),
    Curve(503, 3, 8, 499, "E503"),
    Curve(907, 1, 1, 947, "E907"),
)


@dataclass
class OperationCounter:
    modular_additions: int = 0
    modular_multiplications: int = 0
    modular_inversions: int = 0
    affine_additions: int = 0
    affine_doublings: int = 0
    scalar_multiplications: int = 0
    kernel_mixed_additions: int = 0
    lift_constructions: int = 0
    formal_extractions: int = 0

    def merge(self, other: "OperationCounter") -> None:
        for key, value in asdict(other).items():
            setattr(self, key, getattr(self, key) + value)


class ModRing:
    def __init__(self, modulus: int, counter: OperationCounter) -> None:
        self.n = modulus
        self.counter = counter

    def add(self, left: int, right: int) -> int:
        self.counter.modular_additions += 1
        return (left + right) % self.n

    def sub(self, left: int, right: int) -> int:
        self.counter.modular_additions += 1
        return (left - right) % self.n

    def mul(self, left: int, right: int) -> int:
        self.counter.modular_multiplications += 1
        return left * right % self.n

    def inv(self, value: int) -> int:
        self.counter.modular_inversions += 1
        return pow(value % self.n, -1, self.n)


def deterministic_seed(*parts: object) -> int:
    payload = "|".join(map(str, parts)).encode()
    return int.from_bytes(hashlib.sha256(payload).digest()[:16], "big")


def orbit_points(curve: Curve, generator: Point) -> list[Point]:
    points: list[Point] = [None]
    current = None
    for _ in range(1, curve.r):
        current = curve.add(current, generator)
        points.append(current)
    if curve.add(points[-1], generator) is not None or len(set(points)) != curve.r:
        raise AssertionError("generator orbit failed")
    return points


def hensel_lift(curve: Curve, point: tuple[int, int], u: int,
                counter: OperationCounter) -> tuple[int, int]:
    """Coordinate section s_u(Q) on the lifted curve modulo p^2."""
    p, x, y = curve.p, point[0], point[1]
    if y % p == 0:
        raise ValueError("nonidentity point has y=0")
    n = p * p
    ring_p = ModRing(p, counter)
    discrepancy = x * x * x + curve.a * x + curve.b - y * y
    if discrepancy % p:
        raise AssertionError("base point is not on the reduced curve")
    c_q = discrepancy // p % p
    tangent = ring_p.add(ring_p.mul(3, ring_p.mul(x, x)), curve.a)
    numerator = ring_p.add(c_q, ring_p.mul(tangent, u % p))
    v = ring_p.mul(numerator, ring_p.inv(2 * y))
    counter.lift_constructions += 1
    lifted = ((x + p * (u % p)) % n, (y + p * v) % n)
    if not on_lifted_curve(curve, lifted, counter):
        raise AssertionError("Hensel lift missed the lifted curve")
    return lifted


def on_lifted_curve(curve: Curve, point: tuple[int, int],
                    counter: OperationCounter) -> bool:
    n = curve.p * curve.p
    ring = ModRing(n, counter)
    x, y = point
    lhs = ring.mul(y, y)
    x2 = ring.mul(x, x)
    x3 = ring.mul(x2, x)
    rhs = ring.add(ring.add(x3, ring.mul(curve.a, x)), curve.b)
    return lhs == rhs


def affine_add_lifted(curve: Curve, left: Point, right: Point,
                      counter: OperationCounter) -> Point:
    """Affine addition used only when the denominator is a unit modulo p."""
    if left is None:
        return right
    if right is None:
        return left
    n, p = curve.p * curve.p, curve.p
    ring = ModRing(n, counter)
    x1, y1 = left
    x2, y2 = right
    if left == right:
        denominator = 2 * y1 % n
        if denominator % p == 0:
            raise ZeroDivisionError("nonunit doubling denominator")
        numerator = ring.add(ring.mul(3, ring.mul(x1, x1)), curve.a)
        counter.affine_doublings += 1
    else:
        denominator = ring.sub(x2, x1)
        if denominator % p == 0:
            raise ZeroDivisionError("nonunit addition denominator")
        numerator = ring.sub(y2, y1)
        counter.affine_additions += 1
    slope = ring.mul(numerator, ring.inv(denominator))
    x3 = ring.sub(ring.sub(ring.mul(slope, slope), x1), x2)
    y3 = ring.sub(ring.mul(slope, ring.sub(x1, x3)), y1)
    out = (x3, y3)
    if not on_lifted_curve(curve, out, counter):
        raise AssertionError("affine lifted addition left the curve")
    return out


def affine_mul_lifted(curve: Curve, scalar: int, point: tuple[int, int],
                      counter: OperationCounter) -> Point:
    counter.scalar_multiplications += 1
    out: Point = None
    addend: Point = point
    while scalar:
        if scalar & 1:
            out = affine_add_lifted(curve, out, addend, counter)
        scalar >>= 1
        if scalar:
            addend = affine_add_lifted(curve, addend, addend, counter)
    return out


def mixed_kernel_sum(curve: Curve, left: tuple[int, int],
                     right: tuple[int, int], counter: OperationCounter) -> Jacobian:
    """Polynomial mixed-add formula for two points whose reductions sum to O."""
    n = curve.p * curve.p
    ring = ModRing(n, counter)
    x1, y1 = left
    x2, y2 = right
    h = ring.sub(x2, x1)
    two_h = ring.mul(2, h)
    i = ring.mul(two_h, two_h)
    j = ring.mul(h, i)
    rr = ring.mul(2, ring.sub(y2, y1))
    v = ring.mul(x1, i)
    x3 = ring.sub(ring.sub(ring.mul(rr, rr), j), ring.mul(2, v))
    y3 = ring.sub(ring.mul(rr, ring.sub(v, x3)), ring.mul(2, ring.mul(y1, j)))
    z3 = two_h
    counter.kernel_mixed_additions += 1
    out = (x3, y3, z3)
    if not on_projective_curve(curve, out, counter):
        raise AssertionError("mixed kernel sum left the projective curve")
    return out


def on_projective_curve(curve: Curve, point: Jacobian,
                        counter: OperationCounter) -> bool:
    n = curve.p * curve.p
    ring = ModRing(n, counter)
    x, y, z = point
    y2 = ring.mul(y, y)
    x2 = ring.mul(x, x)
    x3 = ring.mul(x2, x)
    z2 = ring.mul(z, z)
    z4 = ring.mul(z2, z2)
    z6 = ring.mul(z4, z2)
    rhs = ring.add(ring.add(x3, ring.mul(curve.a, ring.mul(x, z4))),
                   ring.mul(curve.b, z6))
    return y2 == rhs


def r_multiple_kernel(curve: Curve, lifted: tuple[int, int], split: int,
                      counter: OperationCounter) -> Jacobian:
    if not 1 <= split < curve.r:
        raise ValueError("invalid split")
    left = affine_mul_lifted(curve, split, lifted, counter)
    right = affine_mul_lifted(curve, curve.r - split, lifted, counter)
    if left is None or right is None:
        raise AssertionError("proper split unexpectedly reached identity")
    if curve.add((left[0] % curve.p, left[1] % curve.p),
                 (right[0] % curve.p, right[1] % curve.p)) is not None:
        raise AssertionError("split reductions do not sum to identity")
    return mixed_kernel_sum(curve, left, right, counter)


def formal_tau(curve: Curve, kernel: Jacobian,
               counter: OperationCounter) -> int:
    p, n = curve.p, curve.p * curve.p
    ring = ModRing(n, counter)
    x, y, z = kernel
    if z % p != 0:
        raise AssertionError("[r] lift is not in the reduction kernel")
    if y % p == 0:
        raise AssertionError("projective Y is not a unit")
    t = ring.mul((-ring.mul(x, z)) % n, ring.inv(y))
    if t % p:
        raise AssertionError("formal parameter is not divisible by p")
    counter.formal_extractions += 1
    return t // p % p


def tau_for_section(curve: Curve, point: tuple[int, int], u: int,
                    split: int, counter: OperationCounter) -> int:
    lifted = hensel_lift(curve, point, u, counter)
    kernel = r_multiple_kernel(curve, lifted, split, counter)
    return formal_tau(curve, kernel, counter)


def section_record(curve: Curve, point: tuple[int, int]) -> tuple[int, dict, OperationCounter]:
    counter = OperationCounter()
    split_half = (curve.r - 1) // 2
    split_third = max(1, curve.r // 3)
    tau0 = tau_for_section(curve, point, 0, split_half, counter)
    tau0_independent = tau_for_section(curve, point, 0, split_third, counter)
    if tau0 != tau0_independent:
        raise AssertionError("independent split formal coordinates disagree")
    tau1 = tau_for_section(curve, point, 1, split_half, counter)
    slope = (tau1 - tau0) % curve.p
    if slope == 0:
        raise AssertionError("section defect has zero first-layer slope")
    torsion_u = -tau0 * pow(slope, -1, curve.p) % curve.p
    torsion_tau = tau_for_section(curve, point, torsion_u, split_half, counter)
    if torsion_tau != 0:
        raise AssertionError("solved torsion lift did not kill tau")
    record = {
        "tau": tau0,
        "tau_u1": tau1,
        "section_slope": slope,
        "torsion_u": torsion_u,
        "torsion_tau": torsion_tau,
    }
    return tau0, record, counter


def finite_difference_profile(values: Sequence[int], prime: int) -> dict:
    if not values:
        raise ValueError("empty interpolation word")
    row = [value % prime for value in values]
    first_differences = [row[0]]
    operations = 0
    degree = len(row) - 1
    for order in range(1, len(row)):
        row = [(row[index + 1] - row[index]) % prime
               for index in range(len(row) - 1)]
        operations += len(row)
        first_differences.append(row[0])
        if all(value == 0 for value in row):
            degree = order - 1
            break
    return {
        "degree": degree,
        "newton_first_differences": first_differences[:degree + 1],
        "subtractions": operations,
    }


def newton_evaluate(first_differences: Sequence[int], scalar: int, prime: int) -> int:
    """Evaluate sum_j Delta^j f(1) * binom(scalar-1,j) over F_prime."""
    total = 0
    binomial = 1
    for index, coefficient in enumerate(first_differences):
        if index:
            binomial = binomial * (scalar - index) % prime
            binomial = binomial * pow(index, -1, prime) % prime
        total = (total + coefficient * binomial) % prime
    return total


def word_summary(values: Sequence[int], interpolation_prime: int,
                 degree_gate: int) -> dict:
    profile = finite_difference_profile(values, interpolation_prime)
    counts = Counter(values)
    digest = hashlib.sha256(json.dumps(list(values), separators=(",", ":")).encode()).hexdigest()
    prefix = profile["newton_first_differences"][:degree_gate + 1]
    prefix_matches = sum(
        newton_evaluate(prefix, scalar, interpolation_prime) == value % interpolation_prime
        for scalar, value in enumerate(values, start=1)
    )
    return {
        "degree": profile["degree"],
        "degree_gate": degree_gate,
        "gate_passed": profile["degree"] <= degree_gate,
        "interpolation_subtractions": profile["subtractions"],
        "newton_prefix": prefix,
        "prefix_matches": prefix_matches,
        "targets": len(values),
        "distinct_outputs": len(counts),
        "maximum_collision_bucket": max(counts.values()),
        "word_sha256": digest,
    }


def decode_word(curve: Curve, generator: tuple[int, int], points: Sequence[Point],
                values: Sequence[int], summary: dict) -> dict | None:
    if not summary["gate_passed"]:
        return None
    coefficients = summary["newton_prefix"][:summary["degree"] + 1]
    recovered = 0
    polynomial_evaluations = 0
    ec_validations = 0
    max_roots = 0
    failures = []
    for target_scalar, (target, value) in enumerate(zip(points[1:], values), start=1):
        roots = []
        for candidate in range(1, curve.r):
            polynomial_evaluations += 1
            if newton_evaluate(coefficients, candidate, curve.r) == value % curve.r:
                roots.append(candidate)
        max_roots = max(max_roots, len(roots))
        valid = []
        for candidate in roots:
            ec_validations += 1
            if curve.mul(candidate, generator) == target:
                valid.append(candidate)
        if valid == [target_scalar]:
            recovered += 1
        else:
            failures.append({"target_scalar": target_scalar, "roots": roots, "valid": valid})
    return {
        "recovered": recovered,
        "targets": curve.r - 1,
        "failures": failures,
        "polynomial_evaluations_toy_route": polynomial_evaluations,
        "ec_validations": ec_validations,
        "maximum_roots": max_roots,
        "asymptotic_route": "factor the degree-bounded polynomial, then EC-verify roots",
    }


def evaluate_generator(curve: Curve, generator: tuple[int, int], multiplier: int) -> dict:
    points = orbit_points(curve, generator)
    values = []
    target_records = []
    total_counter = OperationCounter()
    started = time.perf_counter_ns()
    for scalar, point in enumerate(points[1:], start=1):
        if point is None:
            raise AssertionError("nonzero scalar reached identity")
        value, controls, counter = section_record(curve, point)
        values.append(value)
        target_records.append({"scalar": scalar, "point": list(point), **controls})
        total_counter.merge(counter)
    evaluation_elapsed = time.perf_counter_ns() - started
    gate = math.ceil(math.log2(curve.r)) ** 2
    interpolation_started = time.perf_counter_ns()
    summary = word_summary(values, curve.r, gate)
    interpolation_elapsed = time.perf_counter_ns() - interpolation_started
    decoder = decode_word(curve, generator, points, values, summary)
    return {
        "generator_multiplier": multiplier,
        "generator": list(generator),
        "word": values,
        "target_records": target_records,
        "interpolation": summary,
        "decoder": decoder,
        "cost": {
            **asdict(total_counter),
            "evaluation_elapsed_ns": evaluation_elapsed,
            "interpolation_elapsed_ns": interpolation_elapsed,
            "stored_output_elements": len(values),
            "advice_coefficients_if_gate_passes": min(summary["degree"] + 1, gate + 1),
            "generic_bsgs_group_operations_reference": math.ceil(math.sqrt(curve.r)),
            "generic_rho_ideal_group_operations_reference": math.sqrt(math.pi * curve.r / 2),
        },
    }


def random_section_word(curve: Curve, points: Sequence[Point], replicate: int) -> list[int]:
    values = []
    split = (curve.r - 1) // 2
    for scalar, point in enumerate(points[1:], start=1):
        if point is None:
            raise AssertionError
        u = deterministic_seed("H110", curve.name, "section", replicate, scalar,
                               point[0], point[1]) % curve.p
        values.append(tau_for_section(curve, point, u, split, OperationCounter()))
    return values


def matched_controls(curve: Curve, canonical_values: Sequence[int]) -> dict:
    generator = curve.first_point()
    points = orbit_points(curve, generator)
    gate = math.ceil(math.log2(curve.r)) ** 2
    section = []
    permutation = []
    for replicate in range(SECTION_CONTROLS):
        word = random_section_word(curve, points, replicate)
        section.append({"replicate": replicate, **word_summary(word, curve.r, gate)})
    for replicate in range(PERMUTATION_CONTROLS):
        word = list(canonical_values)
        random.Random(deterministic_seed("H110", curve.name, "permutation", replicate)).shuffle(word)
        permutation.append({"replicate": replicate, **word_summary(word, curve.r, gate)})
    return {"random_sections": section, "matched_permutations": permutation}


def coordinate_isomorphism_control(curve: Curve) -> dict:
    lam = 2
    p = curve.p
    transformed = Curve(
        p,
        curve.a * pow(lam, 4, p) % p,
        curve.b * pow(lam, 6, p) % p,
        curve.r,
        f"{curve.name}-iso-{lam}",
    )
    generator = curve.first_point()
    transformed_generator = (
        generator[0] * pow(lam, 2, p) % p,
        generator[1] * pow(lam, 3, p) % p,
    )
    if not transformed.on_curve(transformed_generator):
        raise AssertionError("coordinate-isomorphic generator missed transformed curve")
    result = evaluate_generator(transformed, transformed_generator, 1)
    return {
        "lambda": lam,
        "transformed_parameters": {"p": p, "a": transformed.a, "b": transformed.b, "r": transformed.r},
        "interpolation": result["interpolation"],
        "cost": result["cost"],
    }


def scientific_payload_sha256(result: dict) -> str:
    def scrub(value):
        if isinstance(value, dict):
            return {
                key: scrub(item)
                for key, item in value.items()
                if not key.endswith("elapsed_ns")
                and key not in {"scientific_payload_sha256", "environment"}
            }
        if isinstance(value, list):
            return [scrub(item) for item in value]
        return value
    payload = json.dumps(scrub(result), sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(payload).hexdigest()


def run_experiment(curves: Sequence[Curve] = CURVES,
                   multipliers: Sequence[int] = GENERATOR_MULTIPLIERS,
                   control_replicates: bool = True) -> dict:
    started = time.perf_counter_ns()
    curve_results = []
    controls = {}
    isomorphisms = {}
    all_rows = []
    total_targets = 0
    for curve in curves:
        if curve.r == curve.p:
            raise AssertionError("H110 excludes anomalous r=p curves")
        base_generator = curve.first_point()
        generators = []
        for multiplier in multipliers:
            generator = curve.mul(multiplier, base_generator)
            if generator is None:
                raise AssertionError("generator relabeling reached identity")
            row = evaluate_generator(curve, generator, multiplier)
            generators.append(row)
            all_rows.append(row)
            total_targets += curve.r - 1
        curve_results.append({
            "curve": curve.name,
            "parameters": {"p": curve.p, "a": curve.a, "b": curve.b, "r": curve.r},
            "canonical_generator": list(base_generator),
            "generators": generators,
        })
        if control_replicates and curve.name in CONTROL_CURVES:
            controls[curve.name] = matched_controls(curve, generators[0]["word"])
            isomorphisms[curve.name] = coordinate_isomorphism_control(curve)

    degrees = [row["interpolation"]["degree"] for row in all_rows]
    gates = [row["interpolation"]["degree_gate"] for row in all_rows]
    passed = [row for row in all_rows if row["interpolation"]["gate_passed"]]
    decoded = [row["decoder"] for row in passed if row["decoder"] is not None]
    result = {
        "experiment": "H110 canonical p2 lift defect cocycle",
        "date": "2026-11-01",
        "preregistration_commit": PREREGISTRATION_COMMIT,
        "pull_requests": {"canonical": CANONICAL_PR, "reference": REFERENCE_PR},
        "protocol": {
            "generator_multipliers": list(multipliers),
            "degree_gate": "ceil(log2(r))^2",
            "section_controls_per_primary_curve": SECTION_CONTROLS if control_replicates else 0,
            "permutation_controls_per_primary_curve": PERMUTATION_CONTROLS if control_replicates else 0,
            "index_calculus_used": False,
            "pollard_rho_used_as_candidate": False,
        },
        "curves": curve_results,
        "matched_controls": controls,
        "coordinate_isomorphism_controls": isomorphisms,
        "summary": {
            "curve_rows": len(all_rows),
            "targets": total_targets,
            "degree_minimum": min(degrees),
            "degree_median": statistics.median(degrees),
            "degree_maximum": max(degrees),
            "gate_minimum": min(gates),
            "gate_maximum": max(gates),
            "rows_passing_degree_gate": len(passed),
            "rows_decoded": len(decoded),
            "decoded_targets": sum(item["recovered"] for item in decoded),
            "all_transfer_checks_passed": True,
        },
        "environment": {
            "python": platform.python_version(),
            "platform": platform.platform(),
            "processor": platform.processor(),
        },
        "total_elapsed_ns": time.perf_counter_ns() - started,
    }
    result["scientific_payload_sha256"] = scientific_payload_sha256(result)
    return result


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=RESULT_PATH)
    parser.add_argument("--quick", action="store_true", help="one curve/generator and no 32-control panel")
    args = parser.parse_args()
    result = run_experiment(
        curves=CURVES[:1] if args.quick else CURVES,
        multipliers=GENERATOR_MULTIPLIERS[:1] if args.quick else GENERATOR_MULTIPLIERS,
        control_replicates=not args.quick,
    )
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({
        "output": str(args.output),
        "scientific_payload_sha256": result["scientific_payload_sha256"],
        **result["summary"],
    }, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
