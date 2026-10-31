"""Independent external-XCNF assignment, original-field, and custody audit."""

import argparse
import gzip
import hashlib
import json
from pathlib import Path

import run

ROOT = Path(__file__).resolve().parent
POLY = (1 << 83) | (1 << 45) | (1 << 2) | (1 << 1) | 1
MASK = (1 << 83) - 1
ORDER = 2417851639230796216685689
VARS = 329_726
XORS = 3_901
CLAUSES = {"positive": 2_807_752, "natural": 2_807_254}
TARGET_NATURAL = (1834913121178197242856238, 8797964190732846213981377)


def multiply(left, right):
    out = 0
    while right:
        if right & 1:
            out ^= left
        right >>= 1
        left <<= 1
        if left & (1 << 83):
            left ^= POLY
    return out


def inverse(value):
    if not value:
        raise ZeroDivisionError
    left, right = value, POLY
    left_coeff, right_coeff = 1, 0
    while left != 1:
        shift = left.bit_length() - right.bit_length()
        if shift < 0:
            left, right = right, left
            left_coeff, right_coeff = right_coeff, left_coeff
            shift = -shift
        left ^= right << shift
        left_coeff ^= right_coeff << shift
    return left_coeff & MASK


def add(left, right):
    if left is None:
        return right
    if right is None:
        return left
    x1, y1 = left
    x2, y2 = right
    if x1 == x2:
        if y1 ^ y2 or not x1:
            return None
        slope = x1 ^ multiply(y1, inverse(x1))
        x3 = multiply(slope, slope) ^ slope
        y3 = multiply(x1, x1) ^ multiply(slope ^ 1, x3)
    else:
        slope = multiply(y1 ^ y2, inverse(x1 ^ x2))
        x3 = multiply(slope, slope) ^ slope ^ x1 ^ x2
        y3 = multiply(slope, x1 ^ x3) ^ x3 ^ y1
    return x3, y3


def scalar_mul(point, scalar):
    out = None
    while scalar:
        if scalar & 1:
            out = add(out, point)
        point = add(point, point)
        scalar >>= 1
    return out


def on_curve(point):
    x, y = point
    return multiply(y, y) ^ multiply(x, y) == multiply(multiply(x, x), x) ^ 1


def assignment(stdout, count, allow_missing=False):
    status = []
    values = bytearray(count + 1)
    seen = bytearray(count + 1)
    for line in stdout.open():
        if line.startswith("s "):
            status.append(line.strip()[2:])
        elif line.startswith("v "):
            for word in line[2:].split():
                literal = int(word)
                if literal == 0:
                    continue
                index = abs(literal)
                if not 1 <= index <= count or seen[index]:
                    raise ValueError("invalid or duplicate assignment variable")
                seen[index] = 1
                values[index] = literal > 0
    if not status and allow_missing:
        return "MISSING_TERMINAL_STATUS", values
    if len(status) != 1:
        raise ValueError("missing or repeated terminal solver status")
    if status[0] == "SATISFIABLE" and sum(seen) != count:
        raise ValueError("SAT assignment is incomplete")
    return status[0], values


def check_formula(path, values, mode):
    clauses = xors = 0
    with path.open() as stream:
        header = next(stream).split()
        if header != ["p", "cnf", str(VARS), str(CLAUSES[mode] + XORS)]:
            raise ValueError("XCNF header differs")
        for line in stream:
            words = line.split()
            if not words or words[-1] != "0":
                raise ValueError("malformed XCNF row")
            is_xor = words[0] == "x"
            literals = [int(word) for word in words[1 if is_xor else 0:-1]]
            if not literals or any(abs(lit) > VARS or lit == 0 for lit in literals):
                raise ValueError("empty or out-of-range XCNF row")
            truth = [bool(values[abs(lit)]) ^ (lit < 0) for lit in literals]
            if is_xor:
                xors += 1
                if sum(truth) % 2 != 1:
                    raise ValueError(f"XOR row {xors} rejected assignment")
            else:
                clauses += 1
                if not any(truth):
                    raise ValueError(f"CNF clause {clauses} rejected assignment")
    if (clauses, xors) != (CLAUSES[mode], XORS):
        raise ValueError("XCNF body counts differ")
    return {"clauses_checked": clauses, "xor_rows_checked": xors}


def summand_x(values):
    return [sum(int(values[1 + block * 83 + bit]) << bit for bit in range(83))
            for block in range(4)]


