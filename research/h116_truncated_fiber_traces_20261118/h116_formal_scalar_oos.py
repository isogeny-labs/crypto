#!/usr/bin/env python3
"""Evaluate the frozen formal-scalar candidate before parsing the OOS oracle."""

import hashlib
import json
import sys
from pathlib import Path

from h116_formal_scalar_jet import (
    Counter,
    formal_scalar_jet,
    normalized_x_ratio_jet,
    recurrence_prefix,
)

CANDIDATE_SHA256 = "914c1909ffdd5e6b87a01f4c2da9de8faca0187da490cb56af323e105e64b6b4"
COHORT_SHA256 = "29e2fd5596ce3ed8e93933161e05b92229cd3065a892bc8ee59d6265f6fa2d99"
PROTOCOL_ID = "H116-FORMAL-SCALAR-OOS-001"
PRECISION = 11


def file_sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def inverse(value, p):
    return pow(value, -1, p)


def nonsingular(p, curve_a, curve_b):
    return (4 * pow(curve_a, 3, p) + 27 * pow(curve_b, 2, p)) % p != 0


def j_invariant(p, curve_a, curve_b):
    four_a3 = 4 * pow(curve_a, 3, p) % p
    denominator = (four_a3 + 27 * pow(curve_b, 2, p)) % p
    if denominator == 0:
        raise ValueError("j-invariant requested for a singular model")
    return 1728 * four_a3 * inverse(denominator, p) % p


def point_count(p, curve_a, curve_b):
    square_counts = [0] * p
    for y in range(p):
        square_counts[y * y % p] += 1
    total = 1
    for x in range(p):
        total += square_counts[(x * x * x + curve_a * x + curve_b) % p]
    return total


def excluded_h115(p, curve_a, curve_b):
    return (p, curve_a, curve_b) in {
        (7, 1, 1),
        (5, 2, 1),
        (13, 2, 4),
        (31, 1, 19),
        (47, 1, 38),
    }


def select_alternative(curve):
    p = curve["p"]
    admitted_j = j_invariant(p, curve["a"], curve["b"])
    examined = 0
    for curve_a in range(p):
        for curve_b in range(p):
            examined += 1
            if not nonsingular(p, curve_a, curve_b):
                continue
            if excluded_h115(p, curve_a, curve_b):
                continue
            if j_invariant(p, curve_a, curve_b) == admitted_j:
                continue
            if point_count(p, curve_a, curve_b) == curve["order"]:
                return curve_a, curve_b, examined
    return None, None, examined


def candidate_cell(curve):
    curve_a, curve_b, examined = select_alternative(curve)
    if curve_a is None:
        return {
            "slug": f"fp{curve['p']}-r{curve['order']}-d{curve['torsion_degree']}",
            "status": "failure",
            "stage": "selection",
            "models_examined": examined,
        }
    p = curve["p"]
    scalar = (curve["trace"] - 1) % p
    construction = Counter()
    internal = Counter()
    candidate, normalized_parameter, target = formal_scalar_jet(
        p,
        curve_a,
        curve_b,
        scalar,
        PRECISION,
        construction,
        internal,
    )
    recurrence_counter = Counter()
    recurrence = recurrence_prefix(
        p, curve_a, curve_b, target[0], target[1], recurrence_counter
    )
    wrong_counter = Counter()
    wrong_scalar, _, _ = formal_scalar_jet(
        p,
        curve_a,
        curve_b,
        (scalar + 1) % p,
        PRECISION,
        wrong_counter,
        wrong_counter,
    )
    unscaled_counter = Counter()
    unscaled = normalized_x_ratio_jet(
        p,
        curve_a,
        curve_b,
        curve_a,
        curve_b,
        normalized_parameter,
        PRECISION,
        unscaled_counter,
        unscaled_counter,
    )
    return {
        "slug": f"fp{p}-r{curve['order']}-d{curve['torsion_degree']}",
        "status": "candidate-computed",
        "p": p,
        "r": curve["order"],
        "trace": curve["trace"],
        "source_a": curve_a,
        "source_b": curve_b,
        "source_j": j_invariant(p, curve_a, curve_b),
        "models_examined": examined,
        "scalar_t_minus_one": scalar,
        "target_a": target[0],
        "target_b": target[1],
        "candidate_prefix": candidate,
        "recurrence_prefix": recurrence,
        "candidate_matches_recurrence": candidate[: len(recurrence)] == recurrence,
        "wrong_scalar_rejected": wrong_scalar != candidate,
        "unscaled_target_rejected": unscaled != candidate,
        "operation_counts": {
            "candidate": construction.as_dict(),
            "candidate_internal_verification": internal.as_dict(),
            "recurrence_control": recurrence_counter.as_dict(),
            "wrong_scalar_control": wrong_counter.as_dict(),
            "unscaled_target_control": unscaled_counter.as_dict(),
        },
    }


