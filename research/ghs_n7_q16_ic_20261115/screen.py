#!/usr/bin/env sage -python
"""Deterministic q=16 screen inside Teske's published n=7 selector."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, ZZ, factor, matrix
from sage.version import version as sage_version


ROOT = Path(__file__).resolve().parent
F2 = GF(2)
R = PolynomialRing(F2, "x")
x = R.gen()
modulus = x**28 + x**3 + 1
assert modulus.is_irreducible()
K = GF(2**28, "z", modulus=modulus)
z = K.gen()


def mask(value):
    return sum(int(bit) << i for i, bit in enumerate(value.polynomial().list()))


def element_from_mask(bits):
    return K(R([(int(bits) >> i) & 1 for i in range(28)]))


def kernel_elements(arm):
    def image(v):
        if arm == 1:
            return v ** (16**3) + v**16 + v
        return v ** (16**3) + v ** (16**2) + v

    columns = [image(z**i) for i in range(28)]
    M = matrix(F2, 28, 28, lambda i, j: F2(columns[j].polynomial()[i]))
    basis = M.right_kernel().basis()
    assert len(basis) == 12
    elements = []
    for selector in range(1, 1 << 12):
        vector = sum((basis[i] for i in range(12) if selector & (1 << i)), basis[0].parent().zero())
        bits = sum(int(vector[i]) << i for i in range(28))
        v = element_from_mask(bits)
        assert v and image(v) == 0
        elements.append(v)
    assert len({mask(v) for v in elements}) == 4095
    return sorted(elements, key=mask)


unit = z ** ((2**28 - 1) // 15)
assert unit.multiplicative_order() == 15
subfield = sorted({K(0)} | {unit**i for i in range(15)}, key=mask)
assert len(subfield) == 16 and all(c**16 == c for c in subfield)
trace_one = [v for v in subfield if sum((v ** (2**i) for i in range(4)), K(0)) == 1]
assert len(trace_one) == 8
a = trace_one[0]
c = next(v for v in subfield if v)
assert c == 1
V1 = kernel_elements(1)
V2 = kernel_elements(2)
assert {mask(v) for v in V1}.isdisjoint({mask(v) for v in V2})

seen = set()
candidate_coefficients = []
for arm, kernel in [(1, V1), (2, V2)]:
    for v in kernel:
        b = v + c
        b_mask = mask(b)
        if b_mask in seen:
            continue
        seen.add(b_mask)
        degree = next(d for d in [1, 2, 4, 7, 14, 28] if b ** (2**d) == b)
        if degree == 28:
            candidate_coefficients.append((arm, b_mask, b))
candidate_coefficients.sort(key=lambda row: (row[0], row[1]))
assert len(candidate_coefficients) >= 128

rows = []
chosen = None
for arm, b_mask, b in candidate_coefficients[:128]:
    assert sum((b ** (16**i) for i in range(7)), K(0)) == 1
    E = EllipticCurve(K, [1, a, 0, 0, b])
    order = ZZ(E.cardinality())
    trace = ZZ(2**28 + 1 - order)
    assert abs(trace) <= 32768
    factors = [(int(p), int(e)) for p, e in factor(order)]
    r = max(p for p, _ in factors)
    cofactor = int(order // r)
    row = {
        "arm": arm,
        "b_mask": b_mask,
        "order": int(order),
        "trace": int(trace),
        "factors": factors,
        "largest_prime_subgroup": r,
        "cofactor": cofactor,
    }
    rows.append(row)
    if chosen is None and cofactor <= 8 and r >= 2**24:
        chosen = row

output = {
    "schema": "ghs-n7-q16-screen/v1",
    "sage_version": sage_version,
    "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    "field_modulus": str(modulus),
    "a_mask": mask(a),
    "c_mask": mask(c),
    "kernel_dimensions_F2": [12, 12],
    "full_field_candidate_count": len(candidate_coefficients),
    "screened_count": len(rows),
    "chosen": chosen,
    "rows": rows,
}
target = ROOT / "screen.json"
if target.exists():
    raise SystemExit("refusing to overwrite immutable screen receipt")
target.write_text(json.dumps(output, indent=2) + "\n")
print(json.dumps({key: value for key, value in output.items() if key != "rows"}, indent=2))
