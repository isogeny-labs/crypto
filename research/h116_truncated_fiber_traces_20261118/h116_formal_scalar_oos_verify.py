#!/usr/bin/env python3
"""Independent retained-JSON verifier for H116 formal-scalar OOS cells."""

import json
import sys
from pathlib import Path

PROTOCOL_ID = "H116-FORMAL-SCALAR-OOS-001"


def add_mod(left, right, p):
    return (left + right) % p


def sub_mod(left, right, p):
    return (left - right) % p


def mul_poly(left, right, p):
    out = [0] * (len(left) + len(right) - 1)
    for i, a in enumerate(left):
        for j, b in enumerate(right):
            out[i + j] = (out[i + j] + a * b) % p
    return trim(out)


def trim(value):
    while len(value) > 1 and value[-1] == 0:
        value.pop()
    return value


def add_poly(left, right, p):
    out = [0] * max(len(left), len(right))
    for i in range(len(out)):
        out[i] = (
            (left[i] if i < len(left) else 0)
            + (right[i] if i < len(right) else 0)
        ) % p
    return trim(out)


def sub_poly(left, right, p):
    out = [0] * max(len(left), len(right))
    for i in range(len(out)):
        out[i] = (
            (left[i] if i < len(left) else 0)
            - (right[i] if i < len(right) else 0)
        ) % p
    return trim(out)


def shift_scale(value, scale, shift, p):
    return [0] * shift + [scale * coefficient % p for coefficient in value]


def power(value, exponent, p):
    out = [1]
    for _ in range(exponent):
        out = mul_poly(out, value, p)
    return out


def derivative(value, p):
    if len(value) <= 1:
        return [0]
    return trim([i * coefficient % p for i, coefficient in enumerate(value)][1:])


def map_identity(cell, numerator=None):
    p = cell["p"]
    numerator = cell["numerator"] if numerator is None else numerator
    denominator = cell["denominator"]
    np = derivative(numerator, p)
    dp = derivative(denominator, p)
    n_minus_wnp = sub_poly(numerator, shift_scale(np, 1, 1, p), p)
    h = add_poly(
        mul_poly(denominator, n_minus_wnp, p),
        shift_scale(mul_poly(dp, numerator, p), 1, 1, p),
        p,
    )
    left = mul_poly(
        [1, 0, cell["source_a"], cell["source_b"]],
        power(h, 2, p),
        p,
    )
    right = add_poly(
        add_poly(
            mul_poly(denominator, power(numerator, 3, p), p),
            shift_scale(
                mul_poly(numerator, power(denominator, 3, p), p),
                cell["target_a"],
                2,
                p,
            ),
            p,
        ),
        shift_scale(power(denominator, 4, p), cell["target_b"], 3, p),
        p,
    )
    return left == right


def quotient(numerator, denominator, p, precision):
    if denominator[0] != 1:
        raise ValueError("oracle denominator is not normalized")
    out = [0] * precision
    for degree in range(precision):
        value = numerator[degree]
        for i in range(1, degree + 1):
            value -= denominator[i] * out[degree - i]
        out[degree] = value % p
    return out


def recurrence(cell):
    p = cell["p"]
    source_a = cell["source_a"]
    source_b = cell["source_b"]
    target_a = cell["target_a"]
    target_b = cell["target_b"]
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


def prefix(value, precision):
    return (value[:precision] + [0] * precision)[:precision]


def series_mul(left, right, p, precision):
    out = [0] * precision
    for i, a in enumerate(left[:precision]):
        for j, b in enumerate(right[: precision - i]):
            out[i + j] = (out[i + j] + a * b) % p
    return out


def series_derivative(value, p, precision):
    out = [0] * precision
    for i in range(1, min(len(value), precision + 1)):
        out[i - 1] = i * value[i] % p
    return out


def series_shift_scale(value, scalar, shift, p, precision):
    out = [0] * precision
    for i, coefficient in enumerate(value[: max(0, precision - shift)]):
        out[i + shift] = scalar * coefficient % p
    return out


def first_order_residual(series, cell):
    p = cell["p"]
    precision = 10
    u = prefix(series, precision)
    up = series_derivative(series, p, precision)
    shifted = series_shift_scale(up, 1, 1, p, precision)
    w_factor = [(u[i] - shifted[i]) % p for i in range(precision)]
    source = [1, 0, cell["source_a"], cell["source_b"]] + [0] * 6
    left = series_mul(
        source,
        series_mul(w_factor, w_factor, p, precision),
        p,
        precision,
    )
    u_squared = series_mul(u, u, p, precision)
    right = series_mul(u_squared, u, p, precision)
    target_a_term = series_shift_scale(
        u, cell["target_a"], 2, p, precision
    )
    target_b_term = series_shift_scale(
        [1], cell["target_b"], 3, p, precision
    )
    right = [(right[i] + target_a_term[i] + target_b_term[i]) % p for i in range(precision)]
    return [(left[i] - right[i]) % p for i in range(precision)]


def nonsingular(p, curve_a, curve_b):
    return (4 * pow(curve_a, 3, p) + 27 * pow(curve_b, 2, p)) % p != 0


