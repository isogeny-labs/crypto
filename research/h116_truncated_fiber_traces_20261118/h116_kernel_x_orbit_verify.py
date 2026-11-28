#!/usr/bin/env python3
"""Independent replay for H116-KERNEL-X-ORBIT-001."""

from __future__ import annotations

import csv
import hashlib
import json
import math
import sys
import time
from pathlib import Path

from h116_pointwise_charpoly_verify import Field, first_irreducible, require


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def evaluate(field: Field, polynomial: list[tuple[int, ...]], value: tuple[int, ...]):
    result = field.zero
    for coefficient in reversed(polynomial):
        result = field.add(field.mul(result, value), coefficient)
    return result


def derivative_evaluate(field: Field, polynomial: list[tuple[int, ...]], value: tuple[int, ...]):
    result = field.zero
    for index in range(len(polynomial) - 1, 0, -1):
        result = field.add(
            field.mul(result, value),
            field.mul(field.base(index), polynomial[index]),
        )
    return result


def divide_by_x_minus_root(
    field: Field, polynomial: list[tuple[int, ...]], root: tuple[int, ...]
) -> list[tuple[int, ...]]:
    require(len(polynomial) >= 2, "cannot divide constant polynomial")
    degree = len(polynomial) - 1
    quotient = [field.zero] * degree
    quotient[-1] = polynomial[-1]
    for index in range(degree - 1, 0, -1):
        quotient[index - 1] = field.add(polynomial[index], field.mul(root, quotient[index]))
    remainder = field.add(polynomial[0], field.mul(root, quotient[0]))
    require(remainder == field.zero, "synthetic division left nonzero remainder")
    return quotient


def root_multiplicity(
    field: Field, polynomial: list[tuple[int, ...]], root: tuple[int, ...]
) -> int:
    quotient = polynomial.copy()
    multiplicity = 0
    while len(quotient) >= 2 and evaluate(field, quotient, root) == field.zero:
        quotient = divide_by_x_minus_root(field, quotient, root)
        multiplicity += 1
    return multiplicity


