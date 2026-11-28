"""Full point-pair law with one frozen bounded coefficient family."""

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
from audit import prime_record, relation  # exact independent formulas from the prior audit


def sha(data):
    return hashlib.sha256(data).hexdigest()


def coefficients(points, ks):
    (x1, y1), (x2, y2) = [tuple(F(v) for v in p) for p in points]
    if x1 == x2:
        return None
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
    source_bytes = (PRIOR / "source-rows-v2.jsonl.gz").read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    rows = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    assert len(rows) == 1722 and sum(x["reduced_relation"] for x in rows) == 42
    candidate_hash = hashlib.sha256()
    global_counts = Counter()
    pair_path = HERE / "pair-rows.jsonl"
    with pair_path.open("w") as out:
        for pair_index, source in enumerate(rows):
            points = [tuple(F(v) for v in P) for P in source["lifted_points"]]
            counts = Counter()
            selected = {"strict": None, "weak": None}
            if points[0][0] == points[1][0]:
                counts["same_x"] = 1
            else:
                for ks in itertools.product((-1, 0, 1), repeat=3):
                    a = coefficients(points, ks)
                    p7 = prime_record(a, points, 7)
                    p11 = prime_record(a, points, 11)
                    assert all(c.denominator % 47 for c in a)
                    assert [int(c.numerator * pow(c.denominator, -1, 47) % 47) for c in a] == [0, 0, 0, 46, 45]
                    counts["candidates"] += 1
                    counts["p7_"+p7["status"]] += 1
                    counts["p11_"+p11["status"]] += 1
                    flags = {}
                    for arm, order in (("strict", 6), ("weak", 7)):
                        ok = gate(p7, 4) and gate(p11, order)
                        flags[arm] = ok
                        counts[arm+"_passing_choices"] += int(ok)
                        if ok and selected[arm] is None:
                            rational = relation(points, a)
                            selected[arm] = {"k": list(ks), "coefficients": list(map(str, a)),
                                             "rational_relation": rational,
                                             "p7_relation": p7["relation"],
                                             "p11_relation": p11["relation"]}
                    candidate_hash.update((json.dumps([pair_index, ks, p7, p11, flags], sort_keys=True, separators=(",", ":"))+"\n").encode())
            row = {"pair_index": pair_index, "reduced_relation": source["reduced_relation"],
                   "source_status": source["status"], "counts": dict(counts), "selected": selected}
            out.write(json.dumps(row, sort_keys=True)+"\n")
            global_counts.update(counts)
            global_counts["source_"+source["status"]] += 1
            for arm in selected:
                if selected[arm] is not None:
                    global_counts[arm+"_selected_pairs"] += 1
                    global_counts[arm+"_selected_reduced"] += int(source["reduced_relation"])
                    global_counts[arm+"_selected_rational"] += int(selected[arm]["rational_relation"])
    result = {"schema": "xedni-full-law-coefficients-result-v1", "status": "PASS",
              "source_sha256": sha(source_bytes), "protocol_sha256": sha(protocol_bytes),
              "script_sha256": sha((HERE / "run.py").read_bytes()),
              "arithmetic_sha256": sha((PRIOR / "audit.py").read_bytes()),
              "pair_rows_sha256": sha(pair_path.read_bytes()),
              "candidate_stream_sha256": candidate_hash.hexdigest(),
              "source_pairs": len(rows), "counts": dict(global_counts)}
    assert global_counts["candidates"] == 45360
    print(json.dumps(result, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
