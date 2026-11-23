#!/usr/bin/env python3
"""Enumerate exact rational multiples and their reductions at 47."""

import gzip
import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE = Path(__file__).resolve().parent
PARENT = HERE.parent


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def bits(q):
    numerator = int(q.numerator())
    denominator = int(q.denominator())
    return {"numerator_bits": abs(numerator).bit_length(),
            "denominator_bits": denominator.bit_length(),
            "value_sha256": sha(f"{numerator}/{denominator}".encode())}


def reduce_q(q, p):
    numerator = int(q.numerator())
    denominator = int(q.denominator())
    assert denominator % p
    return numerator * pow(denominator, -1, p) % p


def main():
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    source_bytes = (PARENT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz").read_bytes()
    assert sha(source_bytes) == protocol["source_packed_sha256"]
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    source_points = {tuple(point) for row in source for point in row["reduced_points"]}
    assert len(source_points) == 42
    E = EllipticCurve(QQ, protocol["rational_curve_coefficients"])
    Q = E(protocol["rational_generator_lift"])
    Ep = EllipticCurve(GF(47), protocol["reduced_curve_coefficients_mod_47"])
    assert Ep.cardinality() == 43
    G = Ep(protocol["rational_generator_lift"])
    assert G.order() == 43
    rows = []
    seen = set()
    R = E(0)
    for k in range(1, 43):
        R += Q
        assert R == k*Q and R != E(0)
        x, y = R[0], R[1]
        reduced = (reduce_q(x, 47), reduce_q(y, 47))
        assert Ep(reduced) == k*G
        assert reduced in source_points and reduced not in seen
        seen.add(reduced)
        rows.append({"coefficient": k, "reduced_point": list(reduced),
                     "x": bits(x), "y": bits(y)})
    assert seen == source_points
    raw = "".join(json.dumps(row, sort_keys=True)+"\n" for row in rows).encode()
    (HERE / "multiple-rows.jsonl").write_bytes(raw)
    return {"schema": "xedni-fixed-lift-map-result-v1", "status": "PASS",
            "protocol_sha256": sha(protocol_bytes),
            "script_sha256": sha((HERE / "run_sage.py").read_bytes()),
            "source_packed_sha256": sha(source_bytes),
            "multiple_rows_raw_sha256": sha(raw),
            "rational_multiples": len(rows), "distinct_reductions": len(seen),
            "max_x_numerator_bits": max(row["x"]["numerator_bits"] for row in rows),
            "max_x_denominator_bits": max(row["x"]["denominator_bits"] for row in rows),
            "max_y_numerator_bits": max(row["y"]["numerator_bits"] for row in rows),
            "max_y_denominator_bits": max(row["y"]["denominator_bits"] for row in rows)}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
