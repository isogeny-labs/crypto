#!/usr/bin/env python3
"""Allocate one bounded A40 Pod using the established REST custody guard."""

import argparse
from decimal import Decimal
import json
import os
from pathlib import Path
import stat
import tomllib

from research.n83_two_tier_root_20261021 import runpod_control, runpod_guard


GPU = "NVIDIA A40"
MAX_RATE = Decimal("0.70")
APPROVED_CAP = Decimal("2.25")
MAX_SECONDS = 9000
NAME_PREFIX = "n83-maxk-split4-"


def api_key() -> str:
    key = os.environ.get("RUNPOD_API_KEY", "").strip()
    if key:
        return key
    config = Path.home() / ".runpod/config.toml"
    if not config.is_file():
        raise ValueError("RunPod API key is not configured")
    if stat.S_IMODE(config.stat().st_mode) & 0o077:
        raise ValueError("RunPod CLI config must be private to its owner")
    key = tomllib.loads(config.read_text()).get("apikey", "").strip()
    if not key:
        raise ValueError("RunPod API key is not configured")
    return key


def preview(name: str, public_key: str) -> dict:
    if not name.startswith(NAME_PREFIX):
        raise ValueError("Pod name must identify the split4 campaign")
    if not public_key.startswith(("ssh-ed25519 ", "ssh-rsa ")):
        raise ValueError("public SSH key differs")
    body = runpod_control.pod_body(name, public_key)
    return {
        "schema": "n83.k5500-split4-runpod-preview/v1",
        "pod_request": body,
        "gpu_type": GPU,
        "maximum_provider_rate_usd_per_hour": str(MAX_RATE),
        "maximum_lifetime_seconds": MAX_SECONDS,
        "approved_cap_usd": str(APPROVED_CAP),
        "maximum_compute_charge_usd": str(MAX_RATE * MAX_SECONDS / 3600),
        "storage_and_billing_reserve_usd": str(
            APPROVED_CAP - MAX_RATE * MAX_SECONDS / 3600
        ),
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--name", required=True)
    parser.add_argument("--public-key", type=Path,
                        default=Path.home() / ".ssh/id_ed25519.pub")
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()
    public_key = args.public_key.read_text().strip()
    runpod_control.GPU = GPU
    runpod_guard.MAX_RATE = MAX_RATE
    plan = preview(args.name, public_key)
    if args.dry_run:
        print(json.dumps(plan, indent=2, sort_keys=True))
        return 0
    return runpod_control.create_and_guard(
        api_key(), args.run_dir.resolve(), args.name, public_key,
        APPROVED_CAP, MAX_SECONDS,
    )


if __name__ == "__main__":
    raise SystemExit(main())
