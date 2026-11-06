#!/usr/bin/env python3
"""Capped exact-parity control for the opt-in N83 split-index sidecar."""

import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import sys
import time


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
TARGETS = ROOT / "research/n83_cold_factor_base_20261016/targets.jsonl"
ORDER = ("baseline", "fp8", "fp8", "baseline")
SHARED_FIELDS = (
    "base_hash", "factor_base_points", "orbit_columns", "regular_states",
    "root_table_entries", "primary_table_entries", "overflow_table_entries",
    "state_digest_blake3", "table_digest_blake3", "overflow_digest_blake3",
    "root_table_allocated_bytes", "root_bloom_bytes", "root_table_layout",
)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--k", type=int, required=True)
    parser.add_argument("--split-at", type=int, required=True)
    parser.add_argument("--cap-seconds", type=int, default=300)
    args = parser.parse_args()
    if not (1 < args.split_at < args.k and args.cap_seconds > 0):
        parser.error("require 1 < split-at < k and a positive per-process cap")
    binary = args.binary.resolve(strict=True)
    if not TARGETS.is_file():
        parser.error("frozen public fixture is absent")
    args.output.mkdir(parents=True, exist_ok=False)
    flags = {
        "KIC_PACKED_ROOT_TABLE": "1", "KIC_SYMMETRIC_S3": "1",
        "KIC_ROOT_ONLY11": "1", "KIC_INDEXED_ROOT4": "1",
        "KIC_ROOT_BLOOM": "1", "KIC_ROOT_BLOOM_PROFILE": "compact4",
        "KIC_SPARSE_CANONICAL": "1", "KIC_INDEX_ROOT_BATCH": "1",
        "KIC_INDEX_ROOT_THREADS": "4", "KIC_INDEX_BUILD_ONLY": "1",
        "KIC_INDEXED4_SPLIT30": "1", "KIC_INDEXED4_SPLIT_AT": str(args.split_at),
    }
    result = {
        "schema": "n83.fp8-index-abba/v1", "scope": "shared-host exploratory index stage",
        "candidate_id": None, "source_sha256": sha256(SOURCE),
        "binary_sha256": sha256(binary), "public_targets_sha256": sha256(TARGETS),
        "k": args.k, "split_at": args.split_at,
        "cap_seconds_per_process": args.cap_seconds, "flags": flags,
        "host": {"system": platform.system(), "machine": platform.machine(),
                 "logical_cpus": os.cpu_count()},
        "shared_fields": SHARED_FIELDS, "started_utc": datetime.now(timezone.utc).isoformat(),
        "rows": [],
    }
    for index, arm in enumerate(ORDER):
        environment = {key: value for key, value in os.environ.items()
                       if not key.startswith("KIC_")}
        environment.update(flags)
        if arm == "fp8":
            environment["KIC_INDEXED4_FINGERPRINT8"] = "1"
        stdout_path = args.output / f"{index}-{arm}.stdout.jsonl"
        stderr_path = args.output / f"{index}-{arm}.stderr.log"
        command = [str(binary), f"construct:83:1:{args.k}", str(TARGETS),
                   "29", str(args.output / f"{index}-unused-target-result.jsonl")]
        started = time.monotonic()
        try:
            child = subprocess.run(command, cwd=ROOT, env=environment,
                                   capture_output=True, timeout=args.cap_seconds, check=False)
            code, stdout, stderr = child.returncode, child.stdout, child.stderr
            status = "PROCESS_EXITED" if code == 0 else "PROCESS_FAILED"
        except subprocess.TimeoutExpired as error:
            code, stdout, stderr = None, error.stdout or b"", error.stderr or b""
            status = "TIMEOUT"
        stdout_path.write_bytes(stdout)
        stderr_path.write_bytes(stderr)
        stages = []
        for line in stdout.decode(errors="replace").splitlines():
            try:
                value = json.loads(line)
            except json.JSONDecodeError:
                continue
            if isinstance(value, dict) and value.get("kind") == "compact_orbit_index_stage":
                stages.append(value)
        stage = stages[0] if len(stages) == 1 else None
        row = {
            "arm": arm, "status": status, "exit_code": code,
            "wall_seconds": time.monotonic() - started, "stage": stage,
            "stdout_sha256": hashlib.sha256(stdout).hexdigest(),
            "stderr_sha256": hashlib.sha256(stderr).hexdigest(),
        }
        result["rows"].append(row)
        (args.output / "summary.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
        print(arm, status, stage.get("index_build_ms") if stage else None, flush=True)
        if status != "PROCESS_EXITED" or stage is None:
            break
    rows = result["rows"]
    parity = len(rows) == len(ORDER) and all(
        len({row["stage"][field] for row in rows}) == 1 for field in SHARED_FIELDS
    ) and all(row["stage"]["n"] == 83
              and row["stage"]["orbit_columns"] == args.k
              and row["stage"]["indexed4_split_at"] == args.split_at for row in rows)
    sidecars = [row["stage"]["root_fingerprint_sidecar_bytes"] for row in rows] if parity else []
    result["status"] = ("PASS_EXACT_TABLE_PARITY" if parity and sidecars[0] == sidecars[3] == 0
                        and sidecars[1] == sidecars[2] > 0 else "FAIL")
    result["stopped_utc"] = datetime.now(timezone.utc).isoformat()
    (args.output / "summary.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(result["status"], flush=True)
    return 0 if result["status"] == "PASS_EXACT_TABLE_PARITY" else 1


if __name__ == "__main__":
    sys.exit(main())
