#!/usr/bin/env python3
"""Verify the tie-break allocation against its sealed provider snapshot."""

import argparse
import hashlib
import json
from datetime import datetime
from decimal import Decimal
from pathlib import Path

from research.n83_highk_online_distribution_20261020 import runpod_provider as prior
from research.n83_highk_online_tiebreak_20261020 import runpod_runner

PRIOR_DISTRIBUTION_POD_ID = "yajulte7twpr9p"
PRIOR_DISTRIBUTION_CREATED_UTC = "2026-10-20T01:19:40.513Z"
PRIOR_DISTRIBUTION_RATE_USD_PER_HOUR = Decimal("0.19")
EARLIER_RESERVED_UPPER_USD = Decimal("3.57")


def timestamp(value):
    result = datetime.fromisoformat(value.replace("Z", "+00:00"))
    if result.tzinfo is None:
        raise ValueError("provider timestamp lacks timezone")
    return result


def controller_digest():
    path = Path(__file__).with_name("allocate_runpod.py")
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify(snapshot_path, results, head_sha):
    snapshot = json.loads(snapshot_path.read_text())
    trigger_path = results / "trigger.json"
    allocation_path = results / "runpod_allocation.json"
    trigger = runpod_runner.run.validate_trigger(trigger_path)
    allocation = runpod_runner.read_allocation(allocation_path, trigger_path)
    termination_path = snapshot_path.parent / "prior_distribution_termination.json"
    prior_pods_path = snapshot_path.parent / "prior_pods.json"
    termination = json.loads(termination_path.read_text())
    prior_pods = json.loads(prior_pods_path.read_text())
    if (snapshot.get("schema") != "n83.online-tiebreak-runpod-snapshot/v1" or
            snapshot.get("source_head_sha") != head_sha or
            snapshot.get("controller_sha256") != controller_digest() or
            snapshot.get("prior_distribution_pod_id") !=
            PRIOR_DISTRIBUTION_POD_ID or
            snapshot.get("prior_distribution_status") != "TERMINATED" or
            termination.get("pod_present") is not False or
            snapshot.get("prior_termination_sha256") != prior.digest(termination_path) or
            snapshot.get("prior_pods_sha256") != prior.digest(prior_pods_path) or
            any(pod.get("id") == PRIOR_DISTRIBUTION_POD_ID or
                pod.get("name") == "n83-online-tiebreak-20261020"
                for pod in prior_pods.get("pods", [])) or
            snapshot.get("trigger_receipt_sha256") != prior.digest(trigger_path) or
            snapshot.get("trigger_aggregate_sha256") !=
            trigger["trigger_aggregate_sha256"] or
            snapshot.get("trigger_archive_sha256") !=
            trigger["trigger_archive_sha256"] or
            len(snapshot.get("pods", [])) != 1):
        raise ValueError("tie-break provider snapshot differs")
    pod = snapshot["pods"][0]
    if (any(allocation.get(key) != pod.get(key) for key in
            ("stratum", "pod_id", "machine_id", "desired_status", "cost_per_hr",
             "image_name", "vcpu_count", "memory_in_gb", "cloud_type",
             "compute_type", "gpu_type_id", "gpu_count", "created_utc",
             "stop_after_utc", "cpu_affinity_ids")) or
            allocation["source_head_sha"] != head_sha or
            allocation["snapshot_sha256"] != prior.digest(snapshot_path)):
        raise ValueError("allocation differs from provider snapshot")
    prior_spend = Decimal(str(snapshot["prior_spend_upper_usd"]))
    campaign_cap = Decimal(str(snapshot["approved_combined_cap_usd"]))
    recorded = timestamp(snapshot["recorded_utc"])
    distribution_started = timestamp(PRIOR_DISTRIBUTION_CREATED_UTC)
    if not (distribution_started <= timestamp(termination["recorded_utc"]) <=
            timestamp(prior_pods["recorded_utc"]) <= recorded):
        raise ValueError("prior Pod termination chronology differs")
    distribution_upper = (PRIOR_DISTRIBUTION_RATE_USD_PER_HOUR *
                          Decimal(str((recorded - distribution_started).total_seconds())) /
                          Decimal(3600))
    if (distribution_upper <= 0 or
            prior_spend < EARLIER_RESERVED_UPPER_USD + distribution_upper):
        raise ValueError("prior campaign spend upper bound is incomplete")
    projected = (Decimal(str(pod["cost_per_hr"])) *
                 Decimal(str((timestamp(pod["stop_after_utc"]) -
                              timestamp(pod["created_utc"])).total_seconds())) /
                 Decimal(3600))
    if (projected <= 0 or
            prior_spend + projected > campaign_cap or campaign_cap != Decimal("5")):
        raise ValueError("tie-break allocation exceeds campaign cap")
    host = json.loads((results / "host.json").read_text())
    source = json.loads((results / "source.json").read_text())
    after = json.loads((results / "screen/host_telemetry_after.json").read_text())
    adapter_hash = prior.digest(Path(runpod_runner.__file__))
    if (host.get("provider") != "runpod" or
            host.get("git_sha") != head_sha or
            host.get("runpod_pod_id") != pod["pod_id"] or
            host.get("runpod_machine_id") != pod["machine_id"] or
            host.get("runpod_frozen_cpu_affinity") != pod["cpu_affinity_ids"] or
            host.get("visible_affinity") != pod["cpu_affinity_ids"] or
            host.get("runpod_allocation_sha256") != prior.digest(allocation_path) or
            host.get("runpod_adapter_sha256") != adapter_hash or
            host.get("instance_id") !=
            f"rp-{pod['pod_id']}-{pod['machine_id']}" or
            source.get("runpod_allocation_sha256") != prior.digest(allocation_path) or
            source.get("runpod_adapter_sha256") != adapter_hash or
            not (recorded <=
                 timestamp(host["recorded_utc"]) <=
                 timestamp(after["recorded_utc"]) <=
                 timestamp(pod["stop_after_utc"]))):
        raise ValueError("tie-break provider binding differs")
    return {"schema": "n83.online-tiebreak-runpod-provider-proof/v1",
            "status": "PASS_SINGLE_RECORDED_RUNPOD_ALLOCATION",
            "snapshot_sha256": prior.digest(snapshot_path),
            "source_head_sha": head_sha,
            "projected_new_spend_usd": str(projected),
            "prior_spend_upper_usd": str(prior_spend),
            "earlier_reserved_upper_usd": str(EARLIER_RESERVED_UPPER_USD),
            "distribution_charge_upper_usd": str(distribution_upper),
            "pod_id": pod["pod_id"], "machine_id": pod["machine_id"],
            "cpu_affinity_ids": pod["cpu_affinity_ids"],
            "allocation_sha256": prior.digest(allocation_path)}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--snapshot", type=Path, required=True)
    parser.add_argument("--results", type=Path, required=True)
    parser.add_argument("--head-sha", required=True)
    args = parser.parse_args()
    print(json.dumps(verify(args.snapshot, args.results, args.head_sha), sort_keys=True))


if __name__ == "__main__":
    main()
