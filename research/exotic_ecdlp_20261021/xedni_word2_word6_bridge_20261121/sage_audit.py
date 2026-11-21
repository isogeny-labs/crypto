"""Independent curve and matrix replay of the six-row cross-word witness."""

import gzip
import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, matrix

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
WORD6 = ROOT / "xedni_relation_word6_orbits_20261121"
WORD2 = ROOT / "xedni_relation_word_orbits_20261120"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def xy(P):
    return int(P[0]), int(P[1])


def fold(P):
    a, b = xy(P), xy(-P)
    return (a, 1) if a < b else (b, -1)


def source_rows(folder):
    result = json.loads((folder / "result.json").read_text())
    raw = gzip.decompress((folder / "rows.jsonl.gz").read_bytes())
    assert sha(raw) == result["rows_raw_sha256"]
    return result, [json.loads(line) for line in raw.splitlines()]


def rank(rows):
    return int(matrix(GF(43), rows).rank())


def main():
    protocol = json.loads((HERE / "sage-audit-protocol.json").read_text())
    source_raw = (HERE / "result.json").read_bytes()
    assert sha(source_raw) == protocol["source_result_sha256"]
    source = json.loads(source_raw)
    six_result, saved6 = source_rows(WORD6)
    two_result, saved2 = source_rows(WORD2)
    E = EllipticCurve(GF(47), [0, 0, 0, 46, 45])
    assert E.cardinality() == 43
    nonzero = [P for P in E.points() if P != E(0)]
    assert len(nonzero) == 42
    reps = sorted({fold(P)[0] for P in nonzero})
    assert [list(P) for P in reps] == six_result["folded_column_points"] == two_result["folded_column_points"]
    col = {P: j for j, P in enumerate(reps)}
    matrices = {}
    for word, saved in ((6, saved6), (2, saved2)):
        byQ = {tuple(row["Q"]): row for row in saved}
        assert len(byQ) == 42
        rows = []
        for Q in nonzero:
            P = -word*Q
            assert P+word*Q == E(0)
            record = byQ[xy(Q)]
            assert tuple(record["P"]) == xy(P)
            row = [0]*len(reps)
            for point, coefficient in ((P, 1), (Q, word)):
                rep, sign = fold(point)
                row[col[rep]] = (row[col[rep]]+coefficient*sign) % 43
            assert row == record["folded_row"]
            rows.append(row)
        matrices[word] = rows
    m6, m2 = matrices[6], matrices[2]
    assert rank(m6) == source["rank_word6"] == 14
    assert rank(m2) == source["rank_word2"] == 18
    assert rank(m6+m2) == source["rank_union"] == 20
    by_index = {row["source_index"]: row for row in saved2}
    witness = source["bridge_witness"]
    assert [x["source_index"] for x in witness] == [425, 2, 85, 201, 353, 511]
    picked = []
    for i, item in enumerate(witness):
        row = by_index[item["source_index"]]
        assert item["P"] == row["P"] and item["Q"] == row["Q"]
        picked.append(row["folded_row"])
        assert rank(m6+picked) == item["rank_after"] == 15+i
    assert rank(m6+picked) == 20
    for omitted in range(6):
        assert rank(m6+[row for i, row in enumerate(picked) if i != omitted]) == 19
    print(json.dumps({"schema": "xedni-word2-word6-bridge-sage-audit-v1",
                      "status": "PASS", "rank_word6": 14, "rank_word2": 18,
                      "rank_union": 20, "witness_indices": [x["source_index"] for x in witness],
                      "step_ranks": [x["rank_after"] for x in witness],
                      "removal_ranks": [19]*6, "folded_columns": len(reps)},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
