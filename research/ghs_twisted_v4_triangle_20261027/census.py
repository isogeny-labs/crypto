#!/usr/bin/env python3
"""Independent bit-field census of the fixed-pole triangle-cover family."""
import hashlib
import json
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "ghs_cubic_hess_closure_q16_20261027" / "certificate.json"
SIMPLE = HERE.parent / "ghs_q16_simple_restriction_bound_20261027" / "certificate.json"
MOD = 0x0EB


def mul(a, b):
    out = 0
    for _ in range(12):
        if b & 1:
            out ^= a
        b >>= 1
        a <<= 1
        if a & 0x1000:
            a ^= 0x1000 | MOD
    return out


def power(a, n):
    out = 1
    while n:
        if n & 1:
            out = mul(out, a)
        a = mul(a, a)
        n >>= 1
    return out


def inverse(a):
    assert a
    return power(a, 4094)


def trace(a):
    out = 0
    for _ in range(12):
        out ^= a
        a = mul(a, a)
    assert out in (0, 1)
    return out


def main():
    source_raw, simple_raw = SOURCE.read_bytes(), SIMPLE.read_bytes()
    source, simple = json.loads(source_raw), json.loads(simple_raw)
    simple_orders = {row["order"] for row in simple["rows"] if row["irreducible"]}
    delta = 14
    assert delta ^ power(delta, 16) ^ power(delta, 256) == 0
    exponent = pow(17, -1, 4095)
    d = pow(2*17,-1,4095)
    assert d == 1084 and (d*17*2-1) % 4095 == 0
    signature = {}
    residue = {}
    for b in range(1, 4096):
        sqrt_b = power(b, 2048)
        A = power(mul(sqrt_b, mul(delta, delta)), exponent)
        beta = mul(A, inverse(delta))
        gamma = mul(power(A, 16), inverse(delta))
        assert mul(mul(beta, gamma), mul(beta, gamma)) == b
        c = beta ^ gamma
        signature[b] = trace(power(b,d))
        assert trace(c) == signature[b]
        residue[b] = A
    assert Counter(signature.values()) == {0: 2047, 1: 2048}
    rows = []
    for item in source["orders"]:
        selected = {a: [b for b in z["hess"]+z["outside"] if signature[b] == int(a == "32")]
                    for a, z in item["by_a"].items()}
        n = sum(len(v) for v in selected.values())
        rows.append({
            "order": item["order"], "source_models": item["source_models"],
            "triangle_models": n, "triangle_b_by_a": selected,
            "outside_hess_closure": not item["in_hess_isogeny_closure"],
            "simple_restriction": item["order"] in simple_orders,
            "catalog_has_jacobian": bool(item["catalog_has_jacobian"]),
            "catalog_label": item["catalog_label"],
            "largest_prime": item["largest_prime"],
        })
    assert sum(r["triangle_models"] for r in rows) == 4095
    outside = [r for r in rows if r["outside_hess_closure"]]
    residual = [r for r in outside if r["simple_restriction"] and r["catalog_has_jacobian"]]
    unresolved = [r for r in outside if r["simple_restriction"] and not r["catalog_has_jacobian"]]
    assert sum(r["triangle_models"] for r in outside) == 1472
    assert len(residual) == 44 and sum(r["triangle_models"] for r in residual) == 1356
    assert [(r["order"],r["triangle_models"]) for r in unresolved] == [(3972,12),(4222,12)]
    output = {
        "schema": "ghs-twisted-v4-triangle-q16-census-v1",
        "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "source_certificate_sha256": hashlib.sha256(source_raw).hexdigest(),
        "simple_certificate_sha256": hashlib.sha256(simple_raw).hexdigest(),
        "delta": delta, "exponent_inverse_q_plus_one": exponent,
        "family_gate_exponent_d":d,
        "signature_histogram": {"0":2047,"1":2048},
        "summary": {"all_triangle_models":4095,"all_source_models":8190,
                    "outside_hess_triangle_models":1472,
                    "simple_outside_catalog_triangle_models":1356,
                    "simple_outside_unknown_triangle_models":24},
        "focus": {str(b): {"a":32 if signature[b] else 0,"A":residue[b]}
                  for b in (51,116,198,853)},
        "rows": rows,
    }
    (HERE/"census.json").write_text(json.dumps(output,indent=2)+"\n")
    print(output["summary"])


if __name__ == "__main__":
    main()
