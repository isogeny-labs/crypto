#!/usr/bin/env python3
"""Deterministically register public prime31 Q and caller-only scalar answers."""

import hashlib
import json
import subprocess
from pathlib import Path

P = 2_147_483_647
N = 2_147_446_831
A = 847_514_846
B = 2_087_444_724
G = (625_037_715, 2_100_627_251)
SEED = b"APS_EQUAL_CORE_HELDOUT_20261125_V1"
COUNT = 48
ROOT = Path(__file__).resolve().parent


def on_curve(point):
    if point is None:
        return True
    x, y = point
    return 0 <= x < P and 0 <= y < P and (y * y - x * x * x - A * x - B) % P == 0


def add(left, right):
    if left is None:
        return right
    if right is None:
        return left
    x, y = left
    z, w = right
    if x == z and (y + w) % P == 0:
        return None
    if x == z:
        slope = ((3 * x * x + A) * pow(2 * y, P - 2, P)) % P
    else:
        slope = ((w - y) * pow((z - x) % P, P - 2, P)) % P
    u = (slope * slope - x - z) % P
    v = (slope * (x - u) - y) % P
    return u, v


def multiply(k, point):
    result = None
    while k:
        if k & 1:
            result = add(result, point)
        point = add(point, point)
        k >>= 1
    return result


def main():
    assert on_curve(G)
    assert multiply(N, G) is None
    development_path = ROOT / "public_targets.json"
    development_bytes = (development_path.read_bytes() if development_path.exists()
        else subprocess.check_output([
            "git", "show",
            "HEAD:research/prime31_aps_parallel_20261125/public_targets.json",
        ]))
    development = {tuple(q) for q in json.loads(development_bytes)}
    targets = []
    records = []
    rejected = []
    used = set()
    counter = 0
    while len(targets) < COUNT:
        digest = hashlib.sha256(SEED + counter.to_bytes(8, "big")).digest()
        k = 2 + int.from_bytes(digest, "big") % (N - 2)
        q = multiply(k, G)
        reason = ("duplicate_scalar" if k in used else
                  "development_Q" if q in development else
                  "invalid_Q" if q is None or not on_curve(q) else None)
        if reason:
            rejected.append({"counter": counter, "reason": reason})
        else:
            used.add(k)
            targets.append(list(q))
            records.append({"case": len(records), "counter": counter,
                            "caller_known_scalar_verifier_only": k,
                            "public_Q": list(q)})
        counter += 1
    public = json.dumps(targets, indent=2) + "\n"
    answers = json.dumps({
        "standing": "registered held-out public benchmark inputs",
        "seed_ascii": SEED.decode(),
        "seed_sha256": hashlib.sha256(SEED).hexdigest(),
        "curve": {"p": P, "a": A, "b": B, "n": N, "g": list(G),
                  "cofactor": 1},
        "case_count": COUNT,
        "selection_attempts": counter,
        "rejected": rejected,
        "records": records,
        "public_targets_sha256": hashlib.sha256(public.encode()).hexdigest(),
    }, indent=2) + "\n"
    (ROOT / "heldout_public_targets.json").write_text(public)
    (ROOT / "heldout_caller_answers.json").write_text(answers)


if __name__ == "__main__":
    main()
