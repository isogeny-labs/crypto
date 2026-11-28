#!/usr/bin/env python3
"""Independent replay for H116-FROBENIUS-FIXED-LOCI-001."""

from __future__ import annotations

import csv
import hashlib
import json
import sys
import time
from pathlib import Path

from h116_pointwise_charpoly_verify import Curve, Field, first_irreducible, require


def characteristic(curve: Curve, point, trace: int, norm: int):
    pi = curve.frobenius(point)
    pi_squared = curve.frobenius(pi)
    return curve.add(curve.subtract(pi_squared, curve.scalar(trace, pi)), curve.scalar(norm, point))


def least_period(curve: Curve, point, degree: int) -> int | None:
    return next(
        (period for period in range(1, degree + 1)
         if curve.frobenius_power(point, period) == point),
        None,
    )


def encode_counts(counts: list[int]) -> str:
    return ",".join(f"{index}:{count}" for index, count in enumerate(counts[1:], start=1))


def compute_cell(cell: dict, parent: dict[str, str]) -> dict[str, int | str]:
    p = int(cell["p"])
    a = int(cell["a"])
    b = int(cell["b"])
    degree = int(cell["degree"])
    field = Field(p, first_irreducible(p, degree))
    curve = Curve(field, a, b)
    points = curve.enumerate_points()
    group_order = len(points)
    base_count = int(parent["base_count"])
    trace = int(parent["trace"])
    require(group_order == int(parent["extension_count"]), "parent count mismatch")

    period_histogram = [0] * (degree + 1)
    charpoly_failures = [0] * (degree + 1)
    wrong_trace_rejections = [0] * (degree + 1)
    wrong_norm_rejections = [0] * (degree + 1)
    period_failures = 0
    nondividing_periods = 0
    for point in points:
        period = least_period(curve, point, degree)
        if period is None:
            period_failures += 1
            continue
        period_histogram[period] += 1
        nondividing_periods += degree % period != 0
        charpoly_failures[period] += characteristic(curve, point, trace, p) is not None
        wrong_trace_rejections[period] += characteristic(curve, point, trace + 1, p) is not None
        wrong_norm_rejections[period] += characteristic(curve, point, trace, p + 1) is not None

    fixed_counts = [0] * (degree + 1)
    fixed_count_mismatches = 0
    for exponent in range(1, degree + 1):
        fixed_counts[exponent] = sum(
            curve.frobenius_power(point, exponent) == point for point in points
        )
        expected = sum(
            period_histogram[period]
            for period in range(1, degree + 1)
            if exponent % period == 0
        )
        fixed_count_mismatches += fixed_counts[exponent] != expected

    histogram_total = sum(period_histogram)
    proper_period_union = sum(
        period_histogram[period]
        for period in range(1, degree)
        if degree % period == 0
    )
    full_period_count = period_histogram[degree]
    return {
        "p": p,
        "degree": degree,
        "group_order": group_order,
        "base_count": base_count,
        "trace": trace,
        "period_histogram": encode_counts(period_histogram),
        "fixed_counts": encode_counts(fixed_counts),
        "charpoly_failure_histogram": encode_counts(charpoly_failures),
        "wrong_trace_rejection_histogram": encode_counts(wrong_trace_rejections),
        "wrong_norm_rejection_histogram": encode_counts(wrong_norm_rejections),
        "period_failures": period_failures,
        "nondividing_periods": nondividing_periods,
        "histogram_total": histogram_total,
        "fixed_count_mismatches": fixed_count_mismatches,
        "base_fixed_count": fixed_counts[1],
        "degree_fixed_count": fixed_counts[degree],
        "proper_period_union": proper_period_union,
        "full_period_count": full_period_count,
        "base_locus_omitted": group_order - fixed_counts[1],
        "proper_period_omitted": group_order - proper_period_union,
        "charpoly_failures": sum(charpoly_failures),
        "wrong_trace_rejections": sum(wrong_trace_rejections),
        "wrong_norm_rejections": sum(wrong_norm_rejections),
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
    producer_rows = {row["slug"]: row for row in read_tsv(result_path)}
    exact_fields = [
        "p", "degree", "group_order", "base_count", "trace", "period_histogram",
        "fixed_counts", "charpoly_failure_histogram", "wrong_trace_rejection_histogram",
        "wrong_norm_rejection_histogram", "period_failures", "nondividing_periods",
        "histogram_total", "fixed_count_mismatches", "base_fixed_count",
        "degree_fixed_count", "proper_period_union", "full_period_count",
        "base_locus_omitted", "proper_period_omitted", "charpoly_failures",
        "wrong_trace_rejections", "wrong_norm_rejections",
    ]
    reports = []
    all_pass = True
    for cell in protocol["cells"]:
        slug = cell["slug"]
        computed = compute_cell(cell, parent_rows[slug])
        producer = producer_rows[slug]
        mismatches = [
            {"field": name, "producer": producer[name], "verifier": str(computed[name])}
            for name in exact_fields if producer[name] != str(computed[name])
        ]
        diagnostics_valid = all(
            int(producer[name]) >= 0
            for name in ["frobenius_maps", "point_additions", "scalar_multiplications", "elapsed_ns"]
        )
        accepted_flag = producer["accepted"] == "1"
        accepted = (
            not mismatches
            and diagnostics_valid
            and accepted_flag
            and computed["period_failures"] == 0
            and computed["nondividing_periods"] == 0
            and computed["histogram_total"] == computed["group_order"]
            and computed["fixed_count_mismatches"] == 0
            and computed["base_fixed_count"] == computed["base_count"]
            and computed["degree_fixed_count"] == computed["group_order"]
            and computed["base_locus_omitted"] > 0
            and computed["proper_period_union"] < computed["group_order"]
            and computed["proper_period_omitted"] == computed["full_period_count"]
            and computed["full_period_count"] > 0
            and computed["charpoly_failures"] == 0
            and computed["wrong_trace_rejections"] > 0
            and computed["wrong_norm_rejections"] > 0
        )
        all_pass &= accepted
        reports.append({"slug": slug, "accepted": accepted, "mismatches": mismatches})
    report = {
        "schema": "h116.frobenius-fixed-loci-verification/v1",
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
    require(all_pass, "fixed-loci verification failed")
    print(f"h116 Frobenius fixed-loci verifier: {len(reports)}/{len(reports)} cells accepted")


def selftest() -> None:
    field = Field(3, first_irreducible(3, 2))
    curve = Curve(field, 1, 1)
    require(least_period(curve, None, 2) == 1, "infinity period failed")
    require(encode_counts([0, 3, 6]) == "1:3,2:6", "count encoding failed")
    print("h116 Frobenius fixed-loci verifier selftest ok")


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
