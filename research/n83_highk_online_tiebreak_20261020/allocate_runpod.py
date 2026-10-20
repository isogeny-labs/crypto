#!/usr/bin/env python3
"""Allocate one budgeted Pod only after the prior N83 screen is replayed."""

import argparse
import datetime as dt
import getpass
import hashlib
import importlib
import json
from decimal import Decimal
from pathlib import Path

from research.n83_highk_online_tiebreak_20261020 import check_trigger
from research.n83_highk_online_tiebreak_20261020 import runpod_provider
from research.n83_highk_online_tiebreak_20261020 import runpod_runner

GPU_TYPES = (
    "NVIDIA GeForce RTX 3070", "NVIDIA RTX A5000", "NVIDIA RTX A4000",
    "NVIDIA RTX A4500", "NVIDIA RTX 4000 SFF Ada Generation",
    "NVIDIA GeForce RTX 3080", "NVIDIA GeForce RTX 3080 Ti",
    "NVIDIA GeForce RTX 4070 Ti", "NVIDIA RTX 4000 Ada Generation",
    "NVIDIA RTX A2000", "Tesla V100-PCIE-16GB",
)
POD_NAME = "n83-online-tiebreak-20261020"
POD_CAP_SECONDS = 4 * 3600
PRICE_CAP = Decimal("0.19")
CAMPAIGN_CAP = Decimal("5.00")


def utc_now():
    return dt.datetime.now(dt.timezone.utc)


def utc_text(value):
    return value.replace(microsecond=0).isoformat().replace("+00:00", "Z")


def digest(path):
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def prior_upper(at):
    created = runpod_provider.timestamp(
        runpod_provider.PRIOR_DISTRIBUTION_CREATED_UTC)
    seconds = Decimal(str((at - created).total_seconds()))
    if seconds <= 0:
        raise ValueError("time precedes prior distribution allocation")
    return (runpod_provider.EARLIER_RESERVED_UPPER_USD +
            runpod_provider.PRIOR_DISTRIBUTION_RATE_USD_PER_HOUR *
            seconds / Decimal(3600))


def preflight(prior_aggregate, prior_archive, prior_termination, output):
    trigger = check_trigger.check(prior_aggregate, prior_archive)
    terminated = json.loads(prior_termination.read_text())
    if (terminated.get("pod_present") is not False or
            terminated.get("pod_id") !=
            runpod_provider.PRIOR_DISTRIBUTION_POD_ID or
            runpod_provider.timestamp(terminated["recorded_utc"]) > utc_now() or
            output.exists()):
        raise ValueError("prior Pod is not proven absent or output already exists")
    upper = prior_upper(utc_now())
    if upper + PRICE_CAP * Decimal(POD_CAP_SECONDS) / Decimal(3600) > CAMPAIGN_CAP:
        raise ValueError("conservative combined Pod reservation exceeds $5")
    return trigger


def graphql(query, key):
    run = importlib.import_module("runpod.api.graphql").run_graphql_query
    return run(query, api_key=key)


def provider_pods(key):
    query = ("query { myself { pods { id name desiredStatus costPerHr "
             "createdAt machineId memoryInGb vcpuCount imageName gpuCount "
             "podType runtime { ports { ip isIpPublic privatePort publicPort "
             "type } } } } }")
    return graphql(query, key)["data"]["myself"]["pods"]


def stop_bad_allocation(pod_id, key):
    for action in ("podStop", "podTerminate"):
        mutation = ("mutation { " + action + "(input: { podId: " +
                    json.dumps(pod_id) + " }) { id desiredStatus } }")
        try:
            graphql(mutation, key)
        except Exception:
            pass