def compute(cell: dict, parent: dict[str, str]) -> dict[str, int | str]:
    p, degree = int(cell["p"]), int(cell["degree"])
    require(int(parent["p"]) == p and int(parent["degree"]) == degree, "parent cell mismatch")
    require(parent["accepted"] == "1", "unaccepted parent row")
    order, trace = int(parent["base_count"]), int(parent["trace"])
    field = Field(p, first_irreducible(p, degree))
    numerator = [field.base(int(coefficient)) for coefficient in parent["numerator_first"].split(",")]
    denominator = [field.base(int(coefficient)) for coefficient in parent["denominator_first"].split(",")]
    require(all(0 <= int(value) < p for value in parent["numerator_first"].split(",")), "non-base numerator")
    require(all(0 <= int(value) < p for value in parent["denominator_first"].split(",")), "non-base denominator")
    require(len(denominator) == order and denominator[-1] == field.one, "bad denominator degree")

    lambda_value = (trace - 1) % order
    require(math.gcd(lambda_value, order) == 1, "noninvertible cyclic action")
    lambda_power = pow(lambda_value, degree, order)
    g_plus = math.gcd(order, (lambda_power - 1) % order)
    g_minus = math.gcd(order, (lambda_power + 1) % order)
    g_intersection = math.gcd(g_plus, g_minus)
    union_size = g_plus + g_minus - g_intersection
    epsilon = int(order % 2 == 0)
    require((union_size - 1 + epsilon) % 2 == 0, "nonintegral orbit prediction")
    predicted_roots = (union_size - 1 + epsilon) // 2
    predicted_simple = epsilon
    predicted_double = predicted_roots - epsilon
    predicted_weighted = predicted_simple + 2 * predicted_double
    plus_only = g_plus // 2

    denominator_mutation = denominator.copy()
    denominator_mutation[0] = field.add(denominator_mutation[0], field.one)
    roots: list[int] = []
    multiplicities: list[int] = []
    derivative_failures = 0
    numerator_cancellations = 0
    mutation_survivors = 0
    for index in range(field.size):
        x_value = field.element(index)
        if evaluate(field, denominator, x_value) != field.zero:
            continue
        roots.append(index)
        multiplicity = root_multiplicity(field, denominator, x_value)
        multiplicities.append(multiplicity)
        derivative_zero = derivative_evaluate(field, denominator, x_value) == field.zero
        derivative_failures += derivative_zero != (multiplicity >= 2)
        numerator_cancellations += evaluate(field, numerator, x_value) == field.zero
        mutation_survivors += evaluate(field, denominator_mutation, x_value) == field.zero
    simple = multiplicities.count(1)
    double = multiplicities.count(2)
    higher = sum(multiplicity not in (1, 2) for multiplicity in multiplicities)
    parent_poles = int(parent["pole_x_count"])
    return {
        "p": p,
        "degree": degree,
        "base_count": order,
        "trace": trace,
        "lambda": lambda_value,
        "lambda_power": lambda_power,
        "g_plus": g_plus,
        "g_minus": g_minus,
        "g_intersection": g_intersection,
        "union_size": union_size,
        "order_two_indicator": epsilon,
        "plus_only_roots": plus_only,
        "predicted_roots": predicted_roots,
        "predicted_simple": predicted_simple,
        "predicted_double": predicted_double,
        "predicted_weighted": predicted_weighted,
        "observed_roots": len(roots),
        "observed_simple": simple,
        "observed_double": double,
        "observed_higher": higher,
        "observed_weighted": sum(multiplicities),
        "parent_point_visible_roots": parent_poles,
        "non_point_visible_roots": len(roots) - parent_poles,
        "sign_sensitive": int(predicted_roots > plus_only),
        "root_indices": ",".join(map(str, roots)) if roots else "-",
        "numerator_cancellations": numerator_cancellations,
        "mutation_survivors": mutation_survivors,
        "derivative_classification_failures": derivative_failures,
        "denominator_degree": len(denominator) - 1,
    }


