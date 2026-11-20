#!/usr/bin/env python3
"""Independent replay for H116-DUAL-PAIR-COMPOSITION-001."""

from __future__ import annotations

import csv
import hashlib
import json
import sys
import time
from pathlib import Path

from h116_pointwise_charpoly_verify import Curve, Field, first_irreducible, require


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def element_index(field: Field, value: tuple[int, ...]) -> int:
    return sum(coefficient * field.p**index for index, coefficient in enumerate(value))


def evaluate(field: Field, polynomial: list[tuple[int, ...]], value: tuple[int, ...]):
    result = field.zero
    for coefficient in reversed(polynomial):
        result = field.add(field.mul(result, value), coefficient)
    return result


def solve_window(field: Field, window: list[tuple], degree: int):
    width = 2 * degree
    require(len(window) == width, "wrong interpolation window")
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
    return solution[: degree + 1], [*solution[degree + 1 :], field.one], len(pivots)


def polynomial_degree(field: Field, polynomial: list[tuple[int, ...]]) -> int:
    return max(index for index, coefficient in enumerate(polynomial) if coefficient != field.zero)


def encode(field: Field, polynomial: list[tuple[int, ...]]) -> str:
    return ",".join(str(element_index(field, coefficient)) for coefficient in polynomial)


def divide_by_x_minus_root(field: Field, polynomial: list[tuple[int, ...]], root: tuple[int, ...]):
    require(len(polynomial) >= 2, "cannot divide constant")
    degree = len(polynomial) - 1
    quotient = [field.zero] * degree
    quotient[-1] = polynomial[-1]
    for index in range(degree - 1, 0, -1):
        quotient[index - 1] = field.add(polynomial[index], field.mul(root, quotient[index]))
    require(field.add(polynomial[0], field.mul(root, quotient[0])) == field.zero, "nonzero remainder")
    return quotient


def root_multiplicity(field: Field, polynomial: list[tuple[int, ...]], root: tuple[int, ...]) -> int:
    quotient = polynomial.copy()
    multiplicity = 0
    while len(quotient) >= 2 and evaluate(field, quotient, root) == field.zero:
        quotient = divide_by_x_minus_root(field, quotient, root)
        multiplicity += 1
    return multiplicity


