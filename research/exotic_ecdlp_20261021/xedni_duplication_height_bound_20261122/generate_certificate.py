#!/usr/bin/env python3
"""Exact interval and Bezout certificates for the frozen duplication forms."""

import json
from fractions import Fraction
from pathlib import Path

F = (1, 0, -1502, 31976, -187435)
G = (4, -188, 3004, -15988)
U = (136089600, -4264140800, 36063744000)
V = (-34022400, -533017600, 42584704000, -423295360000)
R = 8038359040000


def mul_interval(a, b):
    products = (a[0] * b[0], a[0] * b[1], a[1] * b[0], a[1] * b[1])
    return min(products), max(products)


def eval_interval(coeffs, lo, hi):
    value = (Fraction(coeffs[0]), Fraction(coeffs[0]))
    for coefficient in coeffs[1:]:
        x, y = mul_interval(value, (lo, hi))
        value = x + coefficient, y + coefficient
    return value


def encode(q):
    return [q.numerator, q.denominator]


def poly_mul(a, b):
    out = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] += x * y
    return out


def main():
    bezout = [x + y for x, y in zip(poly_mul(U, F), poly_mul(V, G))]
    assert bezout == [0] * 6 + [R], bezout
    assert sum(abs(x) for x in F) < 2**18
    assert sum(abs(x) for x in G) < 2**18
    leaves = []
    stack = [(Fraction(-128), Fraction(128), 0)]
    while stack:
        lo, hi, depth = stack.pop()
        threshold = max(1, abs(lo), abs(hi)) ** 4 / 1024
        witness = None
        for name, coeffs in (("F", F), ("G", G)):
            lower, upper = eval_interval(coeffs, lo, hi)
            if lower >= threshold:
                witness = name + "+"
                break
            if upper <= -threshold:
                witness = name + "-"
                break
        if witness:
            leaves.append({"lo": encode(lo), "hi": encode(hi), "witness": witness})
            continue
        assert depth < 20, (lo, hi)
        mid = (lo + hi) / 2
        stack.extend(((lo, mid, depth + 1), (mid, hi, depth + 1)))
    leaves.sort(key=lambda row: Fraction(*row["lo"]))
    assert leaves[0]["lo"] == [-128, 1]
    assert leaves[-1]["hi"] == [128, 1]
    assert all(leaves[i]["hi"] == leaves[i + 1]["lo"] for i in range(len(leaves) - 1))
    result = {
        "schema": "xedni-duplication-interval-certificate-v1",
        "forms": {"F": F, "G": G},
        "bezout": {"U": U, "V": V, "R": R},
        "interval": [-128, 128],
        "target_denominator": 1024,
        "leaves": leaves,
    }
    output = Path(__file__).with_name("certificate.json")
    output.write_text(json.dumps(result, sort_keys=True, indent=2) + "\n")
    print(f"PASS {len(leaves)} exact intervals; polynomial Bezout identity R={R}")


if __name__ == "__main__":
    main()
