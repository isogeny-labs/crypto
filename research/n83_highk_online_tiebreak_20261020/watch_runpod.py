#!/usr/bin/env python3
"""Transfer a sealed tie-break archive, then stop and terminate its Pod."""

import argparse
import datetime as dt
import getpass
import hashlib
import importlib
import json
import subprocess
import time
from pathlib import Path

from research.n83_highk_online_tiebreak_20261020 import runpod_provider
from research.n83_highk_online_tiebreak_20261020 import runpod_runner


def digest(path):
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def write(path, value):
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def now():
    return dt.datetime.now(dt.timezone.utc)


def graphql(query, key):
    run = importlib.import_module("runpod.api.graphql").run_graphql_query
    return run(query, api_key=key)


def pod_status(pod_id, key):
    query = "query { myself { pods { id desiredStatus costPerHr } } }"
    pods = graphql(query, key)["data"]["myself"]["pods"]
    return next((pod for pod in pods if pod["id"] == pod_id), None)


def remote(ssh, command):
    process = subprocess.run(ssh + [command], capture_output=True, text=True,
                             timeout=20)
    return process.stdout.strip() if process.returncode == 0 else ""


def spool(ssh, destination):
    command = ["rsync", "-a", "--partial", "-e",
               "ssh -o BatchMode=yes -o ConnectTimeout=8 -p " + ssh[ssh.index("-p") + 1],
               ssh[-1] + ":/workspace/n83_tiebreak/", str(destination) + "/"]
    process = subprocess.run(command, capture_output=True, text=True,
                             timeout=600)
    return process.returncode == 0


def archive_ready(root):
    archive = root / "raw_results.tar.gz"
    receipt = root / "raw_results.sha256"
    if not archive.exists() or not receipt.exists():
        return False
    parts = receipt.read_text().split()
    return len(parts) >= 1 and len(parts[0]) == 64 and digest(archive) == parts[0]


def retire(pod_id, key, root):
    before = pod_status(pod_id, key)
    if before is not None and before["desiredStatus"] != "EXITED":
        query = ("mutation { podStop(input: { podId: " +
                 json.dumps(pod_id) + " }) { id desiredStatus } }")
        stopped = False
        try:
            response = graphql(query, key)
            stopped = True
            write(root / "provider_stop.json", {
                "recorded_utc": now().isoformat(), "pod_id": pod_id,
                "response": response})
        except Exception as error:
            write(root / "provider_stop_error.json", {
                "recorded_utc": now().isoformat(), "pod_id": pod_id,
                "error_type": type(error).__name__, "error": str(error)[:300]})
        if stopped:
            for _ in range(40):
                time.sleep(5)
                current = pod_status(pod_id, key)
                if current is None or current["desiredStatus"] == "EXITED":
                    break
    current = pod_status(pod_id, key)
    if current is not None:
        query = ("mutation { podTerminate(input: { podId: " +
                 json.dumps(pod_id) + " }) }")
        response = graphql(query, key)
        write(root / "provider_terminate.json", {
            "recorded_utc": now().isoformat(), "pod_id": pod_id,
            "response": response})
    after = pod_status(pod_id, key)
    for _ in range(20):
        if after is None:
            break
        time.sleep(3)
        after = pod_status(pod_id, key)
    write(root / "provider_post_terminate.json", {
        "recorded_utc": now().isoformat(), "pod_id": pod_id,
        "pod_present": after is not None})
    if after is not None:
        raise ValueError("Pod remains present after termination")


def watch(allocation_path, trigger_path, host, port, root, key):
    receipt = runpod_runner.read_allocation(allocation_path, trigger_path)
    pod_id = receipt["pod_id"]
    deadline = runpod_provider.timestamp(receipt["stop_after_utc"])
    if pod_status(pod_id, key) is None:
        raise ValueError("allocated Pod is absent before watch")
    root.mkdir(parents=True, exist_ok=True)
    ssh = ["ssh", "-o", "BatchMode=yes", "-o", "ConnectTimeout=8",
           "-p", str(port), "root@" + host]
    while now() < deadline:
        try:
            status = remote(ssh, "cat /workspace/n83_tiebreak/run.status 2>/dev/null")
        except (OSError, subprocess.TimeoutExpired):
            status = ""
        if status.startswith("DONE "):
            print(status, flush=True)
            if spool(ssh, root) and archive_ready(root):
                print("ARCHIVE_SHA256_PASS", flush=True)
                retire(pod_id, key, root)
                return 0
        time.sleep(20)
    try:
        spool(ssh, root)
    except (OSError, subprocess.TimeoutExpired):
        pass
    retire(pod_id, key, root)
    print("PROVIDER_DEADLINE_OR_ARCHIVE_FAILURE", flush=True)
    return 2


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--allocation", type=Path, required=True)
    parser.add_argument("--trigger", type=Path, required=True)
    parser.add_argument("--ssh-host", required=True)
    parser.add_argument("--ssh-port", type=int, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    key = getpass.getpass("RunPod API key: ")
    raise SystemExit(watch(args.allocation.resolve(), args.trigger.resolve(),
                           args.ssh_host, args.ssh_port,
                           args.output.resolve(), key))


if __name__ == "__main__":
    main()
