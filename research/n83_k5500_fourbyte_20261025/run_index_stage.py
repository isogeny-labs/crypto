#!/usr/bin/env python3
"""Build the exact four-byte K5500 root index with immutable receipts."""

import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess
import sys
import time


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
PUBLIC_TARGETS = ROOT / "research/n83_cold_factor_base_20261016/targets.jsonl"
SOURCE_SHA256 = "1c741f92a932e452b8210a8c080200ffa6d2511a605721ab156564e0663874ab"
BASE_HASH = "0789049953a67b51dfe23f6fa2788d11e0bb4b35d52180fc038a3ffd2c0efb5f"
K = 5500
STATE_BOUND = 1255377750
COMMON_FLAGS = {
    "KIC_PACKED_ROOT_TABLE": "1",
    "KIC_SYMMETRIC_S3": "1",
    "KIC_ROOT_ONLY11": "1",
    "KIC_INDEXED_ROOT4": "1",
    "KIC_ROOT_BLOOM": "1",
    "KIC_ROOT_BLOOM_PROFILE": "compact4",
    "KIC_SPARSE_CANONICAL": "1",
    "KIC_INDEX_ROOT_BATCH": "1",
    "KIC_INDEX_ROOT_THREADS": "1",
    "KIC_INDEX_BUILD_ONLY": "1",
}
LAYOUTS = {
    "monolithic": {
        "flags": {"KIC_INDEXED4_FIXED32": "1"},
        "table_layout": "indexed83_4byte_exact_id31",
        "state_id_bits": 31,
        "split_at": None,
        "table_bytes": 4 * 2**32,
        "overflow_bytes": 0,
        "bloom_bytes": 2**30,
    },
    "split": {
        "flags": {
            "KIC_INDEXED4_SPLIT30": "1",
            "KIC_INDEXED4_SPLIT_AT": "4549",
        },
        "table_layout": "indexed83_4byte_split_id30",
        "state_id_bits": 30,
        "split_at": 4549,
        "table_bytes": 4 * (2**31 + 2**30),
        "overflow_bytes": 4 * 2**30,
        "bloom_bytes": 2**29 + 2**28,
    },
}


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def save(path: Path, record: dict) -> None:
    path.write_text(json.dumps(record, indent=2, sort_keys=True) + "\n")


def peak_rss_bytes() -> int:
    peak = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss
    return peak if sys.platform == "darwin" else peak * 1024


def run(binary: Path, output: Path, cap_seconds: int, layout: str = "monolithic") -> int:
    if cap_seconds <= 0:
        raise ValueError("cap_seconds must be positive")
    if sha256(SOURCE) != SOURCE_SHA256 or not PUBLIC_TARGETS.is_file():
        raise ValueError("source or public fixture differs from the frozen stage")
    binary = binary.resolve(strict=True)
    mode = LAYOUTS[layout]
    flags = {**COMMON_FLAGS, **mode["flags"]}
    output.mkdir(parents=True, exist_ok=False)
    command = [
        str(binary), f"construct:83:1:{K}", str(PUBLIC_TARGETS), "29",
        str(output / "unused_targets_result.jsonl"),
    ]
    environment = {key: value for key, value in os.environ.items()
                   if not key.startswith("KIC_")}
    environment.update(flags)
    launch = {
        "schema": "n83.k5500-fourbyte-index-launch/v2",
        "layout": layout,
        "source_sha256": SOURCE_SHA256,
        "binary_sha256": sha256(binary),
        "public_targets_sha256": sha256(PUBLIC_TARGETS),
        "command": command,
        "environment_flags": flags,
        "cap_seconds": cap_seconds,
        "started_utc": datetime.now(timezone.utc).isoformat(),
    }
    save(output / "launch.json", launch)
    began = time.monotonic()
    try:
        child = subprocess.run(
            command, cwd=ROOT, env=environment, capture_output=True,
            timeout=cap_seconds, check=False,
        )
        code, stdout, stderr = child.returncode, child.stdout, child.stderr
        status = "PROCESS_EXITED" if code == 0 else "PROCESS_FAILED"
    except subprocess.TimeoutExpired as error:
        code, stdout, stderr = None, error.stdout or b"", error.stderr or b""
        status = "TIMEOUT"
    (output / "stdout.jsonl").write_bytes(stdout)
    (output / "stderr.log").write_bytes(stderr)
    summary = None
    if status == "PROCESS_EXITED":
        try:
            summary = json.loads(stdout)
            if not isinstance(summary, dict):
                raise ValueError("index summary must be a JSON object")
            if (
                summary.get("kind") != "compact_orbit_index_stage"
                or summary.get("n") != 83
                or summary.get("base_hash") != BASE_HASH
                or summary.get("orbit_columns") != K
                or summary.get("factor_base_points") != 166 * K
                or summary.get("regular_states") != STATE_BOUND
                or summary.get("root_table_layout") != mode["table_layout"]
                or summary.get("root_slot_state_id_bits") != mode["state_id_bits"]
                or summary.get("indexed4_split_at") != mode["split_at"]
                or summary.get("root_table_allocated_bytes") != mode["table_bytes"]
                or summary.get("overflow_table_allocated_bytes") != mode["overflow_bytes"]
                or summary.get("root_bloom_bytes") != mode["bloom_bytes"]
                or not isinstance(summary.get("table_digest_blake3"), str)
                or len(summary["table_digest_blake3"]) != 64
                or not isinstance(summary.get("state_digest_blake3"), str)
                or len(summary["state_digest_blake3"]) != 64
                or (
                    layout == "split"
                    and (
                        not isinstance(summary.get("overflow_digest_blake3"), str)
                        or len(summary["overflow_digest_blake3"]) != 64
                    )
                )
            ):
                status = "IDENTITY_MISMATCH"
        except (ValueError, TypeError):
            status = "MALFORMED_OUTPUT"
    receipt = {
        "schema": "n83.k5500-fourbyte-index-receipt/v2",
        "layout": layout,
        "status": status,
        "exit_code": code,
        "started_utc": launch["started_utc"],
        "stopped_utc": datetime.now(timezone.utc).isoformat(),
        "wall_seconds": time.monotonic() - began,
        "peak_child_rss_bytes": peak_rss_bytes(),
        "stdout_sha256": sha256(output / "stdout.jsonl"),
        "stderr_sha256": sha256(output / "stderr.log"),
        "summary": summary,
    }
    save(output / "receipt.json", receipt)
    print(json.dumps({
        "status": status,
        "wall_seconds": receipt["wall_seconds"],
        "peak_child_rss_bytes": receipt["peak_child_rss_bytes"],
    }, sort_keys=True), flush=True)
    return 0 if status == "PROCESS_EXITED" else 2


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--cap-seconds", type=int, required=True)
    parser.add_argument("--layout", choices=sorted(LAYOUTS), default="monolithic")
    arguments = parser.parse_args()
    raise SystemExit(run(
        arguments.binary, arguments.output, arguments.cap_seconds,
        arguments.layout,
    ))
