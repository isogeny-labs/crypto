#!/usr/bin/env python3
"""Bind the fixed31 producer to one frozen RunPod allocation receipt."""

import argparse
from datetime import datetime
from decimal import Decimal
import json
import os
import platform
import shutil
import subprocess
from pathlib import Path

from research.n83_highk_beyond4405_20261021 import run
from research.n83_highk_online_distribution_20261020 import runpod_runner as prior

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def timestamp(value: str) -> datetime:
    result = datetime.fromisoformat(value.replace("Z", "+00:00"))
    if result.tzinfo is None:
        raise ValueError("allocation timestamp lacks timezone")
    return result


def read_allocation(path: Path) -> dict:
    receipt = json.loads(path.read_text())
    dispatch = json.loads((HERE / "dispatch.json").read_text())
    if (receipt.get("schema") != "n83.fixed31-runpod-allocation/v1" or
            receipt.get("approval_status") != "APPROVED_NEW_CAP" or
            receipt.get("desired_status") != "RUNNING" or
            receipt.get("source_sha256") != dispatch["source_sha256"] or
            receipt.get("dispatch_sha256") != run.digest(HERE / "dispatch.json") or
            not isinstance(receipt.get("pod_id"), str) or
            not receipt["pod_id"].isalnum() or
            not isinstance(receipt.get("machine_id"), str) or
            not receipt["machine_id"] or
            receipt.get("cpu_affinity_ids") !=
            sorted(set(receipt.get("cpu_affinity_ids", []))) or
            len(receipt["cpu_affinity_ids"]) != 4 or
            any(not isinstance(cpu, int) or isinstance(cpu, bool) or cpu < 0
                for cpu in receipt["cpu_affinity_ids"]) or
            receipt.get("vcpu_count", 0) < 4 or
            receipt.get("memory_in_gb", 0) < 32 or
            not isinstance(receipt.get("source_head_sha"), str) or
            len(receipt["source_head_sha"]) != 40 or
            not isinstance(receipt.get("snapshot_sha256"), str) or
            len(receipt["snapshot_sha256"]) != 64 or
            any(char not in "0123456789abcdef"
                for char in receipt["snapshot_sha256"])):
        raise ValueError("fixed31 RunPod allocation identity differs")
    rate = Decimal(str(receipt["cost_per_hr"]))
    created = timestamp(receipt["created_utc"])
    stop = timestamp(receipt["stop_after_utc"])
    hours = Decimal(str((stop - created).total_seconds())) / Decimal(3600)
    if (rate <= 0 or rate > Decimal(dispatch["max_quoted_usd_per_hour"]) or
            hours <= 0 or hours > Decimal(10) or
            rate * hours + Decimal(dispatch["billing_reserve_usd"]) >
            Decimal(dispatch["incremental_upper_usd"])):
        raise ValueError("fixed31 RunPod allocation exceeds budget envelope")
    return receipt


def bind_provider(receipt: dict, receipt_path: Path, head_sha: str) -> None:
    original_snapshot = run.parent.cgroup_snapshot
    if prior.CPU_V1.joinpath("cpu.stat").exists():
        snapshot_fn = lambda _group: prior.v1_snapshot()
        interface = "v1"
    else:
        def snapshot_fn(group):
            snapshot = original_snapshot(group)
            if any(snapshot.get(key) is None for key in
                   ("cpu.stat", "cpu.max", "memory.current", "memory.peak",
                    "memory.max", "memory.events")):
                raise ValueError("cgroup v2 telemetry incomplete")
            return {**snapshot, "cgroup_interface": "v2"}
        interface = "v2"
    snapshot = snapshot_fn(run.parent.cgroup_root())
    if sorted(os.sched_getaffinity(0)) != receipt["cpu_affinity_ids"]:
        raise ValueError("producer CPU affinity differs from allocation")
    quota, period = snapshot["cpu.max"].split()
    if (quota == "max" or int(quota) < 4 * int(period) or
            int(snapshot["memory.max"]) < 32 * 1024**3):
        raise ValueError("producer cgroup envelope below frozen requirement")
    original_host = run.parent.host_receipt
    original_write = run.parent.write_json
    adapter_hash = run.digest(Path(__file__))
    allocation_hash = run.digest(receipt_path)
    run.parent.cgroup_snapshot = snapshot_fn

    def host_receipt(group):
        host = original_host(group)
        host.update({
            "provider": "runpod",
            "instance_id": f"rp-{receipt['pod_id']}-{receipt['machine_id']}",
            "run_url": f"https://www.runpod.io/console/pods/{receipt['pod_id']}",
            "git_sha": head_sha,
            "runner_name": receipt["pod_id"],
            "runner_os": platform.system(),
            "runner_arch": platform.machine(),
            "image_version": receipt.get("image_name"),
            "runpod_pod_id": receipt["pod_id"],
            "runpod_machine_id": receipt["machine_id"],
            "runpod_frozen_cpu_affinity": receipt["cpu_affinity_ids"],
            "runpod_allocation_sha256": allocation_hash,
            "runpod_adapter_sha256": adapter_hash,
            "cgroup_interface": interface,
        })
        return host

    def write_json(path, value):
        if Path(path).name == "source.json":
            value = {**value, "runpod_adapter_sha256": adapter_hash,
                     "runpod_allocation_sha256": allocation_hash}
        return original_write(path, value)

    run.parent.host_receipt = host_receipt
    run.parent.write_json = write_json


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--rho-binary", type=Path, required=True)
    parser.add_argument("--provider-receipt", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    receipt_path = args.provider_receipt.resolve()
    receipt = read_allocation(receipt_path)
    head_sha = subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
    if head_sha != receipt["source_head_sha"]:
        raise ValueError("Pod checkout differs from allocation")
    if os.environ.get("RUNPOD_POD_ID") not in (None, receipt["pod_id"]):
        raise ValueError("Pod environment differs from allocation")
    bind_provider(receipt, receipt_path, head_sha)
    output = args.output.resolve()
    try:
        return run.run_screen(output, args.binary.resolve(),
                              args.rho_binary.resolve())
    finally:
        if output.is_dir():
            shutil.copy2(receipt_path, output / "runpod_allocation.json")


if __name__ == "__main__":
    raise SystemExit(main())
