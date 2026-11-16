#!/usr/bin/env python3
"""Run the frozen D18 K2048 export or replay in a bounded Linux container."""

import argparse
import datetime as dt
import hashlib
import json
import subprocess
import time
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
IMAGE = "sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31"
SOURCE_MOUNT = Path("/Volumes/SSD990/crypto")
ALT_OBJECTS = Path("/Volumes/SSD990-2/.codex/git-objects/crypto-n83-20261019")
MAX_SECONDS = 900
MEMORY_BYTES = 4 * 1024**3


def command(*argv, timeout=30):
    return subprocess.run(argv, text=True, capture_output=True, timeout=timeout)


def utc():
    return dt.datetime.now(dt.timezone.utc).isoformat().replace("+00:00", "Z")


def run(mode, binary, output):
    ROOT.relative_to(SOURCE_MOUNT)
    binary = binary.resolve(strict=True)
    if not ALT_OBJECTS.is_dir():
        raise ValueError("pinned Git alternate object store unavailable")
    output = output.resolve()
    if not output.is_dir():
        raise ValueError("output parent must exist outside the repository")
    if output == ROOT or ROOT in output.parents:
        raise ValueError("output must remain outside the source checkout")
    base = output / "base"
    if mode == "export" and base.exists():
        raise ValueError("export directory already exists")
    if mode == "replay" and (
        not (base / "manifest.json").is_file() or (base / "replay.json").exists()
    ):
        raise ValueError("replay requires an unreplayed export")
    receipt = output / f"{mode}-guard.json"
    if receipt.exists():
        raise ValueError("guard receipt already exists")
    status = command("git", "-C", str(ROOT), "status", "--porcelain", timeout=60)
    if status.returncode or status.stdout:
        raise ValueError("source checkout is not clean")
    head = command("git", "-C", str(ROOT), "rev-parse", "HEAD")
    if head.returncode:
        raise ValueError("source HEAD unavailable")
    head = head.stdout.strip()
    image = command("docker", "image", "inspect", IMAGE, "--format", "{{.Id}}")
    if image.returncode or image.stdout.strip() != IMAGE:
        raise ValueError("pinned Linux image unavailable")
    binary_sha256 = hashlib.sha256(binary.read_bytes()).hexdigest()
    name = f"n83-d18-k2048-{head[:12]}-{mode}"
    if command("docker", "inspect", name).returncode == 0:
        raise ValueError("matching container already exists")
    shell = (
        'timeout 900 /runner "$1" /out/base; status=$?; '
        'cat /sys/fs/cgroup/memory.peak > "/out/$1-memory.peak"; '
        'exit "$status"'
    )
    started_utc = utc()
    started = time.monotonic()
    create = command(
        "docker", "run", "--detach", "--name", name,
        "--cpus=1", "--memory=4g", "--memory-swap=4g", "--pids-limit=128",
        "--network=none", "--read-only", "--tmpfs", "/tmp:rw,nosuid,nodev,size=64m",
        "--mount", f"type=bind,src={SOURCE_MOUNT},dst={SOURCE_MOUNT},readonly",
        "--mount", f"type=bind,src={ALT_OBJECTS},dst={ALT_OBJECTS},readonly",
        "--mount", f"type=bind,src={binary},dst=/runner,readonly",
        "--mount", f"type=bind,src={output},dst=/out",
        "--workdir", str(ROOT), "--entrypoint", "sh", IMAGE,
        "-c", shell, "sh", mode,
        timeout=60,
    )
    if create.returncode:
        raise RuntimeError(f"Docker allocation failed: {create.stderr.strip()[:500]}")
    container_id = create.stdout.strip()
    timed_out = False
    wait = None
    try:
        try:
            wait = command("docker", "wait", name, timeout=MAX_SECONDS + 30)
        except subprocess.TimeoutExpired:
            timed_out = True
            command("docker", "kill", name)
            wait = command("docker", "wait", name, timeout=30)
        logs = command("docker", "logs", name, timeout=30)
        (output / f"{mode}.stdout.log").write_text(logs.stdout)
        (output / f"{mode}.stderr.log").write_text(logs.stderr)
        inspect = command("docker", "inspect", name, timeout=30)
        state = json.loads(inspect.stdout)[0] if inspect.returncode == 0 else None
        peak_path = output / f"{mode}-memory.peak"
        peak = int(peak_path.read_text().strip()) if peak_path.is_file() else None
        exit_code = int(wait.stdout.strip()) if wait and wait.returncode == 0 else None
        result = {
            "schema": "n83.d18-k2048-guard/v1", "mode": mode,
            "source_head_sha": head, "binary_sha256": binary_sha256,
            "image_id": IMAGE, "container_id": container_id,
            "started_utc": started_utc, "ended_utc": utc(),
            "outer_elapsed_seconds": time.monotonic() - started,
            "cpu_limit": 1, "memory_limit_bytes": MEMORY_BYTES,
            "network": "none", "source_read_only": True,
            "timeout_seconds": MAX_SECONDS, "outer_watchdog_fired": timed_out,
            "exit_code": exit_code,
            "oom_killed": state["State"]["OOMKilled"] if state else None,
            "actual_memory_limit_bytes": state["HostConfig"]["Memory"] if state else None,
            "actual_nano_cpus": state["HostConfig"]["NanoCpus"] if state else None,
            "cgroup_memory_peak_bytes": peak,
            "status": "PASS" if (
                exit_code == 0 and not timed_out and state is not None
                and not state["State"]["OOMKilled"]
                and state["HostConfig"]["Memory"] == MEMORY_BYTES
                and state["HostConfig"]["NanoCpus"] == 1_000_000_000
                and peak is not None and peak <= MEMORY_BYTES
            ) else "FAILED_OR_CENSORED",
        }
        receipt.write_text(json.dumps(result, sort_keys=True, indent=2) + "\n")
        return result
    finally:
        command("docker", "rm", "-f", name, timeout=30)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=("export", "replay"))
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = run(args.mode, args.binary, args.output)
    print(json.dumps(result, sort_keys=True))
    if result["status"] != "PASS":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
