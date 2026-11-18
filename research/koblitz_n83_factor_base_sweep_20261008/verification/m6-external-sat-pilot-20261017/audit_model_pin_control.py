#!/usr/bin/env python3
"""Independently inspect the N83 CryptoMiniSat complete-model control."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re

N = 118_684
ROWS = 548_445 + 6_059
EXPECTED_ORIGINAL_SHA256 = "c62fe682879435e8f1ee30a80ac7830f2aef81dd4f600d10a004ac45a4f914b8"
EXPECTED_MODEL_SHA256 = "ad95e05e171740aa08efb39bf2b28fd9c7c501f3f5ee4e7c5cced3c9a1c725d9"
EXPECTED_CMS_SHA256 = "a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac"


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def require(ok: bool, message: str) -> None:
    if not ok:
        raise ValueError(message)


def audit(original_path: Path, model_path: Path, raw: Path) -> dict:
    original = original_path.read_bytes()
    model = model_path.read_bytes()
    require(sha256(original) == EXPECTED_ORIGINAL_SHA256, "original export differs")
    require(sha256(model) == EXPECTED_MODEL_SHA256, "native model differs")
    require(model.endswith(b"\n") and len(model) == N + 1, "model length differs")
    receipt = json.loads((raw / "result.json").read_text())
    require(receipt["schema"] == "n83.m6-external-model-pin-control/v1"
            and receipt["status"] == "PASS_EXTERNAL_XOR_SEMANTICS"
            and receipt["cms_sha256"] == EXPECTED_CMS_SHA256
            and receipt["conservative_call_envelopes_seconds"] == 300
            and receipt["selected_best_total_runtime"] is None,
            "control receipt contract differs")
    header, sep, original_rows = original.partition(b"\n")
    require(sep == b"\n" and header == f"p cnf {N} {ROWS}".encode(),
            "original header differs")
    require(model[0] == ord("0") and b"-1 0" in original_rows.splitlines(),
            "first-bit contradiction is not an explicit original unit")
    expected_new_header = f"p cnf {N} {ROWS + N}".encode()
    checked = {}
    for role, file_stem, expected_status, expected_exit, flip in (
        ("complete_model", "complete-model", "SATISFIABLE", 10, False),
        ("flipped_bit_1", "flipped-bit-1", "UNSATISFIABLE", 20, True),
    ):
        call = receipt["calls"][role]
        data = (raw / f"{file_stem}.xor.cnf").read_bytes()
        new_header, sep, rest = data.partition(b"\n")
        require(sep == b"\n" and new_header == expected_new_header,
                f"{role} header differs")
        require(rest.startswith(original_rows), f"{role} original rows changed")
        units = rest[len(original_rows):].splitlines()
        require(len(units) == N, f"{role} unit count differs")
        for index, (bit, unit) in enumerate(zip(model[:-1], units), 1):
            expected_bit = (bit == ord("1")) ^ (flip and index == 1)
            expected = f"{index if expected_bit else -index} 0".encode()
            require(unit == expected, f"{role} unit {index} differs")
        stdout = (raw / f"{file_stem}.stdout").read_bytes()
        stderr = (raw / f"{file_stem}.stderr").read_bytes()
        statuses = re.findall(rb"(?m)^s (SATISFIABLE|UNSATISFIABLE|INDETERMINATE|UNKNOWN)$",
                              stdout)
        rss = re.findall(rb"(?m)^\s*(\d+)\s+maximum resident set size$", stderr)
        require(statuses == [expected_status.encode()] and
                call["statuses"] == [expected_status] and
                call["exit_code"] == expected_exit and
                call["timed_out"] is False and
                len(rss) == 1 and int(rss[0]) == call["peak_rss_bytes_L0"] and
                0 < call["wall_seconds_L0"] < 150 and
                call["input_sha256"] == sha256(data) and
                call["stdout_sha256"] == sha256(stdout) and
                call["stderr_sha256"] == sha256(stderr) and
                call["input_bytes"] == len(data) and
                call["command"][-1] == str(raw / f"{file_stem}.xor.cnf") and
                call["command"][2:-1] == ["/opt/homebrew/bin/cryptominisat5",
                                          "--threads", "1", "--verb", "0",
                                          "--printsol", "0", "--maxtime", "120",
                                          "--maxconfl", "5000"],
                f"{role} solver receipt differs")
        checked[role] = {"status": expected_status, "exit_code": expected_exit,
                         "input_sha256": sha256(data),
                         "wall_seconds_L0": call["wall_seconds_L0"],
                         "peak_rss_bytes_L0": int(rss[0])}
    rows = [[path.name, sha256(path.read_bytes())]
            for path in sorted(raw.iterdir()) if path.is_file() and path.name != "result.json"]
    require(rows == receipt["raw_files"] and
            sha256(json.dumps(rows, separators=(",", ":")).encode()) ==
            receipt["raw_tree_sha256"], "raw tree differs")
    return {"schema": "n83.m6-external-model-pin-audit/v1",
            "status": "PASS_DERIVED_INPUTS_AND_SOLVER_OUTPUTS",
            "original_dimacs_sha256": EXPECTED_ORIGINAL_SHA256,
            "native_model_sha256": EXPECTED_MODEL_SHA256,
            "raw_tree_sha256": receipt["raw_tree_sha256"],
            "original_rows_unchanged": ROWS,
            "unit_rows_checked_per_arm": N,
            "calls": checked, "selected_best_total_runtime": None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dimacs", type=Path, required=True)
    parser.add_argument("--model-bits", type=Path, required=True)
    parser.add_argument("--raw", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.dimacs, args.model_bits, args.raw),
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
