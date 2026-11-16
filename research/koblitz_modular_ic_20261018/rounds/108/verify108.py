#!/usr/bin/env python3
"""Independent 128-bit source residual and Lean-constant binding."""
import hashlib
import json
import re
from pathlib import Path

root = Path(__file__).resolve().parent
base = root.parent / "105" / "result.json"
prior = root.parent / "106" / "result.json"
result = json.loads((root / "result.json").read_text())
lean = (root / "lean" / "BaseSeries128.lean").read_text()
assert hashlib.sha256(base.read_bytes()).hexdigest() == "196593c612e43fe185041373bc481abf4c3a0556a3504b8b0327f26b70ec7ffd"
assert hashlib.sha256(prior.read_bytes()).hexdigest() == "c112aeedde039a3b36376a0e720e00f62da64c5abe336d77296b97baa4cc12b8"
f = json.loads(base.read_text())
old = json.loads(prior.read_text())
mask = (1 << 128) - 1


def mul(a, b):
    out = 0
    for i in range(128):
        if b >> i & 1: out ^= a << i
    return out & mask


def power(a, n):
    out = 1
    for _ in range(n): out = mul(out, a)
    return out


for key, label in (("y0", "f0"), ("y1", "f1")):
    support = [tuple(x) for x in f[f"f_{key}_support"]]
    z = sum(1 << i for i in result[key]["z_exponents"])
    t = mul(2, z)
    assert result[key]["t_exponents"] == [i for i in range(128) if t >> i & 1]
    assert [i for i in result[key]["z_exponents"] if i < 32] == old[key]["z_exponents"]
    assert [i for i in result[key]["t_exponents"] if i < 32] == old[key]["t_exponents"]
    residual = 0
    for a, b in support:
        residual ^= mul(power(t, a), power(z, b))
    assert residual == 0
    m = re.search(rf"def {label} : List \(Nat × Nat\) := \[(.*?)\]", lean)
    assert m is not None and [tuple(map(int, x)) for x in re.findall(r"\((\d+), (\d+)\)", m.group(1))] == support
    m = re.search(rf"def z{key[-1]} : Nat := (\d+)", lean)
    assert m is not None and int(m.group(1)) == z
print("EXTEND108_INDEPENDENT_128_PASS")
