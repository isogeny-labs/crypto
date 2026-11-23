"""Replay every shifted rational relation in Sage's elliptic-curve group."""

import gzip
import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, QQ

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_coefficient_choice_20261120" / "source-rows-v2.jsonl.gz"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    result = json.loads((HERE / "result.json").read_text())
    source_bytes = SOURCE.read_bytes()
    assert sha(source_bytes) == result["source_sha256"]
    source = [json.loads(line) for line in gzip.decompress(source_bytes).splitlines()]
    packed = (HERE / "candidate-rows.jsonl.gz").read_bytes()
    assert sha(packed) == result["candidate_rows_packed_sha256"]
    raw = gzip.decompress(packed)
    assert sha(raw) == result["candidate_rows_raw_sha256"]
    checked = relations = 0
    for line in raw.splitlines():
        row = json.loads(line)
        a = [QQ(z) for z in row["coefficients"]]
        E = EllipticCurve(QQ, a)
        p, q = source[row["pair_index"]]["lifted_points"]
        P = E([QQ(p[0]), QQ(p[1])])
        Q = E([QQ(q[0]), QQ(q[1])])
        found = P + 2*Q == E(0)
        assert bool(found) == row["rational_relation"]
        checked += 1
        relations += int(found)
    assert checked == result["counts"]["constructed"] == 16200
    assert relations == result["counts"]["relations"]
    print(json.dumps({"status": "PASS", "checked": checked, "relations": relations,
                      "candidate_rows_raw_sha256": sha(raw)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
