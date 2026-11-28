"""Independent checked-Sage replay of every local arm and CRT relation."""

import gzip
import hashlib
import itertools
import json
from collections import Counter
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def coefficients(points, k):
    (x1, y1), (x2, y2) = [(QQ(x), QQ(y)) for x, y in points]
    if x1 == x2:
        return None
    a1, a2, a3 = [QQ(47*v) for v in k]
    z1 = y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1
    z2 = y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2
    a4 = (z1-z2)/(x1-x2)
    return [a1, a2, a3, a4, z1-a4*x1]


def local(E, coeffs, points, prime, desired):
    if any(QQ(c).denominator() % prime == 0 for c in coeffs):
        return {"status": "nonintegral"}
    if E.discriminant().valuation(prime) != 0:
        return {"status": "bad_reduction"}
    Ep = E.change_ring(GF(prime))
    P, Q = [Ep(GF(prime)(x), GF(prime)(y)) for x, y in points]
    order = int(Ep.cardinality())
    relation = bool(P+6*Q == Ep(0))
    status = "pass" if order == desired and relation else (
        "order_mismatch" if order != desired else "relation_mismatch")
    return {"status": status, "order": order, "relation": relation}


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    result = json.loads((HERE / "result.json").read_text())
    source_bytes = SOURCE.read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"] == result["source_sha256"]
    points = [json.loads(x)["lifted_points"] for x in gzip.decompress(source_bytes).splitlines()]
    pair = points[31]
    assert pair == result["points"] == [[2, 2], [-16, 17]]
    E47 = EllipticCurve(GF(47), [0, 0, 0, 46, 45])
    P47, Q47 = [E47(GF(47)(x), GF(47)(y)) for x, y in pair]
    assert P47+6*Q47 == E47(0)
    packed = (HERE / "local-rows.jsonl.gz").read_bytes()
    raw = gzip.decompress(packed)
    assert sha(packed) == result["local_rows_packed_sha256"]
    assert sha(raw) == result["local_rows_raw_sha256"]
    rows = [json.loads(line) for line in raw.splitlines()]
    assert len(rows) == 343+1331 == result["local_attempts"]
    counts = Counter()
    selected = {}
    expected = ((p, desired, k) for p, desired, bound in ((7, 4, 3), (11, 8, 5))
                for k in itertools.product(range(-bound, bound+1), repeat=3))
    for row, (prime, desired, k) in zip(rows, expected):
        assert row["prime"] == prime and row["k"] == list(k)
        coeffs = coefficients(pair, k)
        if coeffs is None:
            detail = {"status": "same_x"}
        else:
            E = EllipticCurve(QQ, coeffs)
            detail = {"status": "singular"} if E.is_singular() else local(E, coeffs, pair, prime, desired)
        assert row["local"] == detail
        counts[f"p{prime}_{detail['status']}"] += 1
        if detail["status"] == "pass" and prime not in selected:
            selected[prime] = list(k)
    assert dict(counts) == result["counts"]
    assert {str(p): k for p, k in selected.items()} == result["first_selected"]
    crt = result["crt_k"]
    assert all(-38 <= v <= 38 for v in crt)
    assert all(v % 7 == a % 7 and v % 11 == b % 11
               for v, a, b in zip(crt, selected[7], selected[11]))
    coeffs = coefficients(pair, crt)
    assert list(map(str, coeffs)) == result["crt_coefficients"]
    E = EllipticCurve(QQ, coeffs)
    assert not E.is_singular()
    assert local(E, coeffs, pair, 7, 4) == result["crt_local_7"]
    assert local(E, coeffs, pair, 11, 8) == result["crt_local_11"]
    P, Q = [E(QQ(x), QQ(y)) for x, y in pair]
    assert bool(P+6*Q == E(0)) == result["crt_rational_relation"]
    print(json.dumps({"schema": "xedni-bridge6-aux8-sage-audit-v1", "status": "PASS",
                      "replayed_local_rows": len(rows), "selected": result["first_selected"],
                      "crt_k": crt, "crt_local_7": result["crt_local_7"],
                      "crt_local_11": result["crt_local_11"],
                      "crt_rational_relation": result["crt_rational_relation"]},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
