#!/usr/bin/env python3
"""Freeze solver-blind held-out X1(2,8) curves and target workloads."""

import argparse
import hashlib
import json
import math
import os
from pathlib import Path

HERE = Path(__file__).resolve().parent
PROTOCOL = HERE / "PROTOCOL_Z2XZ8_EQUAL_SIZE_COLD_20261128.md"
BITS = (11, 14)
CURVES_PER_RUNG = 3
TARGETS_PER_CURVE = 16


def sha(data):
    return hashlib.sha256(data).hexdigest()


def encoded(value):
    return (json.dumps(value, sort_keys=True, indent=2) + "\n").encode()


def preserve(path, data):
    if path.exists():
        if path.read_bytes() != data:
            raise ValueError(f"frozen file differs: {path}")
        return
    with os.fdopen(os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644), "wb") as stream:
        stream.write(data)
        stream.flush()
        os.fsync(stream.fileno())


def prime(n):
    return n >= 2 and all(n % d for d in range(2, math.isqrt(n) + 1))


def roots(p):
    count = [0] * p
    first = [None] * p
    for y in range(p):
        z = y * y % p
        count[z] += 1
        if first[z] is None:
            first[z] = y
    return count, first


def add(p, a, left, right):
    if left is None:
        return right
    if right is None:
        return left
    x1, y1 = left
    x2, y2 = right
    if x1 == x2 and (y1 + y2) % p == 0:
        return None
    if x1 == x2:
        numerator, denominator = (3*x1*x1+a) % p, (2*y1) % p
    else:
        numerator, denominator = (y2-y1) % p, (x2-x1) % p
    slope = numerator * pow(denominator, -1, p) % p
    x3 = (slope*slope-x1-x2) % p
    return x3, (slope*(x1-x3)-y1) % p


def multiply(p, a, k, point):
    out = None
    while k:
        if k & 1:
            out = add(p, a, out, point)
        point = add(p, a, point, point)
        k >>= 1
    return out


def select_curve(p):
    count, first = roots(p)
    rejected = 0
    for a in range(1, p):
        for b in range(1, p):
            if (4*a*a*a+27*b*b) % p == 0:
                rejected += 1
                continue
            r = 1 + sum(count[(x*x*x+a*x+b) % p] for x in range(p))
            if not prime(r) or r == p:
                rejected += 1
                continue
            generator = next((x, first[(x*x*x+a*x+b) % p])
                             for x in range(p) if first[(x*x*x+a*x+b) % p] is not None)
            assert multiply(p, a, r, generator) is None
            return a, b, r, generator, rejected
    raise ValueError(f"no prime-order curve at p={p}")


def point(pair):
    return {"x": hex(pair[0]), "y": hex(pair[1])}


def targets(p, a, r, generator, bits, curve_index):
    rows = []
    used = set()
    for j in range(TARGETS_PER_CURVE):
        retry = 0
        while True:
            digest = sha(f"pkm-z2xz8-holdout-v1|{bits}|{curve_index}|{j}|{retry}".encode())
            scalar = 1 + int(digest, 16) % (r-1)
            if scalar not in used:
                break
            retry += 1
        used.add(scalar)
        q = multiply(p, a, scalar, generator)
        assert q is not None
        rows.append({"target_index": j, "scalar": scalar, "point": point(q),
                     "digest": digest, "retry": retry,
                     "relation_seed_sha256": sha(
                         f"pkm-z2xz8-relations-v1|{bits}|{curve_index}|{j}".encode())})
    return rows


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out-dir", type=Path, default=HERE / "z2xz8_holdout")
    args = parser.parse_args()
    protocol_sha = sha(PROTOCOL.read_bytes())
    producer_sha = sha(Path(__file__).read_bytes())
    args.out_dir.mkdir(parents=True, exist_ok=True)
    roster = []
    entries = []
    for bits in BITS:
        primes = []
        p = 1 + 32 * (((1 << bits) - 1) // 32 + 1)
        while len(primes) < CURVES_PER_RUNG:
            if prime(p):
                primes.append(p)
            p += 32
        for curve_index, p in enumerate(primes):
            a, b, r, generator, rejected = select_curve(p)
            ts = targets(p, a, r, generator, bits, curve_index)
            roster.append({"bits": bits, "curve_index": curve_index, "p": p,
                           "a": a, "b": b, "r": r, "generator": list(generator),
                           "rejected_pairs": rejected, "targets": ts})
            workload = {"schema": "ic.prime-workload/v1",
                        "curve": {"field": {"kind": "prime", "modulus": hex(p)},
                                  "a": hex(a), "b": hex(b), "group_order": hex(r),
                                  "subgroup_order": hex(r), "cofactor": "0x1",
                                  "generator": point(generator)},
                        "sampling": f"pkm-z2xz8-holdout/v1; bits={bits}; curve_index={curve_index}; protocol_sha256={protocol_sha}",
                        "targets": [{"scalar": t["scalar"], "point": t["point"]} for t in ts]}
            name = f"b{bits}_c{curve_index}.json"
            data = encoded(workload)
            preserve(args.out_dir / name, data)
            entries.append({"bits": bits, "curve_index": curve_index, "p": p,
                            "r": r, "path": name, "sha256": sha(data),
                            "target_scalars": [t["scalar"] for t in ts]})
    roster_data = encoded({"schema": "pkm-z2xz8-holdout-roster/v1",
                           "protocol_sha256": protocol_sha, "producer_sha256": producer_sha,
                           "curves": roster})
    preserve(args.out_dir / "roster.json", roster_data)
    manifest_data = encoded({"schema": "pkm-z2xz8-holdout-manifest/v1",
                             "protocol_sha256": protocol_sha,
                             "producer_sha256": producer_sha,
                             "roster_sha256": sha(roster_data), "entries": entries})
    preserve(args.out_dir / "manifest.json", manifest_data)
    print(json.dumps({"curves": len(entries), "targets": len(entries)*TARGETS_PER_CURVE,
                      "roster_sha256": sha(roster_data),
                      "manifest_sha256": sha(manifest_data)}, sort_keys=True))


if __name__ == "__main__":
    main()
