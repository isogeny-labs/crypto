#!/usr/bin/env python3
"""Independent standard-library verifier for the excluded H116 map certificate control."""

import json
import sys
from pathlib import Path

J = 8


def trim(value):
    while len(value) > 1 and value[-1] == 0:
        value.pop()
    return value


def add(left, right, p):
    return trim(
        [
            ((left[i] if i < len(left) else 0) + (right[i] if i < len(right) else 0)) % p
            for i in range(max(len(left), len(right)))
        ]
    )


def sub(left, right, p):
    return trim(
        [
            ((left[i] if i < len(left) else 0) - (right[i] if i < len(right) else 0)) % p
            for i in range(max(len(left), len(right)))
        ]
    )


def mul(left, right, p):
    out = [0] * (len(left) + len(right) - 1)
    for i, a in enumerate(left):
        for j, b in enumerate(right):
            out[i + j] = (out[i + j] + a * b) % p
    return trim(out)


def power(value, exponent, p):
    out = [1]
    for _ in range(exponent):
        out = mul(out, value, p)
    return out


def scale_shift(value, scale, shift, p):
    return [0] * shift + [(scale * coefficient) % p for coefficient in value]


def derivative(value, p):
    return trim([(i * value[i]) % p for i in range(1, len(value))] or [0])


def certificate_identity(p, source_a, source_b, target_a, target_b, numerator, denominator):
    np = derivative(numerator, p)
    dp = derivative(denominator, p)
    h = add(
        mul(denominator, sub(numerator, scale_shift(np, 1, 1, p), p), p),
        scale_shift(mul(dp, numerator, p), 1, 1, p),
        p,
    )
    left = mul([1, 0, source_a, source_b], power(h, 2, p), p)
    right = add(
        add(
            mul(denominator, power(numerator, 3, p), p),
            scale_shift(mul(numerator, power(denominator, 3, p), p), target_a, 2, p),
            p,
        ),
        scale_shift(power(denominator, 4, p), target_b, 3, p),
        p,
    )
    return left == right


def quotient_prefix(numerator, denominator, p):
    if denominator[0] != 1:
        raise ValueError("denominator is not normalized")
    out = [0] * (J + 1)
    for n in range(J + 1):
        value = numerator[n] if n < len(numerator) else 0
        for i in range(1, min(n, len(denominator) - 1) + 1):
            value -= denominator[i] * out[n - i]
        out[n] = value % p
    return out


def recurrence_prefix(p, source_a, source_b, target_a, target_b):
    out = [0] * (J + 1)
    out[0] = 1
    out[2] = (source_a - target_a) * pow(5, -1, p) % p
    out[3] = (source_b - target_b) * pow(7, -1, p) % p
    for k in range(3, J):
        convolution = sum(out[i + 1] * out[k - i] for i in range(1, k - 1)) % p
        step = 2 * k + 3
        first = 3 * convolution * pow((k - 2) * step, -1, p)
        second = (2 * k - 3) * source_a * out[k - 1] * pow(step, -1, p)
        third = 2 * (k - 3) * source_b * out[k - 2] * pow(step, -1, p)
        out[k + 1] = (first - second - third) % p
    return out


def verify(record):
    if record.get("schema") != "h116.map-certificate-control/v1":
        raise ValueError("wrong schema")
    if record.get("fixture") != "excluded-h115-fp31":
        raise ValueError("wrong fixture")
    p, source_a, source_b = 31, 1, 19
    if (record.get("p"), record.get("r"), record.get("extension_degree")) != (p, 37, 4):
        raise ValueError("wrong excluded control parameters")
    numerator = record["numerator"]
    denominator = record["denominator"]
    if len(numerator) != 38 or len(denominator) != 37:
        raise ValueError("wrong polynomial degrees")
    target_a, target_b = record["target_a"], record["target_b"]
    differential = (-5 - 1) % p
    if (target_a, target_b) != (
        pow(differential, 4, p) * source_a % p,
        pow(differential, 6, p) * source_b % p,
    ):
        raise ValueError("codomain scaling mismatch")
    identity = certificate_identity(
        p, source_a, source_b, target_a, target_b, numerator, denominator
    )
    quotient = quotient_prefix(numerator, denominator, p)
    recurrence = recurrence_prefix(p, source_a, source_b, target_a, target_b)
    tampered = numerator.copy()
    tampered[2] = (tampered[2] + 1) % p
    tamper_rejected = not certificate_identity(
        p, source_a, source_b, target_a, target_b, tampered, denominator
    )
    if not identity or quotient != recurrence or not tamper_rejected:
        raise ValueError("independent certificate check failed")
    if record.get("quotient_prefix") != quotient:
        raise ValueError("serialized quotient prefix mismatch")
    for flag in ("codomain_match", "identity", "prefix_match", "normalized", "tamper_rejected"):
        if record.get(flag) is not True:
            raise ValueError(f"producer flag {flag} is false")
    return {
        "schema": "h116.map-certificate-independent-verification/v1",
        "fixture": record["fixture"],
        "identity": identity,
        "prefix_match": quotient == recurrence,
        "tamper_rejected": tamper_rejected,
        "quotient_prefix": quotient,
    }


def main():
    if len(sys.argv) != 2:
        raise SystemExit("usage: h116_map_certificate_verify.py RESULT.json")
    record = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    print(json.dumps(verify(record), sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
