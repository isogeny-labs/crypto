#!/usr/bin/env python3
"""Independent replay for H116-RATIONAL-SEPARATION-CAPACITY-001."""

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


def vanishing_polynomial(field: Field, samples: list[Element]) -> list[Element]:
    coefficients = [field.one]
    for root in samples:
        coefficients = multiply_by_linear(field, coefficients, root)
    return coefficients


def evaluate_polynomial(field: Field, coefficients: list[Element], value: Element) -> Element:
    result = field.zero
    for coefficient in reversed(coefficients):
        result = field.add(field.mul(result, value), coefficient)
    return result


def encode_coefficients(field: Field, coefficients: list[Element]) -> str:
    return ",".join(str(element_index(field, coefficient)) for coefficient in coefficients)


def base_distinct_x_count(p: int, a: int, b: int) -> int:
    return sum(
        any(y * y % p == (x**3 + a * x + b) % p for y in range(p))
        for x in range(p)
    )


def compute_cell(cell: dict, parent_group_order: int) -> dict[str, int | str]:
    p = int(cell["p"])
    a = int(cell["a"])
    b = int(cell["b"])
    degree = int(cell["degree"])
    field = Field(p, first_irreducible(p, degree))
    curve = Curve(field, a, b)
    points = curve.enumerate_points()
    fibers: dict[Element, list[Element]] = {}
    for point in points:
        if point is not None:
            x, y = point
            fibers.setdefault(x, []).append(y)
    samples = sorted(fibers, key=lambda value: element_index(field, value))
    singleton_fibers = sum(len(values) == 1 and values[0] == field.zero for values in fibers.values())
    paired_fibers = sum(
        len(values) == 2 and values[1] == field.neg(values[0])
        for values in fibers.values()
    )
    require(singleton_fibers + paired_fibers == len(fibers), "unexpected affine x-fiber")
    coefficients = vanishing_polynomial(field, samples)
    vanishing_eval_failures = sum(
        evaluate_polynomial(field, coefficients, x) != field.zero for x in samples
    )
    mutation = coefficients.copy()
    mutation[0] = field.add(mutation[0], field.one)
    mutation_rejections = sum(
        evaluate_polynomial(field, mutation, x) != field.zero for x in samples
    )
    distinct_x_count = len(samples)
    base_x_count = base_distinct_x_count(p, a, b)
    return {
        "p": p,
        "degree": degree,
        "parent_group_order": parent_group_order,
        "affine_point_count": len(points) - 1,
        "singleton_fibers": singleton_fibers,
        "paired_fibers": paired_fibers,
        "distinct_x_count": distinct_x_count,
        "base_distinct_x_count": base_x_count,
        "extension_x_growth": distinct_x_count - base_x_count,
        "extension_to_base_ratio": f"{distinct_x_count}/{base_x_count}",
        "raw_strict_degree_capacity": distinct_x_count - 1,
        "vanishing_degree": len(coefficients) - 1,
        "vanishing_monic": int(coefficients[-1] == field.one),
        "vanishing_nonzero": int(any(coefficient != field.zero for coefficient in coefficients)),
        "vanishing_coefficients": encode_coefficients(field, coefficients),
        "vanishing_eval_failures": vanishing_eval_failures,
        "mutation_rejections": mutation_rejections,
        "fiber_count_identity": int(
            len(points) - 1 == singleton_fibers + 2 * paired_fibers
            and distinct_x_count == singleton_fibers + paired_fibers
        ),
    }


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify(protocol_path: Path, parent_path: Path, result_path: Path, output_path: Path) -> None:
    started = time.perf_counter_ns()
    protocol = json.loads(protocol_path.read_text(encoding="utf-8"))
    parents = {row["slug"]: row for row in read_tsv(parent_path)}
    producer_rows = {row["slug"]: row for row in read_tsv(result_path)}
    exact_fields = [
        "p", "degree", "parent_group_order", "affine_point_count", "singleton_fibers",
        "paired_fibers", "distinct_x_count", "base_distinct_x_count", "extension_x_growth",
        "extension_to_base_ratio", "raw_strict_degree_capacity", "vanishing_degree",
        "vanishing_monic", "vanishing_nonzero", "vanishing_coefficients",
        "vanishing_eval_failures", "mutation_rejections", "fiber_count_identity",
    ]
    diagnostic_fields = [
        "build_extension_additions", "build_extension_multiplications",
        "vanishing_eval_extension_additions", "vanishing_eval_extension_multiplications",
        "mutation_eval_extension_additions", "mutation_eval_extension_multiplications",
        "elapsed_ns",
    ]
    reports = []
    all_pass = True
    any_growth = False
    for cell in protocol["cells"]:
        slug = cell["slug"]
        parent_group_order = int(parents[slug]["extension_count"])
        computed = compute_cell(cell, parent_group_order)
        producer = producer_rows[slug]
        mismatches = [
            {"field": name, "producer": producer[name], "verifier": str(computed[name])}
            for name in exact_fields
            if producer[name] != str(computed[name])
        ]
        diagnostics_valid = all(int(producer[name]) >= 0 for name in diagnostic_fields)
        accepted = (
            not mismatches
            and diagnostics_valid
            and producer["accepted"] == "1"
            and computed["parent_group_order"] == computed["affine_point_count"] + 1
            and computed["fiber_count_identity"] == 1
            and computed["raw_strict_degree_capacity"] == computed["distinct_x_count"] - 1
            and computed["vanishing_degree"] == computed["distinct_x_count"]
            and computed["vanishing_monic"] == 1
            and computed["vanishing_nonzero"] == 1
            and computed["vanishing_eval_failures"] == 0
            and computed["mutation_rejections"] == computed["distinct_x_count"]
        )
        any_growth |= computed["extension_x_growth"] > 0
        all_pass &= accepted
        reports.append({"slug": slug, "accepted": accepted, "mismatches": mismatches})
    all_pass &= any_growth
    report = {
        "schema": "h116.rational-separation-capacity-verification/v1",
        "protocol_id": protocol["protocol_id"],
        "protocol_sha256": sha256(protocol_path),
        "parent_sha256": sha256(parent_path),
        "result_sha256": sha256(result_path),
        "verifier_sha256": sha256(Path(__file__)),
        "cells": reports,
        "all_pass": all_pass,
        "extension_growth_control": any_growth,
        "elapsed_ns": time.perf_counter_ns() - started,
        "claim_boundary": protocol["claim_boundary"],
    }
    output_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    require(all_pass, "rational-separation capacity verification failed")
    print(f"h116 rational separation verifier: {len(reports)}/{len(reports)} cells accepted")


def selftest() -> None:
    field = Field(3, [0, 1])
    samples = [field.element(index) for index in range(3)]
    coefficients = vanishing_polynomial(field, samples)
    require(encode_coefficients(field, coefficients) == "0,2,0,1", "F3 polynomial mismatch")
    require(
        all(evaluate_polynomial(field, coefficients, x) == field.zero for x in samples),
        "F3 evaluation mismatch",
    )
    print("h116 rational separation verifier selftest ok")


def main() -> None:
    if sys.argv[1:] == ["--selftest"]:
        selftest()
        return
    require(
        len(sys.argv) == 5,
        "usage: verify.py PROTOCOL.json PARENT.tsv RESULT.tsv OUTPUT.json",
    )
    verify(Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3]), Path(sys.argv[4]))


if __name__ == "__main__":
    main()
