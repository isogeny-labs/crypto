#!/usr/bin/env python3
"""Source-bound F2 Hensel series for the two R105 base branches."""
import hashlib
import json
import sys
from pathlib import Path

R105_SHA256 = "196593c612e43fe185041373bc481abf4c3a0556a3504b8b0327f26b70ec7ffd"
PRECISION = 32
MASK = (1 << PRECISION) - 1


def mul(a, b):
    out = 0
    while b:
        if b & 1: out ^= a
        a <<= 1; b >>= 1
    return out & MASK


def pow2(a, n):
    out = 1
    while n:
        if n & 1: out = mul(out, a)
        a = mul(a, a); n >>= 1
    return out


def evaluate(support, s, z):
    out = 0
    for a, b in support:
        out ^= mul(pow2(s, a), pow2(z, b))
    return out


def valuation(bits):
    if not bits: return None
    return (bits & -bits).bit_length() - 1


def solve(local_support, common_z, expected_z, expected_t):
    transformed = sorted({(a, a + b - common_z) for a, b in local_support})
    if len(transformed) != len(local_support) or any(a < 0 or b < 0 for a, b in transformed):
        raise ValueError("invalid common z factor")
    if (0, 0) in transformed or (0, 1) not in transformed:
        raise ValueError("not a simple zero-constant root")
    s, z = 2, 0
    for k in range(PRECISION):
        if evaluate(transformed, s, z) >> k & 1:
            z ^= 1 << k
    t = mul(s, z)
    if evaluate(transformed, s, z) or evaluate(local_support, t, z):
        raise ValueError("source residual nonzero")
    if valuation(z) != expected_z or valuation(t) != expected_t:
        raise ValueError("wrong local valuations")
    return {"common_z_power": common_z, "transformed_support": transformed,
            "simple_root": True, "z_exponents": [i for i in range(PRECISION) if z >> i & 1],
            "t_exponents": [i for i in range(PRECISION) if t >> i & 1],
            "z_valuation": valuation(z), "t_valuation": valuation(t),
            "precision": PRECISION, "transformed_residual": 0, "source_residual": 0}


def main():
    source, output = map(Path, sys.argv[1:3])
    if hashlib.sha256(source.read_bytes()).hexdigest() != R105_SHA256:
        raise ValueError("R105 source hash mismatch")
    data = json.loads(source.read_text())
    y0 = [tuple(p) for p in data["f_y0_support"]]
    y1 = [tuple(p) for p in data["f_y1_support"]]
    result = {"r105_sha256": R105_SHA256, "y0": solve(y0, 2, 2, 3),
              "y1": solve(y1, 3, 3, 4)}
    output.mkdir(parents=True, exist_ok=True)
    (output / "result.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print("SERIES106_ALL_CHECKS_PASS")


if __name__ == "__main__": main()
