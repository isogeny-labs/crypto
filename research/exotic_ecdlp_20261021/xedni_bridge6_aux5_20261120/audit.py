"""Independent exact-fraction and small-field replay of the bridge-six screen."""

import gzip
import hashlib
import itertools
import json
from collections import Counter
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def unpack(name, raw_key, packed_key, result):
    packed = (HERE / name).read_bytes()
    raw = gzip.decompress(packed)
    assert sha(packed) == result[packed_key]
    assert sha(raw) == result[raw_key]
    return [json.loads(line) for line in raw.splitlines()]


def add(P, Q, a, p=None):
    if P is None:
        return Q
    if Q is None:
        return P
    a1, a2, a3, a4, _ = a
    x, y = P
    u, v = Q
    inverse_pair = y+v+a1*x+a3
    if x == u and (inverse_pair % p == 0 if p else inverse_pair == 0):
        return None
    if x == u:
        num = 3*x*x+2*a2*x+a4-a1*y
        den = 2*y+a1*x+a3
    else:
        num = v-y
        den = u-x
    lam = num*pow(int(den), -1, p) % p if p else num/den
    nu = y-lam*x
    xx = lam*lam+a1*lam-a2-x-u
    yy = -(lam+a1)*xx-nu-a3
    return (xx % p, yy % p) if p else (xx, yy)


def mul(Q, n, a, p=None):
    out = None
    for _ in range(n):
        out = add(out, Q, a, p)
    return out


def discriminant(a):
    a1, a2, a3, a4, a6 = a
    b2 = a1*a1+4*a2
    b4 = 2*a4+a1*a3
    b6 = a3*a3+4*a6
    b8 = a1*a1*a6+4*a2*a6-a1*a3*a4+a2*a3*a3-a4*a4
    return -b2*b2*b8-8*b4*b4*b4-27*b6*b6+9*b2*b4*b6


def coefficients(points, k):
    (x1, y1), (x2, y2) = [(F(x), F(y)) for x, y in points]
    if x1 == x2:
        return None
    a1, a2, a3 = [F(47*v) for v in k]
    z1 = y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1
    z2 = y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2
    a4 = (z1-z2)/(x1-x2)
    return (a1, a2, a3, a4, z1-a4*x1)


def local(a, points, p, desired):
    if any(c.denominator % p == 0 for c in a):
        return {"status": "nonintegral"}
    if discriminant(a).numerator % p == 0:
        return {"status": "bad_reduction"}
    ap = tuple(int(c.numerator * pow(c.denominator, -1, p)) % p for c in a)
    order = 1+sum((y*y+ap[0]*x*y+ap[2]*y-x*x*x-ap[1]*x*x-ap[3]*x-ap[4]) % p == 0
                  for x in range(p) for y in range(p))
    P, Q = [tuple(v % p for v in xy) for xy in points]
    relation = add(P, mul(Q, 6, ap, p), ap, p) is None
    status = "pass" if order == desired and relation else (
        "order_mismatch" if order != desired else "relation_mismatch")
    return {"status": status, "order": order, "relation": relation}


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    result = json.loads((HERE / "result.json").read_text())
    source_bytes = SOURCE.read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"] == result["source_sha256"]
    points = [json.loads(line)["lifted_points"] for line in gzip.decompress(source_bytes).splitlines()]
    source = unpack("source-selection.jsonl.gz", "source_selection_raw_sha256",
                    "source_selection_packed_sha256", result)
    rows = unpack("candidate-rows.jsonl.gz", "candidate_rows_raw_sha256",
                  "candidate_rows_packed_sha256", result)
    assert len(points) == len(source) == 1722 and len(rows) == 250
    fixed = (0, 0, 0, 46, 45)
    selected = []
    for i, (pair, row) in enumerate(zip(points, source)):
        P, Q = [tuple(v % 47 for v in xy) for xy in pair]
        compatible = add(P, mul(Q, 6, fixed, 47), fixed, 47) is None
        x_equal = (pair[0][0]-pair[1][0]) % 11 == 0
        status = "candidate" if compatible and x_equal else (
            "x_excluded" if compatible else "reduced_excluded")
        assert row == {"pair_index": i, "reduced_compatible": compatible,
                       "x_equal_mod11": x_equal, "status": status}
        if status == "candidate":
            selected.append(i)
    assert selected == [1328, 1370]
    count = Counter()
    for row, (index, k) in zip(rows, itertools.product(selected, itertools.product(range(-2, 3), repeat=3))):
        pair = points[index]
        assert row["pair_index"] == index and row["points"] == pair and row["k"] == list(k)
        a = coefficients(pair, k)
        assert a is not None and tuple(map(str, a)) == tuple(row["coefficients"])
        assert discriminant(a) != 0
        P, Q = [(F(x), F(y)) for x, y in pair]
        rational = add(P, mul(Q, 6, a), a) is None
        p7, p11 = local(a, pair, 7, 4), local(a, pair, 11, 5)
        status = "accepted" if p7["status"] == p11["status"] == "pass" else "rejected"
        assert row["rational_relation"] == rational
        assert row["p7"] == p7 and row["p11"] == p11 and row["status"] == status
        count[status] += 1
    assert dict(count) == result["counts"] and result["rational_relations"] == 0
    print(json.dumps({"schema": "xedni-bridge6-aux5-audit-v1", "status": "PASS",
                      "source_pairs": len(source), "reduced_compatible": 42,
                      "x_compatible": selected, "candidate_rows": len(rows),
                      "independent_status_counts": dict(count)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
