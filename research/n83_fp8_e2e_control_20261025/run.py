#!/usr/bin/env python3
"""Frozen, capped K1107 baseline/fingerprint full-process ABBA control."""

import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import signal
import subprocess
import time

from research.n83_nearby_grid_tier1_20261024 import protocol as fixture


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
SOURCE_SHA256 = "05b64717b10e17c8cd09e8a6823e6efec600ac4b53c8a0e8e0476771c702cb7b"
ORDER = ("baseline", "fp8", "fp8", "baseline")
CAP_SECONDS = 1200
RSS_CAP_BYTES = 16_374_562_816
FLAGS = {
    "KIC_PACKED_ROOT_TABLE": "1", "KIC_SYMMETRIC_S3": "1",
    "KIC_ROOT_ONLY11": "1", "KIC_INDEXED_ROOT4": "1",
    "KIC_ROOT_BLOOM": "1", "KIC_ROOT_BLOOM_PROFILE": "compact4",
    "KIC_SPARSE_CANONICAL": "1", "KIC_INDEX_ROOT_BATCH": "1",
    "KIC_INDEX_ROOT_THREADS": "4", "KIC_INDEXED4_SPLIT30": "1",
    "KIC_INDEXED4_SPLIT_AT": "512", "KIC_RANK_THREADS": "4",
    "KIC_TARGET_THREADS": "2", "KIC_TARGET_CANCEL": "1",
}


def sha(path):
    h = hashlib.sha256()
    with Path(path).open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def write(path, data):
    Path(path).write_text(json.dumps(data, sort_keys=True, indent=2) + "\n")


def now():
    return datetime.now(timezone.utc).isoformat()


def sample_rss(pid):
    result = subprocess.run(["/bin/ps", "-o", "rss=", "-p", str(pid)],
                            capture_output=True, text=True, check=False)
    raw = result.stdout.strip()
    return int(raw) * 1024 if raw.isdigit() else 0


def stop_group(pid):
    try:
        os.killpg(pid, signal.SIGTERM)
    except ProcessLookupError:
        return
    time.sleep(2)
    try:
        os.killpg(pid, signal.SIGKILL)
    except ProcessLookupError:
        pass


def run(binary, output):
    if (sha(SOURCE) != SOURCE_SHA256 or
            sha(fixture.TARGETS) != fixture.TARGETS_SHA256 or
            sha(fixture.ANSWERS) != fixture.ANSWERS_SHA256):
        raise ValueError("frozen source or public point differs")
    binary = Path(binary).resolve(strict=True)
    output = Path(output).resolve()
    if output.exists() or output == ROOT or ROOT in output.parents:
        raise ValueError("output must be a new directory outside the checkout")
    output.mkdir(parents=True)
    executed = output / "koblitz_orbit_dlp_fast"
    shutil.copy2(binary, executed)
    executed.chmod(0o755)
    receipt = {
        "schema": "n83.fp8-full-abba/v1", "scope": "shared_host_exploratory",
        "source_sha256": SOURCE_SHA256, "binary_sha256": sha(executed),
        "target_sha256": fixture.TARGETS_SHA256,
        "answer_sha256": fixture.ANSWERS_SHA256,
        "git_head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT,
                                             text=True).strip(),
        "host": {"system": platform.system(), "machine": platform.machine(),
                 "logical_cpus": os.cpu_count()},
        "order": ORDER, "base_flags": FLAGS, "k": 1107,
        "cap_seconds_per_child": CAP_SECONDS, "rss_cap_bytes": RSS_CAP_BYTES,
        "started_utc": now(), "rows": [],
    }
    write(output / "summary.json", receipt)
    for index, arm in enumerate(ORDER):
        folder = output / f"{index}-{arm}"
        folder.mkdir()
        flags = dict(FLAGS)
        if arm == "fp8":
            flags["KIC_INDEXED4_FINGERPRINT8"] = "1"
        environment = {k: v for k, v in os.environ.items() if not k.startswith("KIC_")}
        environment.update(flags)
        command = [str(executed), "construct:83:1:1107", str(fixture.TARGETS),
                   "29", str(folder / "target.jsonl")]
        launch = {"command": command, "flags": flags, "started_utc": now()}
        write(folder / "launch.json", launch)
        started = time.monotonic()
        peak = 0
        reason = None
        with (folder / "stdout.jsonl").open("wb") as stdout, \
             (folder / "stderr.log").open("wb") as stderr:
            child = subprocess.Popen(command, cwd=ROOT, env=environment,
                                     stdout=stdout, stderr=stderr,
                                     start_new_session=True)
            while child.poll() is None:
                peak = max(peak, sample_rss(child.pid))
                if peak > RSS_CAP_BYTES:
                    reason = "RSS_CAP"
                    stop_group(child.pid)
                    break
                if time.monotonic() - started > CAP_SECONDS:
                    reason = "TIME_CAP"
                    stop_group(child.pid)
                    break
                time.sleep(0.25)
            code = child.wait()
        row = {"index": index, "arm": arm,
               "status": reason or ("PROCESS_EXITED" if code == 0 else "PROCESS_FAILED"),
               "exit_code": code, "wall_seconds": time.monotonic() - started,
               "peak_sampled_rss_bytes": peak,
               "stdout_sha256": sha(folder / "stdout.jsonl"),
               "stderr_sha256": sha(folder / "stderr.log"),
               "target_output_sha256": sha(folder / "target.jsonl")
               if (folder / "target.jsonl").exists() else None,
               "completed_utc": now()}
        receipt["rows"].append(row)
        write(output / "summary.json", receipt)
        print(index, arm, row["status"], round(row["wall_seconds"], 3), flush=True)
        if row["status"] != "PROCESS_EXITED":
            break
    receipt["stopped_utc"] = now()
    write(output / "summary.json", receipt)
    return 0 if len(receipt["rows"]) == len(ORDER) and all(
        row["status"] == "PROCESS_EXITED" for row in receipt["rows"]) else 1


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    raise SystemExit(run(args.binary, args.output))
