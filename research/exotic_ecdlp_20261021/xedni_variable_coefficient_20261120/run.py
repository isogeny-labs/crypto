#!/usr/bin/env python3
"""Bounded coefficient screen on the frozen full-law CRT curve stream."""

import gzip
import hashlib
import json
import sys
from collections import Counter
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
PARENT = HERE.parent
FULL = PARENT / "xedni_full_law_crt_20261120"
PRIOR = PARENT / "xedni_coefficient_choice_20261120"
sys.path.insert(0, str(FULL))
import run as crt
sys.path.insert(0, str(PRIOR))
from audit import add, discriminant


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def relation_k(points, curve, k, p=None):
    target, generator = points
    acc = None
    for _ in range(k):
        acc = add(acc, generator, curve, p)
    return add(target, acc, curve, p) is None


def main():
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    source_bytes = (PRIOR / "source-rows-v2.jsonl.gz").read_bytes()
    assert digest(source_bytes) == protocol["source_packed_sha256"]
    source = [json.loads(line) for line in gzip.decompress(source_bytes).splitlines()]
    old_result = json.loads((FULL / "result.json").read_text())
    assert old_result["candidate_stream_sha256"] == protocol["crt_candidate_stream_sha256"]
    assert len(source) == 1722

    coefficients = protocol["coefficients"]
    counts = Counter()
    by_k = {str(k): Counter() for k in coefficients}
    positives = []
    old_stream = hashlib.sha256()
    for index, old in enumerate(source):
        points = [tuple(P) for P in old["lifted_points"]]
        reduced = [tuple(P) for P in old["reduced_points"]]
        compatible = {k: relation_k(reduced, [0, 0, 0, 46, 45], k, 47)
                      for k in coefficients}
        counts["reduced_coefficient_checks"] += len(coefficients)
        for k in coefficients:
            by_k[str(k)]["reduced_compatible_pairs"] += compatible[k]

        (x1, _), (x2, _) = points
        if x1 == x2 or (x1-x2) % 7 == 0 or (x1-x2) % 11 == 0:
            continue
        p7 = tuple(tuple(v % 7 for v in P) for P in points)
        p11 = tuple(tuple(v % 11 for v in P) for P in points)
        if crt.point_rank(p7, 7) < 2 or crt.point_rank(p11, 11) < 2:
            continue
        w7, _ = crt.local_gate(p7, 7, 4)
        w11, _ = crt.local_gate(p11, 11, 6)
        Fpoints = [tuple(F(v) for v in P) for P in points]
        for i7, a7 in enumerate(w7):
            for i11, a11 in enumerate(w11):
                ks = [crt.centered_k(a7[i], a11[i]) for i in range(3)]
                a = crt.rational_curve(points, ks)
                assert discriminant(a) != 0
                counts["crt_candidates"] += 1
                candidate = {"pair_index": index, "local_indices": [i7, i11],
                             "k": ks, "coefficients": list(map(str, a)),
                             "rational_relation": relation_k(Fpoints, a, 2)}
                old_stream.update((json.dumps(candidate, sort_keys=True,
                                              separators=(",", ":")) + "\n").encode())
                for k in coefficients:
                    if not compatible[k]:
                        continue
                    row = by_k[str(k)]
                    row["compatible_crt_candidates"] += 1
                    counts["local_checks_7"] += 1
                    if not relation_k(p7, a7, k, 7):
                        continue
                    row["local_7_pass"] += 1
                    counts["local_checks_11"] += 1
                    if not relation_k(p11, a11, k, 11):
                        continue
                    row["local_11_pass"] += 1
                    counts["rational_checks"] += 1
                    if relation_k(Fpoints, a, k):
                        row["rational_relations"] += 1
                        positives.append({"pair_index": index,
                                          "coefficient": k,
                                          "local_indices": [i7, i11],
                                          "coefficients": list(map(str, a)),
                                          "reduced_points": old["reduced_points"],
                                          "lifted_points": old["lifted_points"]})
    assert counts["crt_candidates"] == old_result["counts"]["crt_candidates"] == 32256
    assert old_stream.hexdigest() == protocol["crt_candidate_stream_sha256"]
    assert all(by_k[str(k)]["reduced_compatible_pairs"] == 42 for k in coefficients)
    assert by_k["2"]["rational_relations"] == 8
    positive_raw = "".join(json.dumps(x, sort_keys=True) + "\n" for x in positives).encode()
    (HERE / "positive-rows.jsonl.gz").write_bytes(gzip.compress(positive_raw, mtime=0))
    return {"schema": "xedni-variable-coefficient-result-v1",
            "status": "PASS", "protocol_sha256": digest(protocol_bytes),
            "script_sha256": digest((HERE / "run.py").read_bytes()),
            "source_packed_sha256": digest(source_bytes),
            "crt_candidate_stream_sha256": old_stream.hexdigest(),
            "counts": dict(counts),
            "by_coefficient": {k: dict(v) for k, v in by_k.items()},
            "positive_rows_raw_sha256": digest(positive_raw)}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
