"""All-pair upper-envelope relation matrix and doubling-orbit census."""

import gzip
import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"
ARITHMETIC = ROOT / "xedni_relation_rank_20261120/run.py"
spec = importlib.util.spec_from_file_location("xedni_rank_arithmetic", ARITHMETIC)
arith = importlib.util.module_from_spec(spec)
spec.loader.exec_module(arith)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    source_bytes = SOURCE.read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    assert sha(ARITHMETIC.read_bytes()) == protocol["arithmetic_source_sha256"]
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    p, q = 47, 43
    group = sorted(arith.points(p))
    assert len(group)+1 == protocol["curve"]["group_order"]
    orbits = sorted({arith.folded(P, p)[0] for P in group})
    assert len(orbits) == 21
    col = {P: j for j, P in enumerate(orbits)}
    source_indices = {}
    for i, item in enumerate(source):
        P, Q = [tuple(v % p for v in xy) for xy in item["lifted_points"]]
        source_indices[(P, Q)] = i
    rows = []
    matrix = []
    for Q in group:
        P = arith.neg(arith.add(Q, Q, p), p)
        assert P in group and P != Q
        assert arith.add(P, arith.add(Q, Q, p), p) is None
        index = source_indices[(P, Q)]
        assert source[index]["reduced_relation"]
        row = [0]*len(orbits)
        for point, coefficient in ((P, 1), (Q, 2)):
            rep, sign = arith.folded(point, p)
            row[col[rep]] = (row[col[rep]]+coefficient*sign) % q
        rows.append({"source_index": index, "P": P, "Q": Q, "folded_row": row})
        matrix.append(row)
    assert len(rows) == protocol["expected_rows"]
    assert {r["source_index"] for r in rows} == {i for i, r in enumerate(source)
                                                    if r["reduced_relation"]}
    unseen = set(orbits)
    cycles = []
    while unseen:
        start = min(unseen)
        cycle = []
        point = start
        while point not in cycle:
            assert point in unseen
            cycle.append(point)
            unseen.remove(point)
            point = arith.folded(arith.add(point, point, p), p)[0]
        assert point == start
        cycles.append(cycle)
    cycle_colsets = [{col[P] for P in cycle} for cycle in cycles]
    assert all(sum(any(row[j] for j in columns) for columns in cycle_colsets) == 1
               for row in matrix)
    block_ranks = []
    for columns in cycle_colsets:
        block = [[row[j] for j in sorted(columns)] for row in matrix
                 if any(row[j] for j in columns)]
        block_ranks.append(arith.rank(block, q)[0])
    assert all(sum(bool(row[j]) for cols in cycle_colsets for j in cols) <= 2
               for row in matrix)
    total_rank = arith.rank(matrix, q)[0]
    assert total_rank == sum(block_ranks)
    raw = "".join(json.dumps(row, sort_keys=True, separators=(",", ":"))+"\n"
                  for row in rows).encode()
    packed = gzip.compress(raw, mtime=0)
    (HERE / "rows.jsonl.gz").write_bytes(packed)
    print(json.dumps({"schema": "xedni-relation-word-orbits-result-v1", "status": "PASS",
                      "source_sha256": sha(source_bytes), "rows": len(rows),
                      "group_order": len(group)+1, "sign_orbits": len(orbits),
                      "folded_column_points": [list(P) for P in orbits],
                      "cycles": [[list(P) for P in cycle] for cycle in cycles],
                      "cycle_lengths": [len(cycle) for cycle in cycles],
                      "block_ranks": block_ranks, "rank_mod43": total_rank,
                      "nullity": len(orbits)-total_rank,
                      "rows_raw_sha256": sha(raw), "rows_packed_sha256": sha(packed)},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
