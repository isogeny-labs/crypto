#!/usr/bin/env python3
"""Capped current-source K3596 full-path control on an older public point."""

from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tarfile
import time

ROOT = Path("/Volumes/SSD990/n83-confirm16-20261024")
OUT = Path("/Volumes/SSD990/n83-k3596-current-source-full-20261025/control2")
BIN = Path("/Volumes/SSD990-2/.codex/n83-fresh-cold-target/release/examples/koblitz_orbit_dlp_fast")
SOURCE = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
PUBLIC = ROOT / "research/n83_cold_factor_base_20261016/targets.jsonl"
ANSWERS = ROOT / "research/n83_cold_factor_base_20261016/answers.jsonl"
ARCHIVE = ROOT / "research/n83_root4_frontier_current_20261018/pilots/k3596.tar.gz"
EXPECTED_SOURCE = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8"
EXPECTED_BINARY = "07f7b5b424d0f1ecea1cd7e6564522c4d9c64557a820441c381e7474b13a388a"
CAP_SECONDS = 1200
RSS_CAP_BYTES = 16 * 1024 ** 3
MIN_HOST_FREE_PERCENT = 10


def sha(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def write(path, record):
    path.write_text(json.dumps(record, sort_keys=True, indent=2) + "\n")


def rss(pid):
    try:
        value = subprocess.check_output(["ps", "-o", "rss=", "-p", str(pid)], text=True).strip()
        return int(value) * 1024 if value else 0
    except (ValueError, subprocess.CalledProcessError):
        return 0


def free_percent():
    result = subprocess.check_output(["memory_pressure", "-Q"], text=True)
    found = re.search(r"System-wide memory free percentage:\s*(\d+)%", result)
    return int(found.group(1)) if found else None


def main():
    if sha(SOURCE) != EXPECTED_SOURCE or sha(BIN) != EXPECTED_BINARY:
        raise ValueError("frozen current-source binary or source differs")
    public = json.loads(PUBLIC.read_text().splitlines()[0])
    answer = json.loads(ANSWERS.read_text().splitlines()[0])
    if public != answer["point"] or answer["fixture_index"] != 0:
        raise ValueError("older public control point differs")
    with tarfile.open(ARCHIVE, "r:gz") as bundle:
        stream = bundle.extractfile("pilot_work/launch.json")
        if stream is None:
            raise ValueError("historical K3596 launch missing")
        historical = json.load(stream)
    flags = dict(historical["environment_flags"])
    if (flags.get("KIC_INDEXED_ROOT4") != "1" or
            flags.get("KIC_PACKED_ROOT_TABLE") != "1" or
            flags.get("KIC_INDEX_ROOT_BATCH") != "1" or
            flags.get("KIC_TARGET_CANCEL") != "1"):
        raise ValueError("historical K3596 root4 flags differ")
    OUT.mkdir(parents=True, exist_ok=False)
    target = OUT / "target.jsonl"
    target.write_text(json.dumps(public, sort_keys=True) + "\n")
    flags["KIC_DUMP_BASE"] = str(OUT / "base_dump.jsonl")
    env = {key: value for key, value in os.environ.items() if not key.startswith("KIC_")}
    env.update(flags)
    binary_command = [str(BIN), "construct:83:1:3596", str(target), "29",
                      str(OUT / "targets_result.jsonl")]
    command = binary_command
    start_utc = datetime.now(timezone.utc).isoformat()
    initial_free = free_percent()
    write(OUT / "launch.json", {
        "schema": "n83.k3596-current-source-full-control-launch/v1",
        "source_sha256": EXPECTED_SOURCE,
        "binary_sha256": EXPECTED_BINARY,
        "script_sha256": sha(Path(__file__)),
        "git_head": subprocess.check_output(["git", "rev-parse", "HEAD"],
                                            cwd=ROOT, text=True).strip(),
        "public_targets_sha256": sha(PUBLIC), "answer_sidecar_sha256": sha(ANSWERS),
        "historical_launch_archive_sha256": sha(ARCHIVE),
        "target_sha256": sha(target), "command": command,
        "binary_command": binary_command, "environment_flags": flags,
        "cap_seconds": CAP_SECONDS, "rss_cap_bytes": RSS_CAP_BYTES,
        "min_host_free_percent": MIN_HOST_FREE_PERCENT,
        "initial_host_free_percent": initial_free,
        "started_utc": start_utc, "host_kind": "contended_local_mac_correctness_control",
    })
    print(f"K3596 full control starting; host free {initial_free}%", flush=True)
    start = time.monotonic()
    peak = 0
    lowest_free = initial_free
    reason = None
    with (OUT / "stdout.jsonl").open("wb") as stdout, (OUT / "stderr.log").open("wb") as stderr:
        child = subprocess.Popen(command, cwd=ROOT, env=env, stdout=stdout, stderr=stderr)
        last_memory_check = start
        last_progress = start
        while child.poll() is None:
            now = time.monotonic()
            peak = max(peak, rss(child.pid))
            if peak > RSS_CAP_BYTES:
                reason = "RSS_CAP"
            elif now - start >= CAP_SECONDS:
                reason = "TIME_CAP"
            if now - last_memory_check >= 10:
                current_free = free_percent()
                lowest_free = min(lowest_free, current_free)
                last_memory_check = now
                if current_free < MIN_HOST_FREE_PERCENT:
                    reason = "HOST_MEMORY_PRESSURE"
            if reason:
                child.terminate()
                try:
                    child.wait(timeout=10)
                except subprocess.TimeoutExpired:
                    child.kill()
                    child.wait()
                break
            if now - last_progress >= 30:
                print(f"K3596 elapsed={now-start:.0f}s peak_rss={peak/1024**3:.2f}GiB "
                      f"min_free={lowest_free}%", flush=True)
                last_progress = now
            time.sleep(2)
        code = child.returncode
    receipt = {
        "schema": "n83.k3596-current-source-full-control-receipt/v1",
        "status": reason or ("PROCESS_EXITED" if code == 0 else "PROCESS_FAILED"),
        "exit_code": code, "wall_seconds": time.monotonic() - start,
        "peak_sampled_rss_bytes": peak, "lowest_host_free_percent": lowest_free,
        "stopped_utc": datetime.now(timezone.utc).isoformat(),
        "stdout_sha256": sha(OUT / "stdout.jsonl"),
        "stderr_sha256": sha(OUT / "stderr.log"),
        "base_dump_sha256": sha(OUT / "base_dump.jsonl")
        if (OUT / "base_dump.jsonl").exists() else None,
        "targets_result_sha256": sha(OUT / "targets_result.jsonl")
        if (OUT / "targets_result.jsonl").exists() else None,
    }
    write(OUT / "receipt.json", receipt)
    print(json.dumps({key: receipt[key] for key in
                      ("status", "exit_code", "wall_seconds", "peak_sampled_rss_bytes",
                       "lowest_host_free_percent")}, sort_keys=True), flush=True)
    return 0 if receipt["status"] == "PROCESS_EXITED" else 2


if __name__ == "__main__":
    sys.exit(main())
