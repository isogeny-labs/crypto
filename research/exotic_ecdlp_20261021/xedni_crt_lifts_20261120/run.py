"""Construct rational curves from every strict relation-compatible local residue pair."""

import gzip
import hashlib
import itertools
import json
import sys
from collections import Counter
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
PRIOR = HERE.parent / "xedni_coefficient_choice_20261120"
LOCAL = HERE.parent / "xedni_local_residue_20261120"
sys.path.insert(0, str(PRIOR))
from audit import discriminant, prime_record, relation
sys.path.insert(0, str(LOCAL))
from run import coeffs, incidence_rank


def sha(data):
    return hashlib.sha256(data).hexdigest()


def centered_crt_k(a7, a11):
    r7 = a7 * pow(47, -1, 7) % 7
    r11 = a11 * pow(47, -1, 11) % 11
    k = (r7 + 7 * ((r11-r7)*pow(7, -1, 11) % 11)) % 77
    return k if k <= 38 else k-77


def rational_coeffs(points, ks):
    (x1, y1), (x2, y2) = points
    assert x1 != x2
    a1, a2, a3 = (F(47*k) for k in ks)
    d1 = y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1
    d2 = y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2
    a4 = (d1-d2)/(x1-x2)
    return [a1, a2, a3, a4, d1-a4*x1]


def local_witnesses(points, p, order):
    pts = [tuple(int(v % p) for v in P) for P in points]
    assert pts[0][0] != pts[1][0] and incidence_rank(pts, p) == 2
    result = []
    for a in coeffs(pts, p):
        rec = prime_record([F(v) for v in a], pts, p)
        if rec["status"] == "good" and rec["order"] == order and rec["relation"]:
            result.append(a)
    return result


def bit_height(a):
    return max(max(abs(v.numerator).bit_length(), v.denominator.bit_length()) for v in a)


def main():
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    source_bytes = (PRIOR / "source-rows-v2.jsonl.gz").read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    local_result = json.loads((LOCAL / "result.json").read_text())
    assert local_result["candidate_stream_sha256"] == protocol["local_candidate_stream_sha256"]
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    local_rows = [json.loads(x) for x in gzip.decompress((LOCAL / "pair-rows.jsonl.gz").read_bytes()).splitlines()]
    output = []
    counts = Counter()
    first_relation = None
    seen_pairs = []
    for local in local_rows:
        index = local["pair_index"]
        if not (local["arms"]["7_4"]["first_gate_relation"] and local["arms"]["11_6"]["first_gate_relation"]):
            continue
        seen_pairs.append(index)
        points = [tuple(F(v) for v in P) for P in source[index]["lifted_points"]]
        assert source[index]["reduced_relation"]
        w7 = local_witnesses(points, 7, 4)
        w11 = local_witnesses(points, 11, 6)
        assert len(w7) == local["arms"]["7_4"]["counts"]["gate_relation"] == 2
        assert len(w11) == local["arms"]["11_6"]["counts"]["gate_relation"] == 2
        assert w7[0] == tuple(local["arms"]["7_4"]["first_gate_relation"])
        assert w11[0] == tuple(local["arms"]["11_6"]["first_gate_relation"])
        for i7, i11 in itertools.product(range(2), repeat=2):
            a7, a11 = w7[i7], w11[i11]
            base = [centered_crt_k(a7[i], a11[i]) for i in range(3)]
            for shifts in itertools.product((-1, 0, 1), repeat=3):
                ks = [base[i]+77*shifts[i] for i in range(3)]
                a = rational_coeffs(points, ks)
                assert discriminant(a) != 0
                assert all(v.denominator % 47 for v in a)
                assert [int(v.numerator*pow(v.denominator, -1, 47)%47) for v in a] == [0, 0, 0, 46, 45]
                for p, target in ((7, a7), (11, a11)):
                    assert all(v.denominator % p for v in a)
                    assert tuple(int(v.numerator*pow(v.denominator, -1, p)%p) for v in a) == target
                    rec = prime_record(a, points, p)
                    assert rec["status"] == "good" and rec["relation"] and rec["order"] == (4 if p == 7 else 6)
                rational = relation(points, a)
                row = {"pair_index": index, "local_witness_indices": [i7, i11],
                       "shifts": shifts, "k": ks, "coefficients": list(map(str, a)),
                       "bit_height": bit_height(a), "rational_relation": rational}
                output.append(json.dumps(row, sort_keys=True)+"\n")
                counts["curves"] += 1
                counts["rational_relation"] += int(rational)
                counts["max_bit_height"] = max(counts["max_bit_height"], row["bit_height"])
                if rational and first_relation is None:
                    first_relation = row
    assert len(seen_pairs) == 14 and counts["curves"] == 1512
    raw = "".join(output).encode()
    packed = gzip.compress(raw, mtime=0)
    (HERE / "curve-rows.jsonl.gz").write_bytes(packed)
    print(json.dumps({"schema": "xedni-crt-lifts-result-v1", "status": "PASS",
                      "protocol_sha256": sha(protocol_bytes), "source_sha256": sha(source_bytes),
                      "local_candidate_stream_sha256": local_result["candidate_stream_sha256"],
                      "script_sha256": sha((HERE / "run.py").read_bytes()),
                      "arithmetic_sha256": sha((PRIOR / "audit.py").read_bytes()),
                      "local_script_sha256": sha((LOCAL / "run.py").read_bytes()),
                      "curve_rows_raw_sha256": sha(raw), "curve_rows_packed_sha256": sha(packed),
                      "survivor_pair_indices": seen_pairs, "counts": dict(counts),
                      "first_rational_relation": first_relation}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
