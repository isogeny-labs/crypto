#!/usr/bin/env python3
"""Independent replay for H116-SCALAR-DIVPOLY-DEGREE-001."""

from __future__ import annotations

import csv
import hashlib
import json
import sys
from pathlib import Path

from h116_pointwise_charpoly_verify import Curve, Field, first_irreducible, require

Poly = list[int]
Psi = tuple[Poly, bool]


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def trim(value: Poly) -> Poly:
    while len(value) > 1 and value[-1] == 0:
        value.pop()
    return value


def psub(left: Poly, right: Poly, p: int) -> Poly:
    return trim([
        ((left[i] if i < len(left) else 0) - (right[i] if i < len(right) else 0)) % p
        for i in range(max(len(left), len(right)))
    ])


def pmul(left: Poly, right: Poly, p: int) -> Poly:
    if left == [0] or right == [0]:
        return [0]
    out = [0] * (len(left) + len(right) - 1)
    for i, x_value in enumerate(left):
        for j, y_value in enumerate(right):
            out[i + j] = (out[i + j] + x_value * y_value) % p
    return trim(out)


def pscale(value: Poly, scalar: int, p: int) -> Poly:
    return trim([(scalar * coefficient) % p for coefficient in value])


def pdiv_exact(numerator: Poly, denominator: Poly, p: int) -> Poly:
    remainder = trim(numerator.copy())
    denominator = trim(denominator.copy())
    require(denominator != [0], "zero polynomial divisor")
    require(len(remainder) >= len(denominator), "improper exact division")
    quotient = [0] * (len(remainder) - len(denominator) + 1)
    inverse_lead = pow(denominator[-1], p - 2, p)
    while remainder != [0] and len(remainder) >= len(denominator):
        shift = len(remainder) - len(denominator)
        coefficient = remainder[-1] * inverse_lead % p
        quotient[shift] = coefficient
        for index, term in enumerate(denominator):
            remainder[index + shift] = (
                remainder[index + shift] - coefficient * term
            ) % p
        trim(remainder)
    require(remainder == [0], "nonexact polynomial division")
    return trim(quotient)


def psi_mul(left: Psi, right: Psi, curve: Poly, p: int) -> Psi:
    polynomial = pmul(left[0], right[0], p)
    if left[1] and right[1]:
        polynomial = pmul(polynomial, curve, p)
    return polynomial, left[1] != right[1]


def psi_power(value: Psi, exponent: int, curve: Poly, p: int) -> Psi:
    result: Psi = ([1], False)
    base = (value[0].copy(), value[1])
    while exponent:
        if exponent & 1:
            result = psi_mul(result, base, curve, p)
        exponent >>= 1
        if exponent:
            base = psi_mul(base, base, curve, p)
    return result


def psi_sub(left: Psi, right: Psi, p: int) -> Psi:
    require(left[1] == right[1], "incompatible y parity")
    return psub(left[0], right[0], p), left[1]


def divide_two_y(value: Psi, curve: Poly, p: int) -> Psi:
    inverse_two = pow(2, p - 2, p)
    if value[1]:
        return pscale(value[0], inverse_two, p), False
    return pscale(pdiv_exact(value[0], curve, p), inverse_two, p), True


def scalar_x_map(n: int, a: int, b: int, p: int) -> tuple[Poly, Poly]:
    require(n >= 2, "scalar must be at least two")
    curve = [b % p, a % p, 0, 1]
    psi3 = [
        -(a * a) % p,
        12 * b % p,
        6 * a % p,
        0,
        3 % p,
    ]
    psi4_core = [
        -(8 * b * b + a**3) % p,
        -4 * a * b % p,
        -5 * a * a % p,
        20 * b % p,
        5 * a % p,
        0,
        1,
    ]
    psi: list[Psi] = [
        ([0], False),
        ([1], False),
        ([2], True),
        (psi3, False),
        (pscale(psi4_core, 4, p), True),
    ]
    for index in range(5, n + 2):
        if index & 1:
            m = (index - 1) // 2
            left = psi_mul(psi[m + 2], psi_power(psi[m], 3, curve, p), curve, p)
            right = psi_mul(
                psi[m - 1], psi_power(psi[m + 1], 3, curve, p), curve, p
            )
            next_value = psi_sub(left, right, p)
        else:
            m = index // 2
            left = psi_mul(
                psi[m + 2], psi_power(psi[m - 1], 2, curve, p), curve, p
            )
            right = psi_mul(
                psi[m - 2], psi_power(psi[m + 1], 2, curve, p), curve, p
            )
            next_value = divide_two_y(
                psi_mul(psi[m], psi_sub(left, right, p), curve, p), curve, p
            )
        psi.append(next_value)
    denominator = psi_mul(psi[n], psi[n], curve, p)
    adjacent = psi_mul(psi[n - 1], psi[n + 1], curve, p)
    require(not denominator[1] and not adjacent[1], "x-map retains y")
    numerator = psub([0, *denominator[0]], adjacent[0], p)
    return numerator, denominator[0]


def evaluate(field: Field, polynomial: Poly, value: tuple[int, ...]):
    result = field.zero
    for coefficient in reversed(polynomial):
        result = field.add(field.mul(result, value), field.base(coefficient))
    return result


