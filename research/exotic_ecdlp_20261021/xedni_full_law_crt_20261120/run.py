"""Target-independent full-law CRT curve-selection census."""

import gzip
import hashlib
import itertools
import json
import sys
import time
from collections import Counter
from fractions import Fraction as F
from functools import lru_cache
from pathlib import Path

HERE = Path(__file__).resolve().parent
PRIOR = HERE.parent / "xedni_coefficient_choice_20261120"
sys.path.insert(0, str(PRIOR))
from audit import discriminant, relation

SQUARES = {p: {x*x % p for x in range(p)} for p in (7, 11)}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def point_rank(points, p):
    rows = [tuple(v % p for v in (y*y, x*y, y, x**3, x*x, x, 1)) for x, y in points]
    return 1 if rows[0] == rows[1] else 2


def curve_order(a, p):
    a1, a2, a3, a4, a6 = a
    count = p+1
    squares = SQUARES[p]
    for x in range(p):
        d = ((a1*x+a3)**2 + 4*(x**3+a2*x*x+a4*x+a6)) % p
        count += 0 if d == 0 else (1 if d in squares else -1)
    return count


@lru_cache(maxsize=None)
def local_gate(points, p, target):
    (x1, y1), (x2, y2) = points
    assert x1 != x2 and point_rank(points, p) == 2
    kept = []
    counts = Counter()
    inv_dx = pow((x1-x2)%p, -1, p)
    for a1, a2, a3 in itertools.product(range(p), repeat=3):
        d1 = (y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1) % p
        d2 = (y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2) % p
        a4 = (d1-d2)*inv_dx % p
        a6 = (d1-a4*x1) % p
        a = (a1, a2, a3, a4, a6)
        counts["residues"] += 1
        if discriminant(a) % p == 0:
            counts["singular"] += 1
            continue
        counts["smooth"] += 1
        if curve_order(a, p) == target:
            kept.append(a)
            counts["gate"] += 1
    return tuple(kept), dict(counts)


def centered_k(a7, a11):
    r7 = a7*pow(47, -1, 7) % 7
    r11 = a11*pow(47, -1, 11) % 11
    k = (r7+7*((r11-r7)*pow(7, -1, 11)%11))%77
    return k if k <= 38 else k-77


def rational_curve(points, k):
    (x1, y1), (x2, y2) = [tuple(F(v) for v in P) for P in points]
    a1, a2, a3 = [F(47*v) for v in k]
    d1 = y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1
    d2 = y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2
    a4 = (d1-d2)/(x1-x2)
    return [a1, a2, a3, a4, d1-a4*x1]


def main():
    start = time.monotonic()
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    source_bytes = (PRIOR / "source-rows-v2.jsonl.gz").read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    assert len(source) == 1722 and sum(row["reduced_relation"] for row in source) == 42
    rows = []
    totals = Counter()
    stream = hashlib.sha256()
    positive = []
    for index, old in enumerate(source):
        points = [tuple(P) for P in old["lifted_points"]]
        (x1, y1), (x2, y2) = points
        row = {"pair_index": index, "source_status": old["status"],
               "reduced_relation": old["reduced_relation"],
               "status": None, "local_counts": {}, "crt_candidates": 0,
               "rational_relations": 0, "first_rational_relation": None}
        if x1 == x2:
            row["status"] = "same_x_q"
        elif (x1-x2)%7 == 0:
            row["status"] = "x_collision_7"
        elif (x1-x2)%11 == 0:
            row["status"] = "x_collision_11"
        else:
            p7 = tuple(tuple(v%7 for v in P) for P in points)
            p11 = tuple(tuple(v%11 for v in P) for P in points)
            if point_rank(p7, 7) < 2:
                row["status"] = "rank_lt2_7"
            elif point_rank(p11, 11) < 2:
                row["status"] = "rank_lt2_11"
            else:
                w7, c7 = local_gate(p7, 7, 4)
                w11, c11 = local_gate(p11, 11, 6)
                row["local_counts"] = {"7": c7, "11": c11}
                if not w7:
                    row["status"] = "no_gate_7"
                elif not w11:
                    row["status"] = "no_gate_11"
                else:
                    row["status"] = "constructed"
                    for i7, a7 in enumerate(w7):
                        for i11, a11 in enumerate(w11):
                            k = [centered_k(a7[i], a11[i]) for i in range(3)]
                            a = rational_curve(points, k)
                            assert discriminant(a) != 0
                            assert all(v.denominator%p for v in a for p in (47, 7, 11))
                            assert [v.numerator*pow(v.denominator, -1, 47)%47 for v in a] == [0, 0, 0, 46, 45]
                            assert tuple(v.numerator*pow(v.denominator, -1, 7)%7 for v in a) == a7
                            assert tuple(v.numerator*pow(v.denominator, -1, 11)%11 for v in a) == a11
                            rational = relation([tuple(F(v) for v in P) for P in points], a)
                            row["crt_candidates"] += 1
                            row["rational_relations"] += int(rational)
                            candidate = {"pair_index": index, "local_indices": [i7, i11],
                                         "k": k, "coefficients": list(map(str, a)),
                                         "rational_relation": rational}
                            stream.update((json.dumps(candidate, sort_keys=True, separators=(",", ":"))+"\n").encode())
                            if rational:
                                if row["first_rational_relation"] is None:
                                    row["first_rational_relation"] = candidate
                                positive.append(candidate)
        totals["pairs"] += 1
        totals["status_"+row["status"]] += 1
        totals["local_residues_7"] += row["local_counts"].get("7", {}).get("residues", 0)
        totals["local_residues_11"] += row["local_counts"].get("11", {}).get("residues", 0)
        totals["local_gate_7"] += row["local_counts"].get("7", {}).get("gate", 0)
        totals["local_gate_11"] += row["local_counts"].get("11", {}).get("gate", 0)
        totals["crt_candidates"] += row["crt_candidates"]
        totals["rational_relations"] += row["rational_relations"]
        totals["reduced_event_pairs"] += int(old["reduced_relation"])
        totals["reduced_event_crt_candidates"] += row["crt_candidates"] if old["reduced_relation"] else 0
        totals["reduced_event_rational_relations"] += row["rational_relations"] if old["reduced_relation"] else 0
        rows.append(json.dumps(row, sort_keys=True)+"\n")
    raw = "".join(rows).encode()
    packed = gzip.compress(raw, mtime=0)
    (HERE / "pair-rows.jsonl.gz").write_bytes(packed)
    positives_raw = "".join(json.dumps(row, sort_keys=True)+"\n" for row in positive).encode()
    (HERE / "positive-rows.jsonl.gz").write_bytes(gzip.compress(positives_raw, mtime=0))
    print(json.dumps({"schema": "xedni-full-law-crt-result-v1", "status": "PASS",
                      "protocol_sha256": sha(protocol_bytes), "source_sha256": sha(source_bytes),
                      "script_sha256": sha((HERE / "run.py").read_bytes()),
                      "arithmetic_sha256": sha((PRIOR / "audit.py").read_bytes()),
                      "pair_rows_raw_sha256": sha(raw), "pair_rows_packed_sha256": sha(packed),
                      "positive_rows_raw_sha256": sha(positives_raw),
                      "candidate_stream_sha256": stream.hexdigest(),
                      "counts": dict(totals), "elapsed_s": round(time.monotonic()-start, 3)},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
