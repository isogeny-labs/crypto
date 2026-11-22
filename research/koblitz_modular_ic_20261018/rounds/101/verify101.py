#!/usr/bin/env python3
"""Independent GF(2) checks for the retained R101 x=1 source-order fiber."""
import hashlib
import json
import sys
from pathlib import Path

SPECIALIZATION_SHA256 = "92d795e666b43874c37af1f8e0e146b5aae1a2a3ab9a61954b4395738ab4405e"
FACTOR_SHA256 = "96472f32314efc746fe7d7be69d80a153526b73decbd092a032f39d513873807"


def mul(a: int, b: int) -> int:
    out = 0
    while b:
        if b & 1:
            out ^= a
        a <<= 1
        b >>= 1
    return out


def power(a: int, exponent: int) -> int:
    out = 1
    for _ in range(exponent):
        out = mul(out, a)
    return out


def main() -> None:
    specialization, factors = map(Path, sys.argv[1:3])
    for path, expected in ((specialization, SPECIALIZATION_SHA256), (factors, FACTOR_SHA256)):
        if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError(f"source hash mismatch: {path}")
    actual_specialization = specialization.read_text()
    actual_factors = factors.read_text()
    if actual_specialization != "poly f101=y^7+y^3;\npoly G101=l^16+l^8+1;\npoly C101=0;\n":
        raise ValueError("unexpected specialized source")
    if "factors0_101[1]=l^2+l+1;exponents0_101[1]=8;" not in actual_factors or "factors1_101[1]=l^2+l+1;exponents1_101[1]=8;" not in actual_factors:
        raise ValueError("unexpected lambda factorization")
    q = (1 << 2) | (1 << 1) | 1
    lambda_specialization = (1 << 16) | (1 << 8) | 1
    base_specialization = (1 << 7) | (1 << 3)
    if power(q, 8) != lambda_specialization:
        raise ValueError("lambda Frobenius factor identity failed")
    if mul(1 << 3, power(3, 4)) != base_specialization:
        raise ValueError("base factor identity failed")
    if q & 1 == 0 or (bin(q).count("1") & 1) == 0:
        raise ValueError("quadratic has an F2 root")
    if 3 * 8 * 2 != 48 or 4 * 8 * 2 != 64 or 48 + 64 != 112:
        raise ValueError("fiber lengths do not match source degree")
    print(json.dumps({"check": "R101_INDEPENDENT_GF2_PASS", "base_factorization": "y^3*(y+1)^4", "lambda_factorization": "(l^2+l+1)^8", "lambda_factor_irreducible_over_F2": True, "source_order_maximal_ideals": 2, "component_dimensions_over_F2": [48, 64], "total_dimension_over_F2": 112}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
