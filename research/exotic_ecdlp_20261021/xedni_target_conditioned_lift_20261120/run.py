#!/usr/bin/env python3
"""Per-public-point rational lift search with modular x filtering."""

import gzip
import hashlib
import json
from math import gcd, isqrt
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def search(target, cap, p):
    xt, yt = target
    b_steps = residue_pairs = tests = roots = 0
    for H in range(1, cap + 1):
        for b in range(1, H + 1):
            b_steps += 1
            if b % p == 0:
                continue
            residue = xt*b*b % p
            if b == H:
                start = -H + (residue+H) % p
                aa = range(start, H+1, p)
            else:
                aa = (a for a in (-H, H) if a % p == residue)
            for a in aa:
                residue_pairs += 1
                if gcd(abs(a), b) != 1:
                    continue
                tests += 1
                B = 94*a*b + 188*b**3
                C = a**3 + 1738*a*b**4 + 3805*b**6
                D = B*B + 4*C
                if D < 0:
                    continue
                root = isqrt(D)
                if root*root != D:
                    continue
                for signed in ((root,) if root == 0 else (root, -root)):
                    if (signed-B) % 2:
                        continue
                    c = (signed-B)//2
                    roots += 1
                    if c*pow(b, -3, p) % p == yt:
                        return {"target": list(target), "status": "found", "height": H,
                                "lift": [a, b, c], "b_steps": b_steps,
                                "residue_pairs": residue_pairs,
                                "discriminant_tests": tests, "rational_roots": roots}
    return {"target": list(target), "status": "height_cap_exhausted", "height": cap,
            "lift": None, "b_steps": b_steps, "residue_pairs": residue_pairs,
            "discriminant_tests": tests, "rational_roots": roots}


def main():
    protocol_raw = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_raw)
    source_raw = SOURCE.read_bytes()
    assert digest(source_raw) == protocol["source_packed_sha256"]
    source = [json.loads(line) for line in gzip.decompress(source_raw).splitlines()]
    targets = sorted({tuple(point) for row in source for point in row["reduced_points"]})
    assert len(targets) == 42
    rows = [search(target, protocol["height_cap"], protocol["reduction_prime"])
            for target in targets]
    raw = "".join(json.dumps(row, sort_keys=True)+"\n" for row in rows).encode()
    (HERE / "target-rows.jsonl").write_bytes(raw)
    return {"schema": "xedni-target-conditioned-lift-result-v1", "status": "PASS",
            "protocol_sha256": digest(protocol_raw), "script_sha256": digest((HERE / "run.py").read_bytes()),
            "source_packed_sha256": digest(source_raw), "target_rows_raw_sha256": digest(raw),
            "targets": len(rows), "found": sum(row["status"] == "found" for row in rows),
            "height_cap_exhausted": sum(row["status"] == "height_cap_exhausted" for row in rows),
            "sum_discriminant_tests": sum(row["discriminant_tests"] for row in rows),
            "sum_residue_pairs": sum(row["residue_pairs"] for row in rows),
            "sum_b_steps": sum(row["b_steps"] for row in rows)}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
