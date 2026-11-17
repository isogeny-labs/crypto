#!/usr/bin/env python3
"""Fail closed before a full K5500 child is built or launched on EKS."""

import argparse
from datetime import datetime, timezone
from decimal import Decimal, InvalidOperation
import hashlib
import json
import os
from pathlib import Path
import re
import shutil

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
RUNNER = ROOT / "research/n83_highk_beyond4405_20261021/run_k5500_full_control.py"
SOURCE_SHA256 = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8"
RUNNER_SHA256 = "cffb72ba0079035228012eb4f129934dd6227e7ea05707384006ea47855ae812"
MIN_MEMORY_BYTES = 48 * 1024**3
MIN_DISK_BYTES = 8 * 1024**3
MAX_WORKFLOW_SECONDS = 5 * 3600
CAPACITY_CHECKS = frozenset({
    "source_sha256", "runner_present", "memory_limit_at_least_48_gib",
    "host_memory_at_least_48_gib", "visible_cpus_at_least_4",
    "cpu_quota_at_least_4", "disk_free_at_least_8_gib",
})


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def read(path):
    try:
        return Path(path).read_text().strip()
    except OSError:
        return None


def timestamp(value):
    if not isinstance(value, str):
        raise ValueError("approval timestamp is missing")
    result = datetime.fromisoformat(value.replace("Z", "+00:00"))
    if result.tzinfo is None:
        raise ValueError("approval timestamp lacks timezone")
    return result


def decimal(value, label):
    if not isinstance(value, str):
        raise ValueError(f"{label} must be an exact decimal string")
    try:
        result = Decimal(value)
    except (InvalidOperation, TypeError):
        raise ValueError(f"{label} is not an exact decimal") from None
    if not result.is_finite() or result < 0:
        raise ValueError(f"{label} is negative or nonfinite")
    return result


def validate(approval, capacity, capacity_sha256, now):
    if not isinstance(approval, dict) or not isinstance(capacity, dict):
        raise ValueError("approval and capacity must be JSON objects")
    if (approval.get("schema") != "n83.k5500-highmem-full-approval/v1" or
            approval.get("status") != "APPROVED" or
            approval.get("runner_label") != "crypto-eks-highmem" or
            approval.get("source_sha256") != SOURCE_SHA256 or
            approval.get("runner_script_sha256") != RUNNER_SHA256 or
            not isinstance(approval.get("capacity_head_sha"), str) or
            not re.fullmatch("[0-9a-f]{40}", approval["capacity_head_sha"]) or
            not isinstance(approval.get("approved_by"), str) or
            not approval["approved_by"].strip() or
            not isinstance(approval.get("quote_source"), str) or
            not approval["quote_source"].strip() or
            not isinstance(approval.get("capacity_artifact_id"), int) or
            isinstance(approval["capacity_artifact_id"], bool) or
            approval["capacity_artifact_id"] <= 0 or
            not isinstance(approval.get("capacity_run_id"), int) or
            isinstance(approval["capacity_run_id"], bool) or
            approval["capacity_run_id"] <= 0 or
            approval.get("capacity_receipt_sha256") != capacity_sha256 or
            not isinstance(approval.get("max_allocation_seconds"), int) or
            isinstance(approval["max_allocation_seconds"], bool) or
            approval["max_allocation_seconds"] != MAX_WORKFLOW_SECONDS):
        raise ValueError("K5500 high-memory approval identity or cap differs")
    if not timestamp(approval["approved_utc"]) <= now < timestamp(
            approval["expires_utc"]):
        raise ValueError("K5500 high-memory approval is not current")
    rate = decimal(approval["quoted_usd_per_hour"], "quoted rate")
    reserve = decimal(approval["billing_reserve_usd"], "billing reserve")
    upper = decimal(approval["approved_upper_usd"], "approved upper")
    charged = rate * Decimal(approval["max_allocation_seconds"]) / Decimal(3600)
    if rate == 0 or upper == 0 or charged + reserve > upper:
        raise ValueError("K5500 quote exceeds separate approved upper bound")
    checks = capacity.get("checks")
    if (not isinstance(checks, dict) or
            not CAPACITY_CHECKS.issubset(checks) or
            any(value is not True for value in checks.values())):
        raise ValueError("capacity receipt lacks passing required checks")
    if (capacity.get("schema") != "n83.k5500-existing-highmem-capacity/v1" or
            capacity.get("status") != "PASS_CAPACITY_PREFLIGHT" or
            capacity.get("experiment_launched") is not False or
            capacity.get("github_run_id") != str(approval["capacity_run_id"]) or
            capacity.get("github_sha") != approval.get("capacity_head_sha") or
            capacity.get("source_sha256") != SOURCE_SHA256 or
            capacity.get("runner_script_sha256") != RUNNER_SHA256):
        raise ValueError("capacity artifact does not admit K5500 full run")
    return {"approved_upper_usd": str(upper),
            "quoted_usd_per_hour": str(rate),
            "billing_reserve_usd": str(reserve),
            "max_allocation_seconds": approval["max_allocation_seconds"],
            "quote_source": approval["quote_source"],
            "capacity_run_id": approval["capacity_run_id"],
            "capacity_artifact_id": approval["capacity_artifact_id"]}


