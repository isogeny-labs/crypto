"""Exact finite-field rank diagnostic for the shifted xedni relations."""

import gzip
import hashlib
import json
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_positive_shift_family_20261120/rows.jsonl.gz"
POINT_SOURCE = HERE.parent / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def points(p):
    return [(x, y) for x in range(p) for y in range(p)
            if (y*y-x*x*x+x+2) % p == 0]


def neg(P, p):
    return None if P is None else (P[0], -P[1] % p)


def add(P, Q, p):
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P
    x2, y2 = Q
    if x1 == x2 and (y1+y2) % p == 0:
        return None
    if P == Q:
        slope = (3*x1*x1-1)*pow(2*y1, -1, p) % p
    else:
        slope = (y2-y1)*pow(x2-x1, -1, p) % p
    x3 = (slope*slope-x1-x2) % p
    return x3, (slope*(x1-x3)-y1) % p


def folded(P, p):
    N = neg(P, p)
    rep = min(P, N)
    return rep, 1 if P == rep else -1


def rank(matrix, p):
    m = [row.copy() for row in matrix]
    pivots = []
    rank_value = 0
    for col in range(len(m[0])):
        found = next((j for j in range(rank_value, len(m)) if m[j][col] % p), None)
        if found is None:
            continue
        m[rank_value], m[found] = m[found], m[rank_value]
        inv = pow(m[rank_value][col], -1, p)
        m[rank_value] = [(v*inv) % p for v in m[rank_value]]
        for j in range(len(m)):
            if j != rank_value and m[j][col]:
                factor = m[j][col]
                m[j] = [(a-factor*b) % p for a, b in zip(m[j], m[rank_value])]
        pivots.append(col)
        rank_value += 1
    return rank_value, pivots


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    packed = SOURCE.read_bytes()
    assert sha(packed) == protocol["source_sha256"]
    source = [json.loads(x) for x in gzip.decompress(packed).splitlines()]
    point_bytes = POINT_SOURCE.read_bytes()
    assert sha(point_bytes) == protocol["point_source_sha256"]
    point_source = [json.loads(x) for x in gzip.decompress(point_bytes).splitlines()]
    p = protocol["curve"]["field_prime"]
    q = protocol["curve"]["subgroup_order"]
    group = points(p)
    assert len(group)+1 == protocol["curve"]["group_order"] == q
    orbits = sorted({folded(P, p)[0] for P in group})
    assert len(orbits) == 21
    columns = {P: j for j, P in enumerate(orbits)}
    rows = []
    matrix = []
    raw_pairs = Counter()
    for item in source:
        index = item["source_index"]
        assert index in (425, 467)
        P, Q = [tuple(v % p for v in xy) for xy in point_source[index]["lifted_points"]]
        assert P in group and Q in group
        assert add(P, add(Q, Q, p), p) is None
        row = [0]*len(orbits)
        for point, coefficient in ((P, 1), (Q, 2)):
            rep, sign = folded(point, p)
            row[columns[rep]] = (row[columns[rep]]+coefficient*sign) % q
        raw_pairs[(P, Q)] += 1
        matrix.append(row)
        rows.append({"source_index": index, "local_indices": item["local_indices"],
                     "shift_t": item["shift_t"], "P": P, "Q": Q, "folded_row": row})
    assert len(rows) == protocol["expected_rows"]
    matrix_rank, pivots = rank(matrix, q)
    covered = [j for j in range(len(orbits)) if any(row[j] for row in matrix)]
    raw = "".join(json.dumps(row, sort_keys=True, separators=(",", ":"))+"\n" for row in rows).encode()
    packed_rows = gzip.compress(raw, mtime=0)
    (HERE / "rows.jsonl.gz").write_bytes(packed_rows)
    print(json.dumps({"schema": "xedni-relation-rank-result-v1", "status": "PASS",
                      "source_sha256": sha(packed), "rows": len(rows),
                      "group_order": len(group)+1, "subgroup_order": q,
                      "unfolded_points": len(group), "folded_columns": len(orbits),
                      "folded_column_points": [list(P) for P in orbits],
                      "raw_pair_counts": [{"P": list(P), "Q": list(Q), "count": count}
                                          for (P, Q), count in sorted(raw_pairs.items())],
                      "distinct_folded_rows": len({tuple(row) for row in matrix}),
                      "rank_mod43": matrix_rank, "pivot_columns": pivots,
                      "covered_columns": covered, "uncovered_columns": len(orbits)-len(covered),
                      "rows_raw_sha256": sha(raw), "rows_packed_sha256": sha(packed_rows)},
                     indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
