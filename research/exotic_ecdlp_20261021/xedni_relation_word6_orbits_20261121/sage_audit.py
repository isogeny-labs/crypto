"""Independent checked-Sage replay of the full coefficient-six envelope."""

import gzip
import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, matrix

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"
POLICY = ROOT / "xedni_bridge6_source_policy_20261121/result.json"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def xy(P):
    return int(P[0]), int(P[1])


def fold(P):
    a, b = xy(P), xy(-P)
    return (a, 1) if a < b else (b, -1)


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    result = json.loads((HERE / "result.json").read_text())
    source_bytes = SOURCE.read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    policy_bytes = POLICY.read_bytes()
    assert sha(policy_bytes) == protocol["source_policy_result_sha256"]
    policy = json.loads(policy_bytes)
    packed = (HERE / "rows.jsonl.gz").read_bytes()
    raw = gzip.decompress(packed)
    assert sha(packed) == result["rows_packed_sha256"]
    assert sha(raw) == result["rows_raw_sha256"]
    saved = [json.loads(x) for x in raw.splitlines()]
    F47 = GF(47)
    E = EllipticCurve(F47, [0, 0, 0, 46, 45])
    assert E.cardinality() == 43
    nonzero = [P for P in E.points() if P != E(0)]
    assert len(nonzero) == 42
    reps = sorted({fold(P)[0] for P in nonzero})
    assert len(reps) == result["sign_folded_columns"] == 21
    assert [list(P) for P in reps] == result["folded_column_points"]
    col = {P: j for j, P in enumerate(reps)}
    byQ = {tuple(row["Q"]): row for row in saved}
    assert len(byQ) == len(saved) == 42
    source_by_pair = {}
    for i, old in enumerate(source):
        pair = tuple(tuple(v % 47 for v in pt) for pt in old["lifted_points"])
        source_by_pair[pair] = i
    rows = []
    for Q in nonzero:
        P = -6*Q
        assert P+6*Q == E(0) and P != Q
        record = byQ[xy(Q)]
        assert tuple(record["P"]) == xy(P)
        index = source_by_pair[(xy(P), xy(Q))]
        assert record["source_index"] == index and index in policy["selected_indices"]
        row = [0]*len(reps)
        for point, coefficient in ((P, 1), (Q, 6)):
            rep, sign = fold(point)
            row[col[rep]] = (row[col[rep]]+coefficient*sign) % 43
        assert row == record["folded_row"]
        rows.append(row)
    assert {r["source_index"] for r in saved} == set(policy["selected_indices"])
    unseen = set(reps)
    cycles = []
    while unseen:
        start = min(unseen)
        cycle = []
        point = E(*[F47(v) for v in start])
        while xy(point) not in cycle:
            rep = fold(point)[0]
            assert rep in unseen
            cycle.append(rep)
            unseen.remove(rep)
            point = E(*[F47(v) for v in fold(-6*point)[0]])
        assert xy(point) == start
        cycles.append(cycle)
    assert [[list(P) for P in c] for c in cycles] == result["cycles"]
    colsets = [{col[P] for P in cycle} for cycle in cycles]
    assert all(sum(any(row[j] for j in c) for c in colsets) == 1 for row in rows)
    blocks = []
    for cycle in cycles:
        cols = [col[P] for P in cycle]
        block_rows = [[row[j] for j in cols] for row in rows if any(row[j] for j in cols)]
        blocks.append(int(matrix(GF(43), block_rows).rank()))
    rank = int(matrix(GF(43), rows).rank())
    assert blocks == result["block_ranks"] == [2]*7
    assert rank == result["rank_mod43"] == 14
    assert result["nullity"] == 21-rank == 7
    print(json.dumps({"schema": "xedni-relation-word6-orbits-sage-audit-v1",
                      "status": "PASS", "rows": len(rows),
                      "cycle_lengths": [len(c) for c in cycles],
                      "block_ranks": blocks, "rank_mod43": rank, "nullity": 21-rank,
                      "rows_raw_sha256": sha(raw)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
