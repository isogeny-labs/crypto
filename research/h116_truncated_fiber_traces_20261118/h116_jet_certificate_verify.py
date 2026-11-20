#!/usr/bin/env python3
"""Replay the fixed H116 low-jet certificate on the excluded map fixture."""

import json
import sys
from pathlib import Path

QUOTIENT_PRECISION = 11
IDENTITY_PRECISION = 10
RECURRENCE_PRECISION = 9


class Counter:
    def __init__(self):
        self.additions = 0
        self.multiplications = 0
        self.inversions = 0

    def as_dict(self):
        return {
            "additions": self.additions,
            "multiplications": self.multiplications,
            "inversions": self.inversions,
        }


def prefix(value, precision):
    return (value[:precision] + [0] * precision)[:precision]


def add(left, right, p, precision, counter):
    counter.additions += precision
    return [(left[i] + right[i]) % p for i in range(precision)]


def sub(left, right, p, precision, counter):
    counter.additions += precision
    return [(left[i] - right[i]) % p for i in range(precision)]


def mul(left, right, p, precision, counter):
    out = [0] * precision
    for i, a in enumerate(left[:precision]):
        for j, b in enumerate(right[: precision - i]):
            counter.multiplications += 1
            counter.additions += 1
            out[i + j] = (out[i + j] + a * b) % p
    return out


def power(value, exponent, p, precision, counter):
    out = prefix([1], precision)
    for _ in range(exponent):
        out = mul(out, value, p, precision, counter)
    return out


def scale_shift(value, scale, shift, p, precision, counter):
    out = [0] * precision
    for i, coefficient in enumerate(value[: max(0, precision - shift)]):
        counter.multiplications += 1
        out[i + shift] = scale * coefficient % p
    return out


def derivative(value, p, precision, counter):
    out = [0] * precision
    for i in range(1, min(len(value), precision + 1)):
        counter.multiplications += 1
        out[i - 1] = i * value[i] % p
    return out


def quotient(numerator, denominator, p, precision, counter):
    if denominator[0] != 1:
        raise ValueError("denominator is not normalized")
    out = [0] * precision
    for n in range(precision):
        value = numerator[n]
        for i in range(1, n + 1):
            counter.multiplications += 1
            counter.additions += 1
            value -= denominator[i] * out[n - i]
        out[n] = value % p
    return out


def quotient_relation(numerator, denominator, series, p, counter):
    precision = QUOTIENT_PRECISION
    return mul(denominator, series, p, precision, counter) == prefix(numerator, precision)


def identity_residual(numerator, denominator, p, source_a, source_b, target_a, target_b, counter):
    precision = IDENTITY_PRECISION
    numerator = prefix(numerator, QUOTIENT_PRECISION)
    denominator = prefix(denominator, QUOTIENT_PRECISION)
    np = derivative(numerator, p, precision, counter)
    dp = derivative(denominator, p, precision, counter)
    xnp = scale_shift(np, 1, 1, p, precision, counter)
    xdpn = scale_shift(mul(dp, prefix(numerator, precision), p, precision, counter), 1, 1, p, precision, counter)
    h = add(
        mul(prefix(denominator, precision), sub(prefix(numerator, precision), xnp, p, precision, counter), p, precision, counter),
        xdpn,
        p,
        precision,
        counter,
    )
    source = prefix([1, 0, source_a, source_b], precision)
    left = mul(source, power(h, 2, p, precision, counter), p, precision, counter)
    d3 = power(prefix(denominator, precision), 3, p, precision, counter)
    d4 = power(prefix(denominator, precision), 4, p, precision, counter)
    right = add(
        add(
            mul(prefix(denominator, precision), power(prefix(numerator, precision), 3, p, precision, counter), p, precision, counter),
            scale_shift(mul(prefix(numerator, precision), d3, p, precision, counter), target_a, 2, p, precision, counter),
            p,
            precision,
            counter,
        ),
        scale_shift(d4, target_b, 3, p, precision, counter),
        p,
        precision,
        counter,
    )
    return sub(left, right, p, precision, counter)


