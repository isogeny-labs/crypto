"""Checked-Sage replay of relation-preserving k3 shifts."""

import gzip
import hashlib
import json
from collections import Counter
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
POSITIVE = ROOT / "xedni_full_law_crt_20261120/positive-rows.jsonl.gz"
GATE = ROOT / "xedni_six_pair_shift_gate_20261120/rows.jsonl.gz"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def coefficients(points, k):
    (x1, y1), (x2, y2) = [(QQ(x), QQ(y)) for x, y in points]
    a1, a2, a3 = [QQ(47*v) for v in k]
    z1 = y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1
    z2 = y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2
    a4 = (z1-z2)/(x1-x2)
    return [a1, a2, a3, a4, z1-a4*x1]


def main():
    p = json.loads((HERE / "protocol.json").read_text())
    assert sha(POSITIVE.read_bytes()) == p["source_positive_rows_sha256"]
    assert sha(GATE.read_bytes()) == p["shift_gate_rows_sha256"]
    positives = [json.loads(x) for x in gzip.open(POSITIVE, "rt")]
    gates = [json.loads(x) for x in gzip.open(GATE, "rt")]
    selected = {(r["source_index"], tuple(r["local_indices"])): r for r in gates
                if r["status"] == "centered_relation_necessary"}
    assert len(positives) == len(selected) == 8
    rows = []
    counts = Counter()
    for record in positives:
        index = record["pair_index"]
        key = (index, tuple(record["local_indices"]))
        gate = selected[key]
        assert record["k"] == gate["centered_k"]
        points = gate["points"]
        for t in range(-16, 17):
            k = record["k"].copy()
            k[2] += 77*t
            a = coefficients(points, k)
            E = EllipticCurve(QQ, a)
            P, Q = [E(QQ(x), QQ(y)) for x, y in points]
            related = P+2*Q == E(0)
            reductions = {}
            for prime in (47, 7, 11):
                assert all(c.denominator() % prime for c in a)
                F = GF(prime)
                b = [F(c) for c in a]
                Ep = EllipticCurve(F, b)
                p1, q1 = [Ep(F(x), F(y)) for x, y in points]
                reductions[str(prime)] = {"order": int(Ep.cardinality()),
                                          "relation": bool(p1+2*q1 == Ep(0)),
                                          "coefficients": [int(c) for c in b]}
            assert related
            assert reductions["47"]["coefficients"] == [0, 0, 0, 46, 45]
            assert reductions["7"]["order"] == 4
            assert reductions["11"]["order"] == 6
            assert reductions["7"]["coefficients"] == gate["a7"]
            assert reductions["11"]["coefficients"] == gate["a11"]
            rows.append({"source_index": index, "local_indices": list(key[1]),
                         "shift_t": t, "k": k, "coefficients": list(map(str, a)),
                         "discriminant": str(E.discriminant()), "j": str(E.j_invariant()),
                         "relation": bool(related), "reductions": reductions})
            counts["verified_relation"] += 1
    assert len(rows) == p["expected_rows"]
    raw = "".join(json.dumps(row, sort_keys=True, separators=(",", ":"))+"\n" for row in rows).encode()
    packed = gzip.compress(raw, mtime=0)
    (HERE / "rows.jsonl.gz").write_bytes(packed)
    j_values = {row["j"] for row in rows}
    print(json.dumps({"schema": "xedni-positive-shift-family-result-v1", "status": "PASS",
                      "rows": len(rows), "counts": dict(counts),
                      "distinct_j": len(j_values), "rows_raw_sha256": sha(raw),
                      "rows_packed_sha256": sha(packed),
                      "protocol_sha256": sha((HERE / "protocol.json").read_bytes())},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
