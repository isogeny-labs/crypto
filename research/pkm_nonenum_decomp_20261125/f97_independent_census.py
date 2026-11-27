#!/usr/bin/env python3
"""Independent exact three-sum truth table for the native F97 witness gate."""

import argparse
import hashlib
import json
from pathlib import Path


P, A, B, ORDER, GENERATOR = 97, 3, 15, 101, (4, 24)
WIDTH = 8


def add(left, right):
    if left is None:
        return right
    if right is None:
        return left
    x1, y1 = left
    x2, y2 = right
    if x1 == x2 and (y1 + y2) % P == 0:
        return None
    if left == right:
        slope = (3 * x1 * x1 + A) * pow(2 * y1, -1, P) % P
    else:
        slope = (y2 - y1) * pow(x2 - x1, -1, P) % P
    x3 = (slope * slope - x1 - x2) % P
    return x3, (slope * (x1 - x3) - y1) % P


def table():
    affine = {(x, y) for x in range(P) for y in range(P)
              if (y * y - x * x * x - A * x - B) % P == 0}
    if len(affine) != ORDER - 1 or GENERATOR not in affine:
        raise ValueError("curve order or generator mismatch")

    by_scalar = [None]
    point = None
    for _ in range(1, ORDER):
        point = add(point, GENERATOR)
        by_scalar.append(point)
    if len(set(by_scalar)) != ORDER or set(by_scalar[1:]) != affine:
        raise ValueError("generator does not enumerate the full group")
    logs = {point: scalar for scalar, point in enumerate(by_scalar)}

    root = next(pow(candidate, (P - 1) // WIDTH, P)
                for candidate in range(2, P)
                if pow(pow(candidate, (P - 1) // WIDTH, P), WIDTH // 2, P) != 1)
    xs = sorted({pow(root, exponent, P) for exponent in range(WIDTH)})
    base = sorted(point for point in affine if point[0] in xs)
    if len(xs) != WIDTH or len(base) != 6:
        raise ValueError("Kummer base mismatch")

    triple_logs = {(logs[u] + logs[v] + logs[w]) % ORDER
                   for u in base for v in base for w in base}
    triple_points = {add(add(u, v), w)
                     for u in base for v in base for w in base}
    if {by_scalar[k] for k in triple_logs} != triple_points:
        raise ValueError("scalar and point triple sums disagree")
    if len(triple_logs) != 28 or 0 in triple_logs:
        raise ValueError("unexpected three-sum coverage")

    verdicts = [{"scalar": scalar, "point": list(by_scalar[scalar]),
                 "direct_three_sum": scalar in triple_logs}
                for scalar in range(1, ORDER)]
    return {"schema": "pkm-f97-independent-three-sum/v1",
            "curve": {"p": P, "a": A, "b": B, "order": ORDER,
                      "generator": list(GENERATOR)},
            "map": {"g": 1, "height": 3, "zeta": root,
                    "abscissae": xs, "lifted_points": [list(x) for x in base]},
            "method": "all affine points; scalar table by repeated addition; independent scalar and point triple sums",
            "reachable_affine_targets": len(triple_logs),
            "unreachable_affine_targets": ORDER - 1 - len(triple_logs),
            "verdicts": verdicts}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    result = table()
    result["producer_sha256"] = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    with args.output.open("x", encoding="utf-8") as stream:
        json.dump(result, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps({"output": str(args.output),
                      "reachable": result["reachable_affine_targets"],
                      "targets": len(result["verdicts"])}))


if __name__ == "__main__":
    main()
