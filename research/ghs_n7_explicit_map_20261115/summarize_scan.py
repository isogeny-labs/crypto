#!/usr/bin/env python3
"""Validate the complete Magma c scan and emit its exact genus histogram."""
import collections
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
receipt = json.loads((ROOT / "scan_receipt.json").read_text())
assert receipt["http_status"] == 200
assert receipt["response"].get("success") is True
assert receipt["response"].get("exit_code") == 0
assert receipt["response"].get("truncated") is not True
rows = {}
for line in receipt["response"]["stdout"].splitlines():
    exponent, value = line.split()
    exponent = int(exponent)
    assert exponent not in rows
    rows[exponent] = None if value == "ERR" else int(value)
assert set(rows) == set(range(127))
valid = {i: g for i, g in rows.items() if g is not None}
histogram = dict(sorted(collections.Counter(valid.values()).items()))
minimum = min(valid.values())
summary = {
    "schema": "ghs-n7-c-parameter-scan/v1",
    "total_parameters": len(rows),
    "eligible": len(valid),
    "trace_condition_rejected": len(rows) - len(valid),
    "minimum_genus": minimum,
    "minimum_exponents": [i for i, g in valid.items() if g == minimum],
    "genus_histogram": histogram,
    "genus_seven_count": sum(g == 7 for g in valid.values()),
}
assert summary["minimum_genus"] == 14
assert summary["minimum_exponents"] == [94, 98]
assert summary["genus_seven_count"] == 0
(ROOT / "scan_summary.json").write_text(json.dumps(summary, indent=2) + "\n")
print(json.dumps(summary, indent=2))