def allocate(prior_aggregate, prior_archive, prior_termination, output, key):
    trigger = preflight(prior_aggregate, prior_archive, prior_termination,
                        output)
    pods_before = provider_pods(key)
    if any(pod["id"] == runpod_provider.PRIOR_DISTRIBUTION_POD_ID or
           pod.get("name") == POD_NAME for pod in pods_before):
        raise ValueError("prior or duplicate tie-break Pod still exists")
    now = utc_now()
    stop = utc_text(now + dt.timedelta(seconds=POD_CAP_SECONDS))
    if prior_upper(now) + PRICE_CAP * Decimal(4) > CAMPAIGN_CAP:
        raise ValueError("allocation-time combined reservation exceeds $5")
    output.mkdir(parents=True, exist_ok=False)
    trigger_path = output / "trigger.json"
    trigger_path.write_text(json.dumps(trigger, indent=2, sort_keys=True) + "\n")
    termination_copy = output / "prior_distribution_termination.json"
    termination_copy.write_bytes(prior_termination.read_bytes())
    (output / "prior_pods.json").write_text(
        json.dumps({"recorded_utc": utc_text(now), "pods": pods_before},
                   indent=2, sort_keys=True) + "\n")
    errors = []
    for gpu in GPU_TYPES:
        query = (
            "mutation { podFindAndDeployOnDemand(input: { name: " +
            json.dumps(POD_NAME) + ", gpuTypeId: " + json.dumps(gpu) +
            ", gpuCount: 1, cloudType: COMMUNITY, minMemoryInGb: 27, "
            "minVcpuCount: 4, imageName: \"runpod/base:1.0.2-ubuntu2404\", "
            "templateId: \"runpod-ubuntu\", containerDiskInGb: 30, "
            "startSsh: true, ports: \"22/tcp\", stopAfter: " +
            json.dumps(stop) +
            " }) { id name desiredStatus costPerHr machineId memoryInGb "
            "vcpuCount imageName gpuCount createdAt podType } }")
        try:
            raw = graphql(query, key)["data"]["podFindAndDeployOnDemand"]
            if not raw or not raw.get("id"):
                raise ValueError("provider returned no allocated Pod")
            (output / "provider_api_raw.json").write_text(
                json.dumps(raw, indent=2, sort_keys=True) + "\n")
            recorded = utc_now()
            price = Decimal(str(raw["costPerHr"]))
            created = runpod_provider.timestamp(raw["createdAt"])
            projected = price * Decimal(str((runpod_provider.timestamp(stop) -
                                             created).total_seconds())) / Decimal(3600)
            valid = (raw["desiredStatus"] == "RUNNING" and
                     bool(raw.get("machineId")) and
                     raw.get("memoryInGb", 0) >= 27 and
                     raw.get("vcpuCount", 0) >= 4 and
                     raw.get("gpuCount") == 1 and
                     Decimal(0) < price <= PRICE_CAP and
                     0 < (runpod_provider.timestamp(stop) - created).total_seconds()
                     <= POD_CAP_SECONDS and
                     prior_upper(recorded) + projected <= CAMPAIGN_CAP)
            if not valid:
                stop_bad_allocation(raw["id"], key)
                raise ValueError("allocated Pod outside frozen envelope; stop requested")
            pod = {
                "stratum": 0, "pod_id": raw["id"],
                "machine_id": raw["machineId"], "desired_status": "RUNNING",
                "cost_per_hr": raw["costPerHr"], "image_name": raw["imageName"],
                "vcpu_count": raw["vcpuCount"],
                "memory_in_gb": raw["memoryInGb"],
                "cloud_type": "COMMUNITY", "compute_type": "GPU",
                "gpu_type_id": gpu, "gpu_count": raw["gpuCount"],
                "created_utc": raw["createdAt"], "stop_after_utc": stop,
                "cpu_affinity_ids": [0, 1, 2, 3],
            }
            snapshot = {
                "schema": "n83.online-tiebreak-runpod-snapshot/v1",
                "recorded_utc": utc_text(recorded),
                "source_head_sha": runpod_runner.SOURCE_HEAD_SHA,
                "controller_sha256": digest(Path(__file__)),
                "prior_distribution_pod_id":
                runpod_provider.PRIOR_DISTRIBUTION_POD_ID,
                "prior_distribution_status": "TERMINATED",
                "prior_termination_sha256": digest(termination_copy),
                "prior_pods_sha256": digest(output / "prior_pods.json"),
                "prior_spend_upper_usd": str(prior_upper(recorded)),
                "approved_combined_cap_usd": str(CAMPAIGN_CAP),
                "trigger_receipt_sha256": digest(trigger_path),
                "trigger_aggregate_sha256": trigger["trigger_aggregate_sha256"],
                "trigger_archive_sha256": trigger["trigger_archive_sha256"],
                "pods": [pod],
            }
            snapshot_path = output / "provider_snapshot.json"
            snapshot_path.write_text(
                json.dumps(snapshot, indent=2, sort_keys=True) + "\n")
            allocation = {
                "schema": "n83.online-tiebreak-runpod-allocation/v1",
                **pod, "source_head_sha": runpod_runner.SOURCE_HEAD_SHA,
                "snapshot_sha256": digest(snapshot_path),
                "trigger_receipt_sha256": digest(trigger_path),
                "trigger_aggregate_sha256": trigger["trigger_aggregate_sha256"],
                "trigger_archive_sha256": trigger["trigger_archive_sha256"],
            }
            (output / "allocation_s0.json").write_text(
                json.dumps(allocation, indent=2, sort_keys=True) + "\n")
            return {"pod_id": pod["pod_id"], "cost_per_hr": raw["costPerHr"],
                    "stop_after_utc": stop,
                    "projected_combined_upper_usd":
                    str(prior_upper(recorded) + projected)}
        except Exception as error:
            errors.append({"gpu": gpu, "type": type(error).__name__,
                           "message": str(error)[:180]})
            if (output / "provider_api_raw.json").exists():
                break
    (output / "allocation_errors.json").write_text(
        json.dumps(errors, indent=2, sort_keys=True) + "\n")
    raise ValueError("no admissible RunPod allocation")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--prior-aggregate", type=Path, required=True)
    parser.add_argument("--prior-archive", type=Path, required=True)
    parser.add_argument("--prior-termination", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--preflight", action="store_true")
    args = parser.parse_args()
    inputs = (args.prior_aggregate.resolve(), args.prior_archive.resolve(),
              args.prior_termination.resolve(), args.output.resolve())
    if args.preflight:
        trigger = preflight(*inputs)
        print(json.dumps({"status": trigger["status"],
                          "prior_spend_upper_usd": str(prior_upper(utc_now())),
                          "new_max_usd": str(PRICE_CAP * Decimal(4))},
                         sort_keys=True))
        return
    key = getpass.getpass("RunPod API key: ")
    result = allocate(*inputs, key)
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
