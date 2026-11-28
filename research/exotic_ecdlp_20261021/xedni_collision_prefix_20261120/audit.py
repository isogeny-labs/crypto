#!/usr/bin/env python3
"""Independent Fraction replay of prefix, collision, and target stop counts."""

import hashlib
import json
import statistics
import sys
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0,str(HERE.parent / "xedni_coefficient_choice_20261120"))
from audit import add, neg


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def multiple(k,P,curve):
    if k < 0:
        return multiple(-k,neg(P,curve),curve)
    out = None
    for _ in range(k):
        out = add(out,P,curve)
    return out


def main():
    protocol_raw = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_raw)
    result = json.loads((HERE / "result.json").read_text())
    assert sha(protocol_raw) == result["protocol_sha256"]
    box_raw = (HERE.parent / "xedni_two_generator_lift_20261120/box-rows.jsonl").read_bytes()
    assert sha(box_raw) == protocol["box_rows_raw_sha256"]
    box = [json.loads(line) for line in box_raw.splitlines() if line]
    box = [row for row in box if row["reduced_point"] is not None]
    box.sort(key=lambda row:(max(abs(row["a"]),abs(row["b"])),row["a"],row["b"]))
    prefix_raw = (HERE / "prefix-rows.jsonl").read_bytes()
    target_raw = (HERE / "target-rows.jsonl").read_bytes()
    assert sha(prefix_raw) == result["prefix_rows_raw_sha256"]
    assert sha(target_raw) == result["target_rows_raw_sha256"]
    prefix = [json.loads(line) for line in prefix_raw.splitlines()]
    targets = [json.loads(line) for line in target_raw.splitlines()]
    assert len(prefix) == len(box) == 80 and len(targets) == 42
    curve = [F(v) for v in protocol["curve_coefficients"]]
    Q = tuple(F(v) for v in protocol["Q"])
    R = tuple(F(v) for v in protocol["R"])
    seen = {}
    first_hit = {}
    direct = {}
    collision = None
    maximum = 0
    for i,(saved,row) in enumerate(zip(prefix,box),1):
        a,b = row["a"],row["b"]
        P = add(multiple(a,Q,curve),multiple(b,R,curve),curve)
        assert P is not None
        reduced = tuple(v.numerator*pow(v.denominator,-1,47)%47 for v in P)
        assert list(reduced) == row["reduced_point"] == saved["reduced_point"]
        own_height = max(abs(v.numerator).bit_length() for v in P)
        own_height = max(own_height,*(v.denominator.bit_length() for v in P))
        assert own_height == row["height_bits"] == saved["height_bits"]
        maximum = max(maximum,own_height)
        assert saved == {"index":i,"a":a,"b":b,"reduced_point":list(reduced),
                         "height_bits":own_height,"prefix_max_height_bits":maximum}
        first_hit.setdefault(reduced,(i,a,b))
        if b == 0:
            direct.setdefault(reduced,(i,a))
        if reduced in seen and collision is None:
            old_i,old_a,old_b,old_P = seen[reduced]
            assert P != old_P
            da,db = a-old_a,b-old_b
            assert db%43
            collision = {"index":i,"earlier_index":old_i,
                         "earlier_label":[old_a,old_b],"later_label":[a,b],
                         "delta":[da,db],"derived_R_over_G":(-da*pow(db,-1,43))%43}
        seen.setdefault(reduced,(i,a,b,P))
    assert collision == result["first_collision"]
    assert len(seen) == 42 and max(v[0] for v in first_hit.values()) == result["full_coverage_index"]
    curve_mod_47 = [0,0,0,46,45]
    G = (43,19)
    for row in targets:
        point = tuple(row["target"])
        hit_i,a,b = first_hit[point]
        direct_i,direct_a = direct.get(point,(10**9,None))
        stop_i = min(direct_i,max(hit_i,collision["index"]))
        reason = "direct_generator_multiple" if stop_i == direct_i else "target_lift_and_collision"
        scalar = direct_a%43 if reason == "direct_generator_multiple" else (a+collision["derived_R_over_G"]*b)%43
        assert row["first_hit_index"] == hit_i
        assert row["first_direct_index"] == (None if direct_i == 10**9 else direct_i)
        assert row["stop_index"] == row["rational_point_evaluations"] == row["reductions"] == row["dictionary_lookups"] == stop_i
        assert row["stop_reason"] == reason and row["recovered_scalar_mod_43"] == scalar
        assert row["prefix_max_height_bits"] == prefix[stop_i-1]["prefix_max_height_bits"]
        replay = None
        for _ in range(scalar):
            replay = add(replay,G,curve_mod_47,47)
        assert replay == point
    assert statistics.median(row["stop_index"] for row in targets) == result["target_stop_median"]
    return {"status":"PASS","prefix_points":len(prefix),"targets":len(targets),
            "first_collision_index":collision["index"],
            "full_coverage_index":result["full_coverage_index"],
            "independently_replayed_target_scalars":len(targets),
            "prefix_rows_raw_sha256":sha(prefix_raw),"target_rows_raw_sha256":sha(target_raw)}


if __name__ == "__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
