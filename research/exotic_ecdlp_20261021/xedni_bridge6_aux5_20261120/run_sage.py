"""Exact coefficient-six bridge feasibility screen under order-five at 11."""

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


def packed_rows(rows):
    raw = "".join(json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n"
                  for row in rows).encode()
    return raw, gzip.compress(raw, mtime=0)


def coefficients(points, k):
    (x1, y1), (x2, y2) = [(QQ(x), QQ(y)) for x, y in points]
    if x1 == x2:
        return None
    a1, a2, a3 = [QQ(47*v) for v in k]
    z1 = y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1
    z2 = y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2
    a4 = (z1-z2)/(x1-x2)
    return [a1, a2, a3, a4, z1-a4*x1]


def local(E, coeffs, points, prime, desired_order):
    if any(QQ(c).denominator() % prime == 0 for c in coeffs):
        return {"status": "nonintegral"}
    if E.discriminant().valuation(prime) != 0:
        return {"status": "bad_reduction"}
    Ep = E.change_ring(GF(prime))
    P, Q = [Ep(GF(prime)(x), GF(prime)(y)) for x, y in points]
    order = int(Ep.cardinality())
    relation = bool(P + 6*Q == Ep(0))
    status = "pass" if order == desired_order and relation else (
        "order_mismatch" if order != desired_order else "relation_mismatch")
    return {"status": status, "order": order, "relation": relation}


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    source_bytes = SOURCE.read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    points = [json.loads(line)["lifted_points"] for line in gzip.decompress(source_bytes).splitlines()]
    assert len(points) == 1722
    E47 = EllipticCurve(GF(47), [0, 0, 0, 46, 45])
    source_rows = []
    surviving = []
    for index, pair in enumerate(points):
        P, Q = [E47(GF(47)(x), GF(47)(y)) for x, y in pair]
        compatible = bool(P+6*Q == E47(0))
        x_equal_11 = (pair[0][0]-pair[1][0]) % 11 == 0
        status = "candidate" if compatible and x_equal_11 else (
            "x_excluded" if compatible else "reduced_excluded")
        source_rows.append({"pair_index": index, "reduced_compatible": compatible,
                            "x_equal_mod11": x_equal_11, "status": status})
        if status == "candidate":
            surviving.append((index, pair))
    rows = []
    counts = Counter()
    for index, pair in surviving:
        for k in itertools.product(range(-2, 3), repeat=3):
            row = {"pair_index": index, "points": pair, "k": list(k)}
            coeffs = coefficients(pair, k)
            if coeffs is None:
                row["status"] = "same_x"
            else:
                row["coefficients"] = list(map(str, coeffs))
                E = EllipticCurve(QQ, coeffs)
                if E.is_singular():
                    row["status"] = "singular"
                else:
                    P, Q = [E(QQ(x), QQ(y)) for x, y in pair]
                    row["rational_relation"] = bool(P+6*Q == E(0))
                    row["p7"] = local(E, coeffs, pair, 7, 4)
                    row["p11"] = local(E, coeffs, pair, 11, 5)
                    row["status"] = "accepted" if row["p7"]["status"] == "pass" and row["p11"]["status"] == "pass" else "rejected"
            counts[row["status"]] += 1
            rows.append(row)
    source_raw, source_packed = packed_rows(source_rows)
    rows_raw, rows_packed = packed_rows(rows)
    (HERE / "source-selection.jsonl.gz").write_bytes(source_packed)
    (HERE / "candidate-rows.jsonl.gz").write_bytes(rows_packed)
    print(json.dumps({"schema": "xedni-bridge6-aux5-result-v1", "status": "PASS",
                      "source_sha256": sha(source_bytes),
                      "source_selection_raw_sha256": sha(source_raw),
                      "source_selection_packed_sha256": sha(source_packed),
                      "candidate_rows_raw_sha256": sha(rows_raw),
                      "candidate_rows_packed_sha256": sha(rows_packed),
                      "source_pairs": len(points),
                      "reduced_compatible": sum(row["reduced_compatible"] for row in source_rows),
                      "x_compatible": len(surviving),
                      "constructed": len(rows),
                      "counts": dict(counts),
                      "rational_relations": sum(row.get("rational_relation", False) for row in rows)},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
