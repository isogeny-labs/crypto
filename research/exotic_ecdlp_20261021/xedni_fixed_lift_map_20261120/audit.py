#!/usr/bin/env python3
"""Independent Fraction group-law replay of the fixed rational lift map."""

import gzip
import hashlib
import json
import sys
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
PARENT = HERE.parent
sys.path.insert(0, str(PARENT / "xedni_coefficient_choice_20261120"))
from audit import add, discriminant


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def record(q):
    numerator, denominator = q.numerator, q.denominator
    return {"numerator_bits": abs(numerator).bit_length(),
            "denominator_bits": denominator.bit_length(),
            "value_sha256": sha(f"{numerator}/{denominator}".encode())}


def main():
    result = json.loads((HERE / "result.json").read_text())
    protocol = json.loads((HERE / "protocol.json").read_text())
    raw = (HERE / "multiple-rows.jsonl").read_bytes()
    assert sha(raw) == result["multiple_rows_raw_sha256"]
    rows = [json.loads(x) for x in raw.splitlines()]
    source = [json.loads(x) for x in gzip.decompress(
        (PARENT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz").read_bytes()
    ).splitlines()]
    expected_points = {tuple(P) for x in source for P in x["reduced_points"]}
    a = [F(v) for v in protocol["rational_curve_coefficients"]]
    Q = tuple(F(v) for v in protocol["rational_generator_lift"])
    assert discriminant(a) != 0
    R = None
    seen = set()
    for k, saved in enumerate(rows, start=1):
        R = add(R, Q, a)
        assert R is not None
        assert saved["coefficient"] == k
        assert saved["x"] == record(R[0])
        assert saved["y"] == record(R[1])
        reduced = tuple(int(v.numerator*pow(v.denominator, -1, 47)%47) for v in R)
        assert reduced == tuple(saved["reduced_point"])
        assert reduced in expected_points and reduced not in seen
        seen.add(reduced)
    assert len(rows) == 42 and seen == expected_points
    assert add(R, Q, a) is not None  # rational Q is not forced to have order 43
    return {"status": "PASS", "rational_multiples": len(rows),
            "distinct_reductions": len(seen), "multiple_rows_raw_sha256": sha(raw)}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
