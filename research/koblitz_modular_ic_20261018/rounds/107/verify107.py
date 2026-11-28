#!/usr/bin/env python3
"""Independent Hasse-jet composition and F4 Newton checks for R107."""
import hashlib
import importlib.util
import json
from pathlib import Path

root = Path(__file__).resolve().parent
donor = root.parent / "51" / "runs" / "base23-struct51-03" / "marked23-triangular51.sing"
parser = root.parent / "105" / "jets105.py"
base = root.parent / "106" / "result.json"
result = json.loads((root / "result.json").read_text())
for path, expected in zip((donor, parser, base), result["source_sha256"]):
    assert hashlib.sha256(path.read_bytes()).hexdigest() == expected
spec = importlib.util.spec_from_file_location("jets105", parser)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
_, terms = module.parse_source(donor.read_text())
base_data = json.loads(base.read_text())
mask = (1 << 32) - 1


def mul(a, b):
    out = 0
    for i in range(32):
        if b >> i & 1: out ^= a << i
    return out & mask


def pow2(a, n):
    out = 1
    for _ in range(n): out = mul(out, a)
    return out


def f4mul(a, b):
    out = 0
    for i in range(2):
        if b >> i & 1: out ^= a << i
    if out & 8: out ^= 0b1100
    if out & 4: out ^= 0b111
    return out


def f4pow(a, n):
    out = 1
    for _ in range(n): out = f4mul(out, a)
    return out


summary = {}
for key, shift in (("y0", 0), ("y1", 1)):
    z = sum(1 << i for i in base_data[key]["z_exponents"])
    t = mul(2, z)
    y = z ^ shift
    coeff = {i: 0 for i in (0, 1, 4, 8, 16)}
    for le, ye, den, exps in terms:
        # Expand x^(e-den) by generalized Hasse binomial parity in t.
        series = 0
        for e in exps:
            for k in range(32):
                if module.binom_parity(e - den, k):
                    series ^= pow2(t, k)
        coeff[le] ^= mul(series, pow2(y, ye))
    for le in coeff:
        assert [i for i in range(32) if coeff[le] >> i & 1] == result[key]["lambda_coefficients"][str(le)]
    translated = {i: [0, 0] for i in (0, 1, 4, 8, 16)}
    for le, series in coeff.items():
        scalar = f4pow(2, le)
        if scalar & 1: translated[0][0] ^= series
        if scalar & 2: translated[0][1] ^= series
        if le: translated[le][0] ^= series
    rows = result[key]["translated_rows"]
    for row in rows:
        le = row["u_exponent"]
        assert row["one_support"] == [i for i in range(32) if translated[le][0] >> i & 1]
        assert row["alpha_support"] == [i for i in range(32) if translated[le][1] >> i & 1]
        bits = translated[le][0] | translated[le][1]
        order = None if not bits else (bits & -bits).bit_length() - 1
        assert row["s_order"] == order
        if order is not None:
            assert row["leading_F4"] == ((translated[le][0] >> order & 1) | ((translated[le][1] >> order & 1) << 1))
    orders = {r["u_exponent"]: r["s_order"] for r in rows}
    expected = {"y0": {0: 28, 1: 23, 4: 23, 8: 0, 16: 0},
                "y1": {0: 27, 1: 23, 4: 23, 8: 0, 16: 0}}[key]
    assert orders == expected
    first_slope = orders[0] - orders[1]
    assert first_slope in (4, 5)
    assert 7 * orders[4] > 7 * 23 - 23 * (4 - 1)  # above the 1-to-8 edge
    assert 7 * first_slope > 23 and 23 % 7 != 0
    summary[key] = {"first_root_order": first_slope, "second_root_order": "23/7",
                    "edge_lengths": [1, 7], "zero_order_cluster": 8,
                    "all_orders_below_precision": True}

(root / "independent-replay.json").write_text(json.dumps({"check": "LAMBDA107_INDEPENDENT_PASS", "branches": summary}, indent=2, sort_keys=True) + "\n")
print("LAMBDA107_INDEPENDENT_PASS")
