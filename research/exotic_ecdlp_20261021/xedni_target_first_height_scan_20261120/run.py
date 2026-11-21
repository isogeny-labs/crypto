#!/usr/bin/env python3
"""Exact bounded-height rational x-coordinate census on one fixed curve."""

import gzip
import hashlib
import json
from math import gcd, isqrt
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def main():
    protocol_raw = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_raw)
    source_raw = SOURCE.read_bytes()
    assert digest(source_raw) == protocol["source_packed_sha256"]
    source = [json.loads(line) for line in gzip.decompress(source_raw).splitlines()]
    source_points = {tuple(point) for row in source for point in row["reduced_points"]}
    assert len(source_points) == 42
    a1, a2, a3, a4, a6 = protocol["rational_curve_coefficients"]
    prime = protocol["reduction_prime"]
    max_height = max(protocol["height_checkpoints"])
    rows = []
    checkpoints = []
    seen = set()
    tests = 0
    infinity_points = 0
    # First appearance of each canonical pair occurs at H=max(abs(a),b).
    for H in range(1, max_height + 1):
        for b in range(1, H + 1):
            values = range(-H, H + 1) if b == H else (-H, H)
            for a in values:
                if gcd(abs(a), b) != 1:
                    continue
                tests += 1
                B = a1 * a * b + a3 * b**3
                C = a**3 + a2*a*a*b*b + a4*a*b**4 + a6*b**6
                D = B*B + 4*C
                if D < 0:
                    continue
                root = isqrt(D)
                if root*root != D:
                    continue
                for signed_root in ((root,) if root == 0 else (root, -root)):
                    if (signed_root - B) % 2:
                        continue
                    c = (signed_root - B)//2
                    if b % prime == 0:
                        infinity_points += 1
                        reduced = None
                    else:
                        inv_b = pow(b, -1, prime)
                        reduced = (a*inv_b*inv_b % prime, c*inv_b**3 % prime)
                        assert reduced in source_points
                        seen.add(reduced)
                    rows.append({"height": H, "a": a, "b": b, "c": c,
                                 "reduced_point": list(reduced) if reduced else None})
        if H in protocol["height_checkpoints"]:
            checkpoints.append({"H": H, "discriminant_tests": tests,
                                "rational_points": len(rows),
                                "reduction_to_infinity": infinity_points,
                                "distinct_nonidentity_reductions": len(seen),
                                "uniform_target_coverage": f"{len(seen)}/42"})
    raw = "".join(json.dumps(row, sort_keys=True)+"\n" for row in rows).encode()
    (HERE / "point-rows.jsonl").write_bytes(raw)
    return {"schema": "xedni-target-first-height-scan-result-v1", "status": "PASS",
            "protocol_sha256": digest(protocol_raw), "script_sha256": digest((HERE / "run.py").read_bytes()),
            "source_packed_sha256": digest(source_raw), "point_rows_raw_sha256": digest(raw),
            "checkpoints": checkpoints}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