def compute(cell: dict, pointwise: dict[str, str], parent: dict[str, str]):
    p, a, b, degree = (int(cell[name]) for name in ["p", "a", "b", "degree"])
    order, trace = int(pointwise["base_count"]), int(pointwise["trace"])
    require(parent["accepted"] == "1", "unaccepted V-1 parent")
    field = Field(p, first_irreducible(p, degree))
    curve = Curve(field, a, b)
    points = curve.enumerate_points()
    fibers: dict[tuple[int, ...], list] = {}
    for point in points:
        if point is not None:
            fibers.setdefault(point[0], []).append(point)
    regular, poles = [], []
    pair_failures = 0
    for x_value in sorted(fibers, key=lambda value: element_index(field, value)):
        images = [curve.subtract(curve.frobenius(point), point) for point in fibers[x_value]]
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
    multiplicities = [root_multiplicity(field, first_d, x_value) for x_value, _ in poles]
    denominator_roots_all = sum(
        evaluate(field, first_d, field.element(index)) == field.zero for index in range(field.size)
    )
    regular_mismatches = sum(
        evaluate(field, first_n, x_value) != field.mul(image_x, evaluate(field, first_d, x_value))
        for x_value, image_x in regular
    )
    missed_poles = sum(evaluate(field, first_d, x_value) != field.zero for x_value, _ in poles)
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
    composition_failures = 0
    omitted_failures = 0
    wrong_scalar_failures = 0
    for point in points:
        pi = curve.frobenius(point)
        right = curve.subtract(pi, point)
        composed = curve.subtract(curve.scalar(trace - 1, right), curve.frobenius(right))
        expected = curve.scalar(order, point)
        composition_failures += composed != expected
        omitted = curve.subtract(curve.scalar(trace - 1, pi), curve.frobenius(pi))
        omitted_failures += omitted != expected
        wrong_scalar_failures += composed != curve.scalar(order + 1, point)
    coefficients_base = all(
        all(all(coefficient == 0 for coefficient in value[1:]) for value in polynomial)
        for polynomial in [first_n, first_d, second_n, second_d]
    )
    cross_degree = numerator_degree + denominator_degree
    return {
        "p": p, "degree": degree, "base_count": order, "trace": trace,
        "distinct_x_count": int(parent["distinct_x_count"]),
        "predicted_regular_x": int(parent["distinct_x_count"]) - order // 2,
        "regular_x_count": len(regular), "predicted_pole_roots": order // 2,
        "pole_x_count": len(poles), "pole_multiplicity": sum(value for _, value in poles),
        "predicted_simple": int(order % 2 == 0), "observed_simple": multiplicities.count(1),
        "predicted_double": order // 2 - int(order % 2 == 0),
        "observed_double": multiplicities.count(2),
        "observed_higher": sum(value not in (1, 2) for value in multiplicities),
        "denominator_roots_all": denominator_roots_all, "window_size": window_size,
        "first_rank": first_rank, "second_rank": second_rank,
        "numerator_degree": numerator_degree, "denominator_degree": denominator_degree,
        "parent_numerator_degree": int(parent["numerator_degree"]),
        "parent_denominator_degree": int(parent["denominator_degree"]),
        "degree_pair_equal": int(
            numerator_degree == int(parent["numerator_degree"])
            and denominator_degree == int(parent["denominator_degree"])
        ),
        "cross_degree": cross_degree, "separation_margin": len(regular) - cross_degree,
        "strict_pass": int(len(regular) > cross_degree), "boundary_sample_count": cross_degree,
        "boundary_strict_pass": 0, "pair_consistency_failures": pair_failures,
        "windows_equal": int(first_n == second_n and first_d == second_d),
        "coefficients_base_field": int(coefficients_base),
        "numerator_first": encode(field, first_n), "numerator_second": encode(field, second_n),
        "denominator_first": encode(field, first_d), "denominator_second": encode(field, second_d),
        "regular_mismatches": regular_mismatches, "missed_poles": missed_poles,
        "pole_cancellations": pole_cancellations,
        "numerator_mutation_rejections": numerator_mutation_rejections,
        "denominator_mutation_destroyed_poles": denominator_mutation_destroyed_poles,
        "point_count": len(points), "composition_failures": composition_failures,
        "omitted_minus_one_failures": omitted_failures, "wrong_scalar_failures": wrong_scalar_failures,
    }


