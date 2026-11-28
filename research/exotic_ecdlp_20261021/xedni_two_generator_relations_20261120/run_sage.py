#!/usr/bin/env python3
"""Exact rational-addition/collision relation matrix for the lift box."""

import gzip
import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ, matrix

HERE = Path(__file__).resolve().parent
BOX = HERE.parent / "xedni_two_generator_lift_20261120/box-rows.jsonl"


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def red(q):
    return int(q.numerator())*pow(int(q.denominator()), -1, 47) % 47


def save_gzip(path, rows):
    raw = "".join(json.dumps(row, sort_keys=True)+"\n" for row in rows).encode()
    packed = gzip.compress(raw, mtime=0)
    path.write_bytes(packed)
    return {"rows": len(rows), "raw_sha256": sha(raw), "gzip_sha256": sha(packed)}


def rank(rows, columns):
    return int(matrix(GF(43), len(rows), len(columns),
                      lambda i,j: rows[i].get(j, 0)).rank())


def main():
    protocol_raw = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_raw)
    box_raw = BOX.read_bytes()
    assert sha(box_raw) == protocol["box_rows_raw_sha256"]
    box = [json.loads(line) for line in box_raw.splitlines()]
    labels = [tuple(row[key] for key in ("a", "b")) for row in box]
    assert len(labels) == 81 and labels == sorted(labels)
    E = EllipticCurve(QQ, protocol["curve_coefficients"])
    Q = E(protocol["Q"])
    R = E([QQ(s) for s in protocol["R"]])
    Ep = EllipticCurve(GF(47), [0, 0, 0, 46, 45])
    points = {label: label[0]*Q+label[1]*R for label in labels}
    assert len(set(points.values())) == 81
    buckets = {}
    for row, label in zip(box, labels):
        P = points[label]
        if label == (0,0):
            assert P == E(0) and row["reduced_point"] is None
            continue
        reduced = Ep([red(P[0]), red(P[1])])
        assert [int(reduced[0]),int(reduced[1])] == row["reduced_point"]
        buckets.setdefault(tuple(row["reduced_point"]), []).append(label)
    assert len(buckets) == 42
    columns = [label for label in labels if label != (0,0)]
    col = {label: i for i, label in enumerate(columns)}
    addition_specs = []
    addition_vectors = []
    for i,u in enumerate(columns):
        for v in columns[i:]:
            w = (u[0]+v[0], u[1]+v[1])
            if w not in points:
                continue
            assert points[u]+points[v] == points[w]
            row = {}
            for label, coeff in ((u,1),(v,1),(w,-1)):
                if label != (0,0):
                    j = col[label]
                    row[j] = (row.get(j,0)+coeff) % 43
            assert any(row.values())
            addition_vectors.append(row)
            addition_specs.append({"u": list(u), "v": list(v), "w": list(w)})
    collision_specs = []
    collision_vectors = []
    for reduced in sorted(buckets):
        labels_at_point = sorted(buckets[reduced])
        anchor = labels_at_point[0]
        for other in labels_at_point[1:]:
            assert points[anchor] != points[other]
            collision_vectors.append({col[anchor]:1, col[other]:42})
            collision_specs.append({"reduced_point": list(reduced),
                                    "anchor": list(anchor), "other": list(other)})
    G = Ep([red(Q[0]),red(Q[1])])
    Rbar = Ep([red(R[0]),red(R[1])])
    assert Rbar == 9*G
    logs = [(a+9*b)%43 for a,b in columns]
    assert all(sum(v*logs[j] for j,v in row.items())%43 == 0
               for row in addition_vectors+collision_vectors)
    normalizer = {col[(1,0)]:1}
    addition_rank = rank(addition_vectors, columns)
    collision_rank = rank(addition_vectors+collision_vectors, columns)
    normalized_rank = rank(addition_vectors+collision_vectors+[normalizer], columns)
    add_digest = save_gzip(HERE / "addition-rows.jsonl.gz", addition_specs)
    collision_digest = save_gzip(HERE / "collision-rows.jsonl.gz", collision_specs)
    return {"schema": "xedni-two-generator-relations-result-v1", "status": "PASS",
            "protocol_sha256": sha(protocol_raw), "script_sha256": sha((HERE / "run_sage.py").read_bytes()),
            "box_rows_raw_sha256": sha(box_raw), "columns": len(columns),
            "reduced_buckets": len(buckets), "addition_rows": add_digest,
            "collision_rows": collision_digest, "addition_rank": addition_rank,
            "addition_plus_collision_rank": collision_rank,
            "normalized_rank": normalized_rank,
            "normalizer_label": [1,0], "R_reduced_scalar_relative_to_G": 9}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
