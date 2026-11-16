#!/usr/bin/env python3
"""Independent exact arithmetic and receipt binding for the W1 Prym split."""
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
UPSTREAM = HERE.parent / "ghs_n7_explicit_map_20261115" / "map14_class_finite_receipt.json"
receipt = json.loads((HERE / "factor_receipt.json").read_text())
prior = json.loads(UPSTREAM.read_text())
source_hash = hashlib.sha256((HERE / "check_factor.magma").read_bytes()).hexdigest()
old_source = (HERE.parent / "ghs_n7_explicit_map_20261115" / "map14_class.magma").read_text()
new_source = (HERE / "check_factor.magma").read_text()
for exact_line in (
    "K<z> := ext<GF(2) | x^7+x+1>;",
    "E := EllipticCurve([K | 1, 1, 0, 0, z^3]);",
):
    assert exact_line in old_source and exact_line in new_source
assert "z^94" in old_source and "z^94" in new_source
assert receipt["input_sha256"] == source_hash
assert receipt["http_status"] == 200
assert receipt["response"]["success"] and receipt["response"]["exit_code"] == 0
assert not receipt["response"]["truncated"]
out = receipt["response"]["stdout"]
assert "observed 16384*T^28 + 175*T^14 + 1" in out
assert "<128*T^14 - 9*T^7 + 1, 1>" in out
assert "<128*T^14 + 9*T^7 + 1, 1>" in out
assert "class_number 16560" in prior["response"]["stdout"]
assert "image_order 23" in prior["response"]["stdout"]

# Multiply the sparse factors without relying on Sage, Magma or a CAS.
a = {0: 1, 7: 9, 14: 128}
b = {0: 1, 7: -9, 14: 128}
product = {}
for i, ai in a.items():
    for j, bj in b.items():
        product[i + j] = product.get(i + j, 0) + ai * bj
product = {i: c for i, c in product.items() if c}
assert product == {0: 1, 14: 175, 28: 16384}
assert sum(a.values()) == 138
assert sum(b.values()) == 120
assert sum(product.values()) == 16560

def p_minus(u):
    return (u*u + 9*u + 128) % 23

def p_plus(u):
    return (u*u - 9*u + 128) % 23

def e_minus(u):
    return (6*u**3 + 6*u + 12) % 23

minus_roots = [u for u in range(23) if p_minus(u) == 0]
plus_roots = [u for u in range(23) if p_plus(u) == 0]
assert minus_roots == [1, 13]
assert plus_roots == [10, 22]
assert all(e_minus(u) == 1 for u in minus_roots)
assert all(e_minus(u) == 0 for u in plus_roots)

report = {
    "status": "PASS",
    "magma_source_sha256": source_hash,
    "previous_map_receipt_sha256": hashlib.sha256(UPSTREAM.read_bytes()).hexdigest(),
    "factor_coefficients": product,
    "factor_orders": {"minus": 138, "plus": 120},
    "minus_roots_mod23": minus_roots,
    "plus_roots_mod23": plus_roots,
    "projector_e_minus_F7_mod23": "6*F^21+6*F^7+12",
}
(HERE / "AUDIT.json").write_text(json.dumps(report, indent=2) + "\n")
print(json.dumps(report, indent=2))
