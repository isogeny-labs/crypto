#!/usr/bin/env python3
"""Run a bounded current-source K5500 ID31 split-index correctness control."""

import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import time

from research.n83_highk_beyond4405_20261021 import run_k5500_base_stage as base

ROOT = base.ROOT
PUBLIC = ROOT / "research/n83_cold_factor_base_20261016/targets.jsonl"
ANSWERS = ROOT / "research/n83_cold_factor_base_20261016/answers.jsonl"
CAP_SECONDS = 14400
RSS_CAP_BYTES = 40 * 1024**3
MIN_FREE_BYTES = 8 * 1024**3
POLL_SECONDS = 2
RSS_SAMPLE_TIMEOUT_SECONDS = 5
FLAGS = {
    "KIC_INDEXED5_ID31": "1",
    "KIC_INDEXED5_LOAD80": "1",
    "KIC_INDEXED5_SPLIT_AT": "4549",
    "KIC_INDEXED5_SPLIT_SHARED_BLOOM": "0",
    "KIC_INDEXED_ROOT5": "1",
    "KIC_INDEX_ROOT_BATCH": "1",
    "KIC_INDEX_ROOT_THREADS": "4",
    "KIC_PACKED_ROOT_TABLE": "1",
    "KIC_RANK_THREADS": "4",
    "KIC_ROOT_BLOOM": "1",
    "KIC_ROOT_BLOOM_PROFILE": "wide3",
    "KIC_ROOT_ONLY11": "1",
    "KIC_SPARSE_CANONICAL": "1",
    "KIC_SYMMETRIC_S3": "1",
    "KIC_TARGET_CANCEL": "1",
    "KIC_TARGET_THREADS": "2",
}


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat()


def terminate_group(pid: int, sig: signal.Signals) -> None:
    try:
        os.killpg(pid, sig)
    except ProcessLookupError:
        pass


def rss_bytes(pid: int) -> int | None:
    try:
        sample = subprocess.run(["ps", "-o", "rss=", "-p", str(pid)],
                                capture_output=True, text=True, check=False,
                                timeout=RSS_SAMPLE_TIMEOUT_SECONDS)
    except (OSError, subprocess.TimeoutExpired):
        return None
    value = sample.stdout.strip()
    return int(value) * 1024 if sample.returncode == 0 and value.isdigit() else None


def supervise(child: subprocess.Popen, output: Path,
              began: float) -> tuple[int, str | None, int]:
    peak = 0
    reason = None
    try:
        while child.poll() is None:
            current = rss_bytes(child.pid)
            if current is None:
                if child.poll() is not None:
                    break
                reason = "RSS_SAMPLE_FAILED"
            else:
                peak = max(peak, current)
                if current > RSS_CAP_BYTES:
                    reason = "RSS_CAP"
            if time.monotonic() - began > CAP_SECONDS:
                reason = "TIMEOUT"
            elif shutil.disk_usage(output).free < MIN_FREE_BYTES:
                reason = "DISK_GUARD_STOP"
            if reason:
                break
            time.sleep(POLL_SECONDS)
    finally:
        # Monitoring errors and keyboard interrupts must also retire the child.
        if child.poll() is None:
            terminate_group(child.pid, signal.SIGTERM)
        try:
            code = child.wait(timeout=30)
        except subprocess.TimeoutExpired:
            terminate_group(child.pid, signal.SIGKILL)
            code = child.wait()
    return code, reason, peak


def checked_fixture() -> tuple[dict, dict]:
    target = json.loads(PUBLIC.read_text().splitlines()[0])
    answer = json.loads(ANSWERS.read_text().splitlines()[0])
    if target != answer["point"] or answer["fixture_index"] != 0:
        raise ValueError("earlier public K5500 control fixture differs")
    return target, answer


def plan(binary: Path, output: Path) -> tuple[dict, dict]:
    if base.sha256(base.SOURCE) != base.SOURCE_SHA256:
        raise ValueError("current K5500 producer source differs")
    committed = subprocess.check_output(
        ["git", "show", "HEAD:examples/koblitz_orbit_dlp_fast.rs"], cwd=ROOT)
    if hashlib.sha256(committed).hexdigest() != base.SOURCE_SHA256:
        raise ValueError("committed K5500 producer source differs")
    target, _ = checked_fixture()
    binary = binary.resolve(strict=True)
    command = [str(binary), "construct:83:1:5500",
               str(output / "target.jsonl"), "29",
               str(output / "targets_result.jsonl")]
    return target, {
        "schema": "n83.k5500-current-source-full-control-launch/v1",
        "source_sha256": base.SOURCE_SHA256,
        "source_head_sha": subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
        "binary_sha256": base.sha256(binary),
        "public_targets_sha256": base.sha256(PUBLIC),
        "answer_sidecar_sha256": base.sha256(ANSWERS),
        "command": command,
        "environment_flags": {**FLAGS,
                              "KIC_DUMP_BASE": str(output / "base_dump.jsonl")},
        "cap_seconds": CAP_SECONDS,
        "rss_cap_bytes": RSS_CAP_BYTES,
        "min_free_bytes": MIN_FREE_BYTES,
    }


def run(binary: Path, output: Path, preflight_only: bool = False) -> int:
    output = output.resolve()
    target, launch = plan(binary, output)
    if preflight_only:
        print(json.dumps({"status": "PREFLIGHT_PASS", "launch": launch},
                         sort_keys=True))
        return 0
    if shutil.disk_usage(output.parent).free < MIN_FREE_BYTES:
        raise ValueError("output volume has less than 8 GiB free")
    output.mkdir(parents=True, exist_ok=False)
    (output / "target.jsonl").write_text(json.dumps(target) + "\n")
    launch["target_sha256"] = base.sha256(output / "target.jsonl")
    launch["started_utc"] = utc_now()
    (output / "launch.json").write_text(json.dumps(launch, indent=2,
                                                sort_keys=True) + "\n")
    scratch = output / "scratch"
    scratch.mkdir()
    environment = {key: value for key, value in os.environ.items()
                   if not key.startswith("KIC_")}
    environment.update(launch["environment_flags"])
    environment["TMPDIR"] = str(scratch)
    began = time.monotonic()
    with (output / "stdout.jsonl").open("wb") as stdout, \
         (output / "stderr.log").open("wb") as stderr:
        child = subprocess.Popen(launch["command"], cwd=ROOT, env=environment,
                                 stdout=stdout, stderr=stderr,
                                 start_new_session=True)
        code, reason, peak = supervise(child, output, began)
    receipt = {
        "schema": "n83.k5500-current-source-full-control-receipt/v1",
        "status": reason or ("PROCESS_EXITED" if code == 0 else "PROCESS_FAILED"),
        "exit_code": code,
        "started_utc": launch["started_utc"],
        "stopped_utc": utc_now(),
        "wall_seconds": time.monotonic() - began,
        "sampled_peak_rss_bytes": peak,
        "launch_sha256": base.sha256(output / "launch.json"),
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
    print(json.dumps({"status": receipt["status"],
                      "wall_seconds": receipt["wall_seconds"],
                      "sampled_peak_rss_bytes": peak}, sort_keys=True),
          flush=True)
    return 0 if receipt["status"] == "PROCESS_EXITED" else 2


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--preflight-only", action="store_true")
    args = parser.parse_args()
    return run(args.binary, args.output, args.preflight_only)


if __name__ == "__main__":
    raise SystemExit(main())
