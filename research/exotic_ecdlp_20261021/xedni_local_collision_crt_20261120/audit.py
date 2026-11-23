"""Independent integer/Fraction replay of the frozen local-collision CRT census."""

import gzip
import hashlib
import itertools
import json
from collections import Counter
from fractions import Fraction as F
from functools import lru_cache
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_coefficient_choice_20261120" / "source-rows-v2.jsonl.gz"


def sha(b):
    return hashlib.sha256(b).hexdigest()


def rows(name):
    b = gzip.decompress((HERE / name).read_bytes())
    return [json.loads(line) for line in b.splitlines()], b


def disc(a):
    a1, a2, a3, a4, a6 = a
    b2 = a1*a1+4*a2
    b4 = a1*a3+2*a4
    b6 = a3*a3+4*a6
    b8 = a1*a1*a6+4*a2*a6-a1*a3*a4+a2*a3*a3-a4*a4
    return -b2*b2*b8-8*b4**3-27*b6*b6+9*b2*b4*b6


def fcurve(x, y, a, p):
    a1, a2, a3, a4, a6 = a
    return (y*y+a1*x*y+a3*y-x**3-a2*x*x-a4*x-a6) % p


def order(a, p):
    return 1+sum(fcurve(x, y, a, p) == 0 for x in range(p) for y in range(p))


@lru_cache(maxsize=None)
def gates(points, p, target):
    (x1, y1), (x2, y2) = points
    found = []
    c = Counter()
    if x1 != x2:
        def local_candidates():
            for a1, a2, a3 in itertools.product(range(p), repeat=3):
                d1 = (y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1) % p
                d2 = (y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2) % p
                a4 = (d1-d2)*pow(x1-x2, -1, p) % p
                yield a1, a2, a3, a4
        iterator = local_candidates()
    elif y1 != y2:
        iterator = ((a1, a2, (-y1-y2-a1*x1)%p, a4)
                    for a1, a2, a4 in itertools.product(range(p), repeat=3))
    else:
        iterator = itertools.product(range(p), repeat=4)
    for a1, a2, a3, a4 in iterator:
        a6 = (y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1-a4*x1) % p
        a = (a1, a2, a3, a4, a6)
        c["residues"] += 1
        assert fcurve(x1, y1, a, p) == fcurve(x2, y2, a, p) == 0
        if disc(a)%p == 0:
            c["singular"] += 1
            continue
        c["smooth"] += 1
        if order(a, p) == target:
            c["gate"] += 1
            found.append(a)
    return tuple(found), dict(c)


def mod(v, p):
    v = F(v)
    assert v.denominator % p
    return v.numerator*pow(v.denominator, -1, p)%p


def coeffs(points, k):
    (x1, y1), (x2, y2) = points
    a1, a2, a3 = (F(47*t) for t in k)
    r1 = y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1
    r2 = y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2
    a4 = (r1-r2)/(x1-x2)
    return (a1, a2, a3, a4, r1-a4*x1)


def centered(a7, a11):
    for k in range(-38, 39):
        if 47*k%7 == a7 and 47*k%11 == a11:
            return k
    raise AssertionError("missing centered CRT representative")


def tangent_relation(points, a):
    (x1, y1), (x2, y2) = points
    a1, a2, a3, a4, _ = a
    den = 2*y2+a1*x2+a3
    if den == 0:
        return False
    slope = (3*x2*x2+2*a2*x2+a4-a1*y2)/den
    xdouble = slope*slope+a1*slope-a2-2*x2
    return x1 == xdouble and y1 == y2+slope*(xdouble-x2)


