#!/usr/bin/env python3
"""Reconcile independent q2 W0 and paired W1 census artifacts with the bounds."""
import collections
import csv
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
csv_path = HERE.parent / "ghs_n7_prym_trace_20261115/reports/run-20261115T0821Z/curves.csv"
w0_path = HERE.parent / "ghs_n7_isogenous_w0_20261115/result.json"
rows = list(csv.DictReader(csv_path.open()))
w0 = json.loads(w0_path.read_text())
assert len(rows) == 112 and len({(r["a"], r["b"]) for r in rows}) == 112
w1 = [r for r in rows if r["a"] == "1"]
assert len(w1) == 56
h = collections.Counter((r["classical"], int(r["t"])) for r in w1)
assert h == {
    ("true", -1): 7, ("true", 15): 7,
    ("false", -17): 7, ("false", -9): 21,
    ("false", -1): 7, ("false", 7): 7,
}
hyper = {-15, -13, -9, -1, 1, 9, 13, 15}
classical = sum(r["classical"] == "true" for r in w1)
extra_match = sum(r["classical"] == "false" and int(r["t"]) in hyper for r in w1)
excluded = sum(r["classical"] == "false" and int(r["t"]) not in hyper for r in w1)
assert (classical, extra_match, excluded) == (14, 28, 14)
assert w0["union_count"] == 49 and w0["all_127_orders_replayed"] is True
q = 2
w0_bound = (q**3 - 1) * (q**2 + q + 2)
w1_bound = 2 * (q**3 - 1)**2
assert w0["union_count"] <= w0_bound and len(w1) <= w1_bound
out = {
    "schema": "ghs-n7-family-bounds-q2/v1",
    "field_modulus": "0x83",
    "w0_actual_coefficients": w0["union_count"],
    "w0_general_upper_at_q2": w0_bound,
    "w1_actual_coefficients": len(w1),
    "w1_general_upper_at_q2": w1_bound,
    "w1_direct_genus7_classical": classical,
    "w1_additional_hyperelliptic_isogeny_class_matches": extra_match,
    "w1_hyperelliptic_isogeny_class_exclusions": excluded,
    "csv_sha256": hashlib.sha256(csv_path.read_bytes()).hexdigest(),
    "w0_result_sha256": hashlib.sha256(w0_path.read_bytes()).hexdigest(),
    "source_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
}
(HERE / "q2_partition.json").write_text(json.dumps(out, indent=2) + "\n")
print(json.dumps(out, indent=2))
