#!/usr/bin/env python3
"""Independent exact rational group-law replay of every selected row."""

import gzip
import hashlib
import json
import sys
from collections import Counter
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
PARENT = HERE.parent
sys.path.insert(0, str(PARENT / "xedni_coefficient_choice_20261120"))
from audit import add, discriminant


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def main():
    result = json.loads((HERE / "result.json").read_text())
    raw = gzip.decompress((HERE / "rational-checks.jsonl.gz").read_bytes())
    assert sha(raw) == result["rational_checks_raw_sha256"]
    rows = [json.loads(x) for x in raw.splitlines()]
    source = [json.loads(x) for x in gzip.decompress(
        (PARENT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz").read_bytes()
    ).splitlines()]
    selector = [json.loads(x) for x in gzip.decompress(
        (PARENT / "xedni_local_selector_20261120/candidate-rows.jsonl.gz").read_bytes()
    ).splitlines() if 10 in json.loads(x)["residues"]]
    assert len(rows) == len(selector) == result["counts"]["rational_checks"] == 1488
    counts = Counter()
    postanalysis = Counter()
    for row, selected in zip(rows, selector):
        assert row["pair_index"] == selected["pair_index"]
        assert row["local_indices"] == selected["local_indices"]
        assert row["coefficient"] == 10
        a = [F(x) for x in row["curve_coefficients"]]
        assert discriminant(a) != 0
        points = [tuple(map(F, P)) for P in source[row["pair_index"]]["lifted_points"]]
        acc = None
        for _ in range(10):
            acc = add(acc, points[1], a)
        related = add(points[0], acc, a) is None
        assert related == row["rational_relation"]
        reduced = [tuple(map(int, P)) for P in source[row["pair_index"]]["reduced_points"]]
        reduced_curve = [0, 0, 0, 46, 45]
        racc = None
        for _ in range(10):
            racc = add(racc, reduced[1], reduced_curve, 47)
        compatible = add(reduced[0], racc, reduced_curve, 47) is None
        postanalysis["reduced_compatible_selected_candidates"] += compatible
        postanalysis["reduced_incompatible_selected_candidates"] += not compatible
        assert not related or compatible
        counts["rational_checks"] += 1
        counts["rational_relations"] += related
    assert dict(counts) == result["counts"]
    assert postanalysis["reduced_compatible_selected_candidates"] == 32
    return {"status": "PASS", "counts": dict(counts),
            "postanalysis": dict(postanalysis),
            "rational_checks_raw_sha256": sha(raw)}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
