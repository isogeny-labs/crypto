#!/usr/bin/env python3
"""Batch traversal of all rational x candidates to audit per-target work."""

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
    rows_raw = (HERE / "target-rows.jsonl").read_bytes()
    assert hashlib.sha256(rows_raw).hexdigest() == result["target_rows_raw_sha256"]
    rows = [json.loads(line) for line in rows_raw.splitlines()]
    source_raw = SOURCE.read_bytes()
    assert hashlib.sha256(source_raw).hexdigest() == protocol["source_packed_sha256"]
    source = [json.loads(line) for line in gzip.decompress(source_raw).splitlines()]
    targets = sorted({tuple(point) for row in source for point in row["reduced_points"]})
    assert [tuple(row["target"]) for row in rows] == targets
    E = EllipticCurve(QQ, protocol["curve_coefficients"])
    Ep = EllipticCurve(GF(47), [0, 0, 0, 46, 45])
    assert Ep.cardinality() == 43
    state = {target: {"b_steps": 0, "residue_pairs": 0,
                      "discriminant_tests": 0, "rational_roots": 0,
                      "status": "height_cap_exhausted", "height": 1024, "lift": None}
             for target in targets}
    by_x = {x: [target for target in targets if target[0] == x]
            for x in {target[0] for target in targets}}
    for H in range(1, 1025):
        for b in range(1, H+1):
            for target in targets:
                if state[target]["status"] != "found":
                    state[target]["b_steps"] += 1
            if b % 47 == 0:
                continue
            inv_b = pow(b, -1, 47)
            candidates = range(-H, H+1) if b == H else (-H, H)
            for a in candidates:
                x = a*inv_b*inv_b % 47
                active = [target for target in by_x.get(x, [])
                          if state[target]["status"] != "found"]
                if not active:
                    continue
                for target in active:
                    state[target]["residue_pairs"] += 1
                if ZZ(a).gcd(ZZ(b)) != 1:
                    continue
                for target in active:
                    state[target]["discriminant_tests"] += 1
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
                    reduction = Ep([GF(47)(point[0].numerator())/GF(47)(point[0].denominator()),
                                    GF(47)(point[1].numerator())/GF(47)(point[1].denominator())])
                    for target in active:
                        if state[target]["status"] == "found":
                            continue
                        state[target]["rational_roots"] += 1
                        if (int(reduction[0]), int(reduction[1])) == target:
                            state[target].update(status="found", height=H,
                                                 lift=[a, b, int(c)])
    for row in rows:
        target = tuple(row["target"])
        assert {key: value for key, value in row.items() if key != "target"} == state[target]
    return {"status": "PASS", "targets": len(targets),
            "found": sum(s["status"] == "found" for s in state.values()),
            "height_cap_exhausted": sum(s["status"] == "height_cap_exhausted" for s in state.values()),
            "sum_discriminant_tests": sum(s["discriminant_tests"] for s in state.values()),
            "target_rows_raw_sha256": hashlib.sha256(rows_raw).hexdigest()}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
