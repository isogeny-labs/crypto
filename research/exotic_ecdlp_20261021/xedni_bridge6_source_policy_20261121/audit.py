"""Independent full attempt replay using character sums and double-and-add."""

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


def load_chunks(label, expected):
    files = sorted((HERE / label).glob("[0-9][0-9][0-9].jsonl"))
    raw = b"".join(p.read_bytes() for p in files)
    assert sha(raw) == expected[f"{label}_raw_sha256"]
    assert len(raw) == expected[f"{label}_bytes"] and len(files) == expected[f"{label}_parts"]
    return [json.loads(line) for line in raw.splitlines()]


def coefficients(points, k):
    (x, y), (u, v) = [(F(a), F(b)) for a, b in points]
    if x == u:
        return None
    a1, a2, a3 = [F(47*z) for z in k]
    b = y*y+a1*x*y+a3*y-x**3-a2*x*x
    c = v*v+a1*u*v+a3*v-u**3-a2*u*u
    a4 = (b-c)/(x-u)
    return a1, a2, a3, a4, b-a4*x


def discriminant(a):
    a1, a2, a3, a4, a6 = a
    b2 = a1*a1+4*a2
    b4 = 2*a4+a1*a3
    b6 = a3*a3+4*a6
    b8 = a1*a1*a6+4*a2*a6-a1*a3*a4+a2*a3*a3-a4*a4
    return -b2*b2*b8-8*b4**3-27*b6*b6+9*b2*b4*b6


def add(P, Q, a, p):
    if P is None:
        return Q
    if Q is None:
        return P
    x, y = P
    u, v = Q
    a1, a2, a3, a4, _ = a
    if x == u and (y+v+a1*x+a3) % p == 0:
        return None
    if x == u:
        numerator = 3*x*x+2*a2*x+a4-a1*y
        denominator = 2*y+a1*x+a3
    else:
        numerator, denominator = v-y, u-x
    slope = numerator*pow(denominator % p, -1, p) % p
    x3 = (slope*slope+a1*slope-a2-x-u) % p
    y3 = (-(slope+a1)*x3-y+slope*x-a3) % p
    return x3, y3


def scalar(Q, n, a, p):
    result = None
    while n:
        if n & 1:
            result = add(result, Q, a, p)
        Q = add(Q, Q, a, p)
        n >>= 1
    return result


def point_count(a, p):
    a1, a2, a3, a4, a6 = a
    total = p+1
    for x in range(p):
        rhs = (x**3+a2*x*x+a4*x+a6) % p
        d = ((a1*x+a3)**2+4*rhs) % p
        total += 0 if d == 0 else (1 if pow(d, (p-1)//2, p) == 1 else -1)
    return total


def local(a, points, p, desired):
    if any(c.denominator % p == 0 for c in a):
        return {"status": "nonintegral"}
    if discriminant(a).numerator % p == 0:
        return {"status": "bad_reduction"}
    am = tuple((c.numerator*pow(c.denominator, -1, p)) % p for c in a)
    count = point_count(am, p)
    P, Q = [tuple(z % p for z in xy) for xy in points]
    relation = add(P, scalar(Q, 6, am, p), am, p) is None
    status = "pass" if count == desired and relation else (
        "order_mismatch" if count != desired else "relation_mismatch")
    return {"status": status, "order": count, "relation": relation}


def main():
    protocol = json.loads((HERE / "audit-protocol.json").read_text())
    assert sha((HERE / "result.json").read_bytes()) == protocol["source_result_sha256"]
    result = json.loads((HERE / "result.json").read_text())
    points = [json.loads(line)["lifted_points"] for line in gzip.decompress(SOURCE.read_bytes()).splitlines()]
    assert len(points) == result["source_pairs"] == 1722
    fixed = (0, 0, 0, 46, 45)
    selected = []
    for i, pair in enumerate(points):
        P, Q = [tuple(z % 47 for z in xy) for xy in pair]
        if add(P, scalar(Q, 6, fixed, 47), fixed, 47) is None:
            selected.append(i)
    assert selected == result["selected_indices"]
    attempts = load_chunks("attempts", result)
    outcomes = load_chunks("outcomes", result)
    assert len(attempts) == result["attempts"] == 70308
    assert len(outcomes) == len(selected) == 42
    counts = Counter()
    first = {}
    pair_counts = {i: Counter() for i in selected}
    expected_inputs = ((i, p, k) for i in selected
                       for p, bound in ((7, 3), (11, 5))
                       for k in itertools.product(range(-bound, bound+1), repeat=3))
    for row, (expected_i, expected_p, expected_k) in zip(attempts, expected_inputs):
        index, p, k = row["pair_index"], row["prime"], row["k"]
        assert (index, p, k) == (expected_i, expected_p, list(expected_k))
        a = coefficients(points[index], k)
        expected = {"status": "same_x"} if a is None else (
            {"status": "singular"} if discriminant(a) == 0 else
            local(a, points[index], p, 4 if p == 7 else 8))
        assert row["local"] == expected
        counts[f"p{p}_{expected['status']}"] += 1
        pair_counts[index][f"p{p}_{expected['status']}"] += 1
        if expected["status"] == "pass" and (index, p) not in first:
            first[index, p] = k
    assert dict(counts) == result["attempt_counts"]
    assert len(first) == sum(x["local_counts"].get(f"p{p}_pass", 0) > 0
                              for x in outcomes for p in (7, 11))
    bridge_count = 0
    for row in outcomes:
        index = row["pair_index"]
        assert row["points"] == points[index]
        assert row["local_counts"] == dict(pair_counts[index])
        assert row["first_pass"] == {str(p): first[index, p] for p in (7, 11) if (index, p) in first}
        if row["status"] == "local_bridge":
            bridge_count += 1
            k = row["crt_k"]
            for p in (7, 11):
                assert all(k[j] % p == first[index, p][j] % p for j in range(3))
            a = coefficients(points[index], k)
            assert list(map(str, a)) == row["crt_coefficients"]
            assert local(a, points[index], 7, 4) == row["crt_local_7"]
            assert local(a, points[index], 11, 8) == row["crt_local_11"]
            assert not row["crt_rational_relation"]
        else:
            assert row["status"] == "no_local_bridge_in_boxes"
    assert bridge_count == result["outcome_counts"]["local_bridge"] == 22
    print(json.dumps({"schema": "xedni-bridge6-source-policy-audit-v1",
                      "status": "PASS", "selected_pairs": len(selected),
                      "attempts_replayed": len(attempts), "local_bridges": bridge_count,
                      "attempts_raw_sha256": result["attempts_raw_sha256"]}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