def main():
    protocol = json.loads((HERE / "protocol.json").read_bytes())
    result = json.loads((HERE / "result.json").read_bytes())
    source_bytes = SOURCE.read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"] == result["source_sha256"]
    source = [json.loads(line) for line in gzip.decompress(source_bytes).splitlines()]
    pair_rows, pair_bytes = rows("pair-rows.jsonl.gz")
    candidate_rows, cand_bytes = rows("candidate-rows.jsonl.gz")
    positive_rows, pos_bytes = rows("positive-rows.jsonl.gz")
    assert sha(pair_bytes) == result["pair_rows_raw_sha256"]
    assert sha(cand_bytes) == result["candidate_rows_raw_sha256"]
    assert sha(pos_bytes) == result["positive_rows_raw_sha256"]
    assert result["status"] == "complete"
    assert len(pair_rows) == 336
    counts = Counter()
    cursor = 0
    positives = []
    for row in pair_rows:
        index = row["pair_index"]
        old = source[index]
        points = tuple(tuple(F(v) for v in P) for P in old["lifted_points"])
        (x1, y1), (x2, y2) = points
        assert x1 != x2
        p = 7 if mod(x1-x2, 7) == 0 else 11
        assert row["collision_prime"] == p
        assert (x1-x2)%p == 0
        assert row["reduced_relation"] == bool(old["reduced_relation"])
        assert row["collision_kind"] == ("same_point" if (y1-y2)%p == 0 else "opposite_y")
        local = {}
        for pp, target in ((7, 4), (11, 6)):
            residue_points = tuple(tuple(int(v)%pp for v in P) for P in points)
            local[pp], c = gates(residue_points, pp, target)
            assert row["local_counts"][str(pp)] == c
        w7, w11 = local[7], local[11]
        assert row["local_pairs"] == len(w7)*len(w11)
        assert row["processed_local_pairs"] == row["local_pairs"]
        assert row["status"] == ("constructed" if row["local_pairs"] else "no_local_gate")
        built, no_digit, relations = 0, 0, 0
        for i7, a7 in enumerate(w7):
            for i11, a11 in enumerate(w11):
                base_k = [centered(a7[i], a11[i]) for i in range(3)]
                base = coeffs(points, base_k)
                target_a4 = (a7 if p == 7 else a11)[3]
                base_a4 = mod(base[3], p)
                pivot, shift = None, 0
                if base_a4 != target_a4:
                    dx = x1-x2
                    derivs = (x1*y1-x2*y2, -(x1*x1-x2*x2), y1-y2)
                    for i, deriv in enumerate(derivs):
                        slope = mod(47*77*deriv/dx, p)
                        if slope:
                            pivot = i
                            shift = (target_a4-base_a4)*pow(slope, -1, p)%p
                            if shift > p//2:
                                shift -= p
                            break
                    if pivot is None:
                        no_digit += 1
                        continue
                k = base_k.copy()
                if pivot is not None:
                    k[pivot] += 77*shift
                a = coeffs(points, k)
                assert disc(a) != 0
                assert all(mod(v, pp) == local_w[i]
                           for pp, local_w in ((7, a7), (11, a11))
                           for i, v in enumerate(a))
                assert tuple(mod(v, 47) for v in a) == (0, 0, 0, 46, 45)
                for x, y in points:
                    assert y*y+a[0]*x*y+a[2]*y == x**3+a[1]*x*x+a[3]*x+a[4]
                rational = tangent_relation(points, a)
                item = candidate_rows[cursor]
                cursor += 1
                assert item["pair_index"] == index and item["local_indices"] == [i7, i11]
                assert item["k"] == k and item["pivot"] == pivot and item["shift"] == shift
                assert tuple(item["local_7"]) == a7 and tuple(item["local_11"]) == a11
                assert tuple(map(F, item["coefficients"])) == a
                assert item["rational_relation"] == rational
                built += 1
                relations += int(rational)
                if rational:
                    positives.append(item)
        assert row["constructed"] == built
        assert row["no_extra_digit"] == no_digit
        assert row["rational_relations"] == relations
        counts["pairs"] += 1
        counts["processed_local_pairs"] += row["processed_local_pairs"]
        counts["constructed"] += built
        counts["no_extra_digit"] += no_digit
        counts["rational_relations"] += relations
        counts["reduced_event_pairs"] += int(old["reduced_relation"])
        counts["status_"+row["status"]] += 1
    assert cursor == len(candidate_rows)
    assert positives == positive_rows
    assert dict(counts) == result["counts"]
    print(json.dumps({"status": "PASS", "pairs": len(pair_rows),
                      "candidate_rows": cursor, "relations": len(positives),
                      "source_sha256": sha(source_bytes),
                      "pair_rows_raw_sha256": sha(pair_bytes),
                      "candidate_rows_raw_sha256": sha(cand_bytes)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
