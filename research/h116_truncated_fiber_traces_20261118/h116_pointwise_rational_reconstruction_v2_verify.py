#!/usr/bin/env python3
"""Independent replay for H116-POINTWISE-RATIONAL-RECONSTRUCTION-002."""

from __future__ import annotations

import csv
import hashlib
import json
import math
import sys
import time
from pathlib import Path

from h116_pointwise_charpoly_verify import Curve, Field, first_irreducible, require


def element_index(field: Field, value: tuple[int, ...]) -> int:
    return sum(coefficient * field.p**index for index, coefficient in enumerate(value))


def evaluate(field: Field, polynomial: list[tuple[int, ...]], value: tuple[int, ...]):
    result = field.zero
    for coefficient in reversed(polynomial):
        result = field.add(field.mul(result, value), coefficient)
    return result


def solve_window(field: Field, window: list[tuple], degree: int):
    width = 2 * degree
    require(len(window) == width, "wrong joint window")
    matrix = [[field.zero for _ in range(width + 1)] for _ in range(width)]
    for row, (x_value, image_x) in enumerate(window):
        powers = [field.one]
        for _ in range(degree):
            powers.append(field.mul(powers[-1], x_value))
        matrix[row][: degree + 1] = powers
        for index in range(degree - 1):
            matrix[row][degree + 1 + index] = field.neg(field.mul(image_x, powers[index]))
        matrix[row][width] = field.mul(image_x, powers[degree - 1])
    pivot_row = 0
    pivots = []
    for column in range(width):
        found = next((row for row in range(pivot_row, width) if matrix[row][column] != field.zero), None)
        if found is None:
            continue
        matrix[pivot_row], matrix[found] = matrix[found], matrix[pivot_row]
        inverse = field.inverse(matrix[pivot_row][column])
        for index in range(column, width + 1):
            matrix[pivot_row][index] = field.mul(matrix[pivot_row][index], inverse)
        for row in range(width):
            if row == pivot_row:
                continue
            factor = matrix[row][column]
            if factor == field.zero:
                continue
            for index in range(column, width + 1):
                matrix[row][index] = field.sub(
                    matrix[row][index], field.mul(factor, matrix[pivot_row][index])
                )
        pivots.append(column)
        pivot_row += 1
        if pivot_row == width:
            break
    solution = [field.zero] * width
    for row, column in enumerate(pivots):
        solution[column] = matrix[row][width]
    numerator = solution[: degree + 1]
    denominator = [*solution[degree + 1 :], field.one]
    return numerator, denominator, len(pivots)


def polynomial_degree(field: Field, polynomial: list[tuple[int, ...]]) -> int:
    return max(index for index, coefficient in enumerate(polynomial) if coefficient != field.zero)


def encode(field: Field, polynomial: list[tuple[int, ...]]) -> str:
    return ",".join(str(element_index(field, coefficient)) for coefficient in polynomial)


def predicted_visible_kernel(order: int, trace: int, degree: int) -> int:
    return math.gcd(order, (pow((trace - 1) % order, degree, order) - 1) % order)


