"""Checked-Sage group-law replay of all full-law CRT relation records."""

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
    positives_raw = gzip.decompress((HERE / "positive-rows.jsonl.gz").read_bytes())
    assert sha(positives_raw) == result["positive_rows_raw_sha256"]
    rows = [json.loads(x) for x in positives_raw.splitlines()]
    source = [json.loads(x) for x in gzip.decompress((PRIOR / "source-rows-v2.jsonl.gz").read_bytes()).splitlines()]
    by_pair = Counter()
    j_values = set()
    for row in rows:
        a = [QQ(x) for x in row["coefficients"]]
        E = EllipticCurve(QQ, a)
        P, Q = [E(tuple(QQ(v) for v in xy)) for xy in source[row["pair_index"]]["lifted_points"]]
        assert P+2*Q == E(0)
        assert source[row["pair_index"]]["reduced_relation"]
        for p, target in ((47, 43), (7, 4), (11, 6)):
            assert all(v.denominator()%p for v in a)
            assert E.discriminant().valuation(p) == 0
            Ep = E.change_ring(GF(p))
            Pp, Qp = [Ep((GF(p)(xy[0]), GF(p)(xy[1]))) for xy in source[row["pair_index"]]["lifted_points"]]
            assert int(Ep.cardinality()) == target
            assert Pp+2*Qp == Ep(0)
        by_pair[row["pair_index"]] += 1
        j_values.add(str(E.j_invariant()))
    assert len(rows) == result["counts"]["rational_relations"]
    print(json.dumps({"status": "PASS", "positive_curves_replayed": len(rows),
                      "positive_by_pair": dict(by_pair), "distinct_j_invariants": len(j_values),
                      "positive_rows_raw_sha256": sha(positives_raw)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
