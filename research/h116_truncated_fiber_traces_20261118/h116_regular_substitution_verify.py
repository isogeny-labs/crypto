#!/usr/bin/env python3
"""Independent JSON-only verifier for regular-unit substitution controls."""

import hashlib
import json
import sys
from pathlib import Path

PROTOCOL_ID = "H116-REGULAR-SUBSTITUTION-001"


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify(protocol_path, result_path):
    protocol = json.loads(protocol_path.read_text(encoding="utf-8"))
    result = json.loads(result_path.read_text(encoding="utf-8"))
    if protocol.get("protocol_id") != PROTOCOL_ID or result.get("protocol_id") != PROTOCOL_ID:
        raise ValueError("wrong protocol")
    if result.get("protocol_sha256") != sha256(protocol_path):
        raise ValueError("protocol hash mismatch")
    expected_pairs = [
        (prime, cutoff)
        for prime in protocol["primes"]
        for cutoff in protocol["cutoffs"]
    ]
    if [(case["p"], case["m"]) for case in result["cases"]] != expected_pairs:
        raise ValueError("case grid changed")
    for case in result["cases"]:
        prime, cutoff = case["p"], case["m"]
        correction = (5 * cutoff + 7) % prime
        expected = {
            "correction": correction,
            "substitution_first_difference": cutoff + 1,
            "substitution_boundary_delta": 4 * correction % prime,
            "predicted_substitution_boundary_delta": 4 * correction % prime,
            "inverse_substitution_first_difference": cutoff + 1,
            "inverse_substitution_boundary_delta": -4 * correction * pow(3, -2, prime) % prime,
            "predicted_inverse_substitution_boundary_delta": -4 * correction * pow(3, -2, prime) % prime,
            "normalized_coordinate_first_difference": cutoff,
            "normalized_coordinate_boundary_delta": -2 * correction * pow(2, -3, prime) * pow(3, -1, prime) % prime,
            "predicted_normalized_coordinate_boundary_delta": -2 * correction * pow(2, -3, prime) * pow(3, -1, prime) % prime,
            "zero_correction_equal": True,
            "constant_outer_substitution_equal": True,
            "zero_constant_outer_rejected": True,
            "passed": True,
        }
        for key, value in expected.items():
            if case.get(key) != value:
                raise ValueError(f"case {(prime, cutoff)} failed field {key}")
    if result.get("case_count") != 20 or result.get("passed_cases") != 20:
        raise ValueError("wrong totals")
    if result.get("all_cases_passed") is not True:
        raise ValueError("false result disposition")
    return {
        "schema": "h116.regular-substitution-independent-verification/v1",
        "protocol_id": PROTOCOL_ID,
        "verified_cases": 20,
        "substitution_boundary_matches": 20,
        "inverse_boundary_matches": 20,
        "coordinate_boundary_matches": 20,
        "control_triplets_passed": 20,
        "all_cases_passed": True,
    }


def main():
    if len(sys.argv) != 3:
        raise SystemExit(
            "usage: h116_regular_substitution_verify.py PROTOCOL.json RESULT.json"
        )
    print(json.dumps(verify(Path(sys.argv[1]), Path(sys.argv[2])), sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