def recurrence_prefix(p, source_a, source_b, target_a, target_b):
    out = [0] * RECURRENCE_PRECISION
    out[0] = 1
    out[2] = (source_a - target_a) * pow(5, -1, p) % p
    out[3] = (source_b - target_b) * pow(7, -1, p) % p
    for k in range(3, 8):
        convolution = sum(out[i + 1] * out[k - i] for i in range(1, k - 1)) % p
        step = 2 * k + 3
        first = 3 * convolution * pow((k - 2) * step, -1, p)
        second = (2 * k - 3) * source_a * out[k - 1] * pow(step, -1, p)
        third = 2 * (k - 3) * source_b * out[k - 2] * pow(step, -1, p)
        out[k + 1] = (first - second - third) % p
    return out


def evaluate(record):
    if record.get("schema") != "h116.map-certificate-control/v1":
        raise ValueError("wrong source schema")
    p = record["p"]
    if (p, record.get("r"), record.get("fixture")) != (31, 37, "excluded-h115-fp31"):
        raise ValueError("wrong excluded fixture")
    source_a, source_b = 1, 19
    target_a, target_b = record["target_a"], record["target_b"]
    numerator = record["numerator"]
    denominator = record["denominator"]
    if denominator[0] != 1:
        raise ValueError("non-normalized denominator")

    counter = Counter()
    series = quotient(
        prefix(numerator, QUOTIENT_PRECISION),
        prefix(denominator, QUOTIENT_PRECISION),
        p,
        QUOTIENT_PRECISION,
        counter,
    )
    quotient_ok = quotient_relation(numerator, denominator, series, p, counter)
    residual = identity_residual(
        numerator, denominator, p, source_a, source_b, target_a, target_b, counter
    )
    identity_ok = residual == [0] * IDENTITY_PRECISION
    recurrence = recurrence_prefix(p, source_a, source_b, target_a, target_b)
    recurrence_ok = series[:RECURRENCE_PRECISION] == recurrence

    in_window = numerator.copy()
    in_window[2] = (in_window[2] + 1) % p
    in_counter = Counter()
    in_window_rejected = (
        not quotient_relation(in_window, denominator, series, p, in_counter)
        and identity_residual(
            in_window, denominator, p, source_a, source_b, target_a, target_b, in_counter
        )
        != [0] * IDENTITY_PRECISION
    )

    guard = numerator.copy()
    guard[10] = (guard[10] + 1) % p
    guard_counter = Counter()
    guard_quotient_rejected = not quotient_relation(
        guard, denominator, series, p, guard_counter
    )
    guard_identity_unchanged = identity_residual(
        guard, denominator, p, source_a, source_b, target_a, target_b, guard_counter
    ) == residual

    tail = numerator.copy()
    tail[11] = (tail[11] + 1) % p
    tail_counter = Counter()
    tail_invariant = quotient_relation(tail, denominator, series, p, tail_counter) and (
        identity_residual(
            tail, denominator, p, source_a, source_b, target_a, target_b, tail_counter
        )
        == residual
    )

    if not all(
        (
            quotient_ok,
            identity_ok,
            recurrence_ok,
            in_window_rejected,
            guard_quotient_rejected,
            guard_identity_unchanged,
            tail_invariant,
        )
    ):
        raise ValueError("low-jet certificate or boundary control failed")

    return {
        "schema": "h116.jet-certificate-control/v1",
        "fixture": record["fixture"],
        "p": p,
        "r": record["r"],
        "quotient_precision": QUOTIENT_PRECISION,
        "identity_precision": IDENTITY_PRECISION,
        "recurrence_precision": RECURRENCE_PRECISION,
        "jet_input_coefficients": 2 * QUOTIENT_PRECISION,
        "full_map_input_coefficients": len(numerator) + len(denominator),
        "quotient_relation": quotient_ok,
        "identity_prefix": identity_ok,
        "recurrence_prefix": recurrence_ok,
        "in_window_mutation_rejected": in_window_rejected,
        "guard_mutation_quotient_rejected": guard_quotient_rejected,
        "guard_mutation_identity_unchanged": guard_identity_unchanged,
        "tail_mutation_invariant": tail_invariant,
        "series_prefix": series,
        "recurrence_coefficients": recurrence,
        "dense_slot_operations": counter.as_dict(),
    }


def main():
    if len(sys.argv) != 2:
        raise SystemExit("usage: h116_jet_certificate_verify.py MAP_CERTIFICATE_CONTROL.json")
    record = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    print(json.dumps(evaluate(record), sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
