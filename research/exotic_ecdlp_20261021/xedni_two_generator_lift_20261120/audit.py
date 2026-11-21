#!/usr/bin/env python3
"""Independent Fraction replay of the box and centered coordinate heights."""

import hashlib
import json
import sys
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "xedni_coefficient_choice_20261120"))
from audit import add, neg, discriminant


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def scalar(q):
    return {"numerator_bits": abs(q.numerator).bit_length(),
            "denominator_bits": q.denominator.bit_length(),
            "value_sha256": sha(f"{q.numerator}/{q.denominator}".encode())}


def multiple(k, P, curve):
    if k < 0:
        return multiple(-k, neg(P, curve), curve)
    out = None
    for _ in range(k):
        out = add(out, P, curve)
    return out


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    result = json.loads((HERE / "result.json").read_text())
    box_raw = (HERE / "box-rows.jsonl").read_bytes()
    assert sha(box_raw) == result["box_rows_raw_sha256"]
    box = [json.loads(line) for line in box_raw.splitlines()]
    curve = [F(v) for v in protocol["curve_coefficients"]]
    assert discriminant(curve) != 0
    Q = tuple(F(v) for v in protocol["Q"])
    R = tuple(F(v) for v in protocol["R"])
    seen = set()
    rational_seen = set()
    for row in box:
        a, b = row["a"], row["b"]
        P = add(multiple(a, Q, curve), multiple(b, R, curve), curve)
        assert P not in rational_seen
        rational_seen.add(P)
        if (a,b) == (0,0):
            assert P is None and row["reduced_point"] is None
            continue
        assert P is not None
        x, y = scalar(P[0]), scalar(P[1])
        assert row["x"] == x and row["y"] == y
        assert row["height_bits"] == max(x["numerator_bits"], x["denominator_bits"],
                                          y["numerator_bits"], y["denominator_bits"])
        reduced = tuple(v.numerator*pow(v.denominator, -1, 47)%47 for v in P)
        assert reduced == tuple(row["reduced_point"])
        seen.add(reduced)
    assert len(box) == 81 and len(rational_seen) == 81 and len(seen) == 42
    control = json.loads((HERE / "centered-result.json").read_text())
    centered_raw = (HERE / "centered-comparison-rows.jsonl").read_bytes()
    assert sha(centered_raw) == control["comparison_rows_raw_sha256"]
    centered = [json.loads(line) for line in centered_raw.splitlines()]
    assert len(centered) == 42
    for row in centered:
        P = multiple(row["k"], Q, curve)
        assert P is not None
        reduced = tuple(v.numerator*pow(v.denominator, -1, 47)%47 for v in P)
        assert reduced == tuple(row["reduced_point"])
        own = max(max(scalar(v)["numerator_bits"], scalar(v)["denominator_bits"]) for v in P)
        assert own == row["centered_height_bits"]
        assert row["box_min_height_bits"] == min(
            b["height_bits"] for b in box if b["reduced_point"] == row["reduced_point"])
    return {"status": "PASS", "box_points": len(box),
            "distinct_rational_box_points": len(rational_seen),
            "distinct_nonidentity_reductions": len(seen), "centered_rows": len(centered),
            "box_rows_raw_sha256": sha(box_raw),
            "centered_comparison_rows_raw_sha256": sha(centered_raw)}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
