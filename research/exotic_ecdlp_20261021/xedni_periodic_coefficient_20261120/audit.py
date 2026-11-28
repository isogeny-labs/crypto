#!/usr/bin/env python3
"""Independent exact rational replay of every k=14 survivor."""

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


def main():
    result = json.loads((HERE / "result.json").read_text())
    raw = gzip.decompress((HERE / "rational-checks.jsonl.gz").read_bytes())
    assert hashlib.sha256(raw).hexdigest() == result["rational_checks_raw_sha256"]
    rows = [json.loads(x) for x in raw.splitlines()]
    source = [json.loads(x) for x in gzip.decompress(
        (PARENT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz").read_bytes()
    ).splitlines()]
    assert len(rows) == result["counts"]["rational_checks"] == 48
    counts = Counter()
    for row in rows:
        assert row["coefficient"] == 14
        a = [F(x) for x in row["curve_coefficients"]]
        assert discriminant(a) != 0
        points = [tuple(map(F, P)) for P in source[row["pair_index"]]["lifted_points"]]
        acc = None
        for _ in range(14):
            acc = add(acc, points[1], a)
        related = add(points[0], acc, a) is None
        assert related == row["rational_relation"]
        counts["rational_checks"] += 1
        counts["rational_relations"] += related
    assert counts["rational_relations"] == result["counts"]["rational_relations"]
    return {"status": "PASS", "counts": dict(counts),
            "rational_checks_raw_sha256": hashlib.sha256(raw).hexdigest()}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
