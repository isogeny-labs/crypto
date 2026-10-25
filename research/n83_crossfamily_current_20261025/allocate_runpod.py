#!/usr/bin/env python3
"""Allocate two bounded RunPod hosts for the frozen N83 high-K panel."""

import argparse
from datetime import datetime, timedelta, timezone
from decimal import Decimal
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time
import tomllib
from urllib import error as url_error
from urllib import request as url_request

from research.n83_crossfamily_current_20261025 import arms, run, source_bundle


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
API_URL = "https://api.runpod.io/graphql"
GPU_TYPE = "NVIDIA GeForce RTX 3070"
IMAGE = "runpod/base:1.0.2-ubuntu2404"
CAP_SECONDS = 360 * 60
PRICE_CAP_PER_HOUR = Decimal("0.30")
TOTAL_CAP_USD = Decimal("5.00")
READBACK_ATTEMPTS = 30
READBACK_DELAY_SECONDS = 2


def now():
    return datetime.now(timezone.utc)


def iso(value):
    return value.replace(microsecond=0).isoformat().replace("+00:00", "Z")


def sha(blob):
    return hashlib.sha256(blob).hexdigest()


def write(path, record):
    path.write_text(json.dumps(record, sort_keys=True, indent=2) + "\n")


def configured_key():
    key = os.environ.get("RUNPOD_API_KEY")
    if not key:
        config = Path.home() / ".runpod/config.toml"
        if config.is_file():
            key = tomllib.loads(config.read_text()).get("apikey")
    if not isinstance(key, str) or not key:
        raise ValueError("RunPod API key is not configured")
    return key


def graphql(query, key):
    request = url_request.Request(
        API_URL, data=json.dumps({"query": query}).encode(),
        headers={"Content-Type": "application/json",
                 "Authorization": "Bearer " + key}, method="POST")
    try:
        with url_request.urlopen(request, timeout=30) as response:
            result = json.load(response)
    except url_error.HTTPError as error:
        raise ValueError(f"RunPod API HTTP {error.code}") from None
    if result.get("errors") or not isinstance(result.get("data"), dict):
        raise ValueError("RunPod GraphQL response failed")
    return result["data"]


def provider_pods(key):
    query = ("query { myself { pods { id name desiredStatus costPerHr "
             "createdAt machineId memoryInGb vcpuCount imageName gpuCount "
             "podType } } }")
    return graphql(query, key)["myself"]["pods"]


def readback(name, key, pods=provider_pods):
    for attempt in range(READBACK_ATTEMPTS):
        matches = [pod for pod in pods(key)
                   if pod.get("name") == name and pod.get("id")]
        if len(matches) > 1:
            raise ValueError("duplicate named RunPod allocations")
        if (matches and matches[0].get("createdAt") and
                matches[0].get("machineId") and
                matches[0].get("desiredStatus") == "RUNNING"):
            return matches[0]
        if attempt + 1 < READBACK_ATTEMPTS:
            time.sleep(READBACK_DELAY_SECONDS)
    raise ValueError("RunPod allocation readback is incomplete")


def terminate(pod_id, key):
    graphql("mutation { podTerminate(input: { podId: " +
            json.dumps(pod_id) + " }) }", key)
    for _ in range(12):
        if not any(pod.get("id") == pod_id for pod in provider_pods(key)):
            return True
        time.sleep(5)
    return False


def mutation(name, terminate_at):
    return (
        "mutation { podFindAndDeployOnDemand(input: { name: " +
        json.dumps(name) + ", gpuTypeId: " + json.dumps(GPU_TYPE) +
        ", gpuCount: 1, cloudType: COMMUNITY, minMemoryInGb: 27, "
        "minVcpuCount: 4, imageName: " + json.dumps(IMAGE) +
        ", templateId: \"runpod-ubuntu\", containerDiskInGb: 30, "
        "startSsh: true, supportPublicIp: true, ports: \"22/tcp\", terminateAfter: " +
        json.dumps(terminate_at) +
        " }) { id name desiredStatus costPerHr machineId memoryInGb "
        "vcpuCount imageName gpuCount createdAt podType } }")


