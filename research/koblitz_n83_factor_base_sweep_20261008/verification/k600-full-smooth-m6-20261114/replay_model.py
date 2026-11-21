#!/usr/bin/env python3
"""Replay a K600 m6 SAT model against its original formula and group relation."""

import argparse
from itertools import product
import gzip
import hashlib
import json
from pathlib import Path
from tempfile import TemporaryDirectory


N = 83
MOD = (1 << N) | (1 << 45) | 7
ORDER = 2417851639230796216685689
CURVE = "icv1-f2m83-tm6151469093347-debefd74"
OBJECT = "objects/b4dda45e6c3edfca673db6f3f82c16cce8c860f5e62c092155258df399a0363e.jsonl.gz"
OBJECT_SHA256 = "3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf"
CORPUS_SHA256 = "760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd"


def require(condition, message):
    if not condition:
        raise ValueError(message)


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def mul(a, b):
    result = 0
    while b:
        if b & 1:
            result ^= a
        b >>= 1
        a <<= 1
        if a >> N:
            a ^= MOD
    return result


def inv(a):
    require(a != 0, "zero has no inverse")
    u, v, g, h = a, MOD, 1, 0
    while u != 1:
        require(u != 0, "field inverse failed")
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


def scalar(point, coefficient):
    result = None
    while coefficient:
        if coefficient & 1:
            result = add(result, point)
        point = add(point, point)
        coefficient >>= 1
    return result


def point_of(row):
    require(isinstance(row, list) and len(row) == 2, "point encoding differs")
    point = tuple(map(int, row))
    require(all(0 <= word < (1 << N) for word in point), "point coordinate out of field")
    return point


def read_model(path, variables):
    values = bytearray(variables + 1)
    statuses = []
    with path.open(encoding="ascii") as stream:
        for line in stream:
            if line.startswith("s "):
                statuses.append(line.strip())
            elif line.startswith("v "):
                for token in line.split()[1:]:
                    literal = int(token)
                    if literal == 0:
                        continue
                    index = abs(literal)
                    require(1 <= index <= variables and values[index] == 0,
                            "model variable out of range or duplicated")
                    values[index] = 1 if literal > 0 else 2
    require(statuses == ["s SATISFIABLE"] and all(values[1:]),
            "solver model absent, partial or not SAT")
    return values


def check_formula(path, values, expected_cnf, expected_xor):
    cnf = xor = 0
    with path.open(encoding="ascii") as stream:
        header = stream.readline().split()
        require(header == ["p", "cnf", str(len(values) - 1),
                           str(expected_cnf + expected_xor)], "XCNF header differs")
        for line_number, line in enumerate(stream, start=2):
            if line.startswith("c "):
                continue
            is_xor = line.startswith("x ")
            tokens = line[2:].split() if is_xor else line.split()
            require(tokens and tokens[-1] == "0",
                    f"XCNF row {line_number} is unterminated")
            literals = [int(token) for token in tokens[:-1]]
            require(all(1 <= abs(lit) < len(values) for lit in literals),
                    f"XCNF row {line_number} has an invalid literal")
            truths = [((values[abs(lit)] == 1) == (lit > 0)) for lit in literals]
            if is_xor:
                xor += 1
                require(sum(truths) % 2 == 1, f"XOR row {line_number} fails")
            else:
                cnf += 1
                require(any(truths), f"CNF row {line_number} fails")
    require((cnf, xor) == (expected_cnf, expected_xor),
            "XCNF row counts differ")
    return cnf, xor