def compute_cell(cell: dict, parent: dict[str, str], capacity: dict[str, str]):
    p, a, b, extension_degree = (int(cell[name]) for name in ["p", "a", "b", "degree"])
    order, trace = int(parent["base_count"]), int(parent["trace"])
    field = Field(p, first_irreducible(p, extension_degree))
    curve = Curve(field, a, b)
    fibers = {}
    for point in curve.enumerate_points():
        if point is not None:
            fibers.setdefault(point[0], []).append(point)
    regular, poles = [], []
    pair_failures = 0
    for x_value in sorted(fibers, key=lambda value: element_index(field, value)):
        images = [curve.subtract(curve.scalar(trace - 1, point), curve.frobenius(point)) for point in fibers[x_value]]
        image_x_values = [None if image is None else image[0] for image in images]
        pair_failures += len(set(image_x_values)) != 1
        if image_x_values[0] is None:
            poles.append((x_value, len(images)))
        else:
            regular.append((x_value, image_x_values[0]))
    window_size = 2 * order
    first_n, first_d, first_rank = solve_window(field, regular[:window_size], order)
    second_n, second_d, second_rank = solve_window(field, regular[-window_size:], order)
    numerator_degree = polynomial_degree(field, first_n)
    denominator_degree = polynomial_degree(field, first_d)
    cross_degree = numerator_degree + denominator_degree
    predicted_kernel = predicted_visible_kernel(order, trace, extension_degree)
    regular_mismatches = sum(
        evaluate(field, first_n, x_value) != field.mul(image_x, evaluate(field, first_d, x_value))
        for x_value, image_x in regular
    )
    denominator_zero_regular = sum(evaluate(field, first_d, x_value) == field.zero for x_value, _ in regular)
    missed_observed_poles = sum(evaluate(field, first_d, x_value) != field.zero for x_value, _ in poles)
    pole_cancellations = sum(evaluate(field, first_n, x_value) == field.zero for x_value, _ in poles)
    numerator_mutation = first_n.copy()
    numerator_mutation[0] = field.add(numerator_mutation[0], field.one)
    numerator_mutation_rejections = sum(
        evaluate(field, numerator_mutation, x_value) != field.mul(image_x, evaluate(field, first_d, x_value))
        for x_value, image_x in regular
    )
    denominator_mutation = first_d.copy()
    denominator_mutation[0] = field.add(denominator_mutation[0], field.one)
    denominator_mutation_destroyed_poles = sum(
        evaluate(field, denominator_mutation, x_value) != field.zero for x_value, _ in poles
    )
    coefficients_base = all(
        all(all(coefficient == 0 for coefficient in value[1:]) for value in polynomial)
        for polynomial in [first_n, first_d, second_n, second_d]
    )
    distinct_x_count = len(regular) + len(poles)
    require(distinct_x_count == int(capacity["distinct_x_count"]), "capacity mismatch")
    return {
        "p": p, "degree": extension_degree, "base_count": order, "trace": trace,
        "distinct_x_count": distinct_x_count, "regular_x_count": len(regular),
        "pole_x_count": len(poles), "pole_multiplicity": sum(value for _, value in poles),
        "predicted_visible_kernel": predicted_kernel,
        "observed_visible_kernel": 1 + sum(value for _, value in poles),
        "predicted_pole_x_count": predicted_kernel // 2, "window_size": window_size,
        "first_rank": first_rank, "second_rank": second_rank,
        "numerator_degree": numerator_degree, "denominator_degree": denominator_degree,
        "cross_degree": cross_degree, "separation_margin": len(regular) - cross_degree,
        "regular_strict_pass": int(len(regular) > cross_degree),
        "boundary_sample_count": cross_degree, "boundary_strict_pass": 0,
        "pair_consistency_failures": pair_failures,
        "windows_equal": int(first_n == second_n and first_d == second_d),
        "coefficients_base_field": int(coefficients_base),
        "numerator_first": encode(field, first_n), "numerator_second": encode(field, second_n),
        "denominator_first": encode(field, first_d), "denominator_second": encode(field, second_d),
        "regular_mismatches": regular_mismatches,
        "denominator_zero_regular": denominator_zero_regular,
        "missed_observed_poles": missed_observed_poles, "pole_cancellations": pole_cancellations,
        "numerator_mutation_rejections": numerator_mutation_rejections,
        "denominator_mutation_destroyed_poles": denominator_mutation_destroyed_poles,
    }


