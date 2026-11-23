#!/usr/bin/env python3
"""Solve frozen rational-relation rows and replay every recovered scalar."""

import gzip
import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, matrix, vector

HERE = Path(__file__).resolve().parent
BOX = HERE.parent / "xedni_two_generator_lift_20261120/box-rows.jsonl"


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def read_rows(name, expected):
    raw = gzip.decompress((HERE / name).read_bytes())
    assert sha(raw) == expected
    return [json.loads(line) for line in raw.splitlines()]


def main():
    protocol_raw = (HERE / "solve-protocol.json").read_bytes()
    protocol = json.loads(protocol_raw)
    assert sha((HERE / "protocol.json").read_bytes()) == protocol["parent_protocol_sha256"]
    box_raw = BOX.read_bytes()
    assert sha(box_raw) == protocol["box_rows_raw_sha256"]
    box = [json.loads(line) for line in box_raw.splitlines()]
    columns = [(r["a"],r["b"]) for r in box if (r["a"],r["b"]) != (0,0)]
    assert len(columns) == 80
    col = {label:i for i,label in enumerate(columns)}
    additions = read_rows("addition-rows.jsonl.gz",protocol["addition_rows_raw_sha256"])
    collisions = read_rows("collision-rows.jsonl.gz",protocol["collision_rows_raw_sha256"])
    rows = []
    for spec in additions:
        row = {}
        for key,c in (("u",1),("v",1),("w",-1)):
            label = tuple(spec[key])
            if label != (0,0):
                j = col[label]
                row[j] = (row.get(j,0)+c)%43
        rows.append(row)
    for spec in collisions:
        rows.append({col[tuple(spec["anchor"])]:1,
                     col[tuple(spec["other"])]:42})
    rows.append({col[(1,0)]:1})
    F = GF(43)
    M = matrix(F,len(rows),80,lambda i,j: rows[i].get(j,0))
    rhs = vector(F,[0]*(len(rows)-1)+[1])
    assert M.rank() == 80
    solution = M.solve_right(rhs)
    assert M*solution == rhs
    Ep = EllipticCurve(GF(47),[0,0,0,46,45])
    G = Ep([-4,19])
    recovered = {}
    for record in box:
        label = (record["a"],record["b"])
        if label == (0,0):
            continue
        point = tuple(record["reduced_point"])
        k = int(solution[col[label]])
        assert k*G == Ep(list(point))
        if point in recovered:
            assert recovered[point] == k
        else:
            recovered[point] = k
    assert len(recovered) == 42
    output = [{"reduced_point":list(point),"scalar_mod_43":recovered[point]}
              for point in sorted(recovered)]
    raw = "".join(json.dumps(row,sort_keys=True)+"\n" for row in output).encode()
    (HERE / "target-log-rows.jsonl").write_bytes(raw)
    return {"schema":"xedni-two-generator-relation-solve-result-v1","status":"PASS",
            "protocol_sha256":sha(protocol_raw),"script_sha256":sha((HERE / "solve_sage.py").read_bytes()),
            "addition_rows":len(additions),"collision_rows":len(collisions),
            "matrix_rows":len(rows),"matrix_columns":80,"matrix_rank":int(M.rank()),
            "solved_targets":len(output),"R_scalar_from_relation_system":int(solution[col[(0,1)]]),
            "target_log_rows_raw_sha256":sha(raw)}


if __name__ == "__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
