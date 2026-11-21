"""Independent Sage point-group and matrix-rank replay."""

import gzip
import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, matrix

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_positive_shift_family_20261120/rows.jsonl.gz"
POINT_SOURCE = HERE.parent / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def coords(P):
    return int(P[0]), int(P[1])


def fold(P):
    a, b = coords(P), coords(-P)
    return (a, 1) if a < b else (b, -1)


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    result = json.loads((HERE / "result.json").read_text())
    source_bytes = SOURCE.read_bytes()
    point_bytes = POINT_SOURCE.read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    assert sha(point_bytes) == protocol["point_source_sha256"]
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    points = [json.loads(x) for x in gzip.decompress(point_bytes).splitlines()]
    packed = (HERE / "rows.jsonl.gz").read_bytes()
    raw = gzip.decompress(packed)
    assert sha(packed) == result["rows_packed_sha256"]
    assert sha(raw) == result["rows_raw_sha256"]
    saved = [json.loads(x) for x in raw.splitlines()]
    assert [r["source_index"] for r in saved] == [425]*132 + [467]*132
    F47 = GF(47)
    E = EllipticCurve(F47, [0, 0, 0, 46, 45])
    assert E.cardinality() == 43
    nonzero = [P for P in E.points() if P != E(0)]
    assert len(nonzero) == 42
    orbits = sorted({fold(P)[0] for P in nonzero})
    assert len(orbits) == 21
    assert [list(x) for x in orbits] == result["folded_column_points"]
    column = {rep: j for j, rep in enumerate(orbits)}
    matrix_rows = []
    for item, record in zip(source, saved):
        index = item["source_index"]
        assert index in (425, 467)
        x1, x2 = points[index]["lifted_points"]
        P, Q = E(*[F47(v) for v in x1]), E(*[F47(v) for v in x2])
        assert P+2*Q == E(0)
        row = [0]*len(orbits)
        for point, coefficient in ((P, 1), (Q, 2)):
            rep, sign = fold(point)
            row[column[rep]] = (row[column[rep]]+coefficient*sign) % 43
        assert record["P"] == list(coords(P)) and record["Q"] == list(coords(Q))
        assert record["source_index"] == index
        assert record["local_indices"] == item["local_indices"]
        assert record["shift_t"] == item["shift_t"]
        assert record["folded_row"] == row
        matrix_rows.append(row)
    assert len(matrix_rows) == len(saved) == 264
    M = matrix(GF(43), matrix_rows)
    covered = [j for j in range(21) if any(row[j] for row in matrix_rows)]
    assert M.rank() == result["rank_mod43"] == 1
    assert covered == result["covered_columns"]
    assert len({tuple(row) for row in matrix_rows}) == result["distinct_folded_rows"] == 2
    print(json.dumps({"schema": "xedni-relation-rank-sage-audit-v1", "status": "PASS",
                      "rows": len(matrix_rows), "group_order": int(E.cardinality()),
                      "folded_columns": len(orbits), "rank_mod43": int(M.rank()),
                      "covered_columns": covered, "raw_rows_sha256": sha(raw)},
                     indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