def valid_pod(pod, expected_name):
    price = Decimal(str(pod.get("costPerHr")))
    if (pod.get("name") != expected_name or
            pod.get("desiredStatus") != "RUNNING" or
            not isinstance(pod.get("id"), str) or
            not pod["id"].isalnum() or
            not isinstance(pod.get("machineId"), str) or
            not pod["machineId"] or
            pod.get("imageName") != IMAGE or
            int(pod.get("memoryInGb")) < 27 or
            int(pod.get("vcpuCount")) < 4 or
            int(pod.get("gpuCount")) != 1 or
            price <= 0 or price > PRICE_CAP_PER_HOUR or
            not pod.get("createdAt")):
        raise ValueError("RunPod allocation violates frozen host or price cap")
    return price


def frozen_head():
    head = subprocess.check_output(["git", "rev-parse", "HEAD"],
                                   cwd=ROOT, text=True).strip()
    status = subprocess.check_output(["git", "status", "--porcelain"],
                                     cwd=ROOT, text=True)
    if status:
        raise ValueError("source checkout must be clean before allocation")
    if sha((ROOT / "examples/koblitz_orbit_dlp_fast.rs").read_bytes()) != run.SOURCE_SHA256:
        raise ValueError("IC source differs from frozen panel")
    run.fixture()
    expected, regenerated = run.generator.produce()
    if any((HERE / name).read_bytes() != blob for name, blob in expected.items()):
        raise ValueError("full historical fixture replay differs")
    if regenerated != json.loads((HERE / "fixture_manifest.json").read_text()):
        raise ValueError("full historical manifest replay differs")
    arms.choose()
    return head


