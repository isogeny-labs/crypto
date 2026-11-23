#!/usr/bin/env python3
"""Independent direct-doubling audit of every wide-period rational row."""

import gzip
import hashlib
import importlib.util
import json
from collections import Counter
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
PARENT = HERE.parent
CHECK_PATH = PARENT / "xedni_full_law_crt_20261120/audit.py"
spec = importlib.util.spec_from_file_location("wide_relation_arithmetic_audit", CHECK_PATH)
CHECK = importlib.util.module_from_spec(spec)
spec.loader.exec_module(CHECK)


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def main():
    result = json.loads((HERE / "result.json").read_text())
    raw = gzip.decompress((HERE / "rational-checks.jsonl.gz").read_bytes())
    assert sha(raw) == result["rational_checks_raw_sha256"]
    rows = [json.loads(x) for x in raw.splitlines()]
    selector_raw = gzip.decompress((PARENT / "xedni_wide_period_selector_20261120/candidate-rows.jsonl.gz").read_bytes())
    assert sha(selector_raw) == result["wide_selector_candidate_rows_raw_sha256"]
    selected = [json.loads(x) for x in selector_raw.splitlines()
                if json.loads(x)["selected_coefficient"] == 2]
    source = [json.loads(x) for x in gzip.decompress(
        (PARENT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz").read_bytes()
    ).splitlines()]
    assert len(rows) == len(selected) == 13440
    counts = Counter()
    positives = Counter()
    for row, candidate in zip(rows, selected):
        assert row["pair_index"] == candidate["pair_index"]
        assert row["local_indices"] == candidate["local_indices"]
        assert row["crt_shifts"] == candidate["crt_shifts"]
        assert row["coefficient"] == 2
        a = [F(x) for x in row["curve_coefficients"]]
        points = [tuple(F(v) for v in P) for P in source[row["pair_index"]]["lifted_points"]]
        assert CHECK.on_curve(a, points[0]) and CHECK.on_curve(a, points[1])
        assert all(v.denominator%p for v in a for p in (7, 11, 47))
        assert [v.numerator*pow(v.denominator, -1, 47)%47 for v in a] == [0, 0, 0, 46, 45]
        expected = CHECK.rational_relation(points, a)
        assert expected == row["rational_relation"]
        counts["rational_checks"] += 1
        counts["rational_relations"] += expected
        if expected:
            positives[row["pair_index"]] += 1
    assert counts["rational_checks"] == result["counts"]["rational_checks"]
    assert counts["rational_relations"] == result["counts"]["rational_relations"]
    assert {str(k):v for k,v in positives.items()} == result["positive_pairs"]
    return {"status": "PASS", "counts": dict(counts),
            "positive_pairs": dict(positives),
            "rational_checks_raw_sha256": sha(raw)}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
