#!/usr/bin/env python3
"""Independent Sage integer-square census and elliptic-curve replay."""

import gzip
import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ, ZZ

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    result = json.loads((HERE / "result.json").read_text())
    rows_raw = (HERE / "point-rows.jsonl").read_bytes()
    assert hashlib.sha256(rows_raw).hexdigest() == result["point_rows_raw_sha256"]
    expected = [json.loads(line) for line in rows_raw.splitlines()]
    source_raw = SOURCE.read_bytes()
    assert hashlib.sha256(source_raw).hexdigest() == protocol["source_packed_sha256"]
    source = [json.loads(line) for line in gzip.decompress(source_raw).splitlines()]
    source_points = {tuple(point) for row in source for point in row["reduced_points"]}
    E = EllipticCurve(QQ, protocol["rational_curve_coefficients"])
    Ep = EllipticCurve(GF(47), [0, 0, 0, 46, 45])
    assert Ep.cardinality() == 43 and len(source_points) == 42
    found = []
    tests = 0
    checkpoints = []
    seen = set()
    for H in range(1, max(protocol["height_checkpoints"]) + 1):
        for b in range(1, H + 1):
            aa = range(-H, H + 1) if b == H else (-H, H)
            for a in aa:
                if ZZ(a).gcd(ZZ(b)) != 1:
                    continue
                tests += 1
                B = ZZ(94*a*b + 188*b**3)
                C = ZZ(a**3 + 1738*a*b**4 + 3805*b**6)
                D = B**2 + 4*C
                if D < 0 or not D.is_square():
                    continue
                root = D.sqrt()
                for sign in ([1] if root == 0 else [1, -1]):
                    if (sign*root-B) % 2:
                        continue
                    c = (sign*root-B)//2
                    point = E([QQ(a)/b**2, QQ(c)/b**3])
                    if b % 47:
                        reduction = [int(Ep([ZZ(point[0].numerator())*pow(int(point[0].denominator()), -1, 47),
                                             ZZ(point[1].numerator())*pow(int(point[1].denominator()), -1, 47)])[i])
                                     for i in (0, 1)]
                        assert tuple(reduction) in source_points
                        seen.add(tuple(reduction))
                    else:
                        reduction = None
                    found.append({"height": H, "a": int(a), "b": int(b), "c": int(c),
                                  "reduced_point": reduction})
        if H in protocol["height_checkpoints"]:
            checkpoints.append((H, tests, len(found), len(seen)))
    assert found == expected
    assert [(r["H"], r["discriminant_tests"], r["rational_points"],
             r["distinct_nonidentity_reductions"]) for r in result["checkpoints"]] == checkpoints
    return {"status": "PASS", "discriminant_tests": tests,
            "rational_points": len(found), "distinct_reductions": len(seen),
            "point_rows_raw_sha256": hashlib.sha256(rows_raw).hexdigest()}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
