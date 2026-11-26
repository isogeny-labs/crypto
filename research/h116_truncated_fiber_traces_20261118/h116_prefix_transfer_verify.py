#!/usr/bin/env python3
"""Independent JSON-only verifier for H116 prefix-transfer boundary data."""

import hashlib
import json
import sys
from pathlib import Path

PROTOCOL_ID = "H116-PREFIX-TRANSFER-001"


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify(protocol_path, result_path):
    protocol = json.loads(protocol_path.read_text(encoding="utf-8"))
    result = json.loads(result_path.read_text(encoding="utf-8"))
    if protocol.get("protocol_id") != PROTOCOL_ID:
        raise ValueError("wrong protocol")
    if result.get("protocol_id") != PROTOCOL_ID:
        raise ValueError("wrong result protocol")
    if result.get("protocol_sha256") != sha256(protocol_path):
        raise ValueError("protocol hash mismatch")
    expected_pairs = [
        (prime, cutoff)
        for prime in protocol["primes"]
        for cutoff in protocol["cutoffs"]
    ]
    observed_pairs = [(case["p"], case["m"]) for case in result["cases"]]
    if observed_pairs != expected_pairs:
        raise ValueError("case order or membership changed")
    for case in result["cases"]:
        prime = case["p"]
        cutoff = case["m"]
        correction = (5 * cutoff + 7) % prime
        predicted = -2 * correction * pow(2, -3, prime) % prime
        expected = {
            "correction": correction,
            "local_parameter_first_difference": cutoff + 1,
            "inverse_square_first_difference": cutoff,
            "predicted_boundary_delta": predicted,
            "observed_boundary_delta": predicted,
            "zero_correction_equal": True,
            "shifted_local_first_difference": cutoff + 2,
            "shifted_inverse_square_first_difference": cutoff + 1,
            "passed": True,
        }
        for key, value in expected.items():
            if case.get(key) != value:
                raise ValueError(f"case {(prime, cutoff)} failed field {key}")
    if result.get("case_count") != 20 or result.get("passed_cases") != 20:
        raise ValueError("wrong pass totals")
    if result.get("all_cases_passed") is not True:
        raise ValueError("result disposition is false")
    return {
        "schema": "h116.prefix-transfer-independent-verification/v1",
        "protocol_id": PROTOCOL_ID,
        "verified_cases": 20,
        "boundary_delta_matches": 20,
        "zero_controls_passed": 20,
        "shifted_controls_passed": 20,
        "all_cases_passed": True,
    }


def main():
    if len(sys.argv) != 3:
        raise SystemExit(
            "usage: h116_prefix_transfer_verify.py PROTOCOL.json RESULT.json"
        )
    print(
        json.dumps(
            verify(Path(sys.argv[1]), Path(sys.argv[2])),
            sort_keys=True,
            separators=(",", ":"),
        )
    )


if __name__ == "__main__":
    main()
