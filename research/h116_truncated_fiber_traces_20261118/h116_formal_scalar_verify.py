#!/usr/bin/env python3
"""Independent algebraic verifier for the H116 formal-scalar jet control."""

import json
import sys
from pathlib import Path


def prefix(value, precision):
    return (value[:precision] + [0] * precision)[:precision]


def add(left, right, p, precision):
    return [(left[i] + right[i]) % p for i in range(precision)]


def mul(left, right, p, precision):
    out = [0] * precision
    for i, a in enumerate(left[:precision]):
        for j, b in enumerate(right[: precision - i]):
            out[i + j] = (out[i + j] + a * b) % p
    return out


def derivative(value, p, precision):
    out = [0] * precision
    for i in range(1, min(len(value), precision + 1)):
        out[i - 1] = i * value[i] % p
    return out


def shift_scale(value, scalar, shift, p, precision):
    out = [0] * precision
    for i, coefficient in enumerate(value[: max(0, precision - shift)]):
        out[i + shift] = scalar * coefficient % p
    return out


def quotient(numerator, denominator, p, precision):
    if denominator[0] != 1:
        raise ValueError("full-map denominator is not normalized")
    out = [0] * precision
    for degree in range(precision):
        value = numerator[degree]
        for i in range(1, degree + 1):
            value -= denominator[i] * out[degree - i]
        out[degree] = value % p
    return out


def recurrence(p, source_a, source_b, target_a, target_b):
    out = [0] * 9
    out[0] = 1
    out[2] = (source_a - target_a) * pow(5, -1, p) % p
    out[3] = (source_b - target_b) * pow(7, -1, p) % p
    for k in range(3, 8):
        convolution = sum(out[i + 1] * out[k - i] for i in range(1, k - 1))
        step = 2 * k + 3
        out[k + 1] = (
            3 * convolution * pow((k - 2) * step, -1, p)
            - (2 * k - 3) * source_a * out[k - 1] * pow(step, -1, p)
            - 2 * (k - 3) * source_b * out[k - 2] * pow(step, -1, p)
        ) % p
    return out


def first_order_residual(series, p, source_a, source_b, target_a, target_b):
    precision = 10
    u = prefix(series, precision)
    up = derivative(series, p, precision)
    w_factor = [(u[i] - shift_scale(up, 1, 1, p, precision)[i]) % p for i in range(precision)]
    source_curve = prefix([1, 0, source_a, source_b], precision)
    left = mul(source_curve, mul(w_factor, w_factor, p, precision), p, precision)
    u_squared = mul(u, u, p, precision)
    right = add(
        add(
            mul(u_squared, u, p, precision),
            shift_scale(u, target_a, 2, p, precision),
            p,
            precision,
        ),
        shift_scale([1], target_b, 3, p, precision),
        p,
        precision,
    )
    return [(left[i] - right[i]) % p for i in range(precision)]


def verify(result, full_map):
    if result.get("schema") != "h116.formal-scalar-jet-control/v1":
        raise ValueError("wrong result schema")
    if full_map.get("schema") != "h116.map-certificate-control/v1":
        raise ValueError("wrong full-map schema")
    candidate_input = result["candidate_input"]
    expected_input = {
        "p": 31,
        "source_a": 1,
        "source_b": 19,
        "trace": -5,
        "scalar_t_minus_one": 25,
        "precision": 11,
    }
    if candidate_input != expected_input:
        raise ValueError("wrong candidate input")
    if (full_map["p"], full_map["r"], result["fixture"]) != (
        31,
        37,
        "excluded-h115-fp31",
    ):
        raise ValueError("wrong excluded fixture")
    target_a, target_b = result["target"]["a"], result["target"]["b"]
    expected_target = (pow(25, 4, 31), pow(25, 6, 31) * 19 % 31)
    if (target_a, target_b) != expected_target:
        raise ValueError("wrong target scaling")

    candidate = result["candidate_prefix"]
    oracle = quotient(full_map["numerator"], full_map["denominator"], 31, 11)
    recurrence_prefix = recurrence(31, 1, 19, target_a, target_b)
    residual = first_order_residual(candidate, 31, 1, 19, target_a, target_b)
    if candidate != oracle or candidate[:9] != recurrence_prefix:
        raise ValueError("candidate does not match independent controls")
    if residual != [0] * 10:
        raise ValueError("candidate fails the first-order curve identity")
    if result["oracle_prefix"] != oracle or result["recurrence_prefix"] != recurrence_prefix:
        raise ValueError("serialized control prefix mismatch")

    coordinate = result["coordinate_isomorphism_control"]
    expected_coordinate = [
        coefficient * pow(2, 2 * i, 31) % 31
        for i, coefficient in enumerate(candidate)
    ]
    if coordinate["prefix"] != expected_coordinate or not coordinate["passed"]:
        raise ValueError("coordinate-isomorphism control failed")

    for prime, expected_index in ((23, 11), (19, 9)):
        boundary = result["synthetic_frobenius_boundaries"][f"p{prime}"]
        expected = [1] + [0] * (expected_index - 1) + [2]
        if boundary["differences"] != [expected_index]:
            raise ValueError("wrong Frobenius boundary")
        if boundary["perturbed_prefix"] != expected:
            raise ValueError("wrong leading Frobenius perturbation")

    for flag in (
        "candidate_loaded_before_full_map",
        "candidate_matches_full_map",
        "candidate_matches_recurrence",
        "wrong_scalar_rejected",
        "unscaled_target_rejected",
    ):
        if result.get(flag) is not True:
            raise ValueError(f"false result flag: {flag}")
    return {
        "schema": "h116.formal-scalar-jet-independent-verification/v1",
        "fixture": result["fixture"],
        "candidate_matches_full_map": True,
        "candidate_matches_recurrence": True,
        "first_order_identity_prefix": True,
        "coordinate_isomorphism_control": True,
        "frobenius_boundary_p23": 11,
        "frobenius_boundary_p19": 9,
    }


def main():
    if len(sys.argv) != 3:
        raise SystemExit(
            "usage: h116_formal_scalar_verify.py FORMAL_RESULT.json FULL_MAP.json"
        )
    result = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    full_map = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
    print(json.dumps(verify(result, full_map), sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
