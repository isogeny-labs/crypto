#!/usr/bin/env python3
"""Rebuild the lower-K finalist base with the fixed31 comparison source."""

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
TARGET = ROOT / "research/n83_cold_factor_base_20261016/targets.jsonl"
SOURCE_SHA256 = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8"
PRIOR_BASE_HASH = "b0aea97c1c8b9b46e13df67e6af9c33ca73034d2ec3eff33a2392ceb1452b40e"
K = 1107
CAP_SECONDS = 300


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def run(binary: Path, output: Path, *, k: int = K,
        expected_base_hash: str = PRIOR_BASE_HASH) -> int:
    if sha256(SOURCE) != SOURCE_SHA256 or not TARGET.is_file():
        raise ValueError("current source or earlier public fixture differs")
    binary = binary.resolve(strict=True)
    output.mkdir(parents=True, exist_ok=False)
    dump = output / "base_dump.jsonl"
    command = [str(binary), f"construct:83:1:{k}", str(TARGET), "29",
               str(output / "unused_targets_result.jsonl")]
    env = {key: value for key, value in os.environ.items()
           if not key.startswith("KIC_")}
    env["KIC_BASE_BUILD_ONLY"] = "1"
    env["KIC_DUMP_BASE"] = str(dump)
    started = datetime.now(timezone.utc).isoformat()
    try:
        child = subprocess.run(command, cwd=ROOT, env=env,
                               capture_output=True, timeout=CAP_SECONDS,
                               check=False)
        stdout, stderr = child.stdout, child.stderr
        exit_code = child.returncode
        status = "PROCESS_EXITED" if exit_code == 0 else "PROCESS_FAILED"
    except subprocess.TimeoutExpired as error:
        stdout, stderr = error.stdout or b"", error.stderr or b""
        exit_code, status = None, "TIMEOUT"
    (output / "stdout.jsonl").write_bytes(stdout)
    (output / "stderr.log").write_bytes(stderr)
    summary = None
    if status == "PROCESS_EXITED":
        try:
            summary = json.loads(stdout)
            header = json.loads(dump.read_text())
            if (summary.get("kind") != "compact_orbit_base_stage" or
                    summary.get("n") != 83 or summary.get("a") != 1 or
                    summary.get("orbit_columns") != k or
                    summary.get("factor_base_points") != 166 * k or
                    summary.get("base_hash") != expected_base_hash or
                    header.get("base_hash") != expected_base_hash):
                status = "IDENTITY_MISMATCH"
        except (OSError, ValueError, TypeError):
            status = "MALFORMED_OUTPUT"
    receipt = {
        "schema": "n83.fixed31-lowerk-base-stage/v1",
        "status": status,
        "started_utc": started,
        "stopped_utc": datetime.now(timezone.utc).isoformat(),
        "command": command,
        "environment_flags": {"KIC_BASE_BUILD_ONLY": "1",
                              "KIC_DUMP_BASE": str(dump)},
        "source_sha256": SOURCE_SHA256,
        "binary_sha256": sha256(binary),
        "target_sha256": sha256(TARGET),
        "exit_code": exit_code,
        "cap_seconds": CAP_SECONDS,
        "stdout_sha256": sha256(output / "stdout.jsonl"),
        "stderr_sha256": sha256(output / "stderr.log"),
        "base_dump_sha256": sha256(dump) if dump.exists() else None,
        "summary": summary,
    }
    (output / "receipt.json").write_text(json.dumps(receipt, indent=2,
                                                      sort_keys=True) + "\n")
    print(json.dumps({"status": status, "base_hash":
                      summary.get("base_hash") if summary else None},
                     sort_keys=True), flush=True)
    return 0 if status == "PROCESS_EXITED" else 2


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    raise SystemExit(run(args.binary, args.output))
