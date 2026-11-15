#!/usr/bin/env sage -python
"""Deterministic q64 order screen in the published n=7 genus-eight selector."""

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
    for j in range(1, 22):
        p = x**42 + x**j + 1
        if p.is_irreducible():
            return p
    for i in range(1, 42):
        for j in range(i + 1, 42):
            for k in range(j + 1, 42):
                p = x**42 + x**i + x**j + x**k + 1
                if p.is_irreducible():
                    return p
    raise AssertionError("no prescribed irreducible modulus")


modulus = choose_modulus()
K = GF(2**42, "z", modulus=modulus)
z = K.gen()


def mask(value):
    return sum(int(bit) << i for i, bit in enumerate(value.polynomial().list()))


def from_mask(bits):
    return K(R([(int(bits) >> i) & 1 for i in range(42)]))


def selectors_from_basis(vectors):
    bit_vectors = [sum(int(v[i]) << i for i in range(len(v))) for v in vectors]
    output = [0] * (1 << len(bit_vectors))
    for selector in range(1, len(output)):
        bit = selector & -selector
        output[selector] = output[selector ^ bit] ^ bit_vectors[bit.bit_length() - 1]
    assert len(set(output)) == len(output)
    return output


def arm_coefficients(arm):
    def image(v):
        if arm == 1:
            return v ** (64**3) + v**64 + v
        return v ** (64**3) + v ** (64**2) + v

    columns = [image(z**i) for i in range(42)]
    M = matrix(F2, 42, 42, lambda i, j: F2(columns[j].polynomial()[i]))
    basis = M.right_kernel().basis()
    assert len(basis) == 18
    v_basis = [from_mask(sum(int(vec[i]) << i for i in range(42))) for vec in basis]
    assert all(image(v) == 0 for v in v_basis)

    def subfield_selectors(degree):
        cols = [v ** (2**degree) + v for v in v_basis]
        test = matrix(F2, 42, 18, lambda i, j: F2(cols[j].polynomial()[i]))
        return selectors_from_basis(test.right_kernel().basis())

    small14 = set(subfield_selectors(14))
    small21 = set(subfield_selectors(21))
    assert len(small14) == 64 and len(small21) == 512
    assert len(small14 & small21) == 8
    excluded = small14 | small21
    assert len(excluded) == 568

    squared_basis_masks = [mask(v**2) for v in v_basis]
    squared_masks = [0] * (1 << 18)
    for selector in range(1, len(squared_masks)):
        bit = selector & -selector
        squared_masks[selector] = squared_masks[selector ^ bit] ^ squared_basis_masks[bit.bit_length() - 1]
    b_masks = [1 ^ squared_masks[selector] for selector in range(1, len(squared_masks)) if selector not in excluded]
    assert len(b_masks) == 261576 and len(set(b_masks)) == len(b_masks)
    return sorted(b_masks)


unit = z ** ((2**42 - 1) // 63)
assert unit.multiplicative_order() == 63
subfield = sorted({K(0)} | {unit**i for i in range(63)}, key=mask)
assert len(subfield) == 64 and all(t**64 == t for t in subfield)
trace_one = [t for t in subfield if sum((t ** (2**i) for i in range(6)), K(0)) == 1]
assert len(trace_one) == 32
a = trace_one[0]
k, embed = K.subfields(6)[0]
aa = next(t for t in k if embed(t) == a)
E0 = EllipticCurve(k, [1, aa, 0, 0, 1])
e0_order = ZZ(E0.cardinality())

arm1 = arm_coefficients(1)
arm2 = arm_coefficients(2)
assert set(arm1).isdisjoint(arm2)
candidates = [(1, b) for b in arm1] + [(2, b) for b in arm2]
assert len(candidates) == 523152

rows = []
chosen = None
for arm, b_mask in candidates[:128]:
    b = from_mask(b_mask)
    beta = b.sqrt()
    assert sum((beta ** (64**i) for i in range(7)), K(0)) == 1
    if arm == 1:
        assert (beta + 1) ** (64**3) + (beta + 1) ** 64 + (beta + 1) == 0
    else:
        assert (beta + 1) ** (64**3) + (beta + 1) ** (64**2) + (beta + 1) == 0
    E = EllipticCurve(K, [1, a, 0, 0, b])
    order = ZZ(E.cardinality())
    factors = [(int(p), int(e)) for p, e in factor(order)]
    r = max(p for p, _ in factors)
    h = int(order // r)
    row = {"arm": arm, "b_mask": b_mask, "order": int(order),
           "trace": int(2**42 + 1 - order), "factors": factors,
           "largest_prime_subgroup": r, "cofactor": h}
    rows.append(row)
    if chosen is None and h == 2 and r >= 2**40 and e0_order % r:
        chosen = row

output = {"schema": "ghs-n7-q64-screen/v1", "sage_version": sage_version,
          "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
          "field_modulus": str(modulus), "a_mask": mask(a), "c_mask": 1,
          "e0_order": int(e0_order), "kernel_dimensions_F2": [18, 18],
          "full_field_candidate_count": len(candidates),
          "screened_count": len(rows), "chosen": chosen, "rows": rows}
target = ROOT / "screen.json"
if target.exists():
    raise SystemExit("refusing to overwrite immutable screen receipt")
target.write_text(json.dumps(output, indent=2) + "\n")
print(json.dumps({k: v for k, v in output.items() if k != "rows"}, indent=2))
