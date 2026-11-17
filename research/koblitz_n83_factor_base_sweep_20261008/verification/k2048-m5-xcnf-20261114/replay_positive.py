#!/usr/bin/env python3
"""Independently replay K2048 positive XCNF, group law and relation row."""

from itertools import product
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
LOCK = json.loads((HERE / "POSITIVE_SOLVER_LOCK.json").read_text())
RAW = Path(LOCK["raw_root"])
REFERENCE = HERE.parent / "d18-full-orbit-20261025/ordered-cms-fullgauss-20261114/verify_xcnf_model.py"
N = 83
MASK = (1 << N) - 1
MOD = (1 << N) | (1 << 45) | 7
ORDER = 2417851639230796216685689
OBJECT = "objects/82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851.jsonl.gz"
OBJECT_SHA256 = "aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a"


def require(ok, message):
    if not ok:
        raise ValueError(message)


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


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
    spec = importlib.util.spec_from_file_location("retained_xcnf_verifier", REFERENCE)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    receipt_path = RAW / "solver-positive/receipt.json"
    receipt = json.loads(receipt_path.read_text())
    require(receipt["status"] == "SAT_PENDING_REPLAY" and receipt["exit_code"] == 10, "solver did not return a SAT model")
    require(receipt["xcnf_sha256"] == LOCK["xcnf_sha256"], "solver formula identity differs")
    export = json.loads((RAW / "export-positive/result.json").read_text())
    require(export["source_commit"] == LOCK["source_commit"] and export["curve_a"] == 0, "export source or curve differs")
    require(export["shape"]["summands"] == 5 and export["shape"]["positive_x_pins"] == 747, "positive encoding shape differs")
    xcnf = RAW / "export-positive/result.xcnf"
    stdout = RAW / "solver-positive/stdout.txt"
    require(sha256(xcnf) == LOCK["xcnf_sha256"], "original formula differs")
    require(sha256(stdout) == receipt["stdout_sha256"], "solver model differs")
    values = module.parse_model(stdout, LOCK["variables"])
    cnf, xor = module.verify_formula(xcnf, values, LOCK["cnf_clauses"] + LOCK["xor_rows"])
    require((cnf, xor) == (LOCK["cnf_clauses"], LOCK["xor_rows"]), "constraint types differ")
    xs = [sum(1 << bit for bit in range(N) if values[slot * N + bit + 1] == 1) for slot in range(5)]
    intermediate_xs = [sum(1 << bit for bit in range(N) if values[slot * N + bit + 1] == 1) for slot in range(5, 8)]
    target = point_of(export["target"])
    require(on_curve(target) and scalar(target, ORDER) is None, "target group check failed")
    object_path = Path(LOCK["input_root"]) / OBJECT
    require(sha256(object_path) == OBJECT_SHA256, "retained factor-base object differs")
    found = {x: [] for x in xs}
    with gzip.open(object_path, "rt") as stream:
        header = json.loads(stream.readline())
        require(header["curve_a"] == 0 and header["curve_b"] == "1", "archive curve differs")
        require(header["point_count"] == 339968 and header["orbit_columns"] == 2048, "archive base size differs")
        require(int(header["subgroup_order"]) == ORDER, "archive subgroup differs")
        reps = [point_of(row) for row in header["representatives"]]
        for line in stream:
            row = json.loads(line)
            point = point_of(row["point"])
            if point[0] in found:
                require(on_curve(point), "stored point fails curve equation")
                found[point[0]].append((point, row))
    require(len(reps) == 2048 and all(found.values()), "SAT x coordinate missing from archive")
    require(all(len(points) <= 2 for points in found.values()), "more than two archive lifts for an x coordinate")
    chosen = None
    for combination in product(*(found[x] for x in xs)):
        partial = None
        valid = True
        for index, (point, row) in enumerate(combination):
            column = row["column"]
            coefficient = int(row["coefficient"]) % ORDER
            require(0 <= column < len(reps), "relation column out of range")
            if scalar(reps[column], coefficient) != point:
                valid = False
                break
            partial = add(partial, point)
            if index in (1, 2, 3) and (partial is None or partial[0] != intermediate_xs[index - 1]):
                valid = False
                break
        if valid and partial == target:
            chosen = combination
            break
    require(chosen is not None, "SAT model has no checked relation-row lift")
    require(all(scalar(point, ORDER) is None for point, _ in chosen), "relation point outside subgroup")
    report = {
        "schema": "n83.k2048-m5-positive-replay/v1", "status": "PASS_ORIGINAL_XCNF_GROUP_AND_ROW_REPLAY",
        "source_commit": LOCK["source_commit"], "xcnf_sha256": LOCK["xcnf_sha256"],
        "solver_stdout_sha256": receipt["stdout_sha256"], "object_sha256": OBJECT_SHA256,
        "model_values_sha256": hashlib.sha256(bytes(values)).hexdigest(),
        "reference_xcnf_verifier_sha256": sha256(REFERENCE),
        "checked_cnf_clauses": cnf, "checked_xor_rows": xor,
        "x_coordinates": list(map(str, xs)), "intermediate_x_coordinates": list(map(str, intermediate_xs)),
        "target": list(map(str, target)),
        "chosen_points": [list(map(str, point)) for point, _ in chosen],
        "columns": [row["column"] for _, row in chosen],
        "coefficients": [row["coefficient"] for _, row in chosen],
        "subgroup_order": str(ORDER), "selected_best_total_runtime": None,
    }
    path = RAW / "solver-positive/replay.json"
    with path.open("x") as stream:
        json.dump(report, stream, sort_keys=True, indent=2)
        stream.write("\n")
    print(json.dumps({"status": report["status"], "cnf": cnf, "xor": xor, "columns": report["columns"]}, sort_keys=True))


if __name__ == "__main__":
    main()
