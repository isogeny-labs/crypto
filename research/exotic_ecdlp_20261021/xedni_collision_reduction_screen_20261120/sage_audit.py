"""Sage replay of the F47 gate and exact source-curve reduction for candidate rows."""

import gzip
import hashlib
import json
from fractions import Fraction
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_coefficient_choice_20261120" / "source-rows-v2.jsonl.gz"
LOCAL = ROOT / "xedni_local_collision_crt_20261120"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def rational_on_curve(point, a):
    x, y = map(Fraction, point)
    a1, a2, a3, a4, a6 = a
    return y*y+a1*x*y+a3*y == x*x*x+a2*x*x+a4*x+a6


def main():
    result = json.loads((HERE / "result.json").read_text())
    source_bytes = SOURCE.read_bytes()
    assert sha(source_bytes) == result["source_sha256"]
    source = [json.loads(s) for s in gzip.decompress(source_bytes).splitlines()]
    candidates_packed = (LOCAL / "candidate-rows.jsonl.gz").read_bytes()
    local_result = json.loads((LOCAL / "result.json").read_text())
    assert sha(candidates_packed) == local_result["candidate_rows_packed_sha256"]
    candidate_raw = gzip.decompress(candidates_packed)
    assert sha(candidate_raw) == local_result["candidate_rows_raw_sha256"]

    finite = EllipticCurve(GF(47), [0, 0, 0, -1, -2])
    assert finite.cardinality() == 43
    assert finite.discriminant() != 0
    selected = {row["pair_index"]: row for row in result["rows"]}
    for index, row in selected.items():
        p, q = source[index]["lifted_points"]
        P = finite([p[0] % 47, p[1] % 47])
        Q = finite([q[0] % 47, q[1] % 47])
        assert bool(P+2*Q == finite(0)) == row["reduced_relation"]

    candidate_count = eligible_candidates = eligible_rational_relations = 0
    fixed = [0, 0, 0, 46, 45]
    for line in candidate_raw.splitlines():
        candidate = json.loads(line)
        index = candidate["pair_index"]
        a = list(map(Fraction, candidate["coefficients"]))
        assert all(c.denominator % 47 != 0 for c in a)
        assert [c.numerator * pow(c.denominator, -1, 47) % 47 for c in a] == fixed
        p, q = source[index]["lifted_points"]
        assert rational_on_curve(p, a) and rational_on_curve(q, a)
        candidate_count += 1
        if selected[index]["reduced_relation"]:
            eligible_candidates += 1
            rational = EllipticCurve(QQ, [QQ(c.numerator)/QQ(c.denominator) for c in a])
            P = rational([QQ(p[0]), QQ(p[1])])
            Q = rational([QQ(q[0]), QQ(q[1])])
            eligible_rational_relations += int(P+2*Q == rational(0))
    assert candidate_count == local_result["counts"]["constructed"] == 18576
    assert eligible_candidates == result["counts"]["constructed_on_reduced_relation_pairs"] == 600
    assert eligible_rational_relations == 0
    print(json.dumps({"status": "PASS", "curve_order": int(finite.cardinality()),
                      "selected_pairs": len(selected), "candidate_curves": candidate_count,
                      "reduced_relation_candidate_curves": eligible_candidates,
                      "eligible_rational_relations": eligible_rational_relations,
                      "candidate_rows_raw_sha256": sha(candidate_raw)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
