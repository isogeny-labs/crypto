#!/usr/bin/env python3
"""Check CryptoMiniSat's semantics on a fully pinned N83 XOR-DIMACS witness."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import time

EXPECTED_DIMACS_SHA256 = "c62fe682879435e8f1ee30a80ac7830f2aef81dd4f600d10a004ac45a4f914b8"
EXPECTED_MODEL_SHA256 = "ad95e05e171740aa08efb39bf2b28fd9c7c501f3f5ee4e7c5cced3c9a1c725d9"
EXPECTED_CMS_SHA256 = "a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac"
VARIABLES = 118_684
ORIGINAL_ROWS = 548_445 + 6_059
CONFLICT_CAP = 5_000
SOLVER_WALL_CAP = 120
OUTER_WALL_CAP = 150


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(ok: bool, message: str) -> None:
    if not ok:
        raise ValueError(message)


def build(original: bytes, bits: bytes, path: Path, *, flip_first: bool) -> None:
    header = f"p cnf {VARIABLES} {ORIGINAL_ROWS}\n".encode()
    require(original.startswith(header), "original DIMACS header differs")
    new_header = f"p cnf {VARIABLES} {ORIGINAL_ROWS + VARIABLES}\n".encode()
    with path.open("xb") as output:
        output.write(new_header)
        output.write(original[len(header):])
        for index, bit in enumerate(bits[:-1], 1):
            value = bit == ord("1")
            if flip_first and index == 1:
                value = not value
            output.write(f"{index if value else -index} 0\n".encode())
        output.flush()
        os.fsync(output.fileno())


def run(cms: Path, path: Path, out: Path, name: str) -> dict:
    stdout_path = out / f"{name}.stdout"
    stderr_path = out / f"{name}.stderr"
    command = ["/usr/bin/time", "-l", str(cms), "--threads", "1",
               "--verb", "0", "--printsol", "0",
               "--maxtime", str(SOLVER_WALL_CAP),
               "--maxconfl", str(CONFLICT_CAP), str(path)]
    started = time.monotonic()
    with stdout_path.open("xb") as stdout, stderr_path.open("xb") as stderr:
        process = subprocess.Popen(command, stdout=stdout, stderr=stderr,
                                   start_new_session=True)
        try:
            exit_code = process.wait(timeout=OUTER_WALL_CAP)
            timed_out = False
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            process.wait()
            exit_code = process.returncode
            timed_out = True
    wall = time.monotonic() - started
    statuses = re.findall(r"(?m)^s (SATISFIABLE|UNSATISFIABLE|INDETERMINATE|UNKNOWN)$",
                          stdout_path.read_text(errors="replace"))
    rss_matches = re.findall(r"(?m)^\s*(\d+)\s+maximum resident set size$",
                             stderr_path.read_text(errors="replace"))
    return {"command": command, "input_sha256": sha256(path),
            "input_bytes": path.stat().st_size,
            "stdout_sha256": sha256(stdout_path),
            "stderr_sha256": sha256(stderr_path),
            "exit_code": exit_code, "timed_out": timed_out,
            "statuses": statuses, "wall_seconds_L0": wall,
            "peak_rss_bytes_L0": int(rss_matches[-1]) if rss_matches else None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dimacs", type=Path, required=True)
    parser.add_argument("--model-bits", type=Path, required=True)
    parser.add_argument("--cms", type=Path, default=Path("/opt/homebrew/bin/cryptominisat5"))
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    require(sha256(args.dimacs) == EXPECTED_DIMACS_SHA256,
            "original DIMACS hash differs")
    require(sha256(args.model_bits) == EXPECTED_MODEL_SHA256,
            "native assignment hash differs")
    require(sha256(args.cms) == EXPECTED_CMS_SHA256, "CMS binary hash differs")
    original = args.dimacs.read_bytes()
    bits = args.model_bits.read_bytes()
    require(len(bits) == VARIABLES + 1 and bits.endswith(b"\n")
            and set(bits[:-1]) <= {ord("0"), ord("1")},
            "native assignment is incomplete")
    args.output.mkdir(parents=True, exist_ok=False)
    positive = args.output / "complete-model.xor.cnf"
    negative = args.output / "flipped-bit-1.xor.cnf"
    build(original, bits, positive, flip_first=False)
    build(original, bits, negative, flip_first=True)
    calls = {"complete_model": run(args.cms, positive, args.output, "complete-model"),
             "flipped_bit_1": run(args.cms, negative, args.output, "flipped-bit-1")}
    raw_files = sorted(path for path in args.output.iterdir() if path.is_file())
    tree_rows = [[path.name, sha256(path)] for path in raw_files]
    tree_sha256 = hashlib.sha256(json.dumps(tree_rows, separators=(",", ":")).encode()).hexdigest()
    status = ("PASS_EXTERNAL_XOR_SEMANTICS" if
              calls["complete_model"]["statuses"] == ["SATISFIABLE"] and
              calls["complete_model"]["exit_code"] == 10 and
              calls["flipped_bit_1"]["statuses"] == ["UNSATISFIABLE"] and
              calls["flipped_bit_1"]["exit_code"] == 20 else
              "CONTROL_DID_NOT_PASS")
    result = {"schema": "n83.m6-external-model-pin-control/v1", "status": status,
              "original_dimacs_sha256": EXPECTED_DIMACS_SHA256,
              "native_model_sha256": EXPECTED_MODEL_SHA256,
              "cms_sha256": EXPECTED_CMS_SHA256,
              "variables_pinned": VARIABLES,
              "original_rows": ORIGINAL_ROWS,
              "derived_rows": ORIGINAL_ROWS + VARIABLES,
              "calls": calls, "raw_files": tree_rows,
              "raw_tree_sha256": tree_sha256,
              "conservative_call_envelopes_seconds": 2 * OUTER_WALL_CAP,
              "selected_best_total_runtime": None}
    receipt = args.output / "result.json"
    with receipt.open("x") as stream:
        json.dump(result, stream, sort_keys=True, indent=2)
        stream.write("\n")
    print(json.dumps(result, sort_keys=True, indent=2))
    if status != "PASS_EXTERNAL_XOR_SEMANTICS":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
