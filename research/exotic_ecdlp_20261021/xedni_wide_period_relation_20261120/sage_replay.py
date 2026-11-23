#!/usr/bin/env python3
"""Checked Sage replay of every positive rational and local relation."""

import gzip
import hashlib
import json
from collections import Counter
from fractions import Fraction as F
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE = Path(__file__).resolve().parent
PARENT = HERE.parent


def main():
    result = json.loads((HERE / "result.json").read_text())
    raw = gzip.decompress((HERE / "rational-checks.jsonl.gz").read_bytes())
    assert hashlib.sha256(raw).hexdigest() == result["rational_checks_raw_sha256"]
    positives = [json.loads(x) for x in raw.splitlines()
                 if json.loads(x)["rational_relation"]]
    source = [json.loads(x) for x in gzip.decompress(
        (PARENT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz").read_bytes()
    ).splitlines()]
    assert len(positives) == result["counts"]["rational_relations"] == 40
    counts = Counter()
    j_values = set()
    first = None
    for row in positives:
        index = row["pair_index"]
        a = [F(x) for x in row["curve_coefficients"]]
        aq = [QQ(v.numerator)/QQ(v.denominator) for v in a]
        E = EllipticCurve(QQ, aq)
        p, q = [E([QQ(x), QQ(y)]) for x, y in source[index]["lifted_points"]]
        assert p + 2*q == E(0)
        j_values.add(str(E.j_invariant()))
        for prime, wanted in ((47, 43), (7, 5), (11, 11)):
            Fp = GF(prime)
            ap = [Fp(v.numerator)/Fp(v.denominator) for v in a]
            Ep = EllipticCurve(Fp, ap)
            assert Ep.cardinality() == wanted
            points = source[index]["reduced_points"] if prime == 47 else source[index]["lifted_points"]
            pp, qq = [Ep([Fp(x), Fp(y)]) for x, y in points]
            assert pp + 2*qq == Ep(0)
            counts[f"field_{prime}_checked"] += 1
        counts["rational_relations_checked"] += 1
        if first is None:
            first = {"pair_index": index, "coefficients": row["curve_coefficients"],
                     "points": source[index]["lifted_points"], "j_invariant": str(E.j_invariant())}
    return {"status": "PASS", "counts": dict(counts),
            "distinct_j_invariants": len(j_values), "first_witness": first,
            "rational_checks_raw_sha256": hashlib.sha256(raw).hexdigest()}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