def replay_group(object_path, xs, target, mode):
    wanted = set(xs)
    candidates = {x: [] for x in wanted}
    with gzip.open(object_path, "rt") as stream:
        header = json.loads(next(stream))
        if (header["curve_a"] != 0 or header["curve_b"] != "1"
                or header["subgroup_order"] != str(ORDER)
                or header["orbit_columns"] != 32_643):
            raise ValueError("base header identity differs")
        generator = tuple(map(int, header["generator"]))
        if not on_curve(generator) or scalar_mul(generator, ORDER) is not None:
            raise ValueError("generator fails subgroup check")
        lam = int(header["frobenius_lambda"])
        if (lam * lam + lam + 2) % ORDER != 0:
            raise ValueError("Frobenius eigenvalue polynomial differs")
        if scalar_mul(generator, lam) != tuple(multiply(v, v) for v in generator):
            raise ValueError("Frobenius eigenvalue action differs")
        representatives = [tuple(map(int, point)) for point in header["representatives"]]
        if len(representatives) != 32_643:
            raise ValueError("representative count differs")
        for index, line in enumerate(stream):
            row = json.loads(line)
            x = int(row["point"][0])
            if x in wanted:
                point = tuple(map(int, row["point"]))
                if not on_curve(point) or scalar_mul(point, ORDER) is not None:
                    raise ValueError("factor-base point fails subgroup check")
                candidates[x].append((index, row, point))
            if mode == "positive" and index >= 663:
                break
    if any(not candidates[x] for x in wanted):
        raise ValueError("summand x absent from factor base")
    # For a fixed x on the binary curve there are at most two affine points.
    for x in wanted:
        unique = {}
        for item in candidates[x]:
            unique[item[2]] = item
        candidates[x] = list(unique.values())
        if len(candidates[x]) > 2:
            raise ValueError("more than two points share x")
    from itertools import product
    for choice in product(*(candidates[x] for x in xs)):
        total = None
        for _, _, point in choice:
            total = add(total, point)
        if total != target:
            continue
        if add(target, generator) == total:
            raise ValueError("altered-target control accepted")
        row = {}
        for _, item, point in choice:
            column = item["column"]
            sign = -1 if item["negative"] else 1
            coefficient = sign * pow(lam, item["phase"], ORDER) % ORDER
            if coefficient != int(item["coefficient"]):
                raise ValueError("signed-Frobenius coefficient differs")
            if scalar_mul(representatives[column], coefficient) != point:
                raise ValueError("signed-Frobenius label fails group replay")
            row[column] = (row.get(column, 0) + coefficient) % ORDER
        row = {key: value for key, value in row.items() if value}
        if not row or len(row) > 4 or any(not 0 <= key < 32_643 for key in row):
            raise ValueError("relation row is zero or invalid")
        row_sum = None
        for column, coefficient in row.items():
            row_sum = add(row_sum, scalar_mul(representatives[column], coefficient))
        if row_sum != target:
            raise ValueError("signed-Frobenius coefficient row misses target")
        first = next(iter(row))
        if add(row_sum, representatives[first]) == target:
            raise ValueError("altered-row control accepted")
        digest = hashlib.sha256(json.dumps(sorted(row.items()), separators=(",", ":")).encode()).hexdigest()
        return {"point_indices": [item[0] for item in choice],
                "point_coordinates": [list(map(str, item[2])) for item in choice],
                "coefficient_row_sha256": digest, "nonzero_columns": len(row),
                "rank_after_insert": 1, "altered_target_rejected": True}
    raise ValueError("no original-field signed-orbit point combination sums to target")


def audit(args):
    out = Path(args.output).resolve()
    identity = json.loads((out / "identity.json").read_text())
    source = Path(args.source).resolve()
    binary = Path(args.binary).resolve()
    cms = Path(args.cms).resolve()
    base = Path(args.base).resolve()
    if run.preflight(source, binary, cms, base) != identity:
        raise ValueError("identity preflight differs")
    mode = args.mode
    export_dir = out / ("export-" + mode)
    formula = run.require_export(export_dir, mode, identity)
    receipt = json.loads((out / ("solve-" + mode) / "supervisor.json").read_text())
    if receipt["source_commit"] != identity["source_commit"] or receipt["cms_sha256"] != run.CMS_SHA:
        raise ValueError("solver supervisor identity differs")
    status, values = assignment(out / ("solve-" + mode) / "stdout.txt", VARS,
                                allow_missing=receipt["outer_cap_reached"])
    export = json.loads((export_dir / "export.json").read_text())
    expected = (TARGET_NATURAL if mode == "natural" else tuple(map(int, export["target"])))
    if tuple(map(int, export["target"])) != expected:
        raise ValueError("export target differs")
    result = {"schema": "n83.d18-xcnf-independent-audit/v1", "mode": mode,
              "source_commit": identity["source_commit"], "cms_sha256": run.CMS_SHA,
              "exporter_sha256": identity["exporter_sha256"],
              "xcnf_sha256": export["xcnf_sha256"], "solver_status": status,
              "solver_exit_code": receipt["exit_code"],
              "outer_cap_reached": receipt["outer_cap_reached"],
              "selected_best_total_runtime": None}
    if status == "SATISFIABLE":
        if receipt["exit_code"] != 10 or receipt["outer_cap_reached"]:
            raise ValueError("SAT exit or watchdog differs")
        result.update(check_formula(formula, values, mode))
        xs = summand_x(values)
        result["summand_x"] = list(map(str, xs))
        result.update(replay_group(base / "objects" / run.OBJECT, xs, expected, mode))
        if mode == "positive" and result["point_indices"] != [0, 166, 332, 498]:
            raise ValueError("constructed witness differs")
        result["status"] = "PASS_CONSTRUCTED_XCNF_SAT" if mode == "positive" else "PASS_NATURAL_XCNF_RELATION"
    elif status in ("INDETERMINATE", "UNKNOWN", "MISSING_TERMINAL_STATUS") and receipt["exit_code"] in (0, 15, 124, -9):
        result["status"] = "UNKNOWN_EXTERNAL_SOLVER_CAP"
    else:
        result["status"] = "INVALID_OR_UNRESOLVED_EXTERNAL_SOLVER"
    path = out / (mode + "-audit.json")
    with path.open("x") as stream:
        json.dump(result, stream, sort_keys=True, indent=2)
        stream.write("\n")
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--mode", choices=("positive", "natural"), required=True)
    for name in ("source", "binary", "cms", "base", "output"):
        parser.add_argument("--" + name, required=True)
    print(json.dumps(audit(parser.parse_args()), sort_keys=True, indent=2))
