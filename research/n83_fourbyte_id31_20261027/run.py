"""Bounded N83 K128 split-index control for five- versus four-byte ID31 slots."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import time


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
TARGET_SHA256 = "1c9b53960953fd252d09af08afbfdff9229fbeca151285e698b3e90cb79cd656"
BASE_FLAGS = {
    "KIC_INDEX_BUILD_ONLY": "1",
    "KIC_INDEX_ROOT_BATCH": "1",
    "KIC_INDEX_ROOT_THREADS": "4",
    "KIC_INDEXED5_SPLIT_AT": "64",
    "KIC_PACKED_ROOT_TABLE": "1",
    "KIC_ROOT_BLOOM": "1",
    "KIC_ROOT_BLOOM_PROFILE": "wide3",
    "KIC_ROOT_ONLY11": "1",
    "KIC_SPARSE_CANONICAL": "1",
    "KIC_SYMMETRIC_S3": "1",
}
MODE_FLAGS = {
    "five": {"KIC_INDEXED_ROOT5": "1", "KIC_INDEXED5_ID31": "1",
             "KIC_INDEXED5_LOAD80": "1"},
    "four": {"KIC_INDEXED_ROOT4": "1", "KIC_INDEXED4_ID31": "1"},
}


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def run_one(binary, output, mode, repetition, source_sha256, binary_sha256):
    directory = output / f"r{repetition}-{mode}"
    directory.mkdir()
    flags = {**BASE_FLAGS, **MODE_FLAGS[mode]}
    environment = {key: value for key, value in os.environ.items()
                   if not key.startswith("KIC_")}
    environment.update(flags)
    command = [str(binary), "construct:83:1:128", str(HERE / "target.jsonl"),
               "29", str(directory / "targets_result.jsonl")]
    launch = {"schema": "n83.fourbyte-id31-control/v1", "mode": mode,
              "repetition": repetition, "command": command, "flags": flags,
              "source_sha256": source_sha256, "binary_sha256": binary_sha256,
              "target_sha256": TARGET_SHA256, "timeout_seconds": 90}
    (directory / "launch.json").write_text(json.dumps(launch, sort_keys=True,
                                                     indent=2) + "\n")
    started = time.monotonic()
    with (directory / "stdout.jsonl").open("wb") as stdout, \
         (directory / "stderr.log").open("wb") as stderr:
        process = subprocess.Popen(command, env=environment, stdout=stdout,
                                   stderr=stderr, start_new_session=True)
        try:
            exit_code = process.wait(timeout=90)
            status = "PROCESS_EXITED"
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            exit_code = process.wait()
            status = "TIMEOUT"
    row = {"mode": mode, "repetition": repetition, "status": status,
           "exit_code": exit_code, "process_wall_seconds":
               time.monotonic() - started}
    if status == "PROCESS_EXITED" and exit_code == 0:
        lines = (directory / "stdout.jsonl").read_text().splitlines()
        if len(lines) == 1:
            row["stage"] = json.loads(lines[0])
        else:
            row["status"] = "BAD_STAGE_OUTPUT"
    (directory / "receipt.json").write_text(json.dumps(row, sort_keys=True,
                                                      indent=2) + "\n")
    return row


def validate(rows):
    if len(rows) != 6 or any(row["status"] != "PROCESS_EXITED" or
                             row["exit_code"] != 0 or "stage" not in row
                             for row in rows):
        return "INCOMPLETE"
    reference = rows[0]["stage"]
    shared = ("n", "base_hash", "orbit_columns", "factor_base_points",
              "regular_states", "state_digest_blake3", "root_table_entries",
              "primary_table_entries", "overflow_table_entries")
    if any(any(row["stage"][key] != reference[key] for key in shared)
           for row in rows):
        return "MISMATCH"
    for row in rows:
        stage = row["stage"]
        mode = row["mode"]
        if (stage["kind"] != "compact_orbit_index_stage" or
            stage["orbit_columns"] != 128 or
            stage["indexed5_split_at"] != 64 or
            stage["indexed5_state_id_bits"] != 31 or
            stage["root_table_layout"] !=
                f"indexed83_{5 if mode == 'five' else 4}byte_exact_id31" or
            stage["root_table_slot_bytes"] != (5 if mode == "five" else 4) or
            stage["root_table_indexed4_id31_enabled"] != (mode == "four") or
            not stage["table_digest_blake3"] or
            not stage["overflow_digest_blake3"]):
            return "MISMATCH"
    if any(four["stage"]["root_table_allocated_bytes"] * 5 !=
           five["stage"]["root_table_allocated_bytes"] * 4
           for five, four in zip(rows[::2], rows[1::2])):
        return "MISMATCH"
    return "PASS_SPLIT_STAGE_CONTROL"


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    target = HERE / "target.jsonl"
    if sha256(target) != TARGET_SHA256:
        raise ValueError("public target differs from frozen K128 control")
    source_sha256 = sha256(ROOT / "examples/koblitz_orbit_dlp_fast.rs")
    binary_sha256 = sha256(binary)
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    rows = [run_one(binary, output, mode, repetition, source_sha256,
                    binary_sha256)
            for repetition in range(1, 4) for mode in ("five", "four")]
    status = validate(rows)
    result = {"schema": "n83.fourbyte-id31-control/v1", "status": status,
              "source_sha256": source_sha256,
              "binary_sha256": binary_sha256,
              "target_sha256": TARGET_SHA256, "rows": rows}
    (output / "result.json").write_text(json.dumps(result, sort_keys=True,
                                                   indent=2) + "\n")
    print(json.dumps({"status": status, "output": str(output)}, sort_keys=True))
    return 0 if status == "PASS_SPLIT_STAGE_CONTROL" else 1


if __name__ == "__main__":
    raise SystemExit(main())
