#!/usr/bin/env python3
"""Independent exhaustive replay for H116-POINTWISE-CHARPOLY-001."""

from __future__ import annotations

import csv
import hashlib
import itertools
import json
import sys
import time
from pathlib import Path


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def polynomial_remainder(numerator: list[int], denominator: list[int], p: int) -> list[int]:
    value = [coefficient % p for coefficient in numerator]
    while len(value) > 1 and value[-1] == 0:
        value.pop()
    while len(value) >= len(denominator) and value != [0]:
        shift = len(value) - len(denominator)
        factor = value[-1] * pow(denominator[-1], p - 2, p) % p
        for index, coefficient in enumerate(denominator):
            value[shift + index] = (value[shift + index] - factor * coefficient) % p
        while len(value) > 1 and value[-1] == 0:
            value.pop()
    return value


def irreducible_by_trial_division(polynomial: list[int], p: int) -> bool:
    degree = len(polynomial) - 1
    if degree < 1 or polynomial[-1] % p != 1:
        return False
    for divisor_degree in range(1, degree // 2 + 1):
        for low in itertools.product(range(p), repeat=divisor_degree):
            divisor = [*low, 1]
            if polynomial_remainder(polynomial, divisor, p) == [0]:
                return False
    return True


def first_irreducible(p: int, degree: int) -> list[int]:
    for code in range(p**degree):
        remainder = code
        coefficients = []
        for index in range(degree):
            place = p ** (degree - 1 - index)
            coefficients.append(remainder // place)
            remainder %= place
        candidate = [*coefficients, 1]
        if irreducible_by_trial_division(candidate, p):
            return candidate
    raise ValueError(f"no irreducible polynomial for p={p}, degree={degree}")


class Field:
    def __init__(self, p: int, modulus: list[int]):
        self.p = p
        self.modulus = tuple(coefficient % p for coefficient in modulus)
        self.degree = len(modulus) - 1
        self.size = p**self.degree
        require(self.degree >= 1 and self.modulus[-1] == 1, "bad field modulus")
        self.zero = (0,) * self.degree
        self.one = (1,) + (0,) * (self.degree - 1)

    def element(self, value: int) -> tuple[int, ...]:
        require(0 <= value < self.size, "field element index out of range")
        coefficients = []
        for _ in range(self.degree):
            coefficients.append(value % self.p)
            value //= self.p
        return tuple(coefficients)

    def base(self, value: int) -> tuple[int, ...]:
        return (value % self.p,) + (0,) * (self.degree - 1)

    def add(self, left: tuple[int, ...], right: tuple[int, ...]) -> tuple[int, ...]:
        return tuple((a + b) % self.p for a, b in zip(left, right, strict=True))

    def neg(self, value: tuple[int, ...]) -> tuple[int, ...]:
        return tuple((-coefficient) % self.p for coefficient in value)

    def sub(self, left: tuple[int, ...], right: tuple[int, ...]) -> tuple[int, ...]:
        return self.add(left, self.neg(right))

    def mul(self, left: tuple[int, ...], right: tuple[int, ...]) -> tuple[int, ...]:
        product = [0] * (2 * self.degree - 1)
        for i, a in enumerate(left):
            for j, b in enumerate(right):
                product[i + j] = (product[i + j] + a * b) % self.p
        for power in range(2 * self.degree - 2, self.degree - 1, -1):
            leading = product[power]
            for index in range(self.degree):
                destination = power - self.degree + index
                product[destination] = (
                    product[destination] - leading * self.modulus[index]
                ) % self.p
        return tuple(product[: self.degree])

    def power(self, base: tuple[int, ...], exponent: int) -> tuple[int, ...]:
        result = self.one
        while exponent:
            if exponent & 1:
                result = self.mul(result, base)
            base = self.mul(base, base)
            exponent >>= 1
        return result

    def inverse(self, value: tuple[int, ...]) -> tuple[int, ...]:
        require(value != self.zero, "division by zero")
        return self.power(value, self.size - 2)

    def divide(self, numerator: tuple[int, ...], denominator: tuple[int, ...]) -> tuple[int, ...]:
        return self.mul(numerator, self.inverse(denominator))

    def frobenius(self, value: tuple[int, ...]) -> tuple[int, ...]:
        return self.power(value, self.p)


Point = None | tuple[tuple[int, ...], tuple[int, ...]]


class Curve:
    def __init__(self, field: Field, a: int, b: int):
        self.field = field
        self.a = field.base(a)
        self.b = field.base(b)

    def on_curve(self, point: Point) -> bool:
        if point is None:
            return True
        x, y = point
        field = self.field
        return field.mul(y, y) == field.add(
            field.add(field.mul(field.mul(x, x), x), field.mul(self.a, x)), self.b
        )

    def negate(self, point: Point) -> Point:
        if point is None:
            return None
        return point[0], self.field.neg(point[1])

    def add(self, left: Point, right: Point) -> Point:
        if left is None:
            return right
        if right is None:
            return left
        field = self.field
        x1, y1 = left
        x2, y2 = right
        if x1 == x2 and field.add(y1, y2) == field.zero:
            return None
        if left == right:
            numerator = field.add(field.mul(field.base(3), field.mul(x1, x1)), self.a)
            denominator = field.mul(field.base(2), y1)
        else:
            numerator = field.sub(y2, y1)
            denominator = field.sub(x2, x1)
        slope = field.divide(numerator, denominator)
        x3 = field.sub(field.sub(field.mul(slope, slope), x1), x2)
        y3 = field.sub(field.mul(slope, field.sub(x1, x3)), y1)
        result = (x3, y3)
        require(self.on_curve(result), "point addition left curve")
        return result

    def subtract(self, left: Point, right: Point) -> Point:
        return self.add(left, self.negate(right))

    def scalar(self, multiplier: int, point: Point) -> Point:
        if multiplier < 0:
            return self.negate(self.scalar(-multiplier, point))
        result = None
        addend = point
        while multiplier:
            if multiplier & 1:
                result = self.add(result, addend)
            addend = self.add(addend, addend)
            multiplier >>= 1
        return result

    def frobenius(self, point: Point) -> Point:
        if point is None:
            return None
        return self.field.frobenius(point[0]), self.field.frobenius(point[1])

    def frobenius_power(self, point: Point, exponent: int) -> Point:
        result = point
        for _ in range(exponent):
            result = self.frobenius(result)
        return result

    def enumerate_points(self) -> list[Point]:
        field = self.field
        square_roots: dict[tuple[int, ...], list[tuple[int, ...]]] = {}
        for index in range(field.size):
            y = field.element(index)
            square_roots.setdefault(field.mul(y, y), []).append(y)
        points: list[Point] = [None]
        for index in range(field.size):
            x = field.element(index)
            rhs = field.add(
                field.add(field.mul(field.mul(x, x), x), field.mul(self.a, x)), self.b
            )
            points.extend((x, y) for y in square_roots.get(rhs, []))
        require(len(points) == len(set(points)), "duplicate points")
        require(all(self.on_curve(point) for point in points), "enumerated off-curve point")
        return points


def trace_group_order(p: int, trace: int, degree: int) -> int:
    previous_previous = 2
    previous = trace
    if degree == 1:
        power_sum = previous
    else:
        for _ in range(2, degree + 1):
            previous_previous, previous = previous, trace * previous - p * previous_previous
        power_sum = previous
    return p**degree + 1 - power_sum


def compute_cell(cell: dict[str, int | str]) -> dict[str, int | str]:
    p = int(cell["p"])
    a = int(cell["a"])
    b = int(cell["b"])
    degree = int(cell["degree"])
    require((4 * a**3 + 27 * b**2) % p != 0, "singular curve")
    base_curve = Curve(Field(p, [0, 1]), a, b)
    base_count = len(base_curve.enumerate_points())
    trace = p + 1 - base_count
    modulus = first_irreducible(p, degree)
    curve = Curve(Field(p, modulus), a, b)
    points = curve.enumerate_points()
    pi_order_failures = 0
    charpoly_failures = 0
    v_failures = 0
    wrong_trace_rejections = 0
    wrong_norm_rejections = 0
    missing_inverse_rejections = 0
    for point in points:
        pi = curve.frobenius(point)
        pi_squared = curve.frobenius(pi)
        if curve.frobenius_power(point, degree) != point:
            pi_order_failures += 1
        characteristic = curve.add(
            curve.subtract(pi_squared, curve.scalar(trace, pi)), curve.scalar(p, point)
        )
        if characteristic is not None:
            charpoly_failures += 1
        expected_v = curve.subtract(curve.scalar(trace, point), pi)
        pi_inverse = curve.frobenius_power(point, degree - 1)
        if curve.scalar(p, pi_inverse) != expected_v:
            v_failures += 1
        wrong_trace = curve.add(
            curve.subtract(pi_squared, curve.scalar(trace + 1, pi)), curve.scalar(p, point)
        )
        if wrong_trace is not None:
            wrong_trace_rejections += 1
        if curve.scalar(p + 1, pi_inverse) != expected_v:
            wrong_norm_rejections += 1
        if curve.scalar(p, point) != expected_v:
            missing_inverse_rejections += 1
    return {
        "slug": str(cell["slug"]),
        "p": p,
        "a": a,
        "b": b,
        "degree": degree,
        "modulus": ",".join(map(str, modulus)),
        "field_size": p**degree,
        "base_count": base_count,
        "trace": trace,
        "extension_count": len(points),
        "predicted_count": trace_group_order(p, trace, degree),
        "points_checked": len(points),
        "pi_order_failures": pi_order_failures,
        "charpoly_failures": charpoly_failures,
        "v_failures": v_failures,
        "wrong_trace_rejections": wrong_trace_rejections,
        "wrong_norm_rejections": wrong_norm_rejections,
        "missing_inverse_rejections": missing_inverse_rejections,
    }


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify(protocol_path: Path, result_path: Path, output_path: Path) -> None:
    started = time.perf_counter_ns()
    protocol = json.loads(protocol_path.read_text(encoding="utf-8"))
    with result_path.open(newline="", encoding="utf-8") as handle:
        producer_rows = list(csv.DictReader(handle, delimiter="\t"))
    require(len(producer_rows) == len(protocol["cells"]), "wrong producer row count")
    by_slug = {row["slug"]: row for row in producer_rows}
    exact_fields = [
        "p", "a", "b", "degree", "modulus", "field_size", "base_count", "trace",
        "extension_count", "predicted_count", "points_checked", "pi_order_failures",
        "charpoly_failures", "v_failures", "wrong_trace_rejections",
        "wrong_norm_rejections", "missing_inverse_rejections",
    ]
    reports = []
    all_pass = True
    for cell in protocol["cells"]:
        computed = compute_cell(cell)
        producer = by_slug.get(cell["slug"])
        require(producer is not None, f"missing producer row {cell['slug']}")
        mismatches = []
        for field_name in exact_fields:
            if str(computed[field_name]) != producer[field_name]:
                mismatches.append(
                    {"field": field_name, "producer": producer[field_name], "verifier": computed[field_name]}
                )
        diagnostics_valid = all(
            int(producer[name]) >= 0
            for name in [
                "base_additions", "base_multiplications", "base_inversions",
                "extension_additions", "extension_multiplications", "extension_inversions",
                "frobenius_maps", "point_additions", "scalar_multiplications", "elapsed_ns",
            ]
        )
        positive_pass = (
            computed["extension_count"] == computed["predicted_count"]
            and computed["pi_order_failures"] == 0
            and computed["charpoly_failures"] == 0
            and computed["v_failures"] == 0
        )
        controls_pass = (
            computed["wrong_trace_rejections"] > 0
            and computed["wrong_norm_rejections"] > 0
            and computed["missing_inverse_rejections"] > 0
        )
        accepted = not mismatches and diagnostics_valid and positive_pass and controls_pass
        all_pass &= accepted
        reports.append(
            {
                "slug": cell["slug"],
                "accepted": accepted,
                "positive_pass": positive_pass,
                "controls_pass": controls_pass,
                "diagnostics_valid": diagnostics_valid,
                "mismatches": mismatches,
            }
        )
    report = {
        "schema": "h116.pointwise-frobenius-charpoly-verification/v1",
        "protocol_id": protocol["protocol_id"],
        "protocol_sha256": sha256(protocol_path),
        "result_sha256": sha256(result_path),
        "verifier_sha256": sha256(Path(__file__)),
        "cells": reports,
        "all_pass": all_pass,
        "elapsed_ns": time.perf_counter_ns() - started,
        "claim_boundary": protocol["claim_boundary"],
    }
    output_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    require(all_pass, "independent verification failed")
    print(f"h116 pointwise charpoly verifier: {len(reports)}/{len(reports)} cells accepted")


def selftest() -> None:
    modulus = first_irreducible(3, 2)
    field = Field(3, modulus)
    for index in range(field.size):
        value = field.element(index)
        if value != field.zero:
            require(field.mul(value, field.inverse(value)) == field.one, "inverse selftest failed")
        require(field.frobenius(field.frobenius(value)) == value, "Frobenius selftest failed")
    print("h116 pointwise charpoly verifier selftest ok")


def main() -> None:
    if sys.argv[1:] == ["--selftest"]:
        selftest()
        return
    require(len(sys.argv) == 4, "usage: verify.py PROTOCOL.json RESULT.tsv OUTPUT.json")
    verify(Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3]))


if __name__ == "__main__":
    main()
