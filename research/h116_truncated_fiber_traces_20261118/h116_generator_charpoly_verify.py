#!/usr/bin/env python3
"""Independent replay for H116-GENERATOR-CHARPOLY-002."""

from __future__ import annotations

import csv
import hashlib
import json
import sys
import time
from pathlib import Path

from h116_pointwise_charpoly_verify import Curve, Field, first_irreducible, require


def prime_divisors(value: int) -> list[int]:
    result = []
    divisor = 2
    while divisor * divisor <= value:
        if value % divisor == 0:
            result.append(divisor)
            while value % divisor == 0:
                value //= divisor
        divisor += 1
    if value > 1:
        result.append(value)
    return result


def element_index(field: Field, value: tuple[int, ...]) -> int:
    return sum(coefficient * field.p**index for index, coefficient in enumerate(value))


def point_key(field: Field, point):
    if point is None:
        return 0, 0, 0
    return 1, element_index(field, point[0]), element_index(field, point[1])


def encode_point(field: Field, point) -> str:
    if point is None:
        return "O"
    return f"{element_index(field, point[0])}:{element_index(field, point[1])}"


def point_order(curve: Curve, point, group_order: int) -> int:
    require(curve.scalar(group_order, point) is None, "group order does not kill point")
    order = group_order
    for prime in prime_divisors(group_order):
        while order % prime == 0 and curve.scalar(order // prime, point) is None:
            order //= prime
    return order


def repeated_order(curve: Curve, point, bound: int) -> int:
    total = None
    for order in range(1, bound + 1):
        total = curve.add(total, point)
        if total is None:
            return order
    raise ValueError("repeated order exceeded group order")


def cyclic_points(curve: Curve, point, order: int) -> list:
    result = []
    multiple = None
    for _ in range(order):
        result.append(multiple)
        multiple = curve.add(multiple, point)
    require(multiple is None, "cyclic enumeration did not close")
    return result


def pair_closure(curve: Curve, first, first_order: int, second, second_order: int) -> set:
    return {
        curve.add(left, right)
        for left in cyclic_points(curve, first, first_order)
        for right in cyclic_points(curve, second, second_order)
    }


def characteristic(curve: Curve, point, trace: int, norm: int):
    pi = curve.frobenius(point)
    pi_squared = curve.frobenius(pi)
    return curve.add(curve.subtract(pi_squared, curve.scalar(trace, pi)), curve.scalar(norm, point))


def select_generators(p: int, a: int, b: int, degree: int, trace: int, parent_count: int) -> dict:
    modulus = first_irreducible(p, degree)
    field = Field(p, modulus)
    curve = Curve(field, a, b)
    points = sorted(curve.enumerate_points(), key=lambda point: point_key(field, point))
    require(len(points) == parent_count, "parent count mismatch")
    orders = [point_order(curve, point, len(points)) for point in points]
    maximum_order = max(orders)
    first_index = orders.index(maximum_order)
    first = points[first_index]
    require(first is not None, "identity selected")
    first_closure = set(cyclic_points(curve, first, maximum_order))
    if len(first_closure) == len(points):
        generators = [first]
        generator_orders = [maximum_order]
        closure = first_closure
    else:
        selection = None
        for second, second_order in zip(points[1:], orders[1:], strict=True):
            candidate = pair_closure(curve, first, maximum_order, second, second_order)
            if len(candidate) == len(points):
                selection = second, second_order, candidate
                break
        require(selection is not None, "second generator absent")
        second, second_order, closure = selection
        generators = [first, second]
        generator_orders = [maximum_order, second_order]
    for generator, order in zip(generators, generator_orders, strict=True):
        require(repeated_order(curve, generator, len(points)) == order, "order confirmation failed")

    charpoly_failures = sum(characteristic(curve, point, trace, p) is not None for point in generators)
    wrong_trace_rejections = sum(
        characteristic(curve, point, trace + 1, p) is not None for point in generators
    )
    wrong_norm_rejections = sum(
        characteristic(curve, point, trace, p + 1) is not None for point in generators
    )
    missing_inverse_rejections = 0
    for point in generators:
        expected = curve.subtract(curve.scalar(trace, point), curve.frobenius(point))
        inverse_pi = curve.frobenius_power(point, degree - 1)
        require(curve.scalar(p, inverse_pi) == expected, "positive V identity failed")
        missing_inverse_rejections += curve.scalar(p, point) != expected
    dropped_closure_size = 1 if len(generators) == 1 else len(first_closure)
    second_encoding = "-" if len(generators) == 1 else encode_point(field, generators[1])
    second_order = 0 if len(generators) == 1 else generator_orders[1]
    return {
        "group_order": len(points),
        "trace": trace,
        "generator_count": len(generators),
        "generator_1": encode_point(field, generators[0]),
        "order_1": generator_orders[0],
        "generator_2": second_encoding,
        "order_2": second_order,
        "closure_size": len(closure),
        "dropped_closure_size": dropped_closure_size,
        "charpoly_failures": charpoly_failures,
        "wrong_trace_rejections": wrong_trace_rejections,
        "wrong_norm_rejections": wrong_norm_rejections,
        "missing_inverse_rejections": missing_inverse_rejections,
        "exhaustive_checks": len(points),
        "generator_checks": len(generators),
        "checks_saved": len(points) - len(generators),
    }


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def verify(protocol_path: Path, parent_path: Path, result_path: Path, output_path: Path) -> None:
    started = time.perf_counter_ns()
    protocol = json.loads(protocol_path.read_text(encoding="utf-8"))
    parent_rows = {row["slug"]: row for row in read_tsv(parent_path)}
    result_rows = {row["slug"]: row for row in read_tsv(result_path)}
    cell_protocol = json.loads((protocol_path.parent / "POINTWISE_CHARPOLY_PROTOCOL.json").read_text())
    exact_fields = [
        "group_order", "trace", "generator_count", "generator_1", "order_1",
        "generator_2", "order_2", "closure_size", "dropped_closure_size",
        "charpoly_failures", "wrong_trace_rejections", "wrong_norm_rejections",
        "missing_inverse_rejections", "exhaustive_checks", "generator_checks", "checks_saved",
    ]
    reports = []
    all_pass = True
    for cell in cell_protocol["cells"]:
        slug = cell["slug"]
        parent = parent_rows[slug]
        producer = result_rows[slug]
        computed = select_generators(
            int(cell["p"]), int(cell["a"]), int(cell["b"]), int(cell["degree"]),
            int(parent["trace"]), int(parent["extension_count"]),
        )
        mismatches = [
            {"field": name, "producer": producer[name], "verifier": str(computed[name])}
            for name in exact_fields if producer[name] != str(computed[name])
        ]
        diagnostics_valid = int(producer["elapsed_ns"]) >= 0
        accepted = (
            not mismatches
            and diagnostics_valid
            and 1 <= computed["generator_count"] <= 2
            and computed["closure_size"] == computed["group_order"]
            and computed["dropped_closure_size"] < computed["group_order"]
            and computed["charpoly_failures"] == 0
            and computed["wrong_trace_rejections"] > 0
            and computed["wrong_norm_rejections"] > 0
        )
        all_pass &= accepted
        reports.append({"slug": slug, "accepted": accepted, "mismatches": mismatches})
    report = {
        "schema": "h116.generator-charpoly-certificate-verification/v1",
        "protocol_id": protocol["protocol_id"],
        "protocol_sha256": sha256(protocol_path),
        "parent_sha256": sha256(parent_path),
        "result_sha256": sha256(result_path),
        "verifier_sha256": sha256(Path(__file__)),
        "cells": reports,
        "all_pass": all_pass,
        "elapsed_ns": time.perf_counter_ns() - started,
        "claim_boundary": protocol["claim_boundary"],
    }
    output_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    require(all_pass, "generator certificate verification failed")
    print(f"h116 generator charpoly verifier: {len(reports)}/{len(reports)} cells accepted")


def selftest() -> None:
    require(prime_divisors(60) == [2, 3, 5], "factor selftest failed")
    field = Field(3, first_irreducible(3, 2))
    require(element_index(field, field.element(7)) == 7, "encoding selftest failed")
    print("h116 generator charpoly verifier selftest ok")


def main() -> None:
    if sys.argv[1:] == ["--selftest"]:
        selftest()
        return
    require(len(sys.argv) == 5, "usage: verify.py PROTOCOL.json PARENT.tsv RESULT.tsv OUTPUT.json")
    verify(Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3]), Path(sys.argv[4]))


if __name__ == "__main__":
    main()