def verify(
    protocol_path: Path,
    parent_path: Path,
    result_path: Path,
    output_path: Path,
) -> None:
    started = time.perf_counter_ns()
    protocol = json.loads(protocol_path.read_text(encoding="utf-8"))
    require(sha256(parent_path) == protocol["parent_result_sha256"], "parent result hash mismatch")
    parents = {row["slug"]: row for row in read_tsv(parent_path)}
    producers = {row["slug"]: row for row in read_tsv(result_path)}
    locked = {row["slug"]: row for row in protocol["locked_predictions"]}
    require(set(parents) == set(producers) == {cell["slug"] for cell in protocol["cells"]}, "cell set mismatch")
    exact_fields = [
        "p", "degree", "base_count", "trace", "lambda", "lambda_power", "g_plus", "g_minus",
        "g_intersection", "union_size", "order_two_indicator", "plus_only_roots",
        "predicted_roots", "predicted_simple", "predicted_double", "predicted_weighted",
        "observed_roots", "observed_simple", "observed_double", "observed_higher",
        "observed_weighted", "parent_point_visible_roots", "non_point_visible_roots",
        "sign_sensitive", "root_indices", "numerator_cancellations", "mutation_survivors",
        "derivative_classification_failures", "denominator_degree",
    ]
    reports = []
    sensitive: list[str] = []
    all_pass = True
    for cell in protocol["cells"]:
        slug = cell["slug"]
        computed = compute(cell, parents[slug])
        producer = producers[slug]
        mismatches = [
            {"field": name, "producer": producer[name], "verifier": str(computed[name])}
            for name in exact_fields if producer[name] != str(computed[name])
        ]
        locked_mismatches = []
        locked_map = {
            "g_plus": "g_plus", "g_minus": "g_minus", "g_intersection": "g_intersection",
            "roots": "predicted_roots", "simple": "predicted_simple", "double": "predicted_double",
            "weighted": "predicted_weighted", "plus_only": "plus_only_roots",
        }
        for locked_name, computed_name in locked_map.items():
            if int(locked[slug][locked_name]) != computed[computed_name]:
                locked_mismatches.append({
                    "field": locked_name,
                    "locked": locked[slug][locked_name],
                    "computed": computed[computed_name],
                })
        if computed["sign_sensitive"]:
            sensitive.append(slug)
        accepted = (
            not mismatches and not locked_mismatches and producer["accepted"] == "1"
            and all(int(producer[name]) >= 0 for name in ["extension_additions", "extension_multiplications", "elapsed_ns"])
            and computed["observed_roots"] == computed["predicted_roots"]
            and computed["observed_simple"] == computed["predicted_simple"]
            and computed["observed_double"] == computed["predicted_double"]
            and computed["observed_higher"] == 0
            and computed["observed_weighted"] == computed["predicted_weighted"]
            and computed["parent_point_visible_roots"] == computed["plus_only_roots"]
            and computed["non_point_visible_roots"] == computed["predicted_roots"] - computed["plus_only_roots"]
            and computed["numerator_cancellations"] == 0
            and computed["mutation_survivors"] == 0
            and computed["derivative_classification_failures"] == 0
            and computed["denominator_degree"] == computed["base_count"] - 1
        )
        all_pass &= accepted
        reports.append({
            "slug": slug,
            "accepted": accepted,
            "mismatches": mismatches,
            "locked_prediction_mismatches": locked_mismatches,
        })
    expected_sensitive = ["fp5-a2-b1-d3", "fp7-a1-b1-d2"]
    sign_control_pass = sensitive == expected_sensitive
    all_pass &= sign_control_pass
    directory = protocol_path.parent
    retained_hashes = {
        "v1_result": sha256(directory / "POINTWISE_RATIONAL_RECONSTRUCTION_RESULT.tsv"),
        "v2_result": sha256(parent_path),
    }
    retention_pass = retained_hashes == {
        "v1_result": "d7ff21da47cd866f21fdb21422d0dbf48cd661e65e5963c32d6a2cc114faead9",
        "v2_result": protocol["parent_result_sha256"],
    }
    all_pass &= retention_pass
    report = {
        "schema": "h116.kernel-x-orbit-verification/v1",
        "protocol_id": protocol["protocol_id"],
        "protocol_sha256": sha256(protocol_path),
        "parent_sha256": sha256(parent_path),
        "result_sha256": sha256(result_path),
        "verifier_sha256": sha256(Path(__file__)),
        "cells": reports,
        "sign_sensitive_cells": sensitive,
        "expected_sign_sensitive_cells": expected_sensitive,
        "sign_control_pass": sign_control_pass,
        "retained_hashes": retained_hashes,
        "retention_pass": retention_pass,
        "all_pass": all_pass,
        "elapsed_ns": time.perf_counter_ns() - started,
        "claim_boundary": protocol["claim_boundary"],
    }
    output_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    require(all_pass, "kernel x-orbit verification failed")
    print("h116 kernel x-orbit verifier: 8/8 cells accepted; 2/2 sign-sensitive controls rejected plus-only")


def selftest() -> None:
    require(math.gcd(14, 8) == 2, "gcd selftest failed")
    field = Field(5, first_irreducible(5, 2))
    root = field.element(7)
    polynomial = [field.mul(root, root), field.neg(field.add(root, root)), field.one]
    require(root_multiplicity(field, polynomial, root) == 2, "multiplicity selftest failed")
    require(derivative_evaluate(field, polynomial, root) == field.zero, "derivative selftest failed")
    print("h116 kernel x-orbit verifier selftest ok")


def main() -> None:
    if sys.argv[1:] == ["--selftest"]:
        selftest()
        return
    require(len(sys.argv) == 5, "usage: verify.py PROTOCOL PARENT RESULT OUTPUT")
    verify(*(Path(argument) for argument in sys.argv[1:]))


if __name__ == "__main__":
    main()
