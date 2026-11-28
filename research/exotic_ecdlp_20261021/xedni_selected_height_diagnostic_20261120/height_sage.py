#!/usr/bin/env python3
"""Canonical-height and torsion diagnostic on frozen selected rational points."""

import json
from pathlib import Path

from sage.all import EllipticCurve, QQ, RealField


ROOT = Path(__file__).resolve().parent
FIXTURES = json.loads((ROOT / "fixtures.json").read_text())
RR = RealField(128)


def point(curve, coordinates):
    return curve([QQ(coordinates[0]), QQ(coordinates[1])])


def main():
    records = []
    for item in FIXTURES["curves"]:
        curve = EllipticCurve(QQ, [QQ(x) for x in item["weierstrass_coefficients"]])
        assert curve.discriminant() != 0
        point_rows = []
        height_cache = {}
        for coords in item["points"]:
            P = point(curve, coords)
            pari = RR(P.height(precision=128, algorithm="pari"))
            sage = RR(P.height(precision=128, algorithm="sage"))
            height_cache[tuple(coords)] = pari
            point_rows.append({
                "coordinates": coords, "height_pari": str(pari),
                "height_sage": str(sage),
                "crosscheck_abs_error": str(abs(pari - sage)),
            })
        pair_rows = []
        for pair in item["pairs"]:
            a, b = pair["points"]
            P, Q = point(curve, a), point(curve, b)
            hP, hQ = height_cache[tuple(a)], height_cache[tuple(b)]
            hplus = RR((P + Q).height(precision=128, algorithm="pari"))
            hminus = RR((P - Q).height(precision=128, algorithm="pari"))
            residual = abs(hplus + hminus - 2 * hP - 2 * hQ)
            pair_rows.append({
                "points": pair["points"], "parallelogram_residual": str(residual),
                "reduced_relation": pair["reduced_relation"],
                "rational_relation": pair["rational_relation"],
            })
        records.append({
            "id": item["id"], "discriminant": str(curve.discriminant()),
            "torsion_order": int(curve.torsion_subgroup().order()),
            "points": point_rows, "pairs": pair_rows,
        })
    print(json.dumps({"schema": "xedni-selected-height-result-v1",
                      "status": "complete", "curves": records}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
