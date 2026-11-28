#!/usr/bin/env python3
"""Independent exact Bernstein-basis audit of the interval certificate."""

import json
from fractions import Fraction
from math import comb
from pathlib import Path

HERE = Path(__file__).resolve().parent
F = (1, 0, -1502, 31976, -187435)
G = (4, -188, 3004, -15988)
U = (136089600, -4264140800, 36063744000)
V = (-34022400, -533017600, 42584704000, -423295360000)
R = 8038359040000


def coefficients_after_affine_substitution(descending, left, right):
    ascending = list(reversed(descending))
    width = right - left
    return [
        sum(
            Fraction(ascending[j] * comb(j, k)) * left ** (j - k) * width**k
            for j in range(k, len(ascending))
        )
        for k in range(len(ascending))
    ]


def bernstein_coefficients(descending, left, right):
    power = coefficients_after_affine_substitution(descending, left, right)
    degree = len(power) - 1
    return [
        sum(power[k] * Fraction(comb(i, k), comb(degree, k)) for k in range(i + 1))
        for i in range(degree + 1)
    ]


def multiply(a, b):
    terms = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            terms[i + j] += x * y
    return terms


def main():
    packet = json.loads((HERE / "certificate.json").read_text())
    assert tuple(packet["forms"]["F"]) == F
    assert tuple(packet["forms"]["G"]) == G
    assert tuple(packet["bezout"]["U"]) == U
    assert tuple(packet["bezout"]["V"]) == V
    assert packet["bezout"]["R"] == R
    assert packet["target_denominator"] == 1024
    identity = [a + b for a, b in zip(multiply(U, F), multiply(V, G))]
    assert identity == [0, 0, 0, 0, 0, 0, R]
    assert sum(map(abs, F)) < 2**18 and sum(map(abs, G)) < 2**18
    assert Fraction(1502, 128**2) + Fraction(31976, 128**3) + Fraction(187435, 128**4) < Fraction(1, 4)

    current = Fraction(-128)
    for row in packet["leaves"]:
        left = Fraction(*row["lo"])
        right = Fraction(*row["hi"])
        assert left == current and left < right
        current = right
        name, direction = row["witness"]
        assert name in ("F", "G") and direction in ("+", "-")
        coefficients = bernstein_coefficients(F if name == "F" else G, left, right)
        threshold = max(1, abs(left), abs(right)) ** 4 / 1024
        sign = 1 if direction == "+" else -1
        assert min(sign * coefficient for coefficient in coefficients) >= threshold, row
    assert current == 128
    print(f"PASS {len(packet['leaves'])} Bernstein intervals, both tails, upper bound, Bezout identity")


if __name__ == "__main__":
    main()
