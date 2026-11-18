#!/usr/bin/env python3
"""Produce the frozen H116 leading-parameter boundary control."""

import hashlib
import json
import sys
from pathlib import Path

PROTOCOL_ID = "H116-PREFIX-TRANSFER-001"


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def inverse_series(series, length, prime):
    if not series or series[0] % prime == 0:
        raise ValueError("series is not a unit")
    inverse = [0] * length
    inverse[0] = pow(series[0], -1, prime)
    for degree in range(1, length):
        convolution = 0
        for index in range(1, degree + 1):
            coefficient = series[index] if index < len(series) else 0
            convolution = (convolution + coefficient * inverse[degree - index]) % prime
        inverse[degree] = -inverse[0] * convolution % prime
    return inverse


def multiply(left, right, length, prime):
    result = [0] * length
    for degree in range(length):
        for index in range(degree + 1):
            if index < len(left) and degree - index < len(right):
                result[degree] = (
                    result[degree] + left[index] * right[degree - index]
                ) % prime
    return result


def inverse_square(series, length, prime):
    inverse = inverse_series(series, length, prime)
    return multiply(inverse, inverse, length, prime)


def first_difference(left, right):
    for index, (a, b) in enumerate(zip(left, right, strict=True)):
        if a != b:
            return index
    return None


def base_unit(prime, length):
    return [2] + [((i * i + 3 * i + 2) % prime) for i in range(1, length)]


def run_case(prime, cutoff):
    length = cutoff + 3
    correction = (5 * cutoff + 7) % prime
    if correction == 0:
        raise ValueError("frozen correction is zero")
    base = base_unit(prime, length)
    perturbed = base.copy()
    perturbed[cutoff] = (perturbed[cutoff] + correction) % prime
    shifted = base.copy()
    shifted[cutoff + 1] = (shifted[cutoff + 1] + correction) % prime

    local_base = [0] + base[:-1]
    local_perturbed = [0] + perturbed[:-1]
    local_shifted = [0] + shifted[:-1]
    square_base = inverse_square(base, length, prime)
    square_perturbed = inverse_square(perturbed, length, prime)
    square_shifted = inverse_square(shifted, length, prime)
    square_zero = inverse_square(base.copy(), length, prime)

    local_first = first_difference(local_base, local_perturbed)
    square_first = first_difference(square_base, square_perturbed)
    shifted_local_first = first_difference(local_base, local_shifted)
    shifted_square_first = first_difference(square_base, square_shifted)
    observed_delta = (square_perturbed[cutoff] - square_base[cutoff]) % prime
    predicted_delta = -2 * correction * pow(base[0], -3, prime) % prime
    passed = all(
        (
            local_first == cutoff + 1,
            square_first == cutoff,
            observed_delta == predicted_delta,
            square_zero == square_base,
            shifted_local_first == cutoff + 2,
            shifted_square_first == cutoff + 1,
        )
    )
    return {
        "p": prime,
        "m": cutoff,
        "correction": correction,
        "local_parameter_first_difference": local_first,
        "inverse_square_first_difference": square_first,
        "observed_boundary_delta": observed_delta,
        "predicted_boundary_delta": predicted_delta,
        "zero_correction_equal": square_zero == square_base,
        "shifted_local_first_difference": shifted_local_first,
        "shifted_inverse_square_first_difference": shifted_square_first,
        "passed": passed,
    }


def evaluate(protocol_path):
    protocol = json.loads(protocol_path.read_text(encoding="utf-8"))
    if protocol.get("protocol_id") != PROTOCOL_ID:
        raise ValueError("wrong protocol")
    cases = [
        run_case(prime, cutoff)
        for prime in protocol["primes"]
        for cutoff in protocol["cutoffs"]
    ]
    return {
        "schema": "h116.prefix-transfer-result/v1",
        "protocol_id": PROTOCOL_ID,
        "protocol_sha256": sha256(protocol_path),
        "producer_sha256": sha256(Path(__file__)),
        "case_count": len(cases),
        "passed_cases": sum(case["passed"] for case in cases),
        "all_cases_passed": all(case["passed"] for case in cases),
        "local_difference_offsets": sorted(
            {case["local_parameter_first_difference"] - case["m"] for case in cases}
        ),
        "inverse_square_difference_offsets": sorted(
            {case["inverse_square_first_difference"] - case["m"] for case in cases}
        ),
        "cases": cases,
    }


def main():
    if len(sys.argv) != 2:
        raise SystemExit("usage: h116_prefix_transfer_control.py PROTOCOL.json")
    print(json.dumps(evaluate(Path(sys.argv[1])), sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
