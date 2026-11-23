#!/usr/bin/env python3
"""Height-score-ordered rational lift prefixes and target stop counts."""

import hashlib
import json
import statistics
from decimal import Decimal, getcontext
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE = Path(__file__).resolve().parent
BOX = HERE.parent / "xedni_two_generator_lift_20261120/box-rows.jsonl"
RADIUS = HERE.parent / "xedni_collision_prefix_20261120/target-rows.jsonl"


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def red(q):
    return int(q.numerator())*pow(int(q.denominator()),-1,47)%47


def dump(path,rows):
    raw = "".join(json.dumps(row,sort_keys=True)+"\n" for row in rows).encode()
    path.write_bytes(raw)
    return sha(raw)


def main():
    protocol_raw = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_raw)
    box_raw = BOX.read_bytes()
    assert sha(box_raw) == protocol["box_rows_raw_sha256"]
    box = [json.loads(line) for line in box_raw.splitlines()]
    box = [row for row in box if row["reduced_point"] is not None]
    assert len(box) == 80
    E = EllipticCurve(QQ,protocol["curve_coefficients"])
    Q = E(protocol["Q"])
    R = E([QQ(s) for s in protocol["R"]])
    pairing = E.height_pairing_matrix([Q,R],precision=protocol["height_pairing_precision_bits"])
    assert [[str(pairing[i,j]) for j in range(2)] for i in range(2)] == protocol["height_pairing_matrix"]
    getcontext().prec = 50
    hqq = Decimal(protocol["height_pairing_matrix"][0][0])
    hqr = Decimal(protocol["height_pairing_matrix"][0][1])
    hrr = Decimal(protocol["height_pairing_matrix"][1][1])
    def score(row):
        a,b = row["a"],row["b"]
        return hqq*a*a+2*hqr*a*b+hrr*b*b
    box.sort(key=lambda row:(score(row),row["a"],row["b"]))
    Ep = EllipticCurve(GF(47),[0,0,0,46,45])
    G = Ep([red(Q[0]),red(Q[1])])
    Rbar = Ep([red(R[0]),red(R[1])])
    assert G.order() == 43
    seen = {}
    first_hit = {}
    first_direct = {}
    prefix = []
    first_collision = None
    max_bits = 0
    for i,row in enumerate(box,1):
        a,b = row["a"],row["b"]
        P = a*Q+b*R
        assert P != E(0)
        point = (red(P[0]),red(P[1]))
        assert Ep(list(point)) == a*G+b*Rbar
        assert list(point) == row["reduced_point"]
        for axis in ("x","y"):
            value = P[0] if axis=="x" else P[1]
            n,d = int(value.numerator()),int(value.denominator())
            assert sha(f"{n}/{d}".encode()) == row[axis]["value_sha256"]
        max_bits = max(max_bits,row["height_bits"])
        prefix.append({"index":i,"a":a,"b":b,"reduced_point":list(point),
                       "height_bits":row["height_bits"],"prefix_max_height_bits":max_bits})
        first_hit.setdefault(point,(i,a,b))
        if b == 0:
            first_direct.setdefault(point,(i,a))
        if point in seen and first_collision is None:
            old_a,old_b,old_i = seen[point]
            da,db = a-old_a,b-old_b
            assert db%43
            lam = -da*pow(db,-1,43)%43
            assert lam*G == Rbar
            first_collision = {"index":i,"earlier_index":old_i,
                               "earlier_label":[old_a,old_b],"later_label":[a,b],
                               "delta":[da,db],"derived_R_over_G":lam}
        seen.setdefault(point,(a,b,i))
    assert len(seen) == 42 and first_collision is not None
    rows = []
    for point in sorted(seen):
        hit_i,a,b = first_hit[point]
        direct_i,direct_a = first_direct.get(point,(10**9,None))
        collision_i = first_collision["index"]
        stop_i = min(direct_i,max(hit_i,collision_i))
        if direct_i == stop_i:
            recovered = direct_a%43
            reason = "direct_generator_multiple"
        else:
            recovered = (a+first_collision["derived_R_over_G"]*b)%43
            reason = "target_lift_and_collision"
        assert recovered*G == Ep(list(point))
        rows.append({"target":list(point),"first_hit_index":hit_i,
                     "first_direct_index":None if direct_i==10**9 else direct_i,
                     "stop_index":stop_i,"stop_reason":reason,
                     "rational_point_evaluations":stop_i,"reductions":stop_i,
                     "dictionary_lookups":stop_i,
                     "prefix_max_height_bits":prefix[stop_i-1]["prefix_max_height_bits"],
                     "recovered_scalar_mod_43":recovered})
    prefix_sha = dump(HERE / "prefix-rows.jsonl",prefix)
    target_sha = dump(HERE / "target-rows.jsonl",rows)
    stops = [row["stop_index"] for row in rows]
    radius_raw = RADIUS.read_bytes()
    assert sha(radius_raw) == protocol["radius_target_rows_raw_sha256"]
    radius_rows = [json.loads(line) for line in radius_raw.splitlines()]
    paired = {tuple(row["target"]):row for row in radius_rows}
    assert len(paired) == len(rows) == 42
    assert all(row["recovered_scalar_mod_43"] == paired[tuple(row["target"])]["recovered_scalar_mod_43"] for row in rows)
    return {"schema":"xedni-height-order-prefix-result-v1","status":"PASS",
            "protocol_sha256":sha(protocol_raw),"script_sha256":sha((HERE / "run_sage.py").read_bytes()),
            "box_rows_raw_sha256":sha(box_raw),"prefix_rows_raw_sha256":prefix_sha,
            "radius_target_rows_raw_sha256":sha(radius_raw),
            "height_pairing_precision_bits":protocol["height_pairing_precision_bits"],
            "height_pairing_determinant":str(pairing.det()),
            "target_rows_raw_sha256":target_sha,"candidate_points":len(prefix),
            "distinct_reduced_targets":len(seen),"full_coverage_index":max(v[0] for v in first_hit.values()),
            "first_collision":first_collision,"targets":len(rows),
            "direct_generator_multiple_stops":sum(row["stop_reason"]=="direct_generator_multiple" for row in rows),
            "target_stop_min":min(stops),"target_stop_median":statistics.median(stops),
            "target_stop_max":max(stops),"sum_independent_target_point_evaluations":sum(stops),
            "target_stop_prefix_height_median_bits":statistics.median(row["prefix_max_height_bits"] for row in rows),
            "target_stop_prefix_height_max_bits":max(row["prefix_max_height_bits"] for row in rows),
            "smaller_stop_count_than_radius":sum(row["stop_index"] < paired[tuple(row["target"])]["stop_index"] for row in rows),
            "smaller_prefix_peak_than_radius":sum(row["prefix_max_height_bits"] < paired[tuple(row["target"])]["prefix_max_height_bits"] for row in rows)}


if __name__ == "__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
