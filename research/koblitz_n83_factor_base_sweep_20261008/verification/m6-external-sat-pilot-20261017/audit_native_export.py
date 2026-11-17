#!/usr/bin/env python3
"""Independently verify a native SAT assignment against the frozen XOR-DIMACS."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import sys

import blake3

HERE = Path(__file__).resolve().parent
STUDY = HERE.parents[1]
FIXTURE_DIR = STUDY / "verification/hosted-m6-witness-ladder-20261017"
sys.path[:0] = [str(FIXTURE_DIR), str(STUDY)]
import prepare  # noqa: E402
from chain_search_supervisor import field_mul  # noqa: E402

EXPECTED_DIMACS_BLAKE3 = "9e5c9d22ef57ba4640d2d67d02fbdb39501f0de9f2b779bb818a919207031e6f"
EXPECTED_DIMACS_BYTES = 11641543
EXPECTED_VARIABLES = 118684
EXPECTED_CLAUSES = 548445
EXPECTED_XORS = 6059


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def s3_residual(x: int, y: int, z: int) -> int:
    xy = field_mul(x, y)
    return field_mul(xy, z) ^ field_mul(xy ^ field_mul(x, z) ^ field_mul(y, z),
                                     xy ^ field_mul(x, z) ^ field_mul(y, z)) ^ 1


def audit(dimacs: Path, model_bits: Path, bases: Path) -> dict:
    require(dimacs.stat().st_size == EXPECTED_DIMACS_BYTES
            and blake3.blake3(dimacs.read_bytes()).hexdigest() == EXPECTED_DIMACS_BLAKE3,
            "DIMACS identity differs from three external calls")
    bits = model_bits.read_bytes()
    require(len(bits) == EXPECTED_VARIABLES + 1 and bits.endswith(b"\n")
            and set(bits[:-1]) <= {ord("0"), ord("1")},
            "native model is incomplete or noncanonical")
    assignment = [byte == ord("1") for byte in bits[:-1]]
    clauses = xors = 0
    with dimacs.open() as stream:
        header = stream.readline().split()
        require(header == ["p", "cnf", str(EXPECTED_VARIABLES),
                           str(EXPECTED_CLAUSES + EXPECTED_XORS)],
                "DIMACS header differs")
        for line_number, line in enumerate(stream, 2):
            xor = line.startswith("x")
            values = [int(value) for value in (line[1:] if xor else line).split()]
            require(values and values[-1] == 0, "unterminated DIMACS row")
            literals = values[:-1]
            require(all(1 <= abs(value) <= EXPECTED_VARIABLES for value in literals),
                    "DIMACS literal outside declared variables")
            truth = [assignment[abs(value) - 1] != (value < 0)
                     for value in literals]
            require((sum(truth) % 2 == 1) if xor else any(truth),
                    f"native model violates DIMACS line {line_number}")
            xors += int(xor)
            clauses += int(not xor)
    require((clauses, xors) == (EXPECTED_CLAUSES, EXPECTED_XORS),
            "DIMACS row counts differ")
    blocks = [sum(int(assignment[i * 83 + bit]) << bit for bit in range(83))
              for i in range(11)]
    fixture = json.loads((FIXTURE_DIR / "fixture.json").read_text())
    require(prepare.build(bases) == fixture, "archived full-point fixture differs")
    arm = next(row for row in fixture["arms"] if row["role"] == "gray_pool")
    require(blocks[:6] == list(map(int, arm["ordered_xs"]))
            and blocks[6:10] == list(map(int, arm["partial_xs"]))
            and blocks[10] == int(arm["target_x"]),
            "native model x tuple differs from group witness")
    nodes = ((blocks[0], blocks[1], blocks[6]),
             (blocks[6], blocks[2], blocks[7]),
             (blocks[7], blocks[3], blocks[8]),
             (blocks[8], blocks[4], blocks[9]),
             (blocks[9], blocks[5], blocks[10]))
    require(all(s3_residual(*node) == 0 for node in nodes),
            "native model violates original-field S3")
    return {"schema": "n83.m6-native-export-audit/v1",
            "status": "PASS_NATIVE_MODEL_SATISFIES_EXPORTED_XOR_DIMACS",
            "dimacs_sha256": sha256(dimacs),
            "dimacs_blake3": EXPECTED_DIMACS_BLAKE3,
            "model_bits_sha256": sha256(model_bits),
            "model_bits_blake3": blake3.blake3(bits).hexdigest(),
            "variables": EXPECTED_VARIABLES, "clauses_checked": clauses,
            "xor_rows_checked": xors, "original_s3_nodes_checked": 5,
            "full_group_witness_replayed": True,
            "selected_best_total_runtime": None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dimacs", type=Path, required=True)
    parser.add_argument("--model-bits", type=Path, required=True)
    parser.add_argument("--bases", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.dimacs, args.model_bits, args.bases),
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
