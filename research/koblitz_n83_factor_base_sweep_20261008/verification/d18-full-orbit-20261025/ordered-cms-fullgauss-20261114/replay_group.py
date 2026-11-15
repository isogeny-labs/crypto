#!/usr/bin/env python3
"""Independently lift a D18 SAT assignment and replay its curve relation."""

import argparse
from itertools import product
import gzip
import hashlib
import json
from pathlib import Path

from verify_xcnf_model import parse_model, sha256

N = 83
MASK = (1 << N) - 1
MOD = (1 << N) | (1 << 45) | 7
ORDER = 2417851639230796216685689
XCNF_SHA = "c632c152447cb09dbe7dd7cf7e29f713588c37bd56b67dfbf1fa7dac4cb15b74"
OBJECT_SHA = "eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b"


def require(ok, message):
    if not ok:
        raise ValueError(message)


def mul(a, b):
    out = 0
    while b:
        if b & 1:
            out ^= a
        b >>= 1
        a <<= 1
        if a >> N:
            a ^= MOD
    return out


def inv(a):
    require(a != 0, "zero has no inverse")
    u, v, g, h = a, MOD, 1, 0
    while u != 1:
        require(u != 0, "field modulus does not invert input")
        shift = u.bit_length() - v.bit_length()
        if shift < 0:
            u, v = v, u
            g, h = h, g
            shift = -shift
        u ^= v << shift
        g ^= h << shift
    while g.bit_length() > N:
        g ^= MOD << (g.bit_length() - N - 1)
    return g


def on_curve(point):
    if point is None:
        return True
    x, y = point
    return mul(y, y) ^ mul(x, y) ^ mul(mul(x, x), x) ^ 1 == 0


def add(p, q):
    if p is None:
        return q
    if q is None:
        return p
    x, y = p
    u, v = q
    if x == u and y != v:
        require(y ^ v == x, "same-x points are not inverses")
        return None
    if x == u:
        if x == 0:
            return None
        slope = x ^ mul(y, inv(x))
        rx = mul(slope, slope) ^ slope
        ry = mul(x, x) ^ mul(slope ^ 1, rx)
    else:
        slope = mul(y ^ v, inv(x ^ u))
        rx = mul(slope, slope) ^ slope ^ x ^ u
        ry = mul(slope, x ^ rx) ^ rx ^ y
    return rx, ry


def scalar(point, n):
    result = None
    while n:
        if n & 1:
            result = add(result, point)
        point = add(point, point)
        n >>= 1
    return result


def point_of(row):
    return tuple(map(int, row))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path)
    args = parser.parse_args()
    root = args.root
    verification = json.loads((root / "solver-positive/verification.json").read_text())
    require(verification["status"] == "PASS_ORIGINAL_XCNF_REPLAY", "XCNF replay did not pass")
    require(verification["xcnf_sha256"] == XCNF_SHA, "XCNF identity differs")
    receipt = json.loads((root / "export-positive/result.json").read_text())
    target = point_of(receipt["target"])
    require(receipt["curve_a"] == 0 and receipt["summands"] == 4, "curve or arity differs")
    require(on_curve(target) and scalar(target, ORDER) is None, "target group check failed")
    model = parse_model(root / "solver-positive/stdout.txt", 329808)
    xs = [sum((1 << bit) for bit in range(N) if model[slot * N + bit + 1] == 1) for slot in range(4)]
    object_path = Path("/Volumes/SSD990-2/tmp/n83-d18-cms-20261114/input") / receipt["object"]
    require(sha256(object_path) == OBJECT_SHA, "factor-base object differs")
    found = {x: [] for x in xs}
    with gzip.open(object_path, "rt") as stream:
        header = json.loads(stream.readline())
        require(header["curve_a"] == 0 and header["curve_b"] == "1", "object curve differs")
        require(header["point_count"] == 5418738 and header["orbit_columns"] == 32643, "object shape differs")
        reps = [point_of(row) for row in header["representatives"]]
        for line in stream:
            row = json.loads(line)
            point = point_of(row["point"])
            if point[0] in found:
                require(on_curve(point), "stored point fails curve equation")
                found[point[0]].append((point, row))
    require(all(found.values()), "SAT x-coordinate absent from archived factor base")
    require(all(len(points) <= 2 for points in found.values()), "more than two lifts for an x-coordinate")
    chosen = None
    for combination in product(*(found[x] for x in xs)):
        point_sum = None
        row_sum = None
        for point, row in combination:
            point_sum = add(point_sum, point)
            column = row["column"]
            require(0 <= column < len(reps), "column out of range")
            row_sum = add(row_sum, scalar(reps[column], int(row["coefficient"]) % ORDER))
        if point_sum == target and row_sum == target:
            chosen = combination
            break
    require(chosen is not None, "SAT model lacks a checked curve and relation-row lift")
    for point, row in chosen:
        require(scalar(point, ORDER) is None, "relation point outside subgroup")
    report = {
        "schema": "n83.d18-fullgauss-external-group-replay/v1",
        "status": "PASS_ORIGINAL_GROUP_AND_ROW_REPLAY",
        "xcnf_sha256": XCNF_SHA,
        "object_sha256": OBJECT_SHA,
        "solver_stdout_sha256": sha256(root / "solver-positive/stdout.txt"),
        "x_coordinates": [str(x) for x in xs],
        "target": list(map(str, target)),
        "chosen_points": [list(map(str, point)) for point, _ in chosen],
        "columns": [row["column"] for _, row in chosen],
        "coefficients": [row["coefficient"] for _, row in chosen],
        "subgroup_order": str(ORDER),
        "selected_best_total_runtime": None,
    }
    with (root / "solver-positive/group-replay.json").open("x") as stream:
        json.dump(report, stream, sort_keys=True, indent=2)
        stream.write("\n")
    print(json.dumps({"status": report["status"], "x_coordinates": report["x_coordinates"]}, sort_keys=True))


if __name__ == "__main__":
    main()
