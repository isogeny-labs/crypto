"""Independent CRT and rational doubling replay of every constructed curve."""

import gzip
import hashlib
import importlib.util
import itertools
import json
from collections import Counter
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
PRIOR = HERE.parent / "xedni_coefficient_choice_20261120"
LOCAL = HERE.parent / "xedni_local_residue_20261120"


def load_local_audit():
    spec = importlib.util.spec_from_file_location("local_residue_audit", LOCAL / "audit.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def sha(data):
    return hashlib.sha256(data).hexdigest()


def rational_double_relation(points, a):
    (x1, y1), (x, y) = points
    a1, a2, a3, a4, a6 = a
    den = 2*y+a1*x+a3
    if den == 0:
        return False
    lam = (3*x*x+2*a2*x+a4-a1*y)/den
    nu = (-x**3+a4*x+2*a6-a3*y)/den
    xd = lam*lam+a1*lam-a2-2*x
    yd = -(lam+a1)*xd-nu-a3
    return (xd, yd) == (x1, -y1-a1*x1-a3)


def on_curve(a, P):
    x, y = P
    a1, a2, a3, a4, a6 = a
    return y*y+a1*x*y+a3*y == x**3+a2*x*x+a4*x+a6


def local_witnesses(points, p, order, check):
    pts = [tuple(int(v % p) for v in P) for P in points]
    result = []
    for a in check.candidates(pts, p):
        if check.smooth(a, p) and check.order_character_sum(a, p) == order and check.relation_by_double(pts, a, p):
            result.append(a)
    return result


def crt_center(a7, a11):
    hits = [k for k in range(-38, 39) if (47*k-a7)%7 == 0 and (47*k-a11)%11 == 0]
    assert len(hits) == 1
    return hits[0]


def main():
    check = load_local_audit()
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    result = json.loads((HERE / "result.json").read_text())
    source_bytes = (PRIOR / "source-rows-v2.jsonl.gz").read_bytes()
    local_result = json.loads((LOCAL / "result.json").read_text())
    packed = (HERE / "curve-rows.jsonl.gz").read_bytes()
    raw = gzip.decompress(packed)
    assert result["protocol_sha256"] == sha(protocol_bytes)
    assert result["source_sha256"] == protocol["source_sha256"] == sha(source_bytes)
    assert result["local_candidate_stream_sha256"] == protocol["local_candidate_stream_sha256"] == local_result["candidate_stream_sha256"]
    assert result["script_sha256"] == sha((HERE / "run.py").read_bytes())
    assert result["arithmetic_sha256"] == sha((PRIOR / "audit.py").read_bytes())
    assert result["local_script_sha256"] == sha((LOCAL / "run.py").read_bytes())
    assert result["curve_rows_raw_sha256"] == sha(raw)
    assert result["curve_rows_packed_sha256"] == sha(packed)
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    saved = [json.loads(x) for x in raw.splitlines()]
    assert len(saved) == 1512
    offset = 0
    counts = Counter()
    first = None
    relation_by_pair = Counter()
    unique_relation_coeffs = set()
    max_height = 0
    for pair_index in result["survivor_pair_indices"]:
        old = source[pair_index]
        assert old["reduced_relation"]
        points = [tuple(F(v) for v in P) for P in old["lifted_points"]]
        w7 = local_witnesses(points, 7, 4, check)
        w11 = local_witnesses(points, 11, 6, check)
        assert len(w7) == len(w11) == 2
        for i7, i11 in itertools.product(range(2), repeat=2):
            local7, local11 = w7[i7], w11[i11]
            base = [crt_center(local7[i], local11[i]) for i in range(3)]
            for shifts in itertools.product((-1, 0, 1), repeat=3):
                row = saved[offset]
                offset += 1
                ks = [base[i]+77*shifts[i] for i in range(3)]
                a1, a2, a3 = (F(47*k) for k in ks)
                (x1, y1), (x2, y2) = points
                d1 = y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1
                d2 = y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2
                a4 = (d1-d2)/(x1-x2)
                a = [a1, a2, a3, a4, d1-a4*x1]
                assert row["pair_index"] == pair_index
                assert row["local_witness_indices"] == [i7, i11]
                assert row["shifts"] == list(shifts)
                assert row["k"] == ks and row["coefficients"] == list(map(str, a))
                assert all(on_curve(a, P) for P in points)
                for p, local in ((7, local7), (11, local11)):
                    assert all(v.denominator % p for v in a)
                    assert tuple(v.numerator*pow(v.denominator, -1, p)%p for v in a) == local
                assert [v.numerator*pow(v.denominator, -1, 47)%47 for v in a] == [0, 0, 0, 46, 45]
                related = rational_double_relation(points, a)
                assert row["rational_relation"] == related
                height = max(max(abs(v.numerator).bit_length(), v.denominator.bit_length()) for v in a)
                assert row["bit_height"] == height
                max_height = max(max_height, height)
                counts["curves"] += 1
                counts["rational_relation"] += int(related)
                if related:
                    relation_by_pair[pair_index] += 1
                    unique_relation_coeffs.add(tuple(row["coefficients"]))
                    if first is None:
                        first = row
    assert offset == len(saved)
    assert dict(counts | Counter({"max_bit_height": max_height})) == result["counts"]
    assert first == result["first_rational_relation"]
    assert relation_by_pair == {425: 36, 467: 36}
    assert len(unique_relation_coeffs) == 72
    relation_rows = [row for row in saved if row["rational_relation"]]
    compact = min(relation_rows, key=lambda row: (
        row["bit_height"], sum(abs(int(v)) for v in row["coefficients"]),
        row["pair_index"], row["coefficients"]))
    assert compact["pair_index"] == 425
    assert compact["coefficients"] == ["1081", "1034", "0", "-6816", "-3997"]
    print(json.dumps({"status": "PASS", "curves_replayed": offset,
                      "rational_relation_by_pair": dict(relation_by_pair),
                      "distinct_relation_coefficient_tuples": len(unique_relation_coeffs),
                      "compact_witness": compact,
                      "curve_rows_raw_sha256": sha(raw)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
