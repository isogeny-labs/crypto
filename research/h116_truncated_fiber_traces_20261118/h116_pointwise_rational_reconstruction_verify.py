#!/usr/bin/env python3
"""Independent replay for H116-POINTWISE-RATIONAL-RECONSTRUCTION-001."""

from __future__ import annotations

import csv
import hashlib
import json
import sys
import time
from pathlib import Path

from h116_pointwise_charpoly_verify import Curve, Field, first_irreducible, require

Element = tuple[int, ...]


def element_index(field: Field, value: Element) -> int:
    return sum(coefficient * field.p**index for index, coefficient in enumerate(value))


def multiply_by_linear(field: Field, coefficients: list[Element], root: Element) -> list[Element]:
    result = [field.zero] * (len(coefficients) + 1)
    negative_root = field.neg(root)
    for index, coefficient in enumerate(coefficients):
        result[index] = field.add(result[index], field.mul(negative_root, coefficient))
        result[index + 1] = field.add(result[index + 1], coefficient)
    return result


def evaluate(field: Field, coefficients: list[Element], value: Element) -> Element:
    result = field.zero
    for coefficient in reversed(coefficients):
        result = field.add(field.mul(result, value), coefficient)
    return result


def pole_denominator(field: Field, poles: list[tuple[Element, int]]) -> list[Element]:
    result = [field.one]
    for root, multiplicity in poles:
        for _ in range(multiplicity):
            result = multiply_by_linear(field, result, root)
    return result


def interpolate(
    field: Field,
    samples: list[tuple[Element, Element]],
    denominator: list[Element],
    degree: int,
) -> list[Element]:
    require(len(samples) == degree + 1, "wrong interpolation window")
    result = [field.zero] * (degree + 1)
    for i, (x_value, image_x) in enumerate(samples):
        basis = [field.one]
        divisor = field.one
        for j, (other_x, _) in enumerate(samples):
            if i == j:
                continue
            basis = multiply_by_linear(field, basis, other_x)
            divisor = field.mul(divisor, field.sub(x_value, other_x))
        target = field.mul(image_x, evaluate(field, denominator, x_value))
        scale = field.divide(target, divisor)
        for index, coefficient in enumerate(basis):
            result[index] = field.add(result[index], field.mul(scale, coefficient))
    return result


def degree(field: Field, coefficients: list[Element]) -> int:
    return max(index for index, coefficient in enumerate(coefficients) if coefficient != field.zero)


def encode(field: Field, coefficients: list[Element]) -> str:
    return ",".join(str(element_index(field, coefficient)) for coefficient in coefficients)


def pointwise_map(curve: Curve, point, trace: int):
    return curve.subtract(curve.scalar(trace - 1, point), curve.frobenius(point))


