"""Checked-Sage verification of every rational CRT lift and its local gates."""

import gzip
import hashlib
import json
from collections import Counter
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE = Path(__file__).resolve().parent
PRIOR = HERE.parent / "xedni_coefficient_choice_20261120"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    result = json.loads((HERE / "result.json").read_text())
    packed = (HERE / "curve-rows.jsonl.gz").read_bytes()
    assert sha(packed) == result["curve_rows_packed_sha256"]
    rows = [json.loads(x) for x in gzip.decompress(packed).splitlines()]
    source = [json.loads(x) for x in gzip.decompress((PRIOR / "source-rows-v2.jsonl.gz").read_bytes()).splitlines()]
    relation_by_pair = Counter()
    relation_j = set()
    relation_minimal = set()
    all_coefficients = set()
    for row in rows:
        a = [QQ(x) for x in row["coefficients"]]
        all_coefficients.add(tuple(row["coefficients"]))
        E = EllipticCurve(QQ, a)
        P, Q = [E(tuple(QQ(v) for v in xy)) for xy in source[row["pair_index"]]["lifted_points"]]
        related = bool(P+2*Q == E(0))
        assert related == row["rational_relation"]
        if related:
            relation_by_pair[row["pair_index"]] += 1
            relation_j.add(str(E.j_invariant()))
            relation_minimal.add(tuple(map(str, E.global_minimal_model().ainvs())))
        for p, target in ((47, 43), (7, 4), (11, 6)):
            assert all(v.denominator() % p for v in a)
            assert E.discriminant().valuation(p) == 0
            Ep = E.change_ring(GF(p))
            Pp, Qp = [Ep((GF(p)(xy[0]), GF(p)(xy[1]))) for xy in source[row["pair_index"]]["lifted_points"]]
            assert int(Ep.cardinality()) == target
            local = bool(Pp+2*Qp == Ep(0))
            assert local if p in (7, 11) else local == source[row["pair_index"]]["reduced_relation"]
    assert len(rows) == result["counts"]["curves"]
    assert sum(relation_by_pair.values()) == result["counts"]["rational_relation"]
    print(json.dumps({"status": "PASS", "curves_replayed": len(rows),
                      "distinct_coefficient_tuples": len(all_coefficients),
                      "rational_relation_by_pair": dict(relation_by_pair),
                      "rational_relation_j_invariants": len(relation_j),
                      "rational_relation_global_minimal_models": len(relation_minimal),
                      "curve_rows_packed_sha256": sha(packed)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
