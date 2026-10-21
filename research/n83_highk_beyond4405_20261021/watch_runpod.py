#!/usr/bin/env python3
"""Transfer fixed31 evidence and terminate its Pod before the cost deadline."""

import argparse
from datetime import datetime, timedelta, timezone
from decimal import Decimal
import getpass
import json
from pathlib import Path
import subprocess
import time
from urllib import error as url_error
from urllib import request as url_request

from research.n83_highk_beyond4405_20261021 import runpod_runner
from research.n83_highk_online_tiebreak_20261020 import watch_runpod as provider

REMOTE_ROOT = "/workspace/n83_fixed31"
RETIRE_MARGIN = timedelta(minutes=5)
POLL_SECONDS = 15


def now() -> datetime:
    return datetime.now(timezone.utc)


def write(path: Path, value: dict) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def graphql(query: str, key: str) -> dict:
    payload = json.dumps({"query": query}).encode()
    request = url_request.Request(
        "https://api.runpod.io/graphql", data=payload,
        headers={"Content-Type": "application/json",
                 "Authorization": "Bearer " + key}, method="POST")
    try:
        with url_request.urlopen(request, timeout=30) as response:
            result = json.load(response)
    except url_error.HTTPError as error:
        raise ValueError(f"RunPod API HTTP {error.code}") from None
    if result.get("errors"):
        raise ValueError("RunPod GraphQL response contains errors")
    if not isinstance(result.get("data"), dict):
        raise ValueError("RunPod GraphQL response lacks data")
    return result


def pod_status(pod_id: str, key: str) -> dict | None:
    query = ("query { myself { pods { id desiredStatus costPerHr machineId "
             "createdAt } } }")
    pods = graphql(query, key)["data"]["myself"]["pods"]
    return next((pod for pod in pods if pod.get("id") == pod_id), None)


def verify_live(receipt: dict, pod: dict | None) -> None:
    if (pod is None or pod.get("desiredStatus") != "RUNNING" or
            pod.get("machineId") != receipt["machine_id"] or
            Decimal(str(pod.get("costPerHr"))) !=
            Decimal(str(receipt["cost_per_hr"])) or
            runpod_runner.timestamp(pod["createdAt"]) !=
            runpod_runner.timestamp(receipt["created_utc"])):
        raise ValueError("live Pod differs from frozen allocation")


def remote_status(host: str, port: int) -> str:
    command = ["ssh", "-o", "BatchMode=yes", "-o", "ConnectTimeout=8",
               "-p", str(port), "root@" + host,
               "cat " + REMOTE_ROOT + "/run.status 2>/dev/null"]
    result = subprocess.run(command, capture_output=True, text=True,
                            timeout=15, check=False)
    return result.stdout.strip() if result.returncode == 0 else ""


def spool(host: str, port: int, output: Path, timeout: int) -> bool:
    command = ["rsync", "-a", "--partial", "-e",
               f"ssh -o BatchMode=yes -o ConnectTimeout=8 -p {port}",
               f"root@{host}:{REMOTE_ROOT}/", str(output) + "/"]
    result = subprocess.run(command, capture_output=True, text=True,
                            timeout=timeout, check=False)
    return result.returncode == 0


def archive_ready(output: Path) -> bool:
    return provider.archive_ready(output)


def terminate(pod_id: str, key: str, output: Path | None) -> None:
    """Delete a Pod and verify absence before writing the local receipt."""
    before = pod_status(pod_id, key)
    response = None
    if before is not None:
        query = ("mutation { podTerminate(input: { podId: " +
                 json.dumps(pod_id) + " }) }")
        response = graphql(query, key)
    after = pod_status(pod_id, key)
    for _ in range(12):
        if after is None:
            break
        time.sleep(5)
        after = pod_status(pod_id, key)
    if output is not None:
        if response is not None:
            write(output / "provider_terminate.json", {
                "recorded_utc": now().isoformat(), "pod_id": pod_id,
                "response": response})
        write(output / "provider_post_terminate.json", {
            "recorded_utc": now().isoformat(), "pod_id": pod_id,
            "pod_present": after is not None})
    if after is not None:
        raise ValueError("Pod remains present after termination")


def watch(allocation_path: Path, host: str, port: int, output: Path,
          key: str) -> int:
    receipt = runpod_runner.read_allocation(allocation_path)
    pod_id = receipt["pod_id"]
    deadline = runpod_runner.timestamp(receipt["stop_after_utc"])
    retire_at = deadline - RETIRE_MARGIN
    output_ready = False
    transfer_recorded = False
    reason = "WATCHDOG_START_ERROR"
    transferred = False
    ready = False
    error_type = None
    try:
        output.mkdir(parents=True, exist_ok=False)
        output_ready = True
        write(output / "watchdog_started.json", {
            "schema": "n83.fixed31-watchdog-start/v1",
            "recorded_utc": now().isoformat(), "pod_id": pod_id,
            "allocation_sha256": provider.digest(allocation_path),
            "retire_at_utc": retire_at.isoformat(),
            "provider_deadline_utc": deadline.isoformat(),
        })
        if now() >= retire_at:
            reason = "LATE_WATCHDOG_START"
        else:
            verify_live(receipt, pod_status(pod_id, key))
            write(output / "watchdog_ready.json", {
                "schema": "n83.fixed31-watchdog-ready/v1",
                "recorded_utc": now().isoformat(), "pod_id": pod_id,
                "allocation_sha256": provider.digest(allocation_path),
                "retire_at_utc": retire_at.isoformat(),
            })
            reason = "PROVIDER_DEADLINE"
            while now() < retire_at:
                try:
                    status = remote_status(host, port)
                except (OSError, subprocess.TimeoutExpired):
                    status = ""
                if status.startswith("DONE "):
                    reason = "PRODUCER_DONE"
                    break
                time.sleep(min(
                    POLL_SECONDS,
                    max(0, (retire_at - now()).total_seconds())))
            seconds_left = int((retire_at - now()).total_seconds())
            if seconds_left > 90:
                try:
                    transferred = spool(
                        host, port, output,
                        timeout=min(300, seconds_left - 60))
                except (OSError, subprocess.TimeoutExpired):
                    transferred = False
            ready = transferred and archive_ready(output)
    except Exception as error:
        reason = "WATCHDOG_ERROR"
        error_type = type(error).__name__
    finally:
        if output_ready:
            try:
                write(output / "watchdog_transfer.json", {
                    "recorded_utc": now().isoformat(), "pod_id": pod_id,
                    "reason": reason, "spool_succeeded": transferred,
                    "archive_ready": ready, "error_type": error_type,
                })
                transfer_recorded = True
            except Exception as error:
                reason = "WATCHDOG_RECEIPT_WRITE_ERROR"
                ready = False
                error_type = type(error).__name__
        try:
            terminate(pod_id, key, output if output_ready else None)
        except Exception as error:
            if output_ready:
                try:
                    write(output / "provider_termination_error.json", {
                        "recorded_utc": now().isoformat(),
                        "pod_id": pod_id,
                        "error_type": type(error).__name__,
                    })
                except Exception:
                    pass
            raise
    return (0 if reason == "PRODUCER_DONE" and ready and transfer_recorded
            else 2)

def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--allocation", type=Path, required=True)
    parser.add_argument("--ssh-host", required=True)
    parser.add_argument("--ssh-port", type=int, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    key = getpass.getpass("RunPod API key: ")
    return watch(args.allocation.resolve(strict=True), args.ssh_host,
                 args.ssh_port, args.output.resolve(), key)


if __name__ == "__main__":
    raise SystemExit(main())
