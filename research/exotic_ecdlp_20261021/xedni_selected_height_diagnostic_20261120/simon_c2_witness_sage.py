#!/usr/bin/env python3
"""Retain explicit Simon two-descent points for the C2 rank interval."""

import json
from pathlib import Path

from sage.all import EllipticCurve, QQ, RealField, matrix


ROOT = Path(__file__).resolve().parent


def main():
    fixtures = json.loads((ROOT / "fixtures.json").read_text())
    item, = (x for x in fixtures["curves"] if x["id"] == "C2")
    curve = EllipticCurve(QQ, [QQ(x) for x in item["weierstrass_coefficients"]])
    known = [curve([QQ(x), QQ(y)]) for x, y in item["points"]]
    lower, upper, witnesses = curve.simon_two_descent(known_points=known, limbigprime=0)
    R = RealField(256)
    def height(P):
        return R(P.height(precision=256, algorithm="pari"))
    witness_rows = []
    for P in witnesses:
        assert P.curve() == curve
        witness_rows.append({"coordinates": [str(P[0]), str(P[1])],
                             "height": str(height(P))})
    gram_det = None
    if len(witnesses) >= 3:
        three = witnesses[:3]
        gram = matrix(R, 3, 3,
                      lambda i, j: (height(three[i] + three[j])
                                    - height(three[i]) - height(three[j])) / 2)
        gram_det = str(gram.det())
    print(json.dumps({"schema": "xedni-selected-c2-simon-witness-v1",
                      "id": "C2", "rank_lower": int(lower), "rank_upper": int(upper),
                      "witnesses": witness_rows, "first_three_gram_det": gram_det,
                      "gram_precision_bits": 256}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
