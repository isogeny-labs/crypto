#!/usr/bin/env python3
"""Independently replay the modular relation algebra in a Magma receipt."""

import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent
R = 44735531


def scalar(lines: list[str], name: str) -> int:
    matches = [line for line in lines if line.startswith(name + " ")]
    if len(matches) != 1:
        raise ValueError(f"expected one {name}, got {len(matches)}")
    return int(matches[0].split(" ", 1)[1])


def solve(rows: list[list[int]], values: list[int], width: int) -> tuple[list[int], int]:
    matrix = [[sum(j == col for j in row) % R for col in range(1, width + 1)] + [value % R]
              for row, value in zip(rows, values, strict=True)]
    pivots: list[int] = []
    pivot_row = 0
    for col in range(width):
        found = next((i for i in range(pivot_row, len(matrix)) if matrix[i][col]), None)
        if found is None:
            continue
        matrix[pivot_row], matrix[found] = matrix[found], matrix[pivot_row]
        inverse = pow(matrix[pivot_row][col], -1, R)
        matrix[pivot_row] = [x * inverse % R for x in matrix[pivot_row]]
        for i in range(len(matrix)):
            if i != pivot_row and matrix[i][col]:
                multiplier = matrix[i][col]
                matrix[i] = [(x - multiplier * y) % R
                             for x, y in zip(matrix[i], matrix[pivot_row], strict=True)]
        pivots.append(col)
        pivot_row += 1
    if any(not any(row[:width]) and row[width] for row in matrix):
        raise ValueError("inconsistent relation system")
    if len(pivots) != width:
        raise ValueError(f"rank {len(pivots)} below factor-base size {width}")
    logs = [0] * width
    for i, col in enumerate(pivots):
        logs[col] = matrix[i][width]
    for row, value in zip(rows, values, strict=True):
        if sum(logs[j - 1] for j in row) % R != value % R:
            raise AssertionError("relation replay failed")
    return logs, len(pivots)


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("usage: audit_receipt.py RECEIPT_JSON")
    path = Path(sys.argv[1])
    receipt = json.loads(path.read_text())
    response = receipt["response"]
    if receipt["http_status"] != 200 or not response.get("success"):
        raise ValueError("Magma attempt was not successful")
    lines = response["stdout"].splitlines()
    relations = []
    for line in lines:
        match = re.fullmatch(r"relation (\d+) \[ (.*?) \]", line)
        if match:
            indices = [int(x) for x in match.group(2).split(", ")]
            relations.append((int(match.group(1)), indices))
    width = scalar(lines, "factor_base_classes")
    if scalar(lines, "target_jacobian_order") != 108 * R:
        raise AssertionError("target Jacobian order is not 108r")
    projected_count = scalar(lines, "factor_base_projected_checks")
    if projected_count != width:
        raise AssertionError("missing exact factor-base projection checks")
    if scalar(lines, "factor_base_in_source_subgroup_count") != 0:
        raise AssertionError("unexpected factor-base subgroup count")
    if len(relations) != scalar(lines, "smooth_relations"):
        raise AssertionError("relation count mismatch")
    if not all(1 <= j <= width for _, row in relations for j in row):
        raise AssertionError("factor-base index outside range")
    logs, rank = solve([row for _, row in relations], [value for value, _ in relations], width)
    if rank != scalar(lines, "linear_rank"):
        raise AssertionError("rank mismatch")
    descent = next(line for line in lines if line.startswith("descent_indices "))
    match = re.fullmatch(r"descent_indices \[ (.*?) \]", descent)
    if match is None:
        raise ValueError("missing descent indices")
    indices = [int(x) for x in match.group(1).split(", ")]
    offset = scalar(lines, "descent_attempts") - 1
    recovered = (sum(logs[j - 1] for j in indices) - offset) % R
    if recovered != scalar(lines, "ghs_scalar") or recovered != scalar(lines, "rho_scalar"):
        raise AssertionError("target recovery mismatch")
    result = {
        "receipt_sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
        "relation_count": len(relations),
        "factor_base_size": width,
        "rank": rank,
        "factor_base_projected_coordinates_mod_44735531": logs,
        "target_jacobian_order": 108 * R,
        "exact_projection_checks": projected_count,
        "descent_offset": offset,
        "descent_indices": indices,
        "recovered_scalar": recovered,
        "scope": "independent modular-algebra replay; divisor reconstruction and 108-cofactor projection are checked in Magma",
    }
    output = ROOT / "receipts" / f"{path.stem}-audit.json"
    if output.exists():
        raise SystemExit(f"refusing to overwrite {output}")
    output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
