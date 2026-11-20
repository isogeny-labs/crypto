"""Independent exact arithmetic replay, using no Sage group operations."""

import gzip
import hashlib
import itertools
import json
from collections import Counter
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent


def sha(data):
    return hashlib.sha256(data).hexdigest()


def neg(P, a):
    return None if P is None else (P[0], -P[1] - a[0]*P[0] - a[2])


def add(P, Q, a, p=None):
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P
    x2, y2 = Q
    if x1 == x2 and (y1 + y2 + a[0]*x1 + a[2]) % p == 0 if p else x1 == x2 and y1 + y2 + a[0]*x1 + a[2] == 0:
        return None
    if P == Q:
        num = 3*x1*x1 + 2*a[1]*x1 + a[3] - a[0]*y1
        den = 2*y1 + a[0]*x1 + a[2]
        nu_num = -x1**3 + a[3]*x1 + 2*a[4] - a[2]*y1
    else:
        num = y2-y1
        den = x2-x1
        nu_num = y1*x2-y2*x1
    lam = num * pow(int(den % p), -1, p) % p if p else num/den
    nu = nu_num * pow(int(den % p), -1, p) % p if p else nu_num/den
    x3 = lam*lam + a[0]*lam - a[1] - x1 - x2
    y3 = -(lam+a[0])*x3 - nu - a[2]
    return (x3 % p, y3 % p) if p else (x3, y3)


def relation(points, a, p=None):
    P, Q = points
    return add(P, add(Q, Q, a, p), a, p) is None


def discriminant(a):
    a1, a2, a3, a4, a6 = a
    b2 = a1*a1 + 4*a2
    b4 = 2*a4 + a1*a3
    b6 = a3*a3 + 4*a6
    b8 = a1*a1*a6 + 4*a2*a6 - a1*a3*a4 + a2*a3*a3 - a4*a4
    return -b2*b2*b8 - 8*b4**3 - 27*b6*b6 + 9*b2*b4*b6


def curve_order(a, p):
    return 1 + sum((y*y + a[0]*x*y + a[2]*y - x**3 - a[1]*x*x - a[3]*x - a[4]) % p == 0
                   for x in range(p) for y in range(p))


def prime_record(a, points, p):
    if any(c.denominator % p == 0 for c in a):
        return {"status": "nonintegral"}
    b = [int(c.numerator * pow(c.denominator, -1, p) % p) for c in a]
    if discriminant(b) % p == 0:
        return {"status": "bad_reduction"}
    pts = [tuple(int(v % p) for v in P) for P in points]
    rows = [tuple(v % p for v in (y*y, x*y, y, x**3, x*x, x, 1)) for x, y in pts]
    return {"status": "good", "order": curve_order(b, p),
            "incidence_rank": 1 if rows[0] == rows[1] else 2,
            "relation": relation(pts, b, p)}


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    source = (HERE / "source-rows-v2.jsonl.gz").read_bytes()
    assert sha(source) == protocol["source_sha256"]
    selected = [json.loads(line) for line in gzip.decompress(source).splitlines()
                if json.loads(line)["accepted"]]
    rows_data = (HERE / "rows.jsonl").read_bytes()
    rows = [json.loads(line) for line in rows_data.splitlines()]
    assert len(selected) == 20 and len(rows) == 540
    summary = json.loads((HERE / "result.json").read_text())
    assert summary["rows_sha256"] == sha(rows_data)
    assert summary["protocol_sha256"] == sha((HERE / "protocol.json").read_bytes())
    assert summary["script_sha256"] == sha((HERE / "run_sage.py").read_bytes())
    counts = Counter()
    first = {"strict": {}, "weak": {}}
    for index, (pair_index, ks) in enumerate(itertools.product(range(20), itertools.product((-1, 0, 1), repeat=3))):
        row = rows[index]
        prior = selected[pair_index]
        assert row["pair_index"] == pair_index and row["k"] == list(ks)
        points = [tuple(F(v) for v in P) for P in prior["lifted_points"]]
        assert points[0][0] != points[1][0]
        a1, a2, a3 = (F(47*k) for k in ks)
        d = [y*y+a1*x*y+a3*y-x**3-a2*x*x for x, y in points]
        a4 = (d[0]-d[1])/(points[0][0]-points[1][0])
        a6 = d[0]-a4*points[0][0]
        a = [a1, a2, a3, a4, a6]
        assert row["coefficients"] == list(map(str, a))
        assert discriminant(a) != 0
        assert row["status"] == "good_curve"
        assert row["rational_relation"] == relation(points, a)
        for p in (47, 7, 11):
            assert row["p"+str(p)] == prime_record(a, points, p)
        assert row["p47"]["order"] == 43
        assert row["p47"]["relation"] == prior["reduced_relation"]
        gate7 = row["p7"]["status"] == "good" and row["p7"]["order"] == 4 and row["p7"]["incidence_rank"] == 2
        for arm, target in (("strict", 6), ("weak", 7)):
            ok = gate7 and row["p11"]["status"] == "good" and row["p11"]["order"] == target and row["p11"]["incidence_rank"] == 2
            assert row[arm+"_accepted"] == ok
            counts[arm+"_candidate_accept"] += ok
            if ok and pair_index not in first[arm]:
                first[arm][pair_index] = {"k": list(ks), "reduced_relation": prior["reduced_relation"], "rational_relation": row["rational_relation"]}
        counts[row["status"]] += 1
        counts["p7_"+row["p7"]["status"]] += 1
        counts["p11_"+row["p11"]["status"]] += 1
        counts["rational_relation"] += row["rational_relation"]
    assert summary["counts"] == dict(counts)
    for arm, picked in first.items():
        saved = summary["selected"][arm]
        assert saved["first_by_pair"] == {str(k): v for k, v in picked.items()}
        assert saved["pairs"] == len(picked)
        assert saved["reduced_events"] == sum(v["reduced_relation"] for v in picked.values())
        assert saved["rational_events"] == sum(v["rational_relation"] for v in picked.values())
    print(json.dumps({"status": "PASS", "rows_replayed": len(rows),
                      "source_sha256": sha(source), "rows_sha256": sha(rows_data),
                      "strict_selected_pairs": len(first["strict"]),
                      "weak_selected_pairs": len(first["weak"])}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
