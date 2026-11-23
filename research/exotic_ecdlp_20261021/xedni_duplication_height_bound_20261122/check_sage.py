#!/usr/bin/env python3
"""Independent checked-Sage replay of the duplication forms on fixed points."""

import json
from pathlib import Path

from sage.all import EllipticCurve, QQ

HERE = Path(__file__).resolve().parent
E = EllipticCurve(QQ, [0, -47, 0, 751, -3997])
Q = E(17, 10)


def f(a, b):
    return a**4 - 1502 * a**2 * b**2 + 31976 * a * b**3 - 187435 * b**4


def g(a, b):
    return 4 * a**3 * b - 188 * a**2 * b**2 + 3004 * a * b**3 - 15988 * b**4


def main():
    records = []
    for n in range(-15, 16):
        if n == 0:
            continue
        P = n * Q
        x = P[0]
        a, b = int(x.numerator()), int(x.denominator())
        numerator, denominator = f(a, b), g(a, b)
        doubled = 2 * P
        if doubled == E(0):
            assert denominator == 0
        else:
            assert denominator != 0
            assert QQ(numerator) / QQ(denominator) == doubled[0]
        records.append({"n": n, "duplicated_x_matches": True})
    result = {"schema": "xedni-duplication-sage-replay-v1", "curve": str(E), "records": records}
    (HERE / "sage-result.json").write_text(json.dumps(result, sort_keys=True, indent=2) + "\n")
    print(f"PASS {len(records)} signed nonidentity multiples")


if __name__ == "__main__":
    main()
