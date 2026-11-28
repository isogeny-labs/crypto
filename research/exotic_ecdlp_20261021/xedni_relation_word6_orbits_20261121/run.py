"""Optimistic full-pair coefficient-six relation matrix and orbit census."""

import gzip
import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"
ARITHMETIC = ROOT / "xedni_relation_rank_20261120/run.py"
SOURCE_POLICY = ROOT / "xedni_bridge6_source_policy_20261121/result.json"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def scalar(Q, n, p, arith):
    result = None
    while n:
        if n & 1:
            result = arith.add(result, Q, p)
        Q = arith.add(Q, Q, p)
        n >>= 1
    return result


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    source_bytes = SOURCE.read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    assert sha(ARITHMETIC.read_bytes()) == protocol["arithmetic_source_sha256"]
    policy_bytes = SOURCE_POLICY.read_bytes()
    assert sha(policy_bytes) == protocol["source_policy_result_sha256"]
    policy = json.loads(policy_bytes)
    spec = importlib.util.spec_from_file_location("xedni_rank_arithmetic", ARITHMETIC)
    arith = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(arith)
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    p, q = 47, 43
    group = sorted(arith.points(p))
    assert len(group)+1 == protocol["curve"]["group_order"] == q
    folded = sorted({arith.folded(P, p)[0] for P in group})
    assert len(folded) == 21
    col = {P: j for j, P in enumerate(folded)}
    source_indices = {}
    for i, item in enumerate(source):
        P, Q = [tuple(v % p for v in xy) for xy in item["lifted_points"]]
        source_indices[(P, Q)] = i
    rows, matrix = [], []
    for Q in group:
        P = arith.neg(scalar(Q, 6, p, arith), p)
        assert P in group and P != Q
        assert arith.add(P, scalar(Q, 6, p, arith), p) is None
        index = source_indices[(P, Q)]
        assert index in policy["selected_indices"]
        row = [0]*len(folded)
        for point, coefficient in ((P, 1), (Q, 6)):
            rep, sign = arith.folded(point, p)
            row[col[rep]] = (row[col[rep]]+coefficient*sign) % q
        rows.append({"source_index": index, "P": P, "Q": Q, "folded_row": row})
        matrix.append(row)
    assert len(rows) == protocol["expected_rows"]
    assert {r["source_index"] for r in rows} == set(policy["selected_indices"])
    unseen = set(folded)
    cycles = []
    while unseen:
        start = min(unseen)
        cycle = []
        point = start
        while point not in cycle:
            assert point in unseen
            cycle.append(point)
            unseen.remove(point)
            point = arith.folded(arith.neg(scalar(point, 6, p, arith), p), p)[0]
        assert point == start
        cycles.append(cycle)
    colsets = [{col[P] for P in c} for c in cycles]
    assert all(sum(any(row[j] for j in c) for c in colsets) == 1 for row in matrix)
    block_ranks = []
    for c in colsets:
        block = [[row[j] for j in sorted(c)] for row in matrix if any(row[j] for j in c)]
        block_ranks.append(arith.rank(block, q)[0])
    total_rank = arith.rank(matrix, q)[0]
    assert total_rank == sum(block_ranks)
    assert len(cycles) == 7 and all(len(c) == 3 for c in cycles)
    assert block_ranks == [2]*7 and total_rank == 14
    raw = "".join(json.dumps(row, sort_keys=True, separators=(",", ":"))+"\n"
                  for row in rows).encode()
    packed = gzip.compress(raw, mtime=0)
    (HERE / "rows.jsonl.gz").write_bytes(packed)
    print(json.dumps({"schema": "xedni-relation-word6-orbits-result-v1", "status": "PASS",
                      "source_sha256": sha(source_bytes), "rows": len(rows),
                      "group_order": len(group)+1, "sign_folded_columns": len(folded),
                      "folded_column_points": [list(P) for P in folded],
                      "cycles": [[list(P) for P in c] for c in cycles],
                      "cycle_lengths": [len(c) for c in cycles],
                      "block_ranks": block_ranks, "rank_mod43": total_rank,
                      "nullity": len(folded)-total_rank,
                      "rows_raw_sha256": sha(raw), "rows_packed_sha256": sha(packed)},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
