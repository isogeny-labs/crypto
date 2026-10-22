#!/usr/bin/env python3
"""Run one current-source K3300 full-process identity check on an exposed point."""

import argparse
import hashlib
import json
import os
import subprocess
import time
from pathlib import Path

from research.n83_final_extended_20261018.run import ic_environment

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
OLD = ROOT / "research/n83_k992_cold_followup_20261017"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    manifest_path = next(HERE.glob("candidate_IC1N83Ckb1a1fb547800*.json"))
    manifest = json.loads(manifest_path.read_text())
    target_line = (OLD / "targets.jsonl").read_text().splitlines()[0] + "\n"
    (output / "target.jsonl").write_text(target_line)
    binary = args.binary.resolve()
    command = [str(binary), "construct:83:1:3300", str(output / "target.jsonl"),
               "29", str(output / "targets_result.jsonl")]
    environment = {**os.environ,
                   **ic_environment(output, "root11_wide3", "k3300_wide3")}
    (output / "launch.json").write_text(json.dumps({
        "command": command,
        "binary_sha256": digest(binary),
        "source_sha256": manifest["implementation"]["source_sha256"]["ic"],
        "environment_flags": {key: value for key, value in environment.items()
                              if key.startswith("KIC_")},
        "target_sha256": digest(output / "target.jsonl"),
        "candidate_id": manifest["candidate_id"],
    }, indent=2, sort_keys=True) + "\n")
    started = time.monotonic()
    process = subprocess.run(command, cwd=ROOT, env=environment,
                             capture_output=True, text=True, timeout=1200,
                             check=False)
    elapsed = time.monotonic() - started
    (output / "stdout.jsonl").write_text(process.stdout)
    (output / "stderr.txt").write_text(process.stderr)
    summary = json.loads(process.stdout)
    checks = {
        "exit_zero": process.returncode == 0,
        "base_dump_sha256": digest(output / "base_dump.jsonl") ==
        manifest["factor_base"]["ordered_base_dump_sha256"],
        "base_hash": summary["base_hash"] ==
        manifest["factor_base"]["source_base_hash_blake3"],
        "root_table_allocated_bytes": summary["root_table_allocated_bytes"] ==
        5 * (1 << 30),
        "two_target_workers": summary["target_threads"] == 2,
        "target_cancel": summary["target_cancel"] is True,
        "root_batch_requested": environment["KIC_INDEX_ROOT_BATCH"] == "1",
    }
    result = {"schema": "n83.final-extended-mid-base-pilot/v1",
              "k": 3300, "candidate_id": manifest["candidate_id"],
              "exit_code": process.returncode, "wall_seconds": elapsed,
              "checks": checks, "status": "PASS" if all(checks.values()) else "FAILED"}
    (output / "pilot.json").write_text(json.dumps(result, indent=2,
                                                   sort_keys=True) + "\n")
    print(json.dumps(result, sort_keys=True))
    return 0 if all(checks.values()) else 2


if __name__ == "__main__":
    raise SystemExit(main())
