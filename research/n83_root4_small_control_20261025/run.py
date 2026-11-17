#!/usr/bin/env python3
"""Bounded K1107 fixed-root4 correctness control on an existing N83 point."""

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

from research.n83_crossfamily_current_20261025 import arms, run as wide_run
from research.n83_nearby_grid_tier1_20261024 import protocol as screen
from research.n83_indexed5_20261013 import analyze as group


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
K = 1107
SOURCE_SHA256 = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8"
BASE_HASH = "b0aea97c1c8b9b46e13df67e6af9c33ca73034d2ec3eff33a2392ceb1452b40e"
BASE_DUMP_SHA256 = "08608f6ad2b8c66510246348933740c78d0937cae783d521514f467940edc3dc"
BASE_POINTS = 183762
CAP_SECONDS = 1200
RSS_CAP_BYTES = 16_374_562_816


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write(path, value):
    path.write_text(json.dumps(value, sort_keys=True, indent=2) + "\n")


def now():
    return datetime.now(timezone.utc).isoformat()


def peak_sample(pid):
    value = subprocess.run(["/bin/ps", "-o", "rss=", "-p", str(pid)],
                           capture_output=True, text=True, check=False)
    raw = value.stdout.strip()
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


def fixture():
    target = screen.TARGETS
    answer = screen.ANSWERS
    if (sha(ROOT / "examples/koblitz_orbit_dlp_fast.rs") != SOURCE_SHA256 or
            sha(target) != screen.TARGETS_SHA256 or
            sha(answer) != screen.ANSWERS_SHA256 or
            sha(ROOT / "Cargo.lock") != screen.LOCK_SHA256):
        raise ValueError("source or existing public fixture differs")
    point = group.point_from_json(json.loads(target.read_text()))
    scalar = int(json.loads(answer.read_text())["scalar"])
    if (not group.on_curve(point) or group.multiply(point, group.R) is not None or
            group.multiply(group.G, scalar) != point):
        raise ValueError("existing public target fails scalar replay")
    root4 = arms.manifest(arms.ARMS[2])[1]
    if (wide_run.cold.RSS_CAP_BYTES != RSS_CAP_BYTES or
            root4["implementation"]["source_sha256"]["ic"] != SOURCE_SHA256 or
            root4["point_decomposition"]["root_slot"]["bytes"] != 4 or
            root4["implementation"]["algorithm_flags"]["root_bloom_profile"] !=
            "standard"):
        raise ValueError("wide fixed-root4 control policy differs")
    return target, answer


def run(binary, output):
    target, answer = fixture()
    binary = binary.resolve(strict=True)
    output = output.resolve()
    if output == ROOT or ROOT in output.parents or output.exists():
        raise ValueError("output must be a new directory outside the repository")
    output.mkdir(parents=True)
    executed = output / "executed_bins"
    executed.mkdir()
    copy = executed / "koblitz_orbit_dlp_fast"
    shutil.copy2(binary, copy)
    copy.chmod(0o755)
    folder = output / "run"
    folder.mkdir()
    flags = wide_run.ic_environment(folder, "root4_standard", "k3500_root4")
    if (flags.get("KIC_INDEXED_ROOT4") != "1" or
            flags.get("KIC_INDEXED4_FIXED31") != "1" or
            flags.get("KIC_ROOT_BLOOM_PROFILE") != "standard" or
            flags.get("KIC_TARGET_THREADS") != "2" or
            flags.get("KIC_INDEX_BUILD_ONLY") is not None):
        raise ValueError("fixed-root4 execution flags differ")
    command = [str(copy), f"construct:83:1:{K}", str(target), "29",
               str(folder / "targets_result.jsonl")]
    write(folder / "launch.json", {
        "schema": "n83.root4-small-control-launch/v1",
        "scope": "existing_public_point_correctness_control",
        "source_sha256": SOURCE_SHA256, "binary_sha256": sha(copy),
        "target_sha256": sha(target), "answer_sha256": sha(answer),
        "git_head": subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
        "host": {"system": platform.system(), "machine": platform.machine(),
                 "logical_cpus": os.cpu_count()},
        "rustc_version": subprocess.check_output(
            ["rustup", "run", "1.98", "rustc", "--version"],
            text=True).strip(),
        "command": command,
        "environment_flags": {k: v for k, v in flags.items()
                              if k.startswith("KIC_")},
        "cap_seconds": CAP_SECONDS, "rss_cap_bytes": RSS_CAP_BYTES,
        "started_utc": now(),
    })
    environment = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"),
                   "TMPDIR": os.environ.get("TMPDIR", "/tmp"), **flags}
    started = time.monotonic()
    peak = 0
    reason = None
    with (folder / "stdout.jsonl").open("wb") as stdout, \
         (folder / "stderr.log").open("wb") as stderr:
        child = subprocess.Popen(command, cwd=ROOT, env=environment,
                                 stdout=stdout, stderr=stderr,
                                 start_new_session=True)
        try:
            while child.poll() is None:
                peak = max(peak, peak_sample(child.pid))
                elapsed = time.monotonic() - started
                if peak > RSS_CAP_BYTES:
                    reason = "RSS_CAP"
                    stop_group(child.pid)
                elif elapsed > CAP_SECONDS:
                    reason = "TIME_CAP"
                    stop_group(child.pid)
                if reason:
                    break
                time.sleep(0.5)
            code = child.wait(timeout=10)
        except BaseException:
            stop_group(child.pid)
            child.wait(timeout=10)
            raise
    receipt = {
        "schema": "n83.root4-small-control-receipt/v1",
        "status": reason or ("PROCESS_EXITED" if code == 0 else "PROCESS_FAILED"),
        "exit_code": code, "wall_seconds": time.monotonic() - started,
        "peak_sampled_rss_bytes": peak, "stopped_utc": now(),
        "stdout_sha256": sha(folder / "stdout.jsonl"),
        "stderr_sha256": sha(folder / "stderr.log"),
        "base_dump_sha256": sha(folder / "base_dump.jsonl")
        if (folder / "base_dump.jsonl").exists() else None,
        "target_output_sha256": sha(folder / "targets_result.jsonl")
        if (folder / "targets_result.jsonl").exists() else None,
    }
    write(folder / "receipt.json", receipt)
    print(json.dumps({"status": receipt["status"], "exit_code": code,
                      "wall_seconds": receipt["wall_seconds"],
                      "peak_sampled_rss_bytes": peak}, sort_keys=True))
    return 0 if receipt["status"] == "PROCESS_EXITED" else 2


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    return run(args.binary, args.output)


if __name__ == "__main__":
    raise SystemExit(main())
