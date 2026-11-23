#!/usr/bin/env python3
"""Produce the frozen H116 regular-unit substitution control."""

import hashlib
import json
import sys
from pathlib import Path

PROTOCOL_ID = "H116-REGULAR-SUBSTITUTION-001"


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def add(left, right, length, prime):
    return [
        ((left[i] if i < len(left) else 0) + (right[i] if i < len(right) else 0))
        % prime
        for i in range(length)
    ]


def multiply(left, right, length, prime):
    out = [0] * length
    for degree in range(length):
        for index in range(degree + 1):
            if index < len(left) and degree - index < len(right):
                out[degree] = (
                    out[degree] + left[index] * right[degree - index]
                ) % prime
    return out


def inverse(series, length, prime):
    if not series or series[0] % prime == 0:
        raise ValueError("series is not a unit")
    out = [0] * length
    out[0] = pow(series[0], -1, prime)
    for degree in range(1, length):
        total = 0
        for index in range(1, degree + 1):
            coefficient = series[index] if index < len(series) else 0
            total = (total + coefficient * out[degree - index]) % prime
        out[degree] = -out[0] * total % prime
    return out


def substitute(outer, inner, length, prime):
    if inner[0] % prime != 0:
        raise ValueError("inner series must have zero constant coefficient")
    out = [0] * length
    power = [1] + [0] * (length - 1)
    for coefficient in outer[:length]:
        out = add(out, [(coefficient * value) % prime for value in power], length, prime)
        power = multiply(power, inner, length, prime)
    return out


def first_difference(left, right):
    for index, (a, b) in enumerate(zip(left, right, strict=True)):
        if a != b:
            return index
    return None


def base_unit(prime, length):
    return [2] + [((i * i + 3 * i + 2) % prime) for i in range(1, length)]


def outer_unit(prime, length):
    return [3] + [((i * i + 2 * i + 1) % prime) for i in range(1, length)]


def run_case(prime, cutoff):
    length = cutoff + 3
    correction = (5 * cutoff + 7) % prime
    base = base_unit(prime, length)
    perturbed = base.copy()
    perturbed[cutoff] = (perturbed[cutoff] + correction) % prime
    local_base = [0] + base[:-1]
    local_perturbed = [0] + perturbed[:-1]
    outer = outer_unit(prime, length)

    subst_base = substitute(outer, local_base, length, prime)
    subst_perturbed = substitute(outer, local_perturbed, length, prime)
    inv_subst_base = inverse(subst_base, length, prime)
    inv_subst_perturbed = inverse(subst_perturbed, length, prime)
    pole_base = multiply(inverse(base, length, prime), inverse(base, length, prime), length, prime)
    pole_perturbed = multiply(
        inverse(perturbed, length, prime),
        inverse(perturbed, length, prime),
        length,
        prime,
    )
    coordinate_base = multiply(pole_base, inv_subst_base, length, prime)
    coordinate_perturbed = multiply(
        pole_perturbed, inv_subst_perturbed, length, prime
    )

    subst_first = first_difference(subst_base, subst_perturbed)
    inv_subst_first = first_difference(inv_subst_base, inv_subst_perturbed)
    coordinate_first = first_difference(coordinate_base, coordinate_perturbed)
    subst_delta = (subst_perturbed[cutoff + 1] - subst_base[cutoff + 1]) % prime
    inv_subst_delta = (
        inv_subst_perturbed[cutoff + 1] - inv_subst_base[cutoff + 1]
    ) % prime
    coordinate_delta = (
        coordinate_perturbed[cutoff] - coordinate_base[cutoff]
    ) % prime
    expected_subst = 4 * correction % prime
    expected_inv_subst = -4 * correction * pow(3, -2, prime) % prime
    expected_coordinate = (
        -2 * correction * pow(2, -3, prime) * pow(3, -1, prime) % prime
    )

    constant_outer = [3] + [0] * (length - 1)
    constant_equal = substitute(
        constant_outer, local_base, length, prime
    ) == substitute(constant_outer, local_perturbed, length, prime)
    zero_equal = coordinate_base == multiply(pole_base, inv_subst_base, length, prime)
    zero_outer_rejected = False
    try:
        inverse(substitute([0, 4], local_base, length, prime), length, prime)
    except ValueError:
        zero_outer_rejected = True

    passed = all(
        (
            subst_first == cutoff + 1,
            inv_subst_first == cutoff + 1,
            coordinate_first == cutoff,
            subst_delta == expected_subst,
            inv_subst_delta == expected_inv_subst,
            coordinate_delta == expected_coordinate,
            constant_equal,
            zero_equal,
            zero_outer_rejected,
        )
    )
    return {
        "p": prime,
        "m": cutoff,
        "correction": correction,
        "substitution_first_difference": subst_first,
        "substitution_boundary_delta": subst_delta,
        "predicted_substitution_boundary_delta": expected_subst,
        "inverse_substitution_first_difference": inv_subst_first,
        "inverse_substitution_boundary_delta": inv_subst_delta,
        "predicted_inverse_substitution_boundary_delta": expected_inv_subst,
        "normalized_coordinate_first_difference": coordinate_first,
        "normalized_coordinate_boundary_delta": coordinate_delta,
        "predicted_normalized_coordinate_boundary_delta": expected_coordinate,
        "zero_correction_equal": zero_equal,
        "constant_outer_substitution_equal": constant_equal,
        "zero_constant_outer_rejected": zero_outer_rejected,
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
        "schema": "h116.regular-substitution-result/v1",
        "protocol_id": PROTOCOL_ID,
        "protocol_sha256": sha256(protocol_path),
        "producer_sha256": sha256(Path(__file__)),
        "case_count": len(cases),
        "passed_cases": sum(case["passed"] for case in cases),
        "all_cases_passed": all(case["passed"] for case in cases),
        "cases": cases,
    }


def main():
    if len(sys.argv) != 2:
        raise SystemExit("usage: h116_regular_substitution_control.py PROTOCOL.json")
    print(json.dumps(evaluate(Path(sys.argv[1])), sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