def host_envelope():
    v2 = read("/sys/fs/cgroup/memory.max")
    interface = "v2" if v2 is not None else "v1"
    raw_memory = v2 if v2 is not None else read(
        "/sys/fs/cgroup/memory/memory.limit_in_bytes")
    meminfo = read("/proc/meminfo") or ""
    host_bytes = next((int(line.split()[1]) * 1024 for line in meminfo.splitlines()
                       if line.startswith("MemTotal:")), None)
    effective = (int(raw_memory) if raw_memory and raw_memory.isdigit()
                 else host_bytes if raw_memory == "max" else None)
    cpu_max = read("/sys/fs/cgroup/cpu.max")
    if cpu_max is None:
        quota = read("/sys/fs/cgroup/cpu/cpu.cfs_quota_us")
        period = read("/sys/fs/cgroup/cpu/cpu.cfs_period_us")
        cpu_max = f"{quota} {period}" if quota and period else None
    quota_cores = None
    if cpu_max:
        parts = cpu_max.split()
        if len(parts) == 2 and parts[0] not in ("max", "-1"):
            quota_cores = int(parts[0]) / int(parts[1])
    affinity = sorted(os.sched_getaffinity(0))
    disk = shutil.disk_usage(os.environ["RUNNER_TEMP"]).free
    checks = {
        "source_sha256": digest(SOURCE) == SOURCE_SHA256,
        "runner_sha256": digest(RUNNER) == RUNNER_SHA256,
        "cgroup_memory_at_least_48_gib":
            effective is not None and effective >= MIN_MEMORY_BYTES,
        "host_memory_at_least_48_gib":
            host_bytes is not None and host_bytes >= MIN_MEMORY_BYTES,
        "visible_cpus_at_least_4": len(affinity) >= 4,
        "cpu_quota_at_least_4": cpu_max is not None and
            (quota_cores is None or quota_cores >= 4),
        "free_disk_at_least_8_gib": disk >= MIN_DISK_BYTES,
    }
    return {"schema": "n83.k5500-highmem-full-host/v1",
            "status": "PASS_ENVELOPE" if all(checks.values()) else
                      "REJECTED_ENVELOPE", "checks": checks,
            "cgroup_interface": interface, "memory_max": raw_memory,
            "host_mem_total_bytes": host_bytes, "cpu_max": cpu_max,
            "visible_affinity": affinity, "disk_free_bytes": disk,
            "runner_name": os.environ.get("RUNNER_NAME"),
            "github_sha": os.environ.get("GITHUB_SHA")}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--approval", type=Path, required=True)
    parser.add_argument("--capacity", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = {"schema": "n83.k5500-highmem-full-preflight/v1",
              "status": "REJECTED"}
    try:
        approval = json.loads(args.approval.read_text())
        capacity = json.loads(args.capacity.read_text())
        result["approval"] = validate(approval, capacity,
                                      digest(args.capacity),
                                      datetime.now(timezone.utc))
        result["host"] = host_envelope()
        if result["host"]["status"] != "PASS_ENVELOPE":
            raise ValueError("live runner envelope below K5500 requirement")
        result["status"] = "PASS_APPROVAL_AND_LIVE_CAPACITY"
    except (OSError, ValueError, KeyError, TypeError, ZeroDivisionError) as error:
        result["error"] = str(error)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"status": result["status"],
                      "error": result.get("error")}, sort_keys=True))
    return 0 if result["status"] == "PASS_APPROVAL_AND_LIVE_CAPACITY" else 2


if __name__ == "__main__":
    raise SystemExit(main())
