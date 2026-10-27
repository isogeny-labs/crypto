"""Capped, same-source index-stage control for the N83 split root table."""

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import time
from pathlib import Path


FLAGS = {
    "KIC_INDEX_BUILD_ONLY": "1",
    "KIC_INDEXED5_ID30": "1",
    "KIC_INDEXED5_LOAD80": "1",
    "KIC_INDEXED_ROOT5": "1",
    "KIC_INDEX_ROOT_BATCH": "1",
    "KIC_PACKED_ROOT_TABLE": "1",
    "KIC_ROOT_BLOOM": "1",
    "KIC_ROOT_BLOOM_PROFILE": "wide3",
    "KIC_ROOT_ONLY11": "1",
    "KIC_SPARSE_CANONICAL": "1",
    "KIC_SYMMETRIC_S3": "1",
}


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def run_arm(name, args, common):
    output = args.output / name
    output.mkdir(parents=True, exist_ok=False)
    env = {key: value for key, value in os.environ.items() if not key.startswith("KIC_")}
    env.update(FLAGS)
    if name == "split":
        env["KIC_INDEXED5_SPLIT_AT"] = str(args.split)
    command = [str(args.binary), f"construct:83:1:{args.k}", str(args.target),
               "29", str(output / "targets_result.jsonl")]
    (output / "input.json").write_text(json.dumps({**common, "arm": name,
        "command": command, "environment_flags": {key: env[key] for key in sorted(env)
        if key.startswith("KIC_")}}, indent=2, sort_keys=True) + "\n")
    started = time.monotonic()
    with (output / "stdout.jsonl").open("w") as stdout, \
         (output / "stderr.log").open("w") as stderr:
        process = subprocess.Popen(command, env=env, stdout=stdout, stderr=stderr)
        try:
            exit_code = process.wait(timeout=args.timeout_seconds)
            status = "PROCESS_EXITED"
        except subprocess.TimeoutExpired:
            process.kill()
            exit_code = process.wait()
            status = "TIMEOUT"
    result = {"arm": name, "status": status, "exit_code": exit_code,
              "process_wall_seconds": time.monotonic() - started}
    if status == "PROCESS_EXITED" and exit_code == 0:
        rows = (output / "stdout.jsonl").read_text().splitlines()
        if len(rows) == 1:
            result["stage"] = json.loads(rows[0])
        else:
            result["status"] = "BAD_STAGE_RECEIPT"
    (output / "process_result.json").write_text(
        json.dumps(result, indent=2, sort_keys=True) + "\n")
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--target", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--k", type=int, default=64)
    parser.add_argument("--split", type=int, default=32)
    parser.add_argument("--timeout-seconds", type=int, default=900)
    parser.add_argument("--split-first", action="store_true")
    args = parser.parse_args()
    args.binary = args.binary.resolve(strict=True)
    args.target = args.target.resolve(strict=True)
    assert 0 < args.split < args.k and args.timeout_seconds > 0
    args.output.mkdir(parents=True, exist_ok=False)
    shutil.copyfile(Path(__file__).resolve(), args.output / "runner.py")
    source = Path(__file__).resolve().parents[2] / "examples/koblitz_orbit_dlp_fast.rs"
    common = {"schema": "n83.split-index-smoke/v1", "k": args.k,
              "split_at": args.split, "timeout_seconds": args.timeout_seconds,
              "order": ["split", "monolithic"] if args.split_first else ["monolithic", "split"],
              "runner_sha256": sha256(args.output / "runner.py"),
              "source_sha256": sha256(source), "binary_sha256": sha256(args.binary),
              "target_sha256": sha256(args.target)}
    (args.output / "input.json").write_text(
        json.dumps(common, indent=2, sort_keys=True) + "\n")
    results = [run_arm(name, args, common) for name in common["order"]]
    if all(row["status"] == "PROCESS_EXITED" and row["exit_code"] == 0
           and "stage" in row for row in results):
        first, second = (next(row["stage"] for row in results if row["arm"] == name)
                         for name in ("monolithic", "split"))
        matching = ("n", "base_hash", "orbit_columns", "factor_base_points",
                    "regular_states", "state_digest_blake3")
        valid = (all(first[key] == second[key] for key in matching)
                 and first["indexed5_split_at"] is None
                 and second["indexed5_split_at"] == args.split
                 and first["root_table_entries"] > 0
                 and second["primary_table_entries"] > 0
                 and second["overflow_table_entries"] > 0)
        status = "PASS_INDEX_STAGE_CONTROL" if valid else "MISMATCH"
    else:
        status = "INCOMPLETE"
    result = {"schema": common["schema"], "status": status, "arms": results}
    (args.output / "result.json").write_text(
        json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"status": status, "output": str(args.output)}, sort_keys=True))
    raise SystemExit(0 if status == "PASS_INDEX_STAGE_CONTROL" else 1)


if __name__ == "__main__":
    main()
