#!/usr/bin/env python3
"""Independent Sage incidence and finite-field matrix-rank audit."""

import gzip
import hashlib
import json
from collections import Counter
from fractions import Fraction as F
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ, matrix

HERE = Path(__file__).resolve().parent
PARENT = HERE.parent


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def main():
    result = json.loads((HERE / "result.json").read_text())
    raw = (HERE / "curve-rows.jsonl").read_bytes()
    assert sha(raw) == result["curve_rows_raw_sha256"]
    rows = [json.loads(x) for x in raw.splitlines()]
    source = [json.loads(x) for x in gzip.decompress(
        (PARENT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz").read_bytes()
    ).splitlines()]
    point_lifts = {}
    for old in source:
        for reduced, lift in zip(old["reduced_points"], old["lifted_points"]):
            key = tuple(reduced)
            value = tuple(F(v) for v in lift)
            assert key not in point_lifts or point_lifts[key] == value
            point_lifts[key] = value
    assert len(point_lifts) == 42
    E47 = EllipticCurve(GF(47), [0, 0, 0, 46, 45])
    orbit_keys = set()
    for point in point_lifts:
        P = E47([int(point[0]), int(point[1])])
        N = -P
        neg = (int(N[0]), int(N[1]))
        assert neg in point_lifts and neg != point
        orbit_keys.add(min(point, neg))
    orbits = sorted(orbit_keys)
    assert len(orbits) == 21
    orbit_index = {key:i for i,key in enumerate(orbits)}
    counts = Counter()
    patterns = Counter()
    curve_keys = set()
    matrix_rows = []
    for row in rows:
        a = [F(v) for v in row["curve_coefficients"]]
        aq = [QQ(v.numerator)/QQ(v.denominator) for v in a]
        E = EllipticCurve(QQ, aq)
        incident = sorted(point for point, lift in point_lifts.items()
                          if E.is_on_curve(QQ(lift[0].numerator)/QQ(lift[0].denominator),
                                           QQ(lift[1].numerator)/QQ(lift[1].denominator)))
        assert [list(P) for P in incident] == row["incident_points"]
        designated = [tuple(P) for P in source[row["pair_index"]]["reduced_points"]]
        assert [list(P) for P in designated] == row["designated_points"]
        assert set(incident) == set(designated)
        vector = [0]*21
        for coefficient, point in zip((1,2), designated):
            P = E47([int(point[0]), int(point[1])])
            N = -P
            neg = (int(N[0]), int(N[1]))
            key = min(point, neg)
            sign = 1 if point == key else -1
            vector[orbit_index[key]] = (vector[orbit_index[key]]+coefficient*sign)%43
        assert vector == row["signed_relation_row"]
        expected_pattern = {201:"A", 243:"minus_A", 425:"B", 467:"minus_B"}
        assert row["pair_index"] in expected_pattern
        patterns[expected_pattern[row["pair_index"]]] += 1
        matrix_rows.append(vector)
        curve_keys.add(tuple(a))
        counts["incidences_checked"] += len(point_lifts)
        counts["curves_checked"] += 1
    rank = matrix(GF(43), matrix_rows).rank()
    assert rank == result["global_signed_relation_rank_mod_43"] == 2
    assert len(curve_keys) == result["distinct_exact_curves"] == 40
    row_a = next(row for row in rows if row["pair_index"] == 201)["signed_relation_row"]
    row_b = next(row for row in rows if row["pair_index"] == 425)["signed_relation_row"]
    for row in rows:
        expected = {201:row_a, 243:[-v%43 for v in row_a],
                    425:row_b, 467:[-v%43 for v in row_b]}[row["pair_index"]]
        assert row["signed_relation_row"] == expected
    assert patterns == {"A":8, "minus_A":8, "B":12, "minus_B":12}
    return {"status": "PASS", "counts": dict(counts),
            "signed_row_patterns": dict(patterns),
            "global_signed_relation_rank_mod_43": int(rank),
            "curve_rows_raw_sha256": sha(raw)}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
