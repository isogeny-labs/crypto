#!/usr/bin/env python3
"""Produce exact high-K base identities before any full index allocation."""

import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
TARGETS = HERE / "holdout_targets.jsonl"
SOURCE_SHA256 = "f9f2d6ad9585676a86f54c64f742576ca1865bd9a5f05acd25258d923f5a29c1"
TARGETS_SHA256 = "a0e1f76ddec6ae901d5e766a942ad0b35299e10f175243fe7abd0fc679040a96"
PRIOR_K4405_BASE_HASH = "d64fcee7ba4975873859d256302872681c82b99fbe687ff4fba0bf3ba3a4c8d2"
K_VALUES = (4405, 4500, 4549)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(binary: Path, output: Path) -> None:
    if sha256(SOURCE) != SOURCE_SHA256 or sha256(TARGETS) != TARGETS_SHA256:
        raise ValueError("pinned source or public targets differ")
    binary = binary.resolve(strict=True)
    output.mkdir(parents=True, exist_ok=False)
    for k in K_VALUES:
        arm = output / f"k{k}"
        arm.mkdir()
        dump = arm / "base_dump.jsonl"
        env = {key: value for key, value in os.environ.items()
               if not key.startswith("KIC_")}
        env["KIC_BASE_BUILD_ONLY"] = "1"
        if k != 4405:
            env["KIC_DUMP_BASE"] = str(dump)
        command = [str(binary), f"construct:83:1:{k}", str(TARGETS), "29",
                   str(arm / "unused_target_output.jsonl")]
        started = datetime.now(timezone.utc).isoformat()
        try:
            child = subprocess.run(command, cwd=ROOT, env=env,
                                   capture_output=True, timeout=180, check=False)
            status = "COMPLETE" if child.returncode == 0 else "PROCESS_FAILED"
            stdout, stderr = child.stdout, child.stderr
            exit_code = child.returncode
        except subprocess.TimeoutExpired as error:
            status = "TIMEOUT"
            stdout, stderr = error.stdout or b"", error.stderr or b""
            exit_code = None
        (arm / "stdout.jsonl").write_bytes(stdout)
        (arm / "stderr.log").write_bytes(stderr)
        summary = None
        if status == "COMPLETE":
            summary = json.loads(stdout.decode().strip())
            if (summary.get("kind") != "compact_orbit_base_stage" or
                    summary.get("n") != 83 or
                    summary.get("orbit_columns") != k or
                    summary.get("factor_base_points") != 166 * k or
                    (k == 4405 and
                     summary.get("base_hash") != PRIOR_K4405_BASE_HASH)):
                status = "IDENTITY_MISMATCH"
            if k != 4405:
                header = json.loads(dump.read_text())
                if (header.get("base_hash") != summary.get("base_hash") or
                        header.get("orbit_columns") != k or
                        header.get("factor_base_points") != 166 * k):
                    status = "DUMP_MISMATCH"
        receipt = {
            "schema": "n83.fixed31-base-stage/v1",
            "status": status,
            "started_utc": started,
            "stopped_utc": datetime.now(timezone.utc).isoformat(),
            "command": command,
            "source_sha256": SOURCE_SHA256,
            "binary_sha256": sha256(binary),
            "targets_sha256": TARGETS_SHA256,
            "exit_code": exit_code,
            "stdout_sha256": sha256(arm / "stdout.jsonl"),
            "stderr_sha256": sha256(arm / "stderr.log"),
            "base_dump_sha256": sha256(dump) if dump.exists() else None,
            "summary": summary,
        }
        (arm / "receipt.json").write_text(json.dumps(receipt, indent=2,
                                                      sort_keys=True) + "\n")
        print(json.dumps({"k": k, "status": status, "base_hash":
                          summary.get("base_hash") if summary else None},
                         sort_keys=True), flush=True)
        if status != "COMPLETE":
            raise RuntimeError(f"K{k} base stage {status}; see {arm}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    arguments = parser.parse_args()
    run(arguments.binary, arguments.output)
