#!/usr/bin/env python3
"""Delete the fixed31 Pod at its frozen deadline if the local watcher dies."""

import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
import subprocess
import time

from research.n83_highk_beyond4405_20261021 import run
from research.n83_highk_beyond4405_20261021 import runpod_runner
from research.n83_highk_beyond4405_20261021 import verify_watchdog_ready


def write(path, value):
    path.write_text(json.dumps(value, sort_keys=True) + "\n")


def guard(allocation_path, ready_path, receipt_path, clock=None,
          pause=time.sleep, delete=None):
    clock = clock or (lambda: datetime.now(timezone.utc))
    delete = delete or (lambda pod_id: subprocess.run(
        ["runpodctl", "pod", "delete", pod_id], capture_output=True,
        timeout=30, check=True))
    allocation = runpod_runner.read_allocation(allocation_path)
    ready = json.loads(ready_path.read_text())
    verify_watchdog_ready.verify(allocation, ready, run.digest(allocation_path),
                                 clock())
    deadline = runpod_runner.timestamp(allocation["stop_after_utc"])
    write(receipt_path, {
        "schema": "n83.fixed31-pod-deadline-guard/v1", "status": "ARMED",
        "pod_id": allocation["pod_id"], "stop_after_utc":
        allocation["stop_after_utc"],
        "allocation_sha256": run.digest(allocation_path),
        "recorded_utc": clock().isoformat(),
    })
    while clock() < deadline:
        pause(min(60, max(0.1, (deadline - clock()).total_seconds())))
    while True:
        try:
            delete(allocation["pod_id"])
            write(receipt_path, {
                "schema": "n83.fixed31-pod-deadline-guard/v1",
                "status": "DELETION_REQUESTED", "pod_id": allocation["pod_id"],
                "recorded_utc": clock().isoformat(),
            })
            return
        except Exception as error:
            write(receipt_path, {
                "schema": "n83.fixed31-pod-deadline-guard/v1",
                "status": "RETRYING_DELETION", "pod_id": allocation["pod_id"],
                "error_type": type(error).__name__,
                "recorded_utc": clock().isoformat(),
            })
            pause(15)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--allocation", type=Path, required=True)
    parser.add_argument("--ready", type=Path, required=True)
    parser.add_argument("--receipt", type=Path, required=True)
    args = parser.parse_args()
    guard(args.allocation, args.ready, args.receipt)


if __name__ == "__main__":
    main()
