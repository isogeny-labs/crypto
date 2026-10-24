#!/usr/bin/env python3
"""Run the four frozen ECCp-131 arity arms without overwriting artifacts."""

import argparse
import datetime as dt
import hashlib
import json
import os
import platform
import subprocess
import time
from pathlib import Path


def command_output(*args):
    return subprocess.run(args, capture_output=True, text=True, check=False).stdout.strip()


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def utc_now():
    return dt.datetime.now(dt.timezone.utc).isoformat()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    binary = args.binary.resolve(strict=True)
    meta = {
        "protocol": "THREE_POINT_PROTOCOL.md",
        "started_utc": utc_now(),
        "source_commit": command_output("git", "rev-parse", "HEAD"),
        "source_status": command_output("git", "status", "--short"),
        "binary": str(binary),
        "binary_sha256": sha256(binary),
        "platform": platform.platform(),
        "machine": platform.machine(),
        "processor": command_output("sysctl", "-n", "machdep.cpu.brand_string"),
        "rustc": command_output("rustc", "--version"),
        "cargo": command_output("cargo", "--version"),
        "arms": [],
        "environment_overrides": {"RAYON_NUM_THREADS": "1"},
    }
    for seed in (131, 137):
        for arity in (2, 3):
            stem = f"seed{seed}-arity{arity}"
            cmd = [
                "/usr/bin/time", "-l", str(binary), "prime", "--curve", "eccp-131",
                "--size", "128", "--trials", "0", "--controls", "8",
                "--setup-candidates", "32", "--target-candidates", "16",
                "--seed", str(seed), "--relation-arity", str(arity), "--json",
            ]
            row = {"name": stem, "command": cmd, "started_utc": utc_now()}
            start = time.monotonic()
            try:
                child = subprocess.run(
                    cmd, capture_output=True, text=True, timeout=300,
                    env={**os.environ, "RAYON_NUM_THREADS": "1"},
                )
                row["exit_code"] = child.returncode
                stdout, stderr = child.stdout, child.stderr
            except subprocess.TimeoutExpired as exc:
                row["exit_code"] = None
                row["timeout_seconds"] = 300
                stdout = exc.stdout or b""
                stderr = exc.stderr or b""
                if isinstance(stdout, bytes):
                    stdout = stdout.decode(errors="replace")
                if isinstance(stderr, bytes):
                    stderr = stderr.decode(errors="replace")
            row["cold_wall_seconds"] = time.monotonic() - start
            row["finished_utc"] = utc_now()
            (output / f"{stem}.stdout.json").write_text(stdout)
            (output / f"{stem}.stderr.txt").write_text(stderr)
            row["stdout_sha256"] = sha256(output / f"{stem}.stdout.json")
            row["stderr_sha256"] = sha256(output / f"{stem}.stderr.txt")
            if row["exit_code"] == 0:
                try:
                    data = json.loads(stdout)
                    probe = data["ic_probe"]
                    row["probe"] = {
                        key: probe[key] for key in (
                            "factor_base_sha256", "relation_arity",
                            "setup_relations", "setup_rank", "target_relations",
                            "combined_rank", "target_log_determined", "verified",
                            "elapsed_ms",
                        )
                    }
                except (ValueError, KeyError, TypeError) as exc:
                    row["parse_error"] = str(exc)
            meta["arms"].append(row)
            (output / "manifest.json").write_text(json.dumps(meta, indent=2) + "\n")
    meta["finished_utc"] = utc_now()
    (output / "manifest.json").write_text(json.dumps(meta, indent=2) + "\n")


if __name__ == "__main__":
    main()