def replay_group(object_path, xs, intermediates, target, positive):
    found = {x: [] for x in xs}
    indices = positive["indices"] if positive is not None else []
    require(not indices or (len(indices) == 6
            and len(set(indices)) == 6
            and all(isinstance(index, int) and 0 <= index < 99600
                    for index in indices)), "positive archive indices differ")
    indexed = {}
    with gzip.open(object_path, "rt") as stream:
        header = json.loads(stream.readline())
        require(header["curve_slug"] == CURVE and header["curve_a"] == 0
                and header["curve_b"] == "1" and header["n"] == N,
                "factor-base curve differs")
        require(header["closure"] == "signed_frobenius"
                and header["policy"] == "public_x_hash"
                and header["seed"] == 2026100801
                and header["point_count"] == 99600
                and header["orbit_columns"] == 600
                and int(header["subgroup_order"]) == ORDER,
                "factor-base parameters differ")
        representatives = [point_of(row) for row in header["representatives"]]
        require(len(representatives) == 600, "representative count differs")
        count = 0
        for line in stream:
            row = json.loads(line)
            if count in indices:
                indexed[count] = point_of(row["point"])
            count += 1
            point = point_of(row["point"])
            if point[0] in found:
                require(on_curve(point), "stored point fails curve equation")
                found[point[0]].append((point, row))
        require(count == 99600, "archive point count differs")
    if positive is not None:
        require(len(indexed) == 6
                and len(positive["points"]) == 6
                and len(positive["partial_sums"]) == 6,
                "positive witness length differs")
        partial = None
        for index, declared_point, declared_sum in zip(
            indices, positive["points"], positive["partial_sums"]
        ):
            point = point_of(declared_point)
            require(point == indexed[index] and on_curve(point)
                    and scalar(point, ORDER) is None,
                    "positive point differs from subgroup archive point")
            partial = add(partial, point)
            require(partial is not None and partial == point_of(declared_sum),
                    "positive partial sum differs")
        require(partial == target, "positive witness target differs")
    require(all(1 <= len(candidates) <= 2 for candidates in found.values()),
            "model x-coordinate has no unique archive lifts")
    valid = {}
    for x, candidates in found.items():
        valid[x] = []
        for point, row in candidates:
            column = row["column"]
            coefficient = int(row["coefficient"]) % ORDER
            require(0 <= column < 600, "relation column out of range")
            require(on_curve(representatives[column]), "representative off curve")
            require(scalar(point, ORDER) is None, "relation point outside subgroup")
            require(scalar(representatives[column], coefficient) == point,
                    "archive point/column coefficient mismatch")
            valid[x].append((point, column, coefficient))
    for combination in product(*(valid[x] for x in xs)):
        partial = None
        for index, (point, _, _) in enumerate(combination):
            partial = add(partial, point)
            if index in (1, 2, 3, 4) and (
                partial is None or partial[0] != intermediates[index - 1]
            ):
                break
        else:
            if partial == target:
                return combination
    raise ValueError("SAT model has no checked six-point relation lift")


