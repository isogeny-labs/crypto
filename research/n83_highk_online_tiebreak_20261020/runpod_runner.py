#!/usr/bin/env python3
"""Bind the conditional tie-break producer to one frozen RunPod allocation."""

import argparse
import json
import os
import platform
import shutil
import subprocess
from datetime import datetime
from pathlib import Path

from research.n83_highk_online_distribution_20261020 import runpod_runner as prior_adapter
from research.n83_highk_online_tiebreak_20261020 import run

ROOT = Path(__file__).resolve().parents[2]
SOURCE_HEAD_SHA = "e7634a81c22b60ad37e5d6bdb817a3b84867ae86"


def read_allocation(path, trigger_path):
    receipt = json.loads(path.read_text())
    trigger = run.validate_trigger(trigger_path)
    if (receipt.get("schema") != "n83.online-tiebreak-runpod-allocation/v1" or
            receipt.get("stratum") != 0 or
            receipt.get("desired_status") != "RUNNING" or
            receipt.get("cloud_type") != "COMMUNITY" or
            receipt.get("compute_type") != "GPU" or
            receipt.get("gpu_type_id") not in prior_adapter.runpod_provider.GPU_TYPES or
            receipt.get("gpu_count") != 1 or
            receipt.get("source_head_sha") != SOURCE_HEAD_SHA or
            not isinstance(receipt.get("pod_id"), str) or
            not receipt["pod_id"].isalnum() or
            not isinstance(receipt.get("machine_id"), str) or
            not receipt["machine_id"] or
            not isinstance(receipt.get("cost_per_hr"), (int, float)) or
            not 0 < receipt["cost_per_hr"] <= 0.20 or
            receipt.get("vcpu_count", 0) < 4 or
            receipt.get("memory_in_gb", 0) < 27 or
            receipt.get("cpu_affinity_ids") !=
            sorted(set(receipt.get("cpu_affinity_ids", []))) or
            len(receipt["cpu_affinity_ids"]) != 4 or
            any(not isinstance(cpu, int) or isinstance(cpu, bool) or cpu < 0 for cpu in
                receipt["cpu_affinity_ids"]) or
            receipt.get("trigger_receipt_sha256") != run.parent.digest(trigger_path) or
            receipt.get("trigger_aggregate_sha256") !=
            trigger["trigger_aggregate_sha256"] or
            receipt.get("trigger_archive_sha256") !=
            trigger["trigger_archive_sha256"] or
            not isinstance(receipt.get("snapshot_sha256"), str) or
            len(receipt["snapshot_sha256"]) != 64 or
            any(ch not in "0123456789abcdef" for ch in
                receipt["snapshot_sha256"])):
        raise ValueError("tie-break RunPod allocation differs")
    created = datetime.fromisoformat(receipt["created_utc"].replace("Z", "+00:00"))
    stop = datetime.fromisoformat(receipt["stop_after_utc"].replace("Z", "+00:00"))
    if (created.tzinfo is None or stop.tzinfo is None or
            not 0 < (stop - created).total_seconds() <= 5 * 3600):
        raise ValueError("tie-break RunPod stop cap differs")
    return receipt


def bind_provider(receipt, receipt_path, head_sha):
    snapshot = prior_adapter.v1_snapshot()
    if sorted(os.sched_getaffinity(0)) != receipt["cpu_affinity_ids"]:
        raise ValueError("producer CPU affinity differs from allocation")
    quota, period = snapshot["cpu.max"].split()
    if (quota == "max" or int(quota) < 4 * int(period) or
            int(snapshot["memory.max"]) < run.RSS_CAP_BYTES + 256 * 1024**2):
        raise ValueError("producer cgroup envelope below frozen requirement")
    original_host = run.parent.host_receipt
    original_write = run.parent.write_json
    adapter_hash = run.parent.digest(Path(__file__))
    allocation_hash = run.parent.digest(receipt_path)
    run.parent.cgroup_snapshot = lambda _group: prior_adapter.v1_snapshot()

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
            "cgroup_interface": "v1",
        })
        return host

    def write_json(path, value):
        if Path(path).name == "source.json":
            value = {**value, "runpod_adapter_sha256": adapter_hash,
                     "runpod_allocation_sha256": allocation_hash}
        return original_write(path, value)

    run.parent.host_receipt = host_receipt
    run.parent.write_json = write_json


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--rho-binary", type=Path, required=True)
    parser.add_argument("--trigger-receipt", type=Path, required=True)
    parser.add_argument("--provider-receipt", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    receipt_path = args.provider_receipt.resolve()
    trigger_path = args.trigger_receipt.resolve()
    receipt = read_allocation(receipt_path, trigger_path)
    if os.environ.get("RUNPOD_POD_ID") not in (None, receipt["pod_id"]):
        raise ValueError("Pod environment differs from allocation")
    head_sha = subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
    if head_sha != receipt.get("source_head_sha"):
        raise ValueError("Pod checkout differs from frozen source")
    run.fixture()
    run.arms.choose()
    bind_provider(receipt, receipt_path, head_sha)
    output = args.output.resolve()
    try:
        return run.run_screen(output, args.binary.resolve(),
                              args.rho_binary.resolve(), trigger_path)
    finally:
        if output.is_dir():
            shutil.copy2(receipt_path, output / "runpod_allocation.json")


if __name__ == "__main__":
    raise SystemExit(main())