def evaluate(cohort_path, oracle_path):
    here = Path(__file__).resolve().parent
    candidate_source = here / "h116_formal_scalar_jet.py"
    if file_sha256(candidate_source) != CANDIDATE_SHA256:
        raise ValueError("frozen candidate source hash mismatch")
    if file_sha256(cohort_path) != COHORT_SHA256:
        raise ValueError("frozen cohort source hash mismatch")
    cohort = json.loads(cohort_path.read_text(encoding="utf-8"))
    if cohort.get("schema") != "h116.cohort/v2" or len(cohort["curves"]) != 5:
        raise ValueError("wrong frozen cohort source")

    # This entire list is fixed before the oracle path is opened.
    candidate_cells = [candidate_cell(curve) for curve in cohort["curves"]]

    oracle = json.loads(oracle_path.read_text(encoding="utf-8"))
    if oracle.get("schema") != "h116.formal-scalar-oos-oracle/v1":
        raise ValueError("wrong OOS oracle schema")
    if oracle.get("protocol_id") != PROTOCOL_ID or len(oracle["cells"]) != 5:
        raise ValueError("wrong OOS oracle protocol or cell count")

    completed = 0
    matches = 0
    for candidate, control in zip(candidate_cells, oracle["cells"], strict=True):
        if candidate["slug"] != control["slug"]:
            raise ValueError("candidate/oracle cell order mismatch")
        if candidate["status"] == "failure":
            if control["status"] != "failure" or control["stage"] != "selection":
                raise ValueError("selection failure mismatch")
            candidate["oracle_status"] = "matching-selection-failure"
            continue
        completed += 1
        for field in ("p", "r", "trace", "source_a", "source_b", "source_j"):
            if candidate[field] != control[field]:
                raise ValueError(f"selection mismatch in {field}")
        candidate["candidate_matches_full_map"] = (
            candidate["candidate_prefix"] == control["quotient_prefix"]
        )
        candidate["oracle_identity"] = control["identity"]
        candidate["oracle_codomain_match"] = control["codomain_match"]
        candidate["oracle_tamper_rejected"] = control["tamper_rejected"]
        candidate["oracle_construction_ops"] = control["construction_ops"]
        candidate["oracle_status"] = control["status"]
        if not all(
            (
                candidate["candidate_matches_full_map"],
                candidate["candidate_matches_recurrence"],
                candidate["wrong_scalar_rejected"],
                candidate["unscaled_target_rejected"],
                control["codomain_match"],
                control["identity"],
                control["prefix_match"],
                control["normalized"],
                control["tamper_rejected"],
            )
        ):
            raise ValueError(f"frozen control failed for {candidate['slug']}")
        matches += 1

    return {
        "schema": "h116.formal-scalar-oos-result/v1",
        "protocol_id": PROTOCOL_ID,
        "candidate_sha256": CANDIDATE_SHA256,
        "cohort_sha256": COHORT_SHA256,
        "candidate_all_cells_before_oracle_parse": True,
        "selected_cells": 4,
        "selection_failures": 1,
        "completed_map_cells": completed,
        "candidate_full_map_matches": matches,
        "all_completed_cells_passed": completed == matches == 4,
        "all_five_prediction_passed": False,
        "cells": candidate_cells,
    }


def main():
    if len(sys.argv) != 3:
        raise SystemExit(
            "usage: h116_formal_scalar_oos.py COHORT_V2.json OOS_ORACLE.json"
        )
    result = evaluate(Path(sys.argv[1]), Path(sys.argv[2]))
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
