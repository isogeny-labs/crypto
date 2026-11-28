"""Checked-Sage replay of the first gate and relation witnesses."""

import gzip
import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF

HERE = Path(__file__).resolve().parent


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    result = json.loads((HERE / "result.json").read_text())
    packed = (HERE / "pair-rows.jsonl.gz").read_bytes()
    assert sha(packed) == result["pair_rows_packed_sha256"]
    rows = [json.loads(x) for x in gzip.decompress(packed).splitlines()]
    witness_rows = 0
    relation_witnesses = 0
    for row in rows:
        for arm, rec in row["arms"].items():
            p, order = map(int, arm.split("_"))
            F = GF(p)
            for kind, should_relate in (("first_gate", None), ("first_gate_relation", True)):
                a = rec[kind]
                if a is None:
                    continue
                E = EllipticCurve(F, a)
                P, Q = [E((F(x), F(y))) for x, y in row["lifted_points"]]
                assert int(E.cardinality()) == order
                local_relation = bool(P+2*Q == E(0))
                if should_relate:
                    assert local_relation
                    relation_witnesses += 1
                witness_rows += 1
    assert relation_witnesses == sum(v["pairs_with_gate_relation"] for v in result["counts"].values())
    print(json.dumps({"status": "PASS", "witness_rows_replayed": witness_rows,
                      "relation_witnesses": relation_witnesses,
                      "pair_rows_packed_sha256": sha(packed)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