def j_invariant(p, curve_a, curve_b):
    four_a3 = 4 * pow(curve_a, 3, p) % p
    denominator = (four_a3 + 27 * pow(curve_b, 2, p)) % p
    return 1728 * four_a3 * pow(denominator, -1, p) % p


def point_count(p, curve_a, curve_b):
    roots = [0] * p
    for y in range(p):
        roots[y * y % p] += 1
    return 1 + sum(
        roots[(x * x * x + curve_a * x + curve_b) % p] for x in range(p)
    )


def selected_model(curve):
    p = curve["p"]
    admitted_j = j_invariant(p, curve["a"], curve["b"])
    examined = 0
    excluded = {(7, 1, 1), (5, 2, 1), (13, 2, 4), (31, 1, 19), (47, 1, 38)}
    for curve_a in range(p):
        for curve_b in range(p):
            examined += 1
            if not nonsingular(p, curve_a, curve_b):
                continue
            if (p, curve_a, curve_b) in excluded:
                continue
            if j_invariant(p, curve_a, curve_b) == admitted_j:
                continue
            if point_count(p, curve_a, curve_b) == curve["order"]:
                return curve_a, curve_b, examined
    return None, None, examined


def verify(result, oracle, cohort):
    if result.get("schema") != "h116.formal-scalar-oos-result/v1":
        raise ValueError("wrong result schema")
    if oracle.get("schema") != "h116.formal-scalar-oos-oracle/v1":
        raise ValueError("wrong oracle schema")
    if result.get("protocol_id") != PROTOCOL_ID or oracle.get("protocol_id") != PROTOCOL_ID:
        raise ValueError("protocol mismatch")
    if len(result["cells"]) != 5 or len(oracle["cells"]) != 5:
        raise ValueError("wrong panel length")

    completed = 0
    for source, candidate, control in zip(
        cohort["curves"], result["cells"], oracle["cells"], strict=True
    ):
        curve_a, curve_b, examined = selected_model(source)
        if candidate["models_examined"] != examined:
            raise ValueError("selection examination count mismatch")
        if curve_a is None:
            if candidate["status"] != "failure" or control["status"] != "failure":
                raise ValueError("selection failure was not retained")
            continue
        completed += 1
        if (curve_a, curve_b) != (candidate["source_a"], candidate["source_b"]):
            raise ValueError("candidate selection mismatch")
        if (curve_a, curve_b) != (control["source_a"], control["source_b"]):
            raise ValueError("oracle selection mismatch")
        qprefix = quotient(control["numerator"], control["denominator"], control["p"], 11)
        rprefix = recurrence(control)
        if qprefix != control["quotient_prefix"]:
            raise ValueError("oracle quotient serialization mismatch")
        if rprefix != control["recurrence_prefix"] or qprefix[:9] != rprefix:
            raise ValueError("recurrence mismatch")
        if candidate["candidate_prefix"] != qprefix:
            raise ValueError("candidate/full-map mismatch")
        if first_order_residual(candidate["candidate_prefix"], control) != [0] * 10:
            raise ValueError("candidate first-order residual mismatch")
        if not map_identity(control):
            raise ValueError("full map fails cleared identity")
        tampered = control["numerator"].copy()
        tampered[2] = (tampered[2] + 1) % control["p"]
        if map_identity(control, tampered):
            raise ValueError("full-map mutation was accepted")
        scalar = (control["trace"] - 1) % control["p"]
        expected_a = pow(scalar, 4, control["p"]) * curve_a % control["p"]
        expected_b = pow(scalar, 6, control["p"]) * curve_b % control["p"]
        if (expected_a, expected_b) != (control["target_a"], control["target_b"]):
            raise ValueError("scaled codomain mismatch")
        for flag in (
            "candidate_matches_full_map",
            "candidate_matches_recurrence",
            "wrong_scalar_rejected",
            "unscaled_target_rejected",
            "oracle_identity",
            "oracle_codomain_match",
            "oracle_tamper_rejected",
        ):
            if candidate.get(flag) is not True:
                raise ValueError(f"false result flag: {flag}")

    if completed != 4 or result.get("all_completed_cells_passed") is not True:
        raise ValueError("wrong completed-cell disposition")
    if result.get("all_five_prediction_passed") is not False:
        raise ValueError("five-cell prediction disposition was altered")
    return {
        "schema": "h116.formal-scalar-oos-independent-verification/v1",
        "protocol_id": PROTOCOL_ID,
        "selection_failures": 1,
        "completed_map_cells": 4,
        "candidate_full_map_matches": 4,
        "recurrence_matches": 4,
        "first_order_identity_matches": 4,
        "cleared_map_identity_matches": 4,
        "map_mutations_rejected": 4,
        "all_five_prediction_passed": False,
    }


def main():
    if len(sys.argv) != 4:
        raise SystemExit(
            "usage: h116_formal_scalar_oos_verify.py RESULT.json ORACLE.json COHORT_V2.json"
        )
    result = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    oracle = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
    cohort = json.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
    print(json.dumps(verify(result, oracle, cohort), sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
