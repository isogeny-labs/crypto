#!/usr/bin/env python3
"""Run one frozen N83 K2048 model cell with a host and container watchdog."""

import argparse
import datetime as dt
import hashlib
import json
import subprocess
import time
from pathlib import Path


IMAGE = "sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31"
MEMORY_BYTES = 4 * 1024**3
MAX_SECONDS = 300


def call(*args, timeout=30):
    return subprocess.run(args, text=True, capture_output=True, timeout=timeout)


def utc():
    return dt.datetime.now(dt.timezone.utc).isoformat().replace("+00:00", "Z")


def run(arm, fixture, mode, binary, obj, corpus, output):
    if arm not in ("U", "D") or fixture not in (0, 1) or mode not in ("preflight", "search"):
        raise ValueError("cell outside frozen protocol")
    binary = binary.resolve(strict=True)
    obj = obj.resolve(strict=True)
    corpus = corpus.resolve(strict=True)
    output = output.resolve(strict=True)
    cell = f"{arm.lower()}-{fixture}-{mode}"
    receipt = output / f"{cell}-guard.json"
    result_path = output / f"{cell}.json"
    if receipt.exists() or result_path.exists():
        raise ValueError("cell output already exists")
    image = call("docker", "image", "inspect", IMAGE, "--format", "{{.Id}}")
    if image.returncode or image.stdout.strip() != IMAGE:
        raise ValueError("pinned Linux image unavailable")
    name = f"n83-k2048-m5-{cell}-{int(time.time())}"
    if call("docker", "inspect", name).returncode == 0:
        raise ValueError("container name already exists")
    binary_hash = hashlib.sha256(binary.read_bytes()).hexdigest()
    shell = (
        'timeout 300 /runner "$1" /object /corpus "$2" "$3" "/out/$4.json"; '
        'status=$?; cat /sys/fs/cgroup/memory.peak > "/out/$4-memory.peak"; exit "$status"'
    )
    started = time.monotonic()
    started_utc = utc()
    create = call(
        "docker", "run", "--detach", "--name", name,
        "--cpus=1", "--memory=4g", "--memory-swap=4g", "--pids-limit=128",
        "--network=none", "--read-only", "--security-opt=no-new-privileges",
        "--tmpfs", "/tmp:rw,nosuid,nodev,size=64m",
        "--mount", f"type=bind,src={binary},dst=/runner,readonly",
        "--mount", f"type=bind,src={obj},dst=/object,readonly",
        "--mount", f"type=bind,src={corpus},dst=/corpus,readonly",
        "--mount", f"type=bind,src={output},dst=/out",
        "--entrypoint", "sh", IMAGE,
        "-c", shell, "sh", arm, str(fixture), mode, cell,
        timeout=60,
    )
    if create.returncode:
        raise RuntimeError(f"container allocation failed: {create.stderr.strip()[:500]}")
    timed_out = False
    try:
        try:
            waited = call("docker", "wait", name, timeout=MAX_SECONDS + 30)
        except subprocess.TimeoutExpired:
            timed_out = True
            call("docker", "kill", name)
            waited = call("docker", "wait", name)
        logs = call("docker", "logs", name)
        (output / f"{cell}.stdout.log").write_text(logs.stdout)
        (output / f"{cell}.stderr.log").write_text(logs.stderr)
        inspected = call("docker", "inspect", name)
        state = json.loads(inspected.stdout)[0] if inspected.returncode == 0 else None
        peak = output / f"{cell}-memory.peak"
        result = {
            "schema": "n83.k2048-m5-guard/v1", "cell": cell,
            "started_utc": started_utc, "ended_utc": utc(),
            "binary_sha256": binary_hash, "image_id": IMAGE,
            "object_sha256": hashlib.sha256(obj.read_bytes()).hexdigest(),
            "corpus_sha256": hashlib.sha256(corpus.read_bytes()).hexdigest(),
            "outer_seconds": time.monotonic() - started,
            "cpu_limit": 1, "memory_limit_bytes": MEMORY_BYTES,
            "network": "none", "timeout_seconds": MAX_SECONDS,
            "outer_watchdog_fired": timed_out,
            "exit_code": int(waited.stdout.strip()) if waited.returncode == 0 else None,
            "oom_killed": state["State"]["OOMKilled"] if state else None,
            "actual_memory_limit_bytes": state["HostConfig"]["Memory"] if state else None,
            "actual_nano_cpus": state["HostConfig"]["NanoCpus"] if state else None,
            "cgroup_memory_peak_bytes": int(peak.read_text()) if peak.exists() else None,
            "worker_receipt_exists": result_path.exists(),
        }
        result["status"] = "PASS" if (
            result["exit_code"] == 0 and not timed_out and result["oom_killed"] is False
            and result["actual_memory_limit_bytes"] == MEMORY_BYTES
            and result["actual_nano_cpus"] == 1_000_000_000
            and result["cgroup_memory_peak_bytes"] is not None
            and result["worker_receipt_exists"]
        ) else "FAILED_OR_CENSORED"
        receipt.write_text(json.dumps(result, sort_keys=True, indent=2) + "\n")
        return result
    finally:
        call("docker", "rm", "-f", name)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("arm", choices=("U", "D"))
    parser.add_argument("fixture", type=int, choices=(0, 1))
    parser.add_argument("mode", choices=("preflight", "search"))
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--object", type=Path, required=True)
    parser.add_argument("--corpus", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = run(args.arm, args.fixture, args.mode, args.binary, args.object, args.corpus, args.output)
    print(json.dumps(result, sort_keys=True))
    if result["status"] != "PASS":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
