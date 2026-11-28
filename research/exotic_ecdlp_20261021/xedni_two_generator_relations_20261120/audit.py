#!/usr/bin/env python3
"""Fraction point replay and independent sparse F43 elimination."""

import gzip
import hashlib
import json
import sys
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "xedni_coefficient_choice_20261120"))
from audit import add, neg


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def multiple(k, P, curve):
    if k < 0:
        return multiple(-k, neg(P, curve), curve)
    out = None
    for _ in range(k):
        out = add(out, P, curve)
    return out


def insert(basis, row):
    row = {j: c % 43 for j,c in row.items() if c % 43}
    for pivot in sorted(basis):
        if pivot not in row:
            continue
        factor = row[pivot]
        for j,c in basis[pivot].items():
            value = (row.get(j,0)-factor*c)%43
            if value:
                row[j] = value
            else:
                row.pop(j,None)
    if not row:
        return False
    pivot = min(row)
    inverse = pow(row[pivot],-1,43)
    basis[pivot] = {j:c*inverse%43 for j,c in row.items()}
    return True


def loaded_rows(name, digest_record):
    packed = (HERE / name).read_bytes()
    raw = gzip.decompress(packed)
    assert sha(packed) == digest_record["gzip_sha256"]
    assert sha(raw) == digest_record["raw_sha256"]
    rows = [json.loads(line) for line in raw.splitlines()]
    assert len(rows) == digest_record["rows"]
    return rows


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    result = json.loads((HERE / "result.json").read_text())
    box_raw = (HERE.parent / "xedni_two_generator_lift_20261120/box-rows.jsonl").read_bytes()
    assert sha(box_raw) == result["box_rows_raw_sha256"]
    box = [json.loads(line) for line in box_raw.splitlines()]
    curve = [F(v) for v in protocol["curve_coefficients"]]
    Q = tuple(F(v) for v in protocol["Q"])
    R = tuple(F(v) for v in protocol["R"])
    labels = [(r["a"],r["b"]) for r in box]
    points = {label:add(multiple(label[0],Q,curve),multiple(label[1],R,curve),curve)
              for label in labels}
    assert len(points) == 81 and len(set(points.values())) == 81
    reductions = {label:tuple(r["reduced_point"]) if r["reduced_point"] else None
                  for label,r in zip(labels,box)}
    for label,P in points.items():
        if P is None:
            assert label == (0,0) and reductions[label] is None
        else:
            reduced = tuple(v.numerator*pow(v.denominator,-1,47)%47 for v in P)
            assert reduced == reductions[label]
    columns = [label for label in labels if label != (0,0)]
    col = {label:i for i,label in enumerate(columns)}
    logs = [(a+9*b)%43 for a,b in columns]
    additions = loaded_rows("addition-rows.jsonl.gz",result["addition_rows"])
    collisions = loaded_rows("collision-rows.jsonl.gz",result["collision_rows"])
    basis = {}
    for spec in additions:
        u,v,w = [tuple(spec[k]) for k in ("u","v","w")]
        assert u <= v and (u[0]+v[0],u[1]+v[1]) == w
        assert add(points[u],points[v],curve) == points[w]
        row = {}
        for label,c in ((u,1),(v,1),(w,-1)):
            if label != (0,0):
                j=col[label]
                row[j] = (row.get(j,0)+c)%43
        assert sum(c*logs[j] for j,c in row.items())%43 == 0
        insert(basis,row)
    assert len(basis) == result["addition_rank"]
    first_new = None
    for spec in collisions:
        anchor,other = tuple(spec["anchor"]),tuple(spec["other"])
        assert points[anchor] != points[other]
        assert reductions[anchor] == reductions[other] == tuple(spec["reduced_point"])
        row = {col[anchor]:1,col[other]:42}
        assert sum(c*logs[j] for j,c in row.items())%43 == 0
        if insert(basis,row) and first_new is None:
            da,db = anchor[0]-other[0],anchor[1]-other[1]
            assert db%43 and (da+9*db)%43 == 0
            first_new = {"anchor":list(anchor),"other":list(other),
                         "delta":[da,db],"derived_R_over_G":(-da*pow(db,-1,43))%43}
    assert len(basis) == result["addition_plus_collision_rank"]
    assert first_new is not None and first_new["derived_R_over_G"] == 9
    assert insert(basis,{col[(1,0)]:1})
    assert len(basis) == result["normalized_rank"] == 80
    solve = json.loads((HERE / "solve-result.json").read_text())
    target_raw = (HERE / "target-log-rows.jsonl").read_bytes()
    assert sha(target_raw) == solve["target_log_rows_raw_sha256"]
    targets = [json.loads(line) for line in target_raw.splitlines()]
    recovered = {tuple(r["reduced_point"]):r["scalar_mod_43"] for r in targets}
    assert len(targets) == len(recovered) == solve["solved_targets"] == 42
    G = (43,19)
    curve_mod_47 = [0,0,0,46,45]
    for point,k in recovered.items():
        replay = None
        for _ in range(k):
            replay = add(replay,G,curve_mod_47,47)
        assert replay == point
    candidate_logs = [recovered[reductions[label]] for label in columns]
    assert candidate_logs[col[(1,0)]] == 1
    assert candidate_logs[col[(0,1)]] == solve["R_scalar_from_relation_system"] == 9
    for spec in additions:
        u,v,w = [tuple(spec[k]) for k in ("u","v","w")]
        value = candidate_logs[col[u]]+candidate_logs[col[v]]
        if w != (0,0):
            value -= candidate_logs[col[w]]
        assert value%43 == 0
    for spec in collisions:
        assert candidate_logs[col[tuple(spec["anchor"])]] == candidate_logs[col[tuple(spec["other"])]]
    return {"status":"PASS","columns":len(columns),"addition_rows":len(additions),
            "collision_rows":len(collisions),"addition_rank":result["addition_rank"],
            "addition_plus_collision_rank":result["addition_plus_collision_rank"],
            "normalized_rank":result["normalized_rank"],
            "first_rank_increasing_collision":first_new,
            "independently_replayed_target_scalars":len(recovered),
            "target_log_rows_raw_sha256":sha(target_raw)}


if __name__ == "__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
