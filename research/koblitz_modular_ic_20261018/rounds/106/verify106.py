#!/usr/bin/env python3
"""Bind the Lean constants to R105/R106 JSON and recheck both residuals."""
import hashlib
import json
import re
from pathlib import Path

root = Path(__file__).resolve().parent
base = root.parent / "105" / "result.json"
result = root / "result.json"
lean = (root / "lean" / "BaseUniformizers106.lean").read_text()
assert hashlib.sha256(base.read_bytes()).hexdigest() == "196593c612e43fe185041373bc481abf4c3a0556a3504b8b0327f26b70ec7ffd"
f = json.loads(base.read_text())
r = json.loads(result.read_text())
mask = (1 << 32) - 1


def poly_mul(a, b):
    out = 0
    for i in range(32):
        if b & (1 << i): out ^= a << i
    return out & mask


def poly_pow(a, n):
    out = 1
    for _ in range(n): out = poly_mul(out, a)
    return out


for key, label in (("y0", "f0"), ("y1", "f1")):
    support = [tuple(x) for x in f[f"f_{key}_support"]]
    z = sum(1 << i for i in r[key]["z_exponents"])
    t = poly_mul(2, z)
    assert r[key]["t_exponents"] == [i for i in range(32) if t >> i & 1]
    residual = 0
    for a, b in support:
        residual ^= poly_mul(poly_pow(t, a), poly_pow(z, b))
    assert residual == 0
    match = re.search(rf"def {label} : List \(Nat × Nat\) := \[(.*?)\]", lean)
    assert match is not None
    lean_support = [tuple(map(int, x)) for x in re.findall(r"\((\d+), (\d+)\)", match.group(1))]
    assert lean_support == support
    match = re.search(rf"def z{key[-1]} : Nat := (\d+)", lean)
    assert match is not None and int(match.group(1)) == z
print("SERIES106_INDEPENDENT_LEAN_BINDING_PASS")
