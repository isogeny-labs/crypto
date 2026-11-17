#!/usr/bin/env python3
"""Independent GF(2) product and Rabin irreducibility checks for R100."""
import argparse
import hashlib
import json
import re
from pathlib import Path

FACTOR_SHA256 = "e5aab64332908e0305dd5fb8f0c7780e67954621846ceaec2198247e0d2c2cd9"
ROOT_SHA256 = "edffe01c7dca7dbe83ae3d77c58e9ec2a2274608b48a4d6d93cbd0a3feb52d0f"


def parse(expr: str) -> int:
    terms = expr.split("+")
    if len(terms) != len(set(terms)):
        raise ValueError("duplicate polynomial monomial")
    out = 0
    for term in terms:
        match = re.fullmatch(r"x\^([1-9][0-9]*)", term)
        degree = int(match.group(1)) if match else 1 if term == "x" else 0 if term == "1" else None
        if degree is None:
            raise ValueError(f"bad monomial {term}")
        out ^= 1 << degree
    return out


def mul(a: int, b: int) -> int:
    out = 0
    while b:
        if b & 1:
            out ^= a
        a <<= 1
        b >>= 1
    return out


def rem(a: int, modulus: int) -> int:
    m = modulus.bit_length() - 1
    while a.bit_length() - 1 >= m:
        a ^= modulus << (a.bit_length() - 1 - m)
    return a


def gcd(a: int, b: int) -> int:
    while b:
        a, b = b, rem(a, b)
    return a


def irreducible(poly: int) -> bool:
    degree = poly.bit_length() - 1
    x = rem(2, poly)
    power = x
    checkpoints = {degree // q for q in range(2, degree + 1) if degree % q == 0 and all(q % d for d in range(2, int(q ** 0.5) + 1))}
    for step in range(1, degree + 1):
        power = rem(mul(power, power), poly)
        if step in checkpoints and gcd(power ^ x, poly) != 1:
            return False
    return power == x


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path)
    parser.add_argument("factors", type=Path)
    args = parser.parse_args()
    for path, expected in [(args.root, ROOT_SHA256), (args.factors, FACTOR_SHA256)]:
        actual = hashlib.sha256(path.read_bytes()).hexdigest()
        if actual != expected:
            raise ValueError(f"source hash mismatch for {path}: {actual}")
    root_text = args.root.read_text()
    factor_text = args.factors.read_text()
    root = parse(re.search(r"poly root100=([^;]+);", root_text).group(1))
    numerator = parse(re.search(r"poly numerator100=([^;]+);", root_text).group(1))
    retained_root = parse(re.search(r"poly root100=([^;]+);", factor_text).group(1))
    if root != retained_root or sum(1 << (16 * e) for e in range(root.bit_length()) if root >> e & 1) != numerator:
        raise ValueError("root or Frobenius power mismatch")
    matches = re.findall(r"factors100\[(\d+)\]=([^;]+);exponents100\[\1\]=(\d+);", factor_text)
    if len(matches) != 3 or len(set(i for i, _, _ in matches)) != 3:
        raise ValueError("factor-list shape mismatch")
    product = 1
    rows = []
    for index, expression, exponent_text in matches:
        factor = parse(expression)
        exponent = int(exponent_text)
        if exponent < 1 or not irreducible(factor):
            raise ValueError(f"factor {index} is not irreducible with positive exponent")
        for _ in range(exponent):
            product = mul(product, factor)
        rows.append({"index": int(index), "degree": factor.bit_length() - 1, "exponent": exponent, "polynomial": expression})
    if product != root or root.bit_length() - 1 != 123:
        raise ValueError("independent factor-product mismatch")
    print(json.dumps({"check": "R100_INDEPENDENT_GF2_PASS", "factor_sha256": FACTOR_SHA256, "root_sha256": ROOT_SHA256, "rows": rows}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
