#!/usr/bin/env sage -python
"""Select the first exact order-r source point by the frozen x/y mask rule."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, ZZ
from sage.version import version as sage_version

ROOT = Path(__file__).resolve().parent
F2 = GF(2)
R = PolynomialRing(F2, "x")
x = R.gen()
K = GF(2**35, "z", modulus=x**35 + x**2 + 1)


def from_mask(bits):
    return K(R([(int(bits) >> i) & 1 for i in range(35)]))


def mask(value):
    return sum(int(bit) << i for i, bit in enumerate(value.polynomial().list()))


screen = json.loads((ROOT / "screen.json").read_text())
chosen = screen["chosen"]
assert screen["field_modulus"] == "x^35 + x^2 + 1"
assert screen["a_mask"] == 1 and chosen["b_mask"] == 9698245
r = ZZ(chosen["largest_prime_subgroup"])
assert r == 5726649013 and r.is_prime()
E = EllipticCurve(K, [1, 1, 0, 0, from_mask(chosen["b_mask"])])
assert ZZ(E.cardinality()) == 6 * r

selected = None
for x_mask in range(1 << 16):
    points = sorted(E.lift_x(from_mask(x_mask), all=True), key=lambda R: mask(R[1]))
    for candidate in points:
        P = 6 * candidate
        if P != E(0) and r * P == E(0):
            selected = (x_mask, mask(candidate[1]), P)
            break
    if selected is not None:
        break
assert selected is not None
x_mask, y_mask, P = selected
Q = 1234567 * P
assert Q != E(0) and r * Q == E(0)
output = {
    "schema": "ghs-n7-q32-point/v1",
    "sage_version": sage_version,
    "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    "screen_sha256": hashlib.sha256((ROOT / "screen.json").read_bytes()).hexdigest(),
    "selected_row": chosen,
    "source_x_mask": x_mask,
    "source_y_mask": y_mask,
    "point_x_mask": mask(P[0]),
    "point_y_mask": mask(P[1]),
    "target_x_mask": mask(Q[0]),
    "target_y_mask": mask(Q[1]),
    "subgroup_order": int(r),
    "validation_scalar": 1234567,
}
target = ROOT / "point.json"
if target.exists():
    raise SystemExit("refusing to overwrite immutable point receipt")
target.write_text(json.dumps(output, indent=2) + "\n")
print(json.dumps(output, indent=2))
