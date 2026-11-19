#!/usr/bin/env python3
"""Independent formula replay for SCREEN.json."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from fractions import Fraction
from pathlib import Path
from typing import Any


TARGET_R = 2417851639230796216685689


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def as_fraction(record: dict[str, Any]) -> Fraction:
    value = Fraction(record["numerator"], record["denominator"])
    assert record["text"] == (
        str(value.numerator)
        if value.denominator == 1
        else f"{value.numerator}/{value.denominator}"
    )
    assert math.isclose(record["decimal"], float(value), rel_tol=0, abs_tol=1e-15)
    return value


def close(actual: float, expected: float) -> None:
    assert math.isclose(actual, expected, rel_tol=0, abs_tol=1e-12), (
        actual,
        expected,
    )


def validate(screen_path: Path) -> dict[str, Any]:
    root = screen_path.resolve().parent
    data = json.loads(screen_path.read_text(encoding="utf-8"))
    checks: list[str] = []

    assert data["schema"] == "n83-higher-degree-image-rr-screen-v1"
    assert data["registration"]["protocol_sha256"] == sha256(root / "PROTOCOL.md")
    assert data["registration"]["requirements_sha256"] == sha256(
        root / "REQUIREMENTS.md"
    )
    checks.append("registration_hashes")

    target = data["target"]
    assert target["order_r"] == TARGET_R
    log2_r = math.log2(TARGET_R)
    close(target["log2_r"], log2_r)
    assert as_fraction(target["rho_exponent"]) == Fraction(1, 2)
    close(target["rho_leading_bits"], log2_r / 2)
    checks.append("target_and_rho")

    rows = data["complete_rr_degrees"]
    assert [row["degree"] for row in rows] == list(range(5, 32))
    for row in rows:
        degree = row["degree"]
        assert row["rr_projective_dimension"] == degree - 3
        assert row["candidate_count_q_exponent"] == degree - 3
        assert [item["arity"] for item in row["arities"]] == list(
            range(1, degree + 1)
        )
        for item in row["arities"]:
            arity = item["arity"]
            if arity == 1:
                assert item["independent_relation"] is False
                assert item["small_base"] is None
                assert item["double_large_prime"] is None
                assert item["all_large_prime"] is None
                assert as_fraction(item["generic_lookup"]["exponent"]) == Fraction(
                    1, 2
                )
                assert item["advances"] is False
                continue

            small = Fraction(2 * arity, arity + 1)
            double_large = Fraction(2 * arity - 2, arity)
            all_large = Fraction(1, 1)
            assert as_fraction(item["small_base"]["factor_base_exponent"]) == Fraction(
                arity, arity + 1
            )
            assert as_fraction(item["small_base"]["total_exponent"]) == small
            assert as_fraction(
                item["double_large_prime"]["factor_base_exponent"]
            ) == Fraction(arity - 1, arity)
            assert as_fraction(
                item["double_large_prime"]["total_exponent"]
            ) == double_large
            assert as_fraction(item["all_large_prime"]["total_exponent"]) == all_large
            assert as_fraction(item["best_registered_exponent"]) == min(
                small, double_large, all_large
            )
            assert item["advances"] is False
    checks.append("rr_dimensions_and_arity_grid")

    ladder = data["inverse_cantor_ladder"]
    assert len(ladder) == 14
    for item in ladder:
        parameter_degree = item["k_degree"]
        degree = 2 * parameter_degree + 3
        parameter_dimension = parameter_degree + 1
        complete_dimension = degree - 3
        assert item["effective_degree"] == degree
        assert item["affine_parameter_dimension"] == parameter_dimension
        assert item["complete_rr_projective_dimension"] == complete_dimension
        assert item["dimension_deficit"] == complete_dimension - parameter_dimension
        assert item["complete_affine_chart"] is (degree == 5)
    checks.append("inverse_cantor_degree_ladder")

    expected_highlights = {
        "degree5_full_split_small_base": Fraction(5, 3),
        "degree5_full_split_double_large_prime": Fraction(8, 5),
        "degree5_two_component_all_large_graph": Fraction(1, 1),
        "degree5_one_component_generic_lookup": Fraction(1, 2),
    }
    assert {item["name"] for item in data["highlights"]} == set(expected_highlights)
    for item in data["highlights"]:
        exponent = expected_highlights[item["name"]]
        assert as_fraction(item["exponent"]) == exponent
        expected_bits = float(exponent) * log2_r
        close(item["leading_bits"], expected_bits)
        close(item["gap_vs_rho_bits"], expected_bits - log2_r / 2)
        assert item["beats_rho_exponent"] is (exponent < Fraction(1, 2))
    checks.append("n83_highlights")

    ambient = data["ambient_control"]
    assert as_fraction(ambient["q_exponent"]) == Fraction(4, 3)
    close(ambient["leading_bits"], 83 * 4 / 3)
    close(ambient["gap_vs_rho_bits"], 83 * 4 / 3 - log2_r / 2)
    checks.append("ambient_control")

    assert data["survivors"] == []
    assert data["decision"] == "stop_before_measurement"
    checks.append("strict_advancement_gate")

    return {
        "schema": "n83-higher-degree-image-rr-validation-v1",
        "screen_sha256": sha256(screen_path),
        "status": "pass",
        "check_count": len(checks),
        "checks": checks,
        "note": "Producer-authored independent formula replay; not an independent PCC verifier.",
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("screen", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    report = validate(args.screen)
    encoded = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if args.output:
        args.output.write_text(encoded, encoding="utf-8")
    else:
        print(encoded, end="")


if __name__ == "__main__":
    main()
