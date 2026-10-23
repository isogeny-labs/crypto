#!/usr/bin/env python3
"""Independently check three archived prime-curve labels and base points."""
from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path

SOURCES = {
    "research/volcano_ecdlp_census_20261017/runs/slugmap/V3-ctl-L0-c0.json":
        "16f77524079584e040d6f4d64ca5234728ed0ecafbbec251e24fb6a593546ebb",
    "research/volcano_ecdlp_census_20261017/runs/slugmap/V3-D-L0-c0.json":
        "5230ffc1a2899a87ebab87a949e0a7b46ca24bc18608163fc57ac5ff82347c51",
}
EXPECTED = {
    "icv1-fp26-tm6998-a41c0795",
    "icv1-fp26-tm6998-d53b367d",
    "icv1-fp26-tm6998-eeb6174a",
}


def require(condition: bool, detail) -> None:
    if not condition:
        raise SystemExit(f"validation failed: {detail}")


def is_prime(n: int) -> bool:
    return n >= 2 and all(n % d for d in range(2, math.isqrt(n) + 1))


def add(left, right, p: int, a: int):
    if left is None:
        return right
    if right is None:
        return left
    x, y = left
    u, v = right
    if x == u and (y + v) % p == 0:
        return None
    if x == u:
        slope = (3 * x * x + a) * pow(2 * y, -1, p) % p
    else:
        slope = (v - y) * pow((u - x) % p, -1, p) % p
    z = (slope * slope - x - u) % p
    return z, (slope * (x - z) - y) % p


def multiply(k: int, point, p: int, a: int):
    result = None
    while k:
        if k & 1:
            result = add(result, point, p, a)
        point = add(point, point, p, a)
        k >>= 1
    return result


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[3])
    args = parser.parse_args()
    rows = {}
    for rel, expected_hash in SOURCES.items():
        data = (args.root / rel).read_bytes()
        actual_hash = hashlib.sha256(data).hexdigest()
        require(actual_hash == expected_hash, (rel, actual_hash))
        for slug, row in json.loads(data).items():
            require(slug not in rows, slug)
            rows[slug] = row
    require(set(rows) == EXPECTED, (set(rows), EXPECTED))
    for slug, row in sorted(rows.items()):
        p, a, b, order, r = (int(row[k]) for k in ("p", "a", "b", "group_order", "r"))
        g = int(row["gx"]), int(row["gy"])
        model = {"a": str(a), "b": str(b), "field": f"fp-{p}",
                 "form": "y^2=x^3+a*x+b", "p": str(p), "v": "1"}
        digest = hashlib.sha256(json.dumps(model, sort_keys=True, separators=(",", ":"),
                                           ensure_ascii=True).encode()).hexdigest()
        expected_slug = f"icv1-fp{p.bit_length()}-tm{order-p-1}-{digest[:8]}"
        require(slug == expected_slug, (slug, expected_slug))
        require(is_prime(p) and is_prime(r), (slug, "p/r primality"))
        require(order == 16 * r, (slug, "cofactor"))
        require((g[1] * g[1] - g[0] ** 3 - a * g[0] - b) % p == 0,
                (slug, "point on curve"))
        require(multiply(r, g, p, a) is None, (slug, "[r]G"))
        print(f"{slug}: source hash, prime p/r, cofactor 16, model label, point, and [r]G checked")


if __name__ == "__main__":
    main()
