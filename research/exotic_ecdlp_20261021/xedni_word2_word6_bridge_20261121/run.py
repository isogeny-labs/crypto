"""Find a minimum rank-completing set of coefficient-two rows."""

import gzip
import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
WORD6 = ROOT / "xedni_relation_word6_orbits_20261121"
WORD2 = ROOT / "xedni_relation_word_orbits_20261120"
MEASURED2 = ROOT / "xedni_relation_rank_20261120"
ARITH = MEASURED2 / "run.py"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def load_rows(folder, result):
    packed = (folder / "rows.jsonl.gz").read_bytes()
    raw = gzip.decompress(packed)
    assert sha(packed) == result["rows_packed_sha256"]
    assert sha(raw) == result["rows_raw_sha256"]
    return [json.loads(line) for line in raw.splitlines()]


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    assert sha(ARITH.read_bytes()) == protocol["arithmetic_sha256"]
    loaded = []
    for folder, key in ((WORD6, "word6_result_sha256"), (WORD2, "word2_result_sha256"),
                        (MEASURED2, "measured_word2_result_sha256")):
        raw = (folder / "result.json").read_bytes()
        assert sha(raw) == protocol[key]
        loaded.append(json.loads(raw))
    six, two, measured = loaded
    assert six["folded_column_points"] == two["folded_column_points"] == measured["folded_column_points"]
    spec = importlib.util.spec_from_file_location("xedni_rank_arithmetic", ARITH)
    arith = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(arith)
    rows6, rows2 = load_rows(WORD6, six), load_rows(WORD2, two)
    assert len(rows6) == len(rows2) == 42
    m6 = [row["folded_row"] for row in rows6]
    m2 = [row["folded_row"] for row in rows2]
    rank6 = arith.rank(m6, 43)[0]
    rank2 = arith.rank(m2, 43)[0]
    union_rank = arith.rank(m6+m2, 43)[0]
    assert (rank6, rank2, union_rank) == (14, 18, 20)
    by_index = {row["source_index"]: row for row in rows2}
    assert len(by_index) == 42 and 425 in by_index and 467 in by_index
    anchor = by_index[425]
    sign_twin = by_index[467]
    assert any(anchor["P"] == item["P"] and anchor["Q"] == item["Q"]
               for item in measured["raw_pair_counts"])
    assert arith.rank([anchor["folded_row"], sign_twin["folded_row"]], 43)[0] == 1
    chosen = []
    current = rank6
    for row in [anchor]+[by_index[i] for i in sorted(by_index) if i != 425]:
        newrank = arith.rank(m6+[x["folded_row"] for x in chosen]+[row["folded_row"]], 43)[0]
        if newrank > current:
            assert newrank == current+1
            chosen.append(row)
            current = newrank
        if current == union_rank:
            break
    assert current == 20 and len(chosen) == 6
    assert chosen[0]["source_index"] == 425
    assert arith.rank(m6+[x["folded_row"] for x in chosen], 43)[0] == 20
    for omitted in range(len(chosen)):
        assert arith.rank(m6+[x["folded_row"] for i, x in enumerate(chosen) if i != omitted], 43)[0] == 19
    witness = [{"source_index": row["source_index"], "P": row["P"], "Q": row["Q"],
                "rank_after": rank6+i+1,
                "measured_rational_support": row["source_index"] in (425, 467)}
               for i, row in enumerate(chosen)]
    print(json.dumps({"schema": "xedni-word2-word6-bridge-result-v1",
                      "status": "PASS", "rank_word6": rank6, "rank_word2": rank2,
                      "rank_union": union_rank, "nullity_union": 21-union_rank,
                      "rank_after_measured_word2_row": rank6+1,
                      "minimum_added_rows": len(chosen), "bridge_witness": witness,
                      "witness_measured_rows": sum(x["measured_rational_support"] for x in witness),
                      "word6_rows_raw_sha256": six["rows_raw_sha256"],
                      "word2_rows_raw_sha256": two["rows_raw_sha256"]},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