def replay(args):
    export = json.loads(args.export.read_text())
    receipt = json.loads(args.solver_receipt.read_text())
    require(export["schema"] == "n83.k600-m6-xcnf-export/v1"
            and export["status"] == "PASS_XCNF_EXPORT_ONLY"
            and export["mode"] in ("positive-pinned", "positive-unpinned", "natural"),
            "K600 export identity differs")
    require(export["curve_id"] == CURVE and export["curve_a"] == 0
            and export["source_commit"] == args.source_commit
            and export["source_attestation_mode"] == "clean_checkout"
            and export["binary_sha256"] == args.binary_sha256,
            "export source, binary or curve differs")
    require(export["object"] == OBJECT
            and export["object_sha256"] == OBJECT_SHA256,
            "export factor-base identity differs")
    shape = export["shape"]
    require(shape["summands"] == 6 and shape["identity_mask"] == 0
            and shape["root_encoding"] == "per_representative"
            and shape["domain_representatives"] == 600
            and shape["summand_ordering"] == "none"
            and shape["positive_x_pins"] == (
                913 if export["mode"] == "positive-pinned" else 0
            ), "XCNF shape or search policy differs")
    require(receipt["exit_code"] == 10
            and receipt["status"] in ("SAT_PENDING_REPLAY", "SAT_PENDING_XCNF_REPLAY")
            and receipt["xcnf_sha256"] == export["xcnf_sha256"],
            "solver did not report SAT on the exported formula")
    require(sha256(args.xcnf) == export["xcnf_sha256"]
            and args.xcnf.stat().st_size == export["xcnf_bytes"],
            "original XCNF bytes differ")
    require(sha256(args.solver_stdout) == receipt["stdout_sha256"],
            "solver output bytes differ")
    object_path = args.input_root / OBJECT
    require(sha256(object_path) == OBJECT_SHA256,
            "retained factor-base bytes differ")
    target = point_of(export["target"])
    require(on_curve(target) and scalar(target, ORDER) is None,
            "target curve or subgroup check fails")
    if export["mode"] == "natural":
        require(export["positive"] is None, "natural export has a positive witness")
        corpus = args.input_root / "probe-corpus.json"
        require(sha256(corpus) == CORPUS_SHA256
                and export["public_fixture"] == 0, "public corpus differs")
        targets = json.loads(corpus.read_text())["targets"]
        fixture = [row for row in targets if row["a"] == 0 and row["fixture"] == 0]
        require(len(fixture) == 1 and point_of(fixture[0]["point"]) == target,
                "public fixture point differs")
    else:
        require(export["positive"] is not None, "positive export witness missing")
    values = read_model(args.solver_stdout, shape["variables"])
    cnf, xor = check_formula(args.xcnf, values, shape["clauses"], shape["xor_rows"])
    xs = [sum(1 << bit for bit in range(N)
              if values[block * N + bit + 1] == 1)
          for block in range(6)]
    intermediates = [sum(1 << bit for bit in range(N)
                         if values[block * N + bit + 1] == 1)
                     for block in range(6, 10)]
    require(all(0 <= x < (1 << N) for x in xs + intermediates),
            "model field coordinate out of range")
    require(all((values[10 * N + bit + 1] == 1) == bool(target[0] & (1 << bit))
                for bit in range(N)), "model target block differs")
    chosen = replay_group(object_path, xs, intermediates, target,
                          export["positive"])
    report = {
        "schema": "n83.k600-m6-model-replay/v1",
        "status": "PASS_ORIGINAL_XCNF_GROUP_AND_ROW_REPLAY",
        "mode": export["mode"],
        "source_commit": args.source_commit,
        "binary_sha256": args.binary_sha256,
        "xcnf_sha256": export["xcnf_sha256"],
        "solver_stdout_sha256": receipt["stdout_sha256"],
        "object_sha256": OBJECT_SHA256,
        "model_values_sha256": hashlib.sha256(values).hexdigest(),
        "checked_cnf_clauses": cnf,
        "checked_xor_rows": xor,
        "target": list(map(str, target)),
        "x_coordinates": list(map(str, xs)),
        "intermediate_x_coordinates": list(map(str, intermediates)),
        "chosen_points": [list(map(str, point)) for point, _, _ in chosen],
        "columns": [column for _, column, _ in chosen],
        "coefficients": [str(coefficient) for _, _, coefficient in chosen],
        "subgroup_order": str(ORDER),
        "selected_best_total_runtime": None,
    }
    with args.output.open("x") as stream:
        json.dump(report, stream, sort_keys=True, indent=2)
        stream.write("\n")
    print(json.dumps({"status": report["status"], "cnf": cnf, "xor": xor},
                     sort_keys=True))


def selftest():
    point = (6322576162995772655031682, 7602502822968073983643739)
    require(on_curve(point) and scalar(point, ORDER) is None,
            "field/group selftest failed")
    require(add(point, (point[0], point[0] ^ point[1])) is None,
            "point inverse selftest failed")
    with TemporaryDirectory() as folder:
        formula = Path(folder) / "small.xcnf"
        formula.write_text("p cnf 3 3\n1 -2 0\n3 0\nx -1 2 3 0\n")
        model = Path(folder) / "model.txt"
        model.write_text("s SATISFIABLE\nv 1 -2 3 0\n")
        good = bytearray((0, 1, 2, 1))
        require(read_model(model, 3) == good, "model parser selftest failed")
        require(check_formula(formula, good, 2, 1) == (2, 1),
                "formula selftest failed")
        model.write_text("s SATISFIABLE\nv 1 -2 0\n")
        try:
            read_model(model, 3)
        except ValueError:
            pass
        else:
            raise AssertionError("partial SAT model passed")
        for bad in (bytearray((0, 2, 2, 1)), bytearray((0, 1, 1, 1))):
            try:
                check_formula(formula, bad, 2, 1)
            except ValueError:
                continue
            raise AssertionError("invalid formula assignment passed")
    print("K600 replay arithmetic and XCNF selftest PASS")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--selftest", action="store_true")
    parser.add_argument("--source-commit")
    parser.add_argument("--binary-sha256")
    parser.add_argument("--input-root", type=Path)
    parser.add_argument("--export", type=Path)
    parser.add_argument("--xcnf", type=Path)
    parser.add_argument("--solver-receipt", type=Path)
    parser.add_argument("--solver-stdout", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    if args.selftest:
        selftest()
        return
    require(all(getattr(args, name) is not None for name in (
        "source_commit", "binary_sha256", "input_root", "export", "xcnf",
        "solver_receipt", "solver_stdout", "output"
    )), "all replay bindings and paths are required")
    replay(args)


if __name__ == "__main__":
    main()
