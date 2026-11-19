#!/usr/bin/env python3
"""Exact exponent screen for higher-degree projected RR relations."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from fractions import Fraction
from pathlib import Path


TARGET_R = 2417851639230796216685689
FIELD_LOG2 = 83
MIN_DEGREE = 5
MAX_DEGREE = 31


def fraction_record(value: Fraction) -> dict[str, int | float | str]:
    return {
        "numerator": value.numerator,
        "denominator": value.denominator,
        "text": (
            str(value.numerator)
            if value.denominator == 1
            else f"{value.numerator}/{value.denominator}"
        ),
        "decimal": float(value),
    }


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def exponent_bits(exponent: Fraction, log2_order: float) -> float:
    return float(exponent) * log2_order


def arity_record(arity: int) -> dict[str, object]:
    if arity == 1:
        return {
            "arity": 1,
            "independent_relation": False,
            "small_base": None,
            "double_large_prime": None,
            "all_large_prime": None,
            "generic_lookup": {
                "exponent": fraction_record(Fraction(1, 2)),
                "interpretation": "BSGS/rho parity; no independent row",
            },
            "advances": False,
        }

    small = Fraction(2 * arity, arity + 1)
    double_large = Fraction(2 * arity - 2, arity)
    all_large = Fraction(1, 1)
    return {
        "arity": arity,
        "independent_relation": True,
        "small_base": {
            "factor_base_exponent": fraction_record(
                Fraction(arity, arity + 1)
            ),
            "total_exponent": fraction_record(small),
        },
        "double_large_prime": {
            "factor_base_exponent": fraction_record(
                Fraction(arity - 1, arity)
            ),
            "total_exponent": fraction_record(double_large),
        },
        "all_large_prime": {
            "total_exponent": fraction_record(all_large),
            "interpretation": "optimistic random-hypergraph edge/memory floor",
        },
        "best_registered_exponent": fraction_record(
            min(small, double_large, all_large)
        ),
        "advances": min(small, double_large, all_large) < Fraction(1, 2),
    }


def degree_record(degree: int) -> dict[str, object]:
    return {
        "degree": degree,
        "rr_projective_dimension": degree - 3,
        "candidate_count_q_exponent": degree - 3,
        "arities": [arity_record(arity) for arity in range(1, degree + 1)],
    }


def highlighted_route(
    name: str, exponent: Fraction, log2_order: float, rho_bits: float
) -> dict[str, object]:
    bits = exponent_bits(exponent, log2_order)
    return {
        "name": name,
        "exponent": fraction_record(exponent),
        "leading_bits": bits,
        "gap_vs_rho_bits": bits - rho_bits,
        "beats_rho_exponent": exponent < Fraction(1, 2),
    }


def build_screen(root: Path) -> dict[str, object]:
    protocol = root / "PROTOCOL.md"
    requirements = root / "REQUIREMENTS.md"
    log2_order = math.log2(TARGET_R)
    rho_bits = log2_order / 2.0

    inverse_cantor = []
    for parameter_degree in range(1, 15):
        degree = 2 * parameter_degree + 3
        if degree > MAX_DEGREE:
            break
        parameter_dimension = parameter_degree + 1
        complete_dimension = degree - 3
        inverse_cantor.append(
            {
                "k_degree": parameter_degree,
                "effective_degree": degree,
                "affine_parameter_dimension": parameter_dimension,
                "complete_rr_projective_dimension": complete_dimension,
                "dimension_deficit": complete_dimension - parameter_dimension,
                "complete_affine_chart": degree == 5,
            }
        )

    highlights = [
        highlighted_route(
            "degree5_full_split_small_base",
            Fraction(5, 3),
            log2_order,
            rho_bits,
        ),
        highlighted_route(
            "degree5_full_split_double_large_prime",
            Fraction(8, 5),
            log2_order,
            rho_bits,
        ),
        highlighted_route(
            "degree5_two_component_all_large_graph",
            Fraction(1, 1),
            log2_order,
            rho_bits,
        ),
        highlighted_route(
            "degree5_one_component_generic_lookup",
            Fraction(1, 2),
            log2_order,
            rho_bits,
        ),
    ]

    degree_rows = [
        degree_record(degree) for degree in range(MIN_DEGREE, MAX_DEGREE + 1)
    ]
    survivors = [
        {
            "degree": row["degree"],
            "arity": arity["arity"],
        }
        for row in degree_rows
        for arity in row["arities"]
        if arity["advances"]
    ]

    return {
        "schema": "n83-higher-degree-image-rr-screen-v1",
        "claim_scope": "exact symbolic cost-model screen; no relation measurement",
        "registration": {
            "protocol_sha256": sha256(protocol),
            "requirements_sha256": sha256(requirements),
        },
        "target": {
            "field_log2": FIELD_LOG2,
            "order_r": TARGET_R,
            "log2_r": log2_order,
            "rho_exponent": fraction_record(Fraction(1, 2)),
            "rho_leading_bits": rho_bits,
        },
        "favorable_grants": [
            "unit-cost factorization, norm, projection and membership",
            "unlimited candidate supply within the complete linear system",
            "uniform independent nonidentity projected labels",
            "independent useful rows until full rank",
        ],
        "complete_rr_degrees": degree_rows,
        "inverse_cantor_ladder": inverse_cantor,
        "highlights": highlights,
        "ambient_control": {
            "name": "ordinary_full_jacobian_genus3",
            "q_exponent": fraction_record(Fraction(4, 3)),
            "leading_bits": FIELD_LOG2 * 4.0 / 3.0,
            "gap_vs_rho_bits": FIELD_LOG2 * 4.0 / 3.0 - rho_bits,
        },
        "survivors": survivors,
        "decision": (
            "advance_to_measurement" if survivors else "stop_before_measurement"
        ),
        "open_obligation": (
            "A future survivor must give a named, costed non-generic norm-label "
            "property, such as a sublinear targeted norm-preimage algorithm; "
            "degree and preimage multiplicity alone do not satisfy the gate."
        ),
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()

    root = Path(__file__).resolve().parent
    payload = build_screen(root)
    if args.output:
        encoded = json.dumps(payload, separators=(",", ":"), sort_keys=True) + "\n"
        args.output.write_text(encoded, encoding="utf-8")
    else:
        encoded = json.dumps(payload, indent=2, sort_keys=True) + "\n"
        print(encoded, end="")


if __name__ == "__main__":
    main()