def compute(cell: dict) -> dict[str, int | str]:
    p, a, b, degree, scalar = (
        int(cell[name]) for name in ["p", "a", "b", "degree", "N"]
    )
    field = Field(p, first_irreducible(p, degree))
    curve = Curve(field, a, b)
    numerator, denominator = scalar_x_map(scalar, a, b, p)
    _, char_denominator = scalar_x_map(p, a, b, p)
    expected_char_degree = p * p - 1
    affine_points = 0
    regular_points = 0
    pole_points = 0
    map_failures = 0
    pole_failures = 0
    cancellations = 0
    mutation_failures = 0
    for point in curve.enumerate_points():
        if point is None:
            continue
        affine_points += 1
        x_value = point[0]
        image = curve.scalar(scalar, point)
        numerator_value = evaluate(field, numerator, x_value)
        denominator_value = evaluate(field, denominator, x_value)
        if image is None:
            pole_points += 1
            pole_failures += denominator_value != field.zero
            cancellations += numerator_value == field.zero
        else:
            regular_points += 1
            if denominator_value == field.zero:
                pole_failures += 1
                map_failures += 1
                mutation_failures += 1
                continue
            predicted = field.divide(numerator_value, denominator_value)
            map_failures += predicted != image[0]
            mutated = field.divide(
                field.add(numerator_value, field.one), denominator_value
            )
            mutation_failures += mutated != image[0]
    characteristic_control = (
        len(char_denominator) - 1 < expected_char_degree
        and (
            char_denominator[expected_char_degree]
            if expected_char_degree < len(char_denominator)
            else 0
        )
        == 0
    )
    accepted = (
        len(numerator) - 1 == scalar * scalar
        and len(denominator) - 1 == scalar * scalar - 1
        and numerator[-1] == 1
        and denominator[-1] == scalar * scalar % p
        and map_failures == pole_failures == cancellations == 0
        and mutation_failures > 0
        and characteristic_control
    )
    return {
        "p": p,
        "a": a,
        "b": b,
        "extension_degree": degree,
        "N": scalar,
        "numerator_degree": len(numerator) - 1,
        "denominator_degree": len(denominator) - 1,
        "numerator_lead": numerator[-1],
        "denominator_lead": denominator[-1],
        "char_denominator_degree": len(char_denominator) - 1,
        "char_expected_degree": expected_char_degree,
        "char_expected_coefficient": (
            char_denominator[expected_char_degree]
            if expected_char_degree < len(char_denominator)
            else 0
        ),
        "affine_points": affine_points,
        "regular_points": regular_points,
        "pole_points": pole_points,
        "map_failures": map_failures,
        "pole_failures": pole_failures,
        "cancellations": cancellations,
        "mutation_failures": mutation_failures,
        "characteristic_control": str(characteristic_control).lower(),
        "accepted": str(accepted).lower(),
    }


def read_result(path: Path) -> dict[str, dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle, delimiter="\t"))
    return {row["slug"]: row for row in rows}


def verify(protocol_path: Path, result_path: Path, output_path: Path) -> None:
    protocol = json.loads(protocol_path.read_text(encoding="utf-8"))
    producer = read_result(result_path)
    slugs = {cell["slug"] for cell in protocol["cells"]}
    require(set(producer) == slugs, "producer cell set mismatch")
    exact_fields = [
        "p", "a", "b", "extension_degree", "N", "numerator_degree",
        "denominator_degree", "numerator_lead", "denominator_lead",
        "char_denominator_degree", "char_expected_degree",
        "char_expected_coefficient", "affine_points", "regular_points",
        "pole_points", "map_failures", "pole_failures", "cancellations",
        "mutation_failures", "characteristic_control", "accepted",
    ]
    reports = []
    all_pass = True
    for cell in protocol["cells"]:
        slug = cell["slug"]
        computed = compute(cell)
        mismatches = [
            {
                "field": field,
                "producer": producer[slug][field],
                "verifier": str(computed[field]),
            }
            for field in exact_fields
            if producer[slug][field] != str(computed[field])
        ]
        counters_nonnegative = all(
            int(producer[slug][field]) >= 0
            for field in [
                "construction_additions", "construction_multiplications",
                "construction_inversions", "char_control_additions",
                "char_control_multiplications", "char_control_inversions",
                "extension_additions", "extension_multiplications",
                "extension_inversions", "point_additions",
                "scalar_multiplications",
            ]
        )
        accepted = not mismatches and counters_nonnegative and computed["accepted"] == "true"
        reports.append({
            "slug": slug,
            "accepted": accepted,
            "mismatches": mismatches,
            "computed": computed,
        })
        all_pass &= accepted
    output = {
        "schema": "h116.scalar-divpoly-degree-verify/v1",
        "protocol_id": protocol["protocol_id"],
        "protocol_sha256": sha256(protocol_path),
        "producer_result_sha256": sha256(result_path),
        "independent_implementation": True,
        "cells": reports,
        "accepted_cells": sum(report["accepted"] for report in reports),
        "total_cells": len(reports),
        "all_pass": all_pass,
    }
    output_path.write_text(json.dumps(output, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    require(all_pass, "independent verification failed")


def main() -> None:
    base = Path(__file__).resolve().parent
    protocol = Path(sys.argv[1]) if len(sys.argv) > 1 else base / "SCALAR_DIVPOLY_DEGREE_PROTOCOL.json"
    result = Path(sys.argv[2]) if len(sys.argv) > 2 else base / "SCALAR_DIVPOLY_DEGREE_RESULT.tsv"
    output = Path(sys.argv[3]) if len(sys.argv) > 3 else base / "SCALAR_DIVPOLY_DEGREE_VERIFY.json"
    verify(protocol, result, output)
    print(output.read_text(encoding="utf-8"), end="")


if __name__ == "__main__":
    main()
