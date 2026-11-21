#!/usr/bin/env python3
"""Enumerate a two-generator rational coefficient box and compare heights."""

import gzip
import hashlib
import json
import statistics
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"
BASELINE = HERE.parent / "xedni_fixed_lift_map_20261120/multiple-rows.jsonl"


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def scalar(q):
    n, d = int(q.numerator()), int(q.denominator())
    return {"numerator_bits": abs(n).bit_length(), "denominator_bits": d.bit_length(),
            "value_sha256": sha(f"{n}/{d}".encode())}


def red(q):
    return int(q.numerator())*pow(int(q.denominator()), -1, 47) % 47


def main():
    protocol_raw = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_raw)
    source_raw = SOURCE.read_bytes()
    assert sha(source_raw) == protocol["source_packed_sha256"]
    source = [json.loads(line) for line in gzip.decompress(source_raw).splitlines()]
    source_points = {tuple(point) for row in source for point in row["reduced_points"]}
    assert len(source_points) == 42
    E = EllipticCurve(QQ, protocol["curve_coefficients"])
    Q = E(protocol["Q"])
    R = E([QQ(s) for s in protocol["R"]])
    rank_bounds = E.rank_bounds(known_points=[Q, R], limbigprime=0)
    assert rank_bounds == (2, 2)
    Ep = EllipticCurve(GF(47), [0, 0, 0, 46, 45])
    assert Ep.cardinality() == 43
    G = Ep([red(Q[0]), red(Q[1])])
    Rbar = Ep([red(R[0]), red(R[1])])
    assert G.order() == 43 and Rbar == 9*G
    rows = []
    buckets = {}
    for a in range(-4, 5):
        for b in range(-4, 5):
            P = a*Q+b*R
            reduced = a*G+b*Rbar
            if P == E(0):
                assert reduced == Ep(0) and (a,b) == (0,0)
                row = {"a": a, "b": b, "reduced_point": None,
                       "x": None, "y": None, "height_bits": None}
            else:
                assert P[0].denominator() % 47 and P[1].denominator() % 47
                assert reduced == Ep([red(P[0]), red(P[1])])
                point = (int(reduced[0]), int(reduced[1]))
                assert point in source_points
                x, y = scalar(P[0]), scalar(P[1])
                height = max(x["numerator_bits"], x["denominator_bits"],
                             y["numerator_bits"], y["denominator_bits"])
                row = {"a": a, "b": b, "reduced_point": list(point),
                       "x": x, "y": y, "height_bits": height}
                buckets.setdefault(point, []).append(row)
            rows.append(row)
    assert set(buckets) == source_points
    baseline_raw = BASELINE.read_bytes()
    baseline = [json.loads(line) for line in baseline_raw.splitlines()]
    assert len(baseline) == 42
    comparisons = []
    for old in baseline:
        point = tuple(old["reduced_point"])
        best = min(buckets[point], key=lambda row: (row["height_bits"], abs(row["a"])+abs(row["b"]), row["a"], row["b"]))
        old_height = max(old[axis][field] for axis in ("x", "y")
                         for field in ("numerator_bits", "denominator_bits"))
        comparisons.append({"reduced_point": list(point), "known_multiple_k": old["coefficient"],
                            "known_multiple_height_bits": old_height,
                            "box_a": best["a"], "box_b": best["b"],
                            "box_height_bits": best["height_bits"]})
    row_raw = "".join(json.dumps(row, sort_keys=True)+"\n" for row in rows).encode()
    compare_raw = "".join(json.dumps(row, sort_keys=True)+"\n" for row in comparisons).encode()
    (HERE / "box-rows.jsonl").write_bytes(row_raw)
    (HERE / "comparison-rows.jsonl").write_bytes(compare_raw)
    return {"schema": "xedni-two-generator-lift-result-v1", "status": "PASS",
            "protocol_sha256": sha(protocol_raw), "script_sha256": sha((HERE / "run_sage.py").read_bytes()),
            "source_packed_sha256": sha(source_raw), "baseline_raw_sha256": sha(baseline_raw),
            "box_rows_raw_sha256": sha(row_raw), "comparison_rows_raw_sha256": sha(compare_raw),
            "rank_bounds": [int(v) for v in rank_bounds], "R_reduced": [int(Rbar[0]),int(Rbar[1])],
            "R_reduced_scalar_relative_to_G": 9,
            "box_points": len(rows), "nonidentity_box_points": len(rows)-1,
            "distinct_nonidentity_reductions": len(buckets),
            "known_multiple_median_height_bits": statistics.median(r["known_multiple_height_bits"] for r in comparisons),
            "box_median_height_bits": statistics.median(r["box_height_bits"] for r in comparisons),
            "known_multiple_max_height_bits": max(r["known_multiple_height_bits"] for r in comparisons),
            "box_max_height_bits": max(r["box_height_bits"] for r in comparisons)}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
