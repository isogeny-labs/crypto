#!/usr/bin/env python3
"""Separate frozen mu10 follow-up using the v1 exact-curve screen engine."""
import argparse
import hashlib
import json
import time
from pathlib import Path

from screen import ROOT, CURVE_SOURCE, TARGET_SOURCE, parameters, lift, small_base, screen_base


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--seed", type=int, required=True, choices=(131, 132))
    args = ap.parse_args()
    start = time.perf_counter()
    v = parameters()
    p = v["p"]
    assert (p - 1) % 10 == 0 and (p - 1) % 20 != 0
    zeta = next(z for h in range(2, 100) if (z := pow(h, (p - 1) // 10, p)) not in (1, p - 1)
                and pow(z, 5, p) != 1 and pow(z, 10, p) == 1)
    arms, used = [], set()
    for g in range(1, 17):
        xs = sorted({g * pow(zeta, j, p) % p for j in range(10)})
        base = [P for x in xs if (P := lift(x, v)) is not None]
        if len(base) < 2:
            arms.append({"representative": g, "lifted": len(base), "status": "too_few_columns"})
            continue
        digest = tuple(x for x, _ in base)
        if digest in used:
            arms.append({"representative": g, "lifted": len(base), "status": "duplicate_coset"})
            continue
        used.add(digest)
        for kind, chosen in (("coset", base), ("small_x", small_base(len(base), v))):
            t = time.perf_counter()
            result = screen_base(chosen, v, args.seed, 256, 256)
            arms.append({"representative": g, "kind": kind, "lifted": len(chosen),
                         "status": "complete", "elapsed_seconds": time.perf_counter() - t, **result})
    print(json.dumps({"schema": "eccp131-coset-screen-mu10/v1", "seed": args.seed,
                      "parameters": {k: hex(val) for k, val in v.items()},
                      "source_sha256": {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
                                        for path in (CURVE_SOURCE, TARGET_SOURCE)},
                      "kummer": {"order": 10, "zeta10": hex(zeta),
                                  "factorization_p_minus_1": [2, 5, 155003179783434785924857641481313994241]},
                      "arms": arms, "elapsed_seconds": time.perf_counter() - start},
                     separators=(",", ":")))


if __name__ == "__main__":
    main()
