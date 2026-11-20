"""Nested, exact xedni coefficient-budget census on the fixed full law."""

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
sys.path.insert(0, str(PRIOR))
from audit import prime_record, relation


def sha(data):
    return hashlib.sha256(data).hexdigest()


def coefficients(points, ks):
    (x1, y1), (x2, y2) = points
    a1, a2, a3 = (F(47*k) for k in ks)
    d1 = y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1
    d2 = y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2
    a4 = (d1-d2)/(x1-x2)
    return [a1, a2, a3, a4, d1-a4*x1]


def gate(rec, order):
    return rec["status"] == "good" and rec["order"] == order and rec["incidence_rank"] == 2


def main():
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    assert protocol["nested_budgets"] == [1, 2]
    source_bytes = (PRIOR / "source-rows-v2.jsonl.gz").read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    source = [json.loads(line) for line in gzip.decompress(source_bytes).splitlines()]
    assert len(source) == 1722 and sum(row["reduced_relation"] for row in source) == 42
    rows = []
    stream_hash = hashlib.sha256()
    totals = {str(K): Counter() for K in (1, 2)}
    for pair_index, prior in enumerate(source):
        points = [tuple(F(x) for x in P) for P in prior["lifted_points"]]
        counts = {str(K): Counter() for K in (1, 2)}
        picks = {str(K): {"strict": None, "weak": None} for K in (1, 2)}
        if points[0][0] == points[1][0]:
            for K in (1, 2):
                counts[str(K)]["same_x"] = 1
        else:
            for ks in itertools.product(range(-2, 3), repeat=3):
                a = coefficients(points, ks)
                assert all(v.denominator % 47 for v in a)
                assert [int(v.numerator * pow(v.denominator, -1, 47) % 47) for v in a] == [0, 0, 0, 46, 45]
                p7 = prime_record(a, points, 7)
                p11 = prime_record(a, points, 11)
                b = max(abs(k) for k in ks)
                flags = {arm: gate(p7, 4) and gate(p11, target)
                         for arm, target in (("strict", 6), ("weak", 7))}
                for K in (1, 2):
                    if b > K:
                        continue
                    group = counts[str(K)]
                    group["candidates"] += 1
                    group["p7_"+p7["status"]] += 1
                    group["p11_"+p11["status"]] += 1
                    for arm, ok in flags.items():
                        group[arm+"_passing_choices"] += int(ok)
                        if ok and picks[str(K)][arm] is None:
                            picks[str(K)][arm] = {
                                "k": list(ks), "coefficients": list(map(str, a)),
                                "rational_relation": relation(points, a),
                                "p7_relation": p7["relation"], "p11_relation": p11["relation"]}
                stream_hash.update((json.dumps([pair_index, ks, p7, p11, flags], sort_keys=True, separators=(",", ":"))+"\n").encode())
        row = {"pair_index": pair_index, "source_status": prior["status"],
               "reduced_relation": prior["reduced_relation"],
               "counts": {k: dict(v) for k, v in counts.items()}, "selected": picks}
        rows.append(json.dumps(row, sort_keys=True)+"\n")
        for K in (1, 2):
            group = totals[str(K)]
            group.update(counts[str(K)])
            for arm, pick in picks[str(K)].items():
                if pick is not None:
                    group[arm+"_selected_pairs"] += 1
                    group[arm+"_selected_reduced"] += int(prior["reduced_relation"])
                    group[arm+"_selected_rational"] += int(pick["rational_relation"])
    raw = "".join(rows).encode()
    packed = gzip.compress(raw, mtime=0)
    (HERE / "pair-rows.jsonl.gz").write_bytes(packed)
    assert gzip.decompress(packed) == raw
    assert totals["1"]["candidates"] == 45360 and totals["2"]["candidates"] == 210000
    print(json.dumps({"schema": "xedni-coefficient-budget-result-v1", "status": "PASS",
                      "protocol_sha256": sha(protocol_bytes), "source_sha256": sha(source_bytes),
                      "script_sha256": sha((HERE / "run.py").read_bytes()),
                      "arithmetic_sha256": sha((PRIOR / "audit.py").read_bytes()),
                      "raw_rows_sha256": sha(raw), "packed_rows_sha256": sha(packed),
                      "candidate_stream_sha256": stream_hash.hexdigest(),
                      "source_pairs": len(source), "counts": {k: dict(v) for k, v in totals.items()}},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
