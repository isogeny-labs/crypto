#!/usr/bin/env sage -python
"""Deterministic q32 order screen in Hess's published n=7 genus-eight selector."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, ZZ, factor, matrix
from sage.version import version as sage_version

ROOT = Path(__file__).resolve().parent
F2 = GF(2)
R = PolynomialRing(F2, "x")
x = R.gen()


def choose_modulus():
    for j in range(1, 18):
        p = x**35 + x**j + 1
        if p.is_irreducible():
            return p
    for i in range(1, 35):
        for j in range(i + 1, 35):
            for k in range(j + 1, 35):
                p = x**35 + x**i + x**j + x**k + 1
                if p.is_irreducible():
                    return p
    raise AssertionError("no prescribed irreducible modulus")


modulus = choose_modulus()
K = GF(2**35, "z", modulus=modulus)
z = K.gen()


def mask(value):
    return sum(int(bit) << i for i, bit in enumerate(value.polynomial().list()))


def from_mask(bits):
    return K(R([(int(bits) >> i) & 1 for i in range(35)]))


def kernel_elements(arm):
    def image(v):
        if arm == 1:
            return v ** (32**3) + v**32 + v
        return v ** (32**3) + v ** (32**2) + v

    columns = [image(z**i) for i in range(35)]
    M = matrix(F2, 35, 35, lambda i, j: F2(columns[j].polynomial()[i]))
    basis = M.right_kernel().basis()
    assert len(basis) == 15
    elements = []
    for selector in range(1, 1 << 15):
        vector = sum((basis[i] for i in range(15) if selector & (1 << i)), basis[0].parent().zero())
        v = from_mask(sum(int(vector[i]) << i for i in range(35)))
        assert v and image(v) == 0
        elements.append(v)
    assert len({mask(v) for v in elements}) == 32767
    return sorted(elements, key=mask)


unit = z ** ((2**35 - 1) // 31)
assert unit.multiplicative_order() == 31
subfield = sorted({K(0)} | {unit**i for i in range(31)}, key=mask)
assert len(subfield) == 32 and all(t**32 == t for t in subfield)
trace_one = [t for t in subfield if sum((t ** (2**i) for i in range(5)), K(0)) == 1]
assert len(trace_one) == 16
a = trace_one[0]
c = K(1)
# E0 is counted after identifying a in the canonical degree-five subfield.
k, embed = K.subfields(5)[0]
aa = next(t for t in k if embed(t) == a)
E0 = EllipticCurve(k, [1, aa, 0, 0, 1])
e0_order = ZZ(E0.cardinality())
V1, V2 = kernel_elements(1), kernel_elements(2)
assert {mask(v) for v in V1}.isdisjoint({mask(v) for v in V2})

seen = set()
candidates = []
for arm, kernel in ((1, V1), (2, V2)):
    for v in kernel:
        beta = c + v
        b = beta**2
        b_mask = mask(b)
        if b_mask in seen:
            continue
        seen.add(b_mask)
        degree = next(d for d in (1, 5, 7, 35) if b ** (2**d) == b)
        if degree == 35:
            candidates.append((arm, b_mask, b))
candidates.sort(key=lambda row: (row[0], row[1]))
assert len(candidates) >= 128

rows = []
chosen = None
for arm, b_mask, b in candidates[:128]:
    beta = b.sqrt()
    assert sum((beta ** (32**i) for i in range(7)), K(0)) == 1
    E = EllipticCurve(K, [1, a, 0, 0, b])
    order = ZZ(E.cardinality())
    factors = [(int(p), int(e)) for p, e in factor(order)]
    r = max(p for p, _ in factors)
    h = int(order // r)
    row = {"arm": arm, "b_mask": b_mask, "order": int(order),
           "trace": int(2**35 + 1 - order), "factors": factors,
           "largest_prime_subgroup": r, "cofactor": h}
    rows.append(row)
    if chosen is None and r >= 2**30 and h <= 8 and e0_order % r != 0:
        chosen = row

output = {"schema": "ghs-n7-q32-screen/v1", "sage_version": sage_version,
          "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
          "field_modulus": str(modulus), "a_mask": mask(a), "c_mask": 1,
          "e0_order": int(e0_order), "kernel_dimensions_F2": [15, 15],
          "full_field_candidate_count": len(candidates),
          "screened_count": len(rows), "chosen": chosen, "rows": rows}
target = ROOT / "screen.json"
if target.exists():
    raise SystemExit("refusing to overwrite immutable screen receipt")
target.write_text(json.dumps(output, indent=2) + "\n")
print(json.dumps({k: v for k, v in output.items() if k != "rows"}, indent=2))
