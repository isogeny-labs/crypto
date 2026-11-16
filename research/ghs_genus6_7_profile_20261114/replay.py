#!/usr/bin/env python3
"""Independent GF(2^15) replay of a complete generalized Hess profile scan."""

from __future__ import annotations

import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path


DEGREE = 15
MODULUS = 0x8003
A = 0x1
B = 0x524B
BASE_DEGREE = 5
FIELD_SIZE = 1 << DEGREE
SUMMARY = Path(__file__).with_name("summary.json")


def multiply(a: int, b: int) -> int:
    product = 0
    while b:
        if b & 1:
            product ^= a
        b >>= 1
        a <<= 1
        if a & FIELD_SIZE:
            a ^= MODULUS
    return product


def inverse(a: int) -> int:
    if not a:
        raise ZeroDivisionError
    u, v, x, y = a, MODULUS, 1, 0
    while u != 1:
        shift = u.bit_length() - v.bit_length()
        if shift < 0:
            u, v = v, u
            x, y = y, x
            shift = -shift
        u ^= v << shift
        x ^= y << shift
    while x.bit_length() > DEGREE:
        x ^= MODULUS << (x.bit_length() - DEGREE - 1)
    return x


def frobenius(x: int, steps: int = BASE_DEGREE) -> int:
    for _ in range(steps):
        x = multiply(x, x)
    return x


def relative_trace(x: int) -> int:
    value = 0
    for _ in range(DEGREE // BASE_DEGREE):
        value ^= x
        x = frobenius(x)
    return value


def order_polynomial(x: int) -> int:
    pivots: dict[int, tuple[int, int]] = {}
    for i in range(DEGREE // BASE_DEGREE + 1):
        value, relation = x, 1 << i
        while value:
            pivot = value.bit_length() - 1
            if pivot not in pivots:
                pivots[pivot] = (value, relation)
                break
            old_value, old_relation = pivots[pivot]
            value ^= old_value
            relation ^= old_relation
        if not value:
            return relation
        x = frobenius(x)
    raise AssertionError("relative Frobenius orbit did not close")


def polynomial_remainder(a: int, b: int) -> int:
    while a and a.bit_length() >= b.bit_length():
        a ^= b << (a.bit_length() - b.bit_length())
    return a


def irreducible_modulus() -> bool:
    """Rabin's F2 irreducibility test for the declared degree-15 modulus."""
    powers = [0x2]
    for _ in range(DEGREE):
        x = powers[-1]
        square = sum(((x >> i) & 1) << (2 * i) for i in range(x.bit_length()))
        powers.append(polynomial_remainder(square, MODULUS))
    if powers[DEGREE] != 0x2:
        return False
    for exponent in (DEGREE // 3, DEGREE // 5):
        a, b = powers[exponent] ^ 0x2, MODULUS
        while b:
            a, b = b, polynomial_remainder(a, b)
        if a != 1:
            return False
    return True


def polynomial_quotient(a: int, b: int) -> int:
    quotient = 0
    while a and a.bit_length() >= b.bit_length():
        shift = a.bit_length() - b.bit_length()
        a ^= b << shift
        quotient ^= 1 << shift
    return quotient


def polynomial_product(a: int, b: int) -> int:
    product = 0
    while b:
        if b & 1:
            product ^= a
        a <<= 1
        b >>= 1
    return product


def polynomial_lcm(a: int, b: int) -> int:
    x, y = a, b
    while y:
        x, y = y, polynomial_remainder(x, y)
    return polynomial_product(polynomial_quotient(a, x), b)


def profile(c: int) -> dict:
    beta = frobenius(B, DEGREE - 1)
    d = multiply(beta, inverse(c))
    f1, f2 = order_polynomial(c), order_polynomial(d)
    f = polynomial_lcm(polynomial_lcm(f1, f2), 0x3 if A else 0x1)
    s1, s2, t = f1.bit_length() - 1, f2.bit_length() - 1, f.bit_length() - 1
    genus = (1 << t) + 1 - (1 << (t - s1)) - (1 << (t - s2))
    descends = relative_trace(c) != 0 or relative_trace(d) != 0
    return {
        "c": f"0x{c:x}",
        "d": f"0x{d:x}",
        "f1": f"0x{f1:x}",
        "f2": f"0x{f2:x}",
        "s1": s1,
        "s2": s2,
        "compositum_rank": t,
        "genus": str(genus),
        "descends": descends,
        "hyperelliptic_sufficient": frobenius(c) == c or frobenius(d) == d,
    }


def run() -> dict:
    if not irreducible_modulus():
        raise ValueError("declared binary modulus is reducible")
    counts = Counter()
    witnesses: dict[str, dict] = {}
    for c in range(1, FIELD_SIZE):
        item = profile(c)
        if item["genus"] == "5" and (item["f1"], item["f2"]) != ("0x7", "0x7"):
            raise AssertionError(f"unexpected genus-five order polynomials at c={c}")
        key = (item["genus"], item["descends"], item["hyperelliptic_sufficient"])
        counts[key] += 1
        witnesses.setdefault(f"{item['genus']}:{str(item['descends']).lower()}", item)
    return {
        "schema": "ghs-n15-hess-profile-replay/v1",
        "input": {"degree": DEGREE, "modulus": f"0x{MODULUS:x}", "a": f"0x{A:x}",
                  "b": f"0x{B:x}", "base_degree": BASE_DEGREE,
                  "c_range": [1, FIELD_SIZE - 1]},
        "counts": [
            {"genus": genus, "descends": descends,
             "hyperelliptic_sufficient": hyperelliptic, "count": count}
            for (genus, descends, hyperelliptic), count in sorted(
                counts.items(), key=lambda x: (int(x[0][0]), x[0][1], x[0][2])
            )
        ],
        "genus_five_order_seven_all": True,
        "first_witnesses": witnesses,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--native-json", type=Path,
                        help="also compare every profile against a native ghs_generalized output")
    args = parser.parse_args()
    result = run()
    if args.native_json:
        native = json.loads(args.native_json.read_text())
        if native["c_range"] != [1, FIELD_SIZE - 1] or native["candidates_truncated"]:
            raise SystemExit("native output is not a complete parameter sweep")
        if native["candidates_total"] != len(native["candidates"]):
            raise SystemExit("native candidate list is incomplete")
        if any(row["under_bound"] for row in native["base_rows"] if row["base_degree"] != BASE_DEGREE):
            raise SystemExit("other base degrees need a separate independent replay")
        native_profiles = {int(row["c"], 16): row for row in native["candidates"]}
        if len(native_profiles) != len(native["candidates"]):
            raise SystemExit("native candidate parameters are not unique")
        for c in range(1, FIELD_SIZE):
            independent = profile(c)
            candidate = native_profiles.get(c)
            if independent["descends"] and int(independent["genus"]) <= 8:
                if candidate is None or any(candidate[key] != independent[key] for key in independent):
                    raise SystemExit(f"native profile differs at c={c}")
            elif candidate is not None:
                raise SystemExit(f"unexpected native candidate at c={c}")
        result["native_output_sha256"] = hashlib.sha256(args.native_json.read_bytes()).hexdigest()
    if args.check:
        expected = json.loads(SUMMARY.read_text())
        for key in ("schema", "input", "counts", "genus_five_order_seven_all", "first_witnesses"):
            if result[key] != expected[key]:
                raise SystemExit(f"frozen summary differs: {key}")
        if args.native_json and result["native_output_sha256"] != expected["native_output_sha256"]:
            raise SystemExit("native output hash differs")
        print("GHS_N15_PROFILE_REPLAY_PASS", FIELD_SIZE - 1)
    else:
        print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