def read_tsv(path: Path):
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify(protocol_path: Path, parent_path: Path, capacity_path: Path, result_path: Path, output_path: Path):
    started = time.perf_counter_ns()
    protocol = json.loads(protocol_path.read_text(encoding="utf-8"))
    parents = {row["slug"]: row for row in read_tsv(parent_path)}
    capacities = {row["slug"]: row for row in read_tsv(capacity_path)}
    producers = {row["slug"]: row for row in read_tsv(result_path)}
    exact_fields = [
        "p", "degree", "base_count", "trace", "distinct_x_count", "regular_x_count",
        "pole_x_count", "pole_multiplicity", "predicted_visible_kernel", "observed_visible_kernel",
        "predicted_pole_x_count", "window_size", "first_rank", "second_rank", "numerator_degree",
        "denominator_degree", "cross_degree", "separation_margin", "regular_strict_pass",
        "boundary_sample_count", "boundary_strict_pass", "pair_consistency_failures", "windows_equal",
        "coefficients_base_field", "numerator_first", "numerator_second", "denominator_first",
        "denominator_second", "regular_mismatches", "denominator_zero_regular",
        "missed_observed_poles", "pole_cancellations", "numerator_mutation_rejections",
        "denominator_mutation_destroyed_poles",
    ]
    diagnostics = ["extension_additions", "extension_multiplications", "extension_inversions", "point_additions", "scalar_multiplications", "elapsed_ns"]
    reports, recovered = [], {}
    all_pass, family_stable = True, True
    for cell in protocol["cells"]:
        slug = cell["slug"]
        computed = compute_cell(cell, parents[slug], capacities[slug])
        producer = producers[slug]
        mismatches = [
            {"field": name, "producer": producer[name], "verifier": str(computed[name])}
            for name in exact_fields if producer[name] != str(computed[name])
        ]
        key = tuple(int(cell[name]) for name in ["p", "a", "b"])
        value = (str(computed["numerator_first"]), str(computed["denominator_first"]))
        if key in recovered:
            family_stable &= recovered[key] == value
        recovered[key] = value
        accepted = (
            not mismatches and all(int(producer[name]) >= 0 for name in diagnostics)
            and producer["accepted"] == "1" and computed["pair_consistency_failures"] == 0
            and computed["observed_visible_kernel"] == computed["predicted_visible_kernel"]
            and computed["pole_x_count"] == computed["predicted_pole_x_count"]
            and computed["first_rank"] == computed["window_size"]
            and computed["second_rank"] == computed["window_size"]
            and computed["numerator_degree"] == computed["base_count"]
            and computed["denominator_degree"] == computed["base_count"] - 1
            and computed["windows_equal"] == 1 and computed["coefficients_base_field"] == 1
            and computed["regular_mismatches"] == 0 and computed["denominator_zero_regular"] == 0
            and computed["missed_observed_poles"] == 0 and computed["pole_cancellations"] == 0
            and computed["regular_strict_pass"] == 1 and computed["boundary_strict_pass"] == 0
            and computed["numerator_mutation_rejections"] == computed["regular_x_count"]
            and computed["denominator_mutation_destroyed_poles"] == computed["pole_x_count"]
        )
        all_pass &= accepted
        reports.append({"slug": slug, "accepted": accepted, "mismatches": mismatches})
    all_pass &= family_stable and len(recovered) == 3
    report = {
        "schema": "h116.pointwise-rational-reconstruction-verification/v2",
        "protocol_id": protocol["protocol_id"], "protocol_sha256": sha256(protocol_path),
        "parent_sha256": sha256(parent_path), "capacity_sha256": sha256(capacity_path),
        "result_sha256": sha256(result_path), "verifier_sha256": sha256(Path(__file__)),
        "cells": reports, "family_count": len(recovered), "family_stable": family_stable,
        "all_pass": all_pass, "elapsed_ns": time.perf_counter_ns() - started,
        "claim_boundary": protocol["claim_boundary"],
    }
    output_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    require(all_pass, "pointwise rational reconstruction v2 verification failed")
    print("h116 pointwise rational verifier v2: 8/8 cells accepted; 3/3 families stable")


def selftest():
    require(predicted_visible_kernel(7, -1, 2) == 1, "p5 d2 eigenspace failed")
    require(predicted_visible_kernel(5, 3, 4) == 5, "p7 d4 eigenspace failed")
    require(predicted_visible_kernel(14, -2, 2) == 2, "p11 d2 eigenspace failed")
    require(predicted_visible_kernel(14, -2, 3) == 14, "p11 d3 eigenspace failed")
    print("h116 pointwise rational verifier v2 selftest ok")


def main():
    if sys.argv[1:] == ["--selftest"]:
        selftest()
        return
    require(len(sys.argv) == 6, "usage: verify.py PROTOCOL PARENT CAPACITY RESULT OUTPUT")
    verify(*(Path(argument) for argument in sys.argv[1:]))


if __name__ == "__main__":
    main()