def compute_cell(cell: dict, parent: dict[str, str], capacity: dict[str, str]) -> dict[str, int | str]:
    p = int(cell["p"])
    a = int(cell["a"])
    b = int(cell["b"])
    extension_degree = int(cell["degree"])
    base_count = int(parent["base_count"])
    trace = int(parent["trace"])
    field = Field(p, first_irreducible(p, extension_degree))
    curve = Curve(field, a, b)
    points = curve.enumerate_points()
    fibers: dict[Element, list] = {}
    for point in points:
        if point is not None:
            fibers.setdefault(point[0], []).append(point)
    regular: list[tuple[Element, Element]] = []
    poles: list[tuple[Element, int]] = []
    pair_failures = 0
    for x_value in sorted(fibers, key=lambda value: element_index(field, value)):
        images = [pointwise_map(curve, point, trace) for point in fibers[x_value]]
        image_x_values = [None if image is None else image[0] for image in images]
        pair_failures += len(set(image_x_values)) != 1
        if image_x_values[0] is None:
            poles.append((x_value, len(images)))
        else:
            regular.append((x_value, image_x_values[0]))
    denominator = pole_denominator(field, poles)
    first = interpolate(field, regular[: base_count + 1], denominator, base_count)
    second = interpolate(field, regular[-base_count - 1 :], denominator, base_count)
    numerator_degree = degree(field, first)
    denominator_degree = degree(field, denominator)
    cross_degree = numerator_degree + denominator_degree
    full_mismatches = sum(
        evaluate(field, first, x_value) != field.mul(image_x, evaluate(field, denominator, x_value))
        for x_value, image_x in regular
    )
    pole_cancellations = sum(evaluate(field, first, x_value) == field.zero for x_value, _ in poles)
    numerator_mutation = first.copy()
    numerator_mutation[0] = field.add(numerator_mutation[0], field.one)
    numerator_mutation_rejections = sum(
        evaluate(field, numerator_mutation, x_value)
        != field.mul(image_x, evaluate(field, denominator, x_value))
        for x_value, image_x in regular
    )
    denominator_mutation = denominator.copy()
    denominator_mutation[0] = field.add(denominator_mutation[0], field.one)
    denominator_mutation_destroyed_poles = sum(
        evaluate(field, denominator_mutation, x_value) != field.zero for x_value, _ in poles
    )
    coefficients_base = all(
        all(coefficient[1:] == (0,) * (field.degree - 1) for coefficient in polynomial)
        for polynomial in [first, second, denominator]
    )
    distinct_x_count = len(regular) + len(poles)
    require(distinct_x_count == int(capacity["distinct_x_count"]), "capacity mismatch")
    return {
        "p": p,
        "degree": extension_degree,
        "base_count": base_count,
        "trace": trace,
        "distinct_x_count": distinct_x_count,
        "regular_x_count": len(regular),
        "pole_x_count": len(poles),
        "pole_multiplicity": sum(multiplicity for _, multiplicity in poles),
        "numerator_degree": numerator_degree,
        "denominator_degree": denominator_degree,
        "cross_degree": cross_degree,
        "separation_margin": len(regular) - cross_degree,
        "regular_strict_pass": int(len(regular) > cross_degree),
        "boundary_sample_count": cross_degree,
        "boundary_strict_pass": int(cross_degree > cross_degree),
        "pair_consistency_failures": pair_failures,
        "interpolation_window": base_count + 1,
        "numerator_windows_equal": int(first == second),
        "coefficients_base_field": int(coefficients_base),
        "numerator_first": encode(field, first),
        "numerator_second": encode(field, second),
        "denominator": encode(field, denominator),
        "full_sample_mismatches": full_mismatches,
        "pole_cancellations": pole_cancellations,
        "numerator_mutation_rejections": numerator_mutation_rejections,
        "denominator_mutation_destroyed_poles": denominator_mutation_destroyed_poles,
    }


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify(
    protocol_path: Path,
    parent_path: Path,
    capacity_path: Path,
    result_path: Path,
    output_path: Path,
) -> None:
    started = time.perf_counter_ns()
    protocol = json.loads(protocol_path.read_text(encoding="utf-8"))
    parents = {row["slug"]: row for row in read_tsv(parent_path)}
    capacities = {row["slug"]: row for row in read_tsv(capacity_path)}
    producer_rows = {row["slug"]: row for row in read_tsv(result_path)}
    exact_fields = [
        "p", "degree", "base_count", "trace", "distinct_x_count", "regular_x_count",
        "pole_x_count", "pole_multiplicity", "numerator_degree", "denominator_degree",
        "cross_degree", "separation_margin", "regular_strict_pass", "boundary_sample_count",
        "boundary_strict_pass", "pair_consistency_failures", "interpolation_window",
        "numerator_windows_equal", "coefficients_base_field", "numerator_first",
        "numerator_second", "denominator", "full_sample_mismatches", "pole_cancellations",
        "numerator_mutation_rejections", "denominator_mutation_destroyed_poles",
    ]
    diagnostics = [
        "extension_additions", "extension_multiplications", "extension_inversions",
        "point_additions", "scalar_multiplications", "elapsed_ns",
    ]
    reports = []
    recovered: dict[tuple[int, int, int], tuple[str, str]] = {}
    family_stable = True
    all_pass = True
    for cell in protocol["cells"]:
        slug = cell["slug"]
        computed = compute_cell(cell, parents[slug], capacities[slug])
        producer = producer_rows[slug]
        mismatches = [
            {"field": name, "producer": producer[name], "verifier": str(computed[name])}
            for name in exact_fields if producer[name] != str(computed[name])
        ]
        key = (int(cell["p"]), int(cell["a"]), int(cell["b"]))
        value = (str(computed["numerator_first"]), str(computed["denominator"]))
        if key in recovered:
            family_stable &= recovered[key] == value
        recovered[key] = value
        diagnostics_valid = all(int(producer[name]) >= 0 for name in diagnostics)
        accepted = (
            not mismatches
            and diagnostics_valid
            and producer["accepted"] == "1"
            and computed["pair_consistency_failures"] == 0
            and computed["pole_multiplicity"] == computed["base_count"] - 1
            and computed["numerator_degree"] == computed["base_count"]
            and computed["denominator_degree"] == computed["base_count"] - 1
            and computed["numerator_windows_equal"] == 1
            and computed["coefficients_base_field"] == 1
            and computed["full_sample_mismatches"] == 0
            and computed["pole_cancellations"] == 0
            and computed["regular_strict_pass"] == 1
            and computed["boundary_strict_pass"] == 0
            and computed["numerator_mutation_rejections"] == computed["regular_x_count"]
            and computed["denominator_mutation_destroyed_poles"] == computed["pole_x_count"]
        )
        all_pass &= accepted
        reports.append({"slug": slug, "accepted": accepted, "mismatches": mismatches})
    all_pass &= family_stable and len(recovered) == 3
    report = {
        "schema": "h116.pointwise-rational-reconstruction-verification/v1",
        "protocol_id": protocol["protocol_id"],
        "protocol_sha256": sha256(protocol_path),
        "parent_sha256": sha256(parent_path),
        "capacity_sha256": sha256(capacity_path),
        "result_sha256": sha256(result_path),
        "verifier_sha256": sha256(Path(__file__)),
        "cells": reports,
        "family_count": len(recovered),
        "family_stable": family_stable,
        "all_pass": all_pass,
        "elapsed_ns": time.perf_counter_ns() - started,
        "claim_boundary": protocol["claim_boundary"],
    }
    output_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    require(all_pass, "pointwise rational reconstruction verification failed")
    print("h116 pointwise rational verifier: 8/8 cells accepted; 3/3 families stable")


def selftest() -> None:
    field = Field(5, [0, 1])
    samples = [(field.base(0), field.base(1)), (field.base(1), field.base(3))]
    polynomial = interpolate(field, samples, [field.one], 1)
    require(encode(field, polynomial) == "1,2", "linear interpolation failed")
    print("h116 pointwise rational verifier selftest ok")


def main() -> None:
    if sys.argv[1:] == ["--selftest"]:
        selftest()
        return
    require(
        len(sys.argv) == 6,
        "usage: verify.py PROTOCOL.json PARENT.tsv CAPACITY.tsv RESULT.tsv OUTPUT.json",
    )
    verify(*(Path(argument) for argument in sys.argv[1:]))


if __name__ == "__main__":
    main()
