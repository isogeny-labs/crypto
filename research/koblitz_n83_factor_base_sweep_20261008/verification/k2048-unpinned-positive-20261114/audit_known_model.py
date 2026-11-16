#!/usr/bin/env python3
"""Check the archived pinned-positive model on the new unpinned XCNF."""

import argparse
import hashlib
import json
from pathlib import Path


EXPECTED_PINNED_SHA = "f46a900495ec1a3594add973ee641de21a318a8fda6c9bfa67bd19057326d44c"
EXPECTED_TARGET = ["985527933806849610456454", "7623568253424067907890851"]
EXPECTED_INDICES = [0, 166, 332, 498, 664]


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def read_assignment(path, variables):
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
                    if not 1 <= index <= variables or values[index]:
                        raise ValueError("model variable range or duplication")
                    values[index] = 1 if literal > 0 else 2
    if statuses != ["s SATISFIABLE"] or 0 in values[1:]:
        raise ValueError("archived model is absent or incomplete")
    return values


def check_formula(path, values):
    cnf = xor = 0
    with path.open(encoding="ascii") as stream:
        header = stream.readline().split()
        if header[:2] != ["p", "cnf"] or len(header) != 4:
            raise ValueError("XCNF header differs")
        variables, rows = map(int, header[2:])
        if variables != len(values) - 1:
            raise ValueError("XCNF variable count differs")
        for line in stream:
            if line.startswith("c "):
                continue
            is_xor = line.startswith("x ")
            literals = [int(token) for token in line[2 if is_xor else 0 :].split()]
            if len(literals) < 2 or literals[-1] != 0:
                raise ValueError("XCNF row terminator differs")
            parity = False
            satisfied = False
            for literal in literals[:-1]:
                index = abs(literal)
                if not 1 <= index <= variables:
                    raise ValueError("XCNF literal outside model")
                truth = (values[index] == 1) == (literal > 0)
                parity ^= truth
                satisfied |= truth
            if is_xor:
                xor += 1
                if not parity:
                    raise ValueError(f"XOR row {xor} fails")
            else:
                cnf += 1
                if not satisfied:
                    raise ValueError(f"CNF row {cnf} fails")
    if cnf + xor != rows:
        raise ValueError("XCNF header row count differs")
    return cnf, xor


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--unpinned-export", type=Path, required=True)
    parser.add_argument("--pinned-export", type=Path, required=True)
    parser.add_argument("--pinned-stdout", type=Path, required=True)
    parser.add_argument("--receipt", type=Path, required=True)
    args = parser.parse_args()
    pinned = json.loads(args.pinned_export.read_text())
    unpinned = json.loads(args.unpinned_export.read_text())
    if pinned["mode"] != "positive" or pinned["xcnf_sha256"] != EXPECTED_PINNED_SHA:
        raise ValueError("archived positive export differs")
    if unpinned["mode"] != "positive-unpinned" or unpinned["status"] != "PASS_XCNF_EXPORT_ONLY":
        raise ValueError("new export mode or status differs")
    if pinned["target"] != EXPECTED_TARGET or unpinned["target"] != EXPECTED_TARGET:
        raise ValueError("constructed target differs")
    if pinned["positive"]["indices"] != EXPECTED_INDICES or unpinned["positive"] != pinned["positive"]:
        raise ValueError("validation witness receipt differs")
    if unpinned["shape"]["positive_x_pins"] != 0:
        raise ValueError("new export still pins x coordinates")
    if pinned["object_sha256"] != unpinned["object_sha256"] or pinned["point_set_blake3"] != unpinned["point_set_blake3"]:
        raise ValueError("factor-base identity differs")
    formula = Path(unpinned["xcnf_path"])
    formula_sha = sha256(formula)
    if formula_sha != unpinned["xcnf_sha256"]:
        raise ValueError("new formula hash differs")
    values = read_assignment(args.pinned_stdout, unpinned["shape"]["variables"])
    cnf, xor = check_formula(formula, values)
    if (cnf, xor) != (unpinned["shape"]["clauses"], unpinned["shape"]["xor_rows"]):
        raise ValueError("new formula row counts differ from receipt")
    receipt = {
        "schema": "n83.k2048-unpinned-known-model-audit/v1",
        "status": "PASS_ORIGINAL_XCNF",
        "source_commit": unpinned["source_commit"],
        "xcnf_sha256": formula_sha,
        "archived_positive_sha256": EXPECTED_PINNED_SHA,
        "archived_stdout_sha256": sha256(args.pinned_stdout),
        "target": EXPECTED_TARGET,
        "checked_cnf": cnf,
        "checked_xor": xor,
        "assigned_variables": len(values) - 1,
    }
    with args.receipt.open("x", encoding="utf-8") as stream:
        json.dump(receipt, stream, sort_keys=True, indent=2)
        stream.write("\n")
    print(json.dumps(receipt, sort_keys=True))


if __name__ == "__main__":
    main()