def allocate(output, key, bundle_proof, api=graphql, pods=provider_pods,
             terminate_pod=terminate):
    head = frozen_head()
    if (bundle_proof.get("source_head_sha") != head or
            not isinstance(bundle_proof.get("source_archive_sha256"), str) or
            len(bundle_proof["source_archive_sha256"]) != 64 or
            not isinstance(bundle_proof.get("source_manifest_sha256"), str) or
            len(bundle_proof["source_manifest_sha256"]) != 64):
        raise ValueError("source bundle differs from allocation head")
    if output.exists():
        raise ValueError("allocation output already exists")
    names = [f"n83-crossfamily-{head[:12]}-s{i}" for i in range(2)]
    before = pods(key)
    if any(pod.get("name") in names for pod in before):
        raise ValueError("matching Pod already exists; inspect before retry")
    output.mkdir(parents=True)
    started = now()
    terminate_at = iso(started + timedelta(seconds=CAP_SECONDS))
    write(output / "allocation_plan.json", {
        "schema": "n83.crossfamily-current-allocation-plan/v1",
        "source_head_sha": head, "names": names, "gpu_type": GPU_TYPE,
        "source_archive_sha256": bundle_proof["source_archive_sha256"],
        "source_manifest_sha256": bundle_proof["source_manifest_sha256"],
        "image_name": IMAGE, "min_memory_gb": 27, "min_vcpus": 4,
        "cap_seconds": CAP_SECONDS, "terminate_at_utc": terminate_at,
        "price_cap_per_hour_usd": str(PRICE_CAP_PER_HOUR),
        "total_reserved_cap_usd": str(TOTAL_CAP_USD),
        "existing_named_pods": [],
    })
    acquired = []
    observed = []
    uncertain = False
    try:
        for stratum, name in enumerate(names):
            try:
                response = api(mutation(name, terminate_at), key)
                raw = response["podFindAndDeployOnDemand"]
                if not isinstance(raw, dict) or not raw.get("id"):
                    raise ValueError("RunPod mutation returned no Pod ID")
                acquired.append(raw["id"])
                pod = readback(name, key, pods)
            except Exception:
                uncertain = True
                try:
                    matches = [p.get("id") for p in pods(key)
                               if p.get("name") == name and p.get("id")]
                    acquired.extend(x for x in matches if x not in acquired)
                except Exception:
                    pass
                raise
            price = valid_pod(pod, name)
            if pod["id"] != raw["id"]:
                raise ValueError("RunPod mutation ID and readback differ")
            projected = (sum((Decimal(str(p["costPerHr"])) for p in observed),
                             Decimal(0)) + price) * Decimal(CAP_SECONDS) / Decimal(3600)
            if projected > TOTAL_CAP_USD:
                raise ValueError("two-Pod reservation exceeds total cap")
            observed.append(pod)
            write(output / f"provider_readback_s{stratum}.json", pod)
        if len({p["id"] for p in observed}) != 2 or len({p["machineId"] for p in observed}) != 2:
            raise ValueError("two distinct RunPod machines required")
        snapshot = {
            "schema": "n83.crossfamily-current-runpod-snapshot/v1",
            "source_head_sha": head, "recorded_utc": iso(now()),
            "source_archive_sha256": bundle_proof["source_archive_sha256"],
            "source_manifest_sha256": bundle_proof["source_manifest_sha256"],
            "pods": [{"stratum": i, "pod_id": p["id"],
                      "machine_id": p["machineId"],
                      "desired_status": p["desiredStatus"],
                      "cost_per_hr": p["costPerHr"],
                      "image_name": p["imageName"]}
                     for i, p in enumerate(observed)],
        }
        snapshot_path = output / "provider_snapshot.json"
        write(snapshot_path, snapshot)
        snapshot_sha = sha(snapshot_path.read_bytes())
        for i, pod in enumerate(snapshot["pods"]):
            write(output / f"allocation_s{i}.json", {
                "schema": "n83.crossfamily-current-runpod-allocation/v1",
                "source_head_sha": head, "snapshot_sha256": snapshot_sha,
                "source_archive_sha256": bundle_proof["source_archive_sha256"],
                "source_manifest_sha256": bundle_proof["source_manifest_sha256"],
                "stratum": i, **pod,
            })
        return {"status": "PASS_TWO_BUDGETED_PODS", "source_head_sha": head,
                "snapshot_sha256": snapshot_sha,
                "pod_ids": [p["id"] for p in observed]}
    except Exception as error:
        cleanup = {}
        for pod_id in dict.fromkeys(acquired):
            try:
                cleanup[pod_id] = terminate_pod(pod_id, key)
            except Exception:
                cleanup[pod_id] = False
        write(output / "allocation_failure.json", {
            "status": "UNRESOLVED_PROVIDER_MUTATION" if uncertain or
                      not all(cleanup.values()) else "FAILED_ALLOCATIONS_TERMINATED",
            "error_type": type(error).__name__,
            "allocated_pod_ids": acquired,
            "terminated_and_absent": cleanup,
            "recorded_utc": iso(now()),
        })
        raise


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--source-bundle", type=Path)
    parser.add_argument("--execute", action="store_true")
    args = parser.parse_args()
    head = frozen_head()
    bundle_proof = None
    if args.source_bundle is not None:
        path = args.source_bundle.resolve()
        record = source_bundle.verify(path / "source.tar.gz",
                                      path / "manifest.json")
        bundle_proof = {
            "source_head_sha": record["source_head_sha"],
            "source_archive_sha256": record["source_archive_sha256"],
            "source_manifest_sha256": source_bundle.digest(path / "manifest.json"),
        }
        if bundle_proof["source_head_sha"] != head:
            raise ValueError("source bundle differs from allocation head")
    if not args.execute:
        print(json.dumps({"status": "READY_FOR_CREDENTIAL_AND_DISPATCH",
                          "source_head_sha": head,
                          "source_bundle_verified": bundle_proof is not None,
                          "projected_max_usd": str(2 * PRICE_CAP_PER_HOUR *
                                                   Decimal(CAP_SECONDS) /
                                                   Decimal(3600))}, sort_keys=True))
        return
    if bundle_proof is None:
        raise ValueError("--source-bundle is required for allocation")
    print(json.dumps(allocate(args.output.resolve(), configured_key(), bundle_proof),
                     sort_keys=True))


if __name__ == "__main__":
    main()
