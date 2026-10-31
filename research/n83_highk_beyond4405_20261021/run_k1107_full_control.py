#!/usr/bin/env python3
"""Run one public K1107 root11 full-process source-bridge control."""

import argparse
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import resource
import subprocess
import time

from research.n83_highk_beyond4405_20261021 import run_k1107_base_stage as base
from research.n83_online_finalists_20261018.run import ic_environment

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
PUBLIC = ROOT / "research/n83_cold_factor_base_20261016/targets.jsonl"
ANSWERS = ROOT / "research/n83_cold_factor_base_20261016/answers.jsonl"
CAP_SECONDS = 900


def run(binary: Path, output: Path) -> int:
    if base.sha256(base.SOURCE) != base.SOURCE_SHA256:
        raise ValueError("current producer source differs")
    binary = binary.resolve(strict=True)
    output.mkdir(parents=True, exist_ok=False)
    target = json.loads(PUBLIC.read_text().splitlines()[0])
    answer = json.loads(ANSWERS.read_text().splitlines()[0])
    if target != answer["point"] or answer["fixture_index"] != 0:
        raise ValueError("earlier public control fixture differs")
    (output / "target.jsonl").write_text(json.dumps(target) + "\n")
    flags = ic_environment(output, "root11_dynamic", "k1107_root11")
    environment = {key: value for key, value in os.environ.items()
                   if not key.startswith("KIC_")}
    environment.update(flags)
    command = [str(binary), "construct:83:1:1107",
               str(output / "target.jsonl"), "29",
               str(output / "targets_result.jsonl")]
    started = datetime.now(timezone.utc).isoformat()
    launch = {
        "schema": "n83.fixed31-k1107-full-control-launch/v1",
        "source_sha256": base.SOURCE_SHA256,
        "binary_sha256": base.sha256(binary),
        "public_targets_sha256": base.sha256(PUBLIC),
        "answer_sidecar_sha256": base.sha256(ANSWERS),
        "target_sha256": base.sha256(output / "target.jsonl"),
        "command": command,
        "environment_flags": {key: value for key, value in environment.items()
                              if key.startswith("KIC_")},
        "cap_seconds": CAP_SECONDS,
        "started_utc": started,
    }
    (output / "launch.json").write_text(json.dumps(launch, indent=2,
                                                    sort_keys=True) + "\n")
    began = time.monotonic()
    try:
        process = subprocess.run(command, cwd=ROOT, env=environment,
                                 capture_output=True, timeout=CAP_SECONDS,
                                 check=False)
        code = process.returncode
        status = "PROCESS_EXITED" if code == 0 else "PROCESS_FAILED"
        stdout, stderr = process.stdout, process.stderr
    except subprocess.TimeoutExpired as error:
        code, status = None, "TIMEOUT"
        stdout, stderr = error.stdout or b"", error.stderr or b""
    (output / "stdout.jsonl").write_bytes(stdout)
    (output / "stderr.log").write_bytes(stderr)
    receipt = {
        "schema": "n83.fixed31-k1107-full-control-receipt/v1",
        "status": status, "exit_code": code,
        "started_utc": started,
        "stopped_utc": datetime.now(timezone.utc).isoformat(),
        "wall_seconds": time.monotonic() - began,
        "peak_child_rss_bytes": resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss,
        "stdout_sha256": base.sha256(output / "stdout.jsonl"),
        "stderr_sha256": base.sha256(output / "stderr.log"),
        "base_dump_sha256": base.sha256(output / "base_dump.jsonl")
                            if (output / "base_dump.jsonl").exists() else None,
        "targets_result_sha256": base.sha256(output / "targets_result.jsonl")
                                 if (output / "targets_result.jsonl").exists()
                                 else None,
    }
    (output / "receipt.json").write_text(json.dumps(receipt, indent=2,
                                                     sort_keys=True) + "\n")
    print(json.dumps({"status": status, "wall_seconds": receipt["wall_seconds"],
                      "peak_child_rss_bytes": receipt["peak_child_rss_bytes"]},
                     sort_keys=True), flush=True)
    return 0 if status == "PROCESS_EXITED" else 2


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    raise SystemExit(run(args.binary, args.output))
