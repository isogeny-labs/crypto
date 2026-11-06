#!/usr/bin/env python3
"""Check the external watchdog binding before running work on a Pod."""

import argparse
from datetime import datetime, timedelta, timezone
import json
from pathlib import Path

from research.n83_highk_beyond4405_20261021 import run
from research.n83_highk_beyond4405_20261021 import runpod_runner


def verify(receipt: dict, ready: dict, allocation_sha256: str,
           at: datetime) -> None:
    recorded = runpod_runner.timestamp(ready["recorded_utc"])
    retire_at = runpod_runner.timestamp(ready["retire_at_utc"])
    deadline = runpod_runner.timestamp(receipt["stop_after_utc"])
    if (ready.get("schema") != "n83.fixed31-watchdog-ready/v1" or
            ready.get("pod_id") != receipt["pod_id"] or
            ready.get("allocation_sha256") != allocation_sha256 or
            recorded > at or (at - recorded).total_seconds() > 1800 or
            recorded < runpod_runner.timestamp(receipt["created_utc"]) or
            retire_at != deadline - timedelta(minutes=5) or
            at >= retire_at):
        raise ValueError("external watchdog readiness differs from allocation")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--allocation", type=Path, required=True)
    parser.add_argument("--ready", type=Path, required=True)
    args = parser.parse_args()
    allocation = args.allocation.resolve(strict=True)
    ready = json.loads(args.ready.read_text())
    verify(runpod_runner.read_allocation(allocation), ready,
           run.digest(allocation), datetime.now(timezone.utc))
    print("PASS_WATCHDOG_READY")


if __name__ == "__main__":
    main()