def verify(protocol_path: Path, pointwise_path: Path, parent_path: Path, result_path: Path, output_path: Path):
    started = time.perf_counter_ns()
    protocol = json.loads(protocol_path.read_text(encoding="utf-8"))
    require(sha256(pointwise_path) == protocol["parent_pointwise_result_sha256"], "pointwise hash mismatch")
    require(sha256(parent_path) == protocol["parent_v_minus_one_result_sha256"], "V-1 hash mismatch")
    pointwise = {row["slug"]: row for row in read_tsv(pointwise_path)}
    parents = {row["slug"]: row for row in read_tsv(parent_path)}
    producers = {row["slug"]: row for row in read_tsv(result_path)}
    cell_slugs = {cell["slug"] for cell in protocol["cells"]}
    require(set(pointwise) == set(parents) == set(producers) == cell_slugs, "cell set mismatch")
    exact_fields = [
        "p", "degree", "base_count", "trace", "distinct_x_count", "predicted_regular_x",
        "regular_x_count", "predicted_pole_roots", "pole_x_count", "pole_multiplicity",
        "predicted_simple", "observed_simple", "predicted_double", "observed_double",
        "observed_higher", "denominator_roots_all", "window_size", "first_rank", "second_rank",
        "numerator_degree", "denominator_degree", "parent_numerator_degree",
        "parent_denominator_degree", "degree_pair_equal", "cross_degree", "separation_margin",
        "strict_pass", "boundary_sample_count", "boundary_strict_pass", "pair_consistency_failures",
        "windows_equal", "coefficients_base_field", "numerator_first", "numerator_second",
        "denominator_first", "denominator_second", "regular_mismatches", "missed_poles",
        "pole_cancellations", "numerator_mutation_rejections",
        "denominator_mutation_destroyed_poles", "point_count", "composition_failures",
        "omitted_minus_one_failures", "wrong_scalar_failures",
    ]
    reports = []
    recovered = {}
    family_stable = True
    all_pass = True
    for cell in protocol["cells"]:
        slug = cell["slug"]
        computed = compute(cell, pointwise[slug], parents[slug])
        producer = producers[slug]
        mismatches = [
            {"field": name, "producer": producer[name], "verifier": str(computed[name])}
            for name in exact_fields if producer[name] != str(computed[name])
        ]
        locked_mismatches = []
        for protocol_name, computed_name in [
            ("dual_pole_roots", "pole_x_count"), ("dual_simple_roots", "observed_simple"),
            ("dual_double_roots", "observed_double"), ("dual_regular_x", "regular_x_count"),
        ]:
            if int(cell[protocol_name]) != computed[computed_name]:
                locked_mismatches.append({"field": protocol_name, "locked": cell[protocol_name], "computed": computed[computed_name]})
        key = tuple(int(cell[name]) for name in ["p", "a", "b"])
        value = (str(computed["numerator_first"]), str(computed["denominator_first"]))
        if key in recovered:
            family_stable &= recovered[key] == value
        recovered[key] = value
        accepted = (
            not mismatches and not locked_mismatches and producer["accepted"] == "1"
            and all(int(producer[name]) >= 0 for name in [
                "extension_additions", "extension_multiplications", "extension_inversions",
                "point_additions", "scalar_multiplications", "elapsed_ns",
            ])
            and computed["regular_x_count"] == computed["predicted_regular_x"]
            and computed["pole_x_count"] == computed["predicted_pole_roots"]
            and computed["pole_multiplicity"] + 1 == computed["base_count"]
            and computed["observed_simple"] == computed["predicted_simple"]
            and computed["observed_double"] == computed["predicted_double"]
            and computed["observed_higher"] == 0
            and computed["denominator_roots_all"] == computed["pole_x_count"]
            and computed["first_rank"] == computed["window_size"]
            and computed["second_rank"] == computed["window_size"]
            and computed["numerator_degree"] == computed["base_count"]
            and computed["denominator_degree"] == computed["base_count"] - 1
            and computed["degree_pair_equal"] == 1 and computed["strict_pass"] == 1
            and computed["boundary_strict_pass"] == 0 and computed["pair_consistency_failures"] == 0
            and computed["windows_equal"] == 1 and computed["coefficients_base_field"] == 1
            and computed["regular_mismatches"] == 0 and computed["missed_poles"] == 0
            and computed["pole_cancellations"] == 0
            and computed["numerator_mutation_rejections"] == computed["regular_x_count"]
            and computed["denominator_mutation_destroyed_poles"] == computed["pole_x_count"]
            and computed["point_count"] == int(pointwise[slug]["extension_count"])
            and computed["composition_failures"] == 0
            and computed["omitted_minus_one_failures"] > 0 and computed["wrong_scalar_failures"] > 0
        )
        all_pass &= accepted
        reports.append({"slug": slug, "accepted": accepted, "mismatches": mismatches,
                        "locked_prediction_mismatches": locked_mismatches})
    all_pass &= family_stable and len(recovered) == 3
    report = {
        "schema": "h116.dual-pair-composition-verification/v1",
        "protocol_id": protocol["protocol_id"], "protocol_sha256": sha256(protocol_path),
        "pointwise_parent_sha256": sha256(pointwise_path), "v_minus_one_parent_sha256": sha256(parent_path),
        "result_sha256": sha256(result_path), "verifier_sha256": sha256(Path(__file__)),
        "cells": reports, "family_count": len(recovered), "family_stable": family_stable,
        "all_pass": all_pass, "elapsed_ns": time.perf_counter_ns() - started,
        "claim_boundary": protocol["claim_boundary"],
    }
    output_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    require(all_pass, "dual-pair composition verification failed")
    print("h116 dual-pair verifier: 8/8 cells accepted; 3/3 families stable")


def selftest():
    field = Field(5, first_irreducible(5, 2))
    root = field.element(7)
    polynomial = [field.mul(root, root), field.neg(field.add(root, root)), field.one]
    require(root_multiplicity(field, polynomial, root) == 2, "multiplicity selftest failed")
    print("h116 dual-pair verifier selftest ok")


def main():
    if sys.argv[1:] == ["--selftest"]:
        selftest()
        return
    require(len(sys.argv) == 6, "usage: verify.py PROTOCOL POINTWISE V_MINUS_ONE RESULT OUTPUT")
    verify(*(Path(argument) for argument in sys.argv[1:]))


if __name__ == "__main__":
    main()
