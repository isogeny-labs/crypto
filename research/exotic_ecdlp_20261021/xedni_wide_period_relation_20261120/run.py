#!/usr/bin/env python3
"""Exact k=2 rational relation check on every wide-period selected curve."""

import gzip
import hashlib
import json
import sys
from collections import Counter
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
PARENT = HERE.parent
sys.path.insert(0, str(PARENT / "xedni_full_law_crt_20261120"))
import run as crt
sys.path.insert(0, str(PARENT / "xedni_coefficient_choice_20261120"))
from audit import discriminant, relation


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def main():
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    source_bytes = (PARENT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz").read_bytes()
    selector_raw = gzip.decompress((PARENT / "xedni_wide_period_selector_20261120/candidate-rows.jsonl.gz").read_bytes())
    assert sha(source_bytes) == protocol["source_packed_sha256"]
    assert sha(selector_raw) == protocol["wide_selector_candidate_rows_raw_sha256"]
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    selected = [json.loads(x) for x in selector_raw.splitlines()
                if json.loads(x)["selected_coefficient"] == 2]
    assert len(source) == 1722 and len(selected) == 13440
    counts = Counter()
    rows = []
    positive_pairs = Counter()
    cache = {}
    for candidate in selected:
        index = candidate["pair_index"]
        if index not in cache:
            points = source[index]["lifted_points"]
            p7 = tuple(tuple(v%7 for v in P) for P in points)
            p11 = tuple(tuple(v%11 for v in P) for P in points)
            w7, _ = crt.local_gate(p7, 7, 5)
            w11, _ = crt.local_gate(p11, 11, 11)
            cache[index] = (points, w7, w11)
        points, w7, w11 = cache[index]
        i7, i11 = candidate["local_indices"]
        a7, a11 = w7[i7], w11[i11]
        shifts = [crt.centered_k(a7[j], a11[j]) for j in range(3)]
        assert shifts == candidate["crt_shifts"]
        a = crt.rational_curve(points, shifts)
        assert discriminant(a) != 0
        related = relation([tuple(F(v) for v in P) for P in points], a)
        counts["rational_checks"] += 1
        counts["rational_relations"] += related
        if related:
            reduced = [tuple(P) for P in source[index]["reduced_points"]]
            assert relation(reduced, [0, 0, 0, 46, 45], 47)
            counts["positive_reduced_postchecks"] += 1
            positive_pairs[index] += 1
        rows.append({"pair_index": index, "local_indices": [i7, i11],
                     "coefficient": 2, "crt_shifts": shifts,
                     "curve_coefficients": list(map(str, a)),
                     "rational_relation": bool(related)})
    raw = "".join(json.dumps(row, sort_keys=True) + "\n" for row in rows).encode()
    (HERE / "rational-checks.jsonl.gz").write_bytes(gzip.compress(raw, mtime=0))
    return {"schema": "xedni-wide-period-relation-result-v1", "status": "PASS",
            "protocol_sha256": sha(protocol_bytes),
            "script_sha256": sha((HERE / "run.py").read_bytes()),
            "source_packed_sha256": sha(source_bytes),
            "wide_selector_candidate_rows_raw_sha256": sha(selector_raw),
            "counts": dict(counts),
            "positive_pairs": dict(sorted(positive_pairs.items())),
            "rational_checks_raw_sha256": sha(raw)}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
