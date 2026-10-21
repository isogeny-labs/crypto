#!/usr/bin/env python3
"""Create one bounded N83 RunPod Pod, then guard it until verified deletion."""

import argparse
from datetime import datetime, timezone
from decimal import Decimal
import getpass
import hashlib
import json
from pathlib import Path
import time
import urllib.error
import urllib.request

if __package__:
    from . import runpod_guard
else:
    import runpod_guard


BASE = "https://rest.runpod.io/v1"
GPU = "NVIDIA GeForce RTX 3090"
IMAGE = "rust:1.93-bookworm"


def now():
    return datetime.now(timezone.utc).isoformat()


def save_new(path, data):
    with path.open("x") as output:
        json.dump(data, output, indent=2, sort_keys=True)
        output.write("\n")


def request(key, method, path, body=None):
    encoded = None if body is None else json.dumps(body).encode()
    headers = {"Authorization": "Bearer " + key,
               "User-Agent": "cryptanalysis-n83-maxk/1.0"}
    if encoded is not None:
        headers["Content-Type"] = "application/json"
    req = urllib.request.Request(BASE + path, data=encoded,
                                 method=method, headers=headers)
    try:
        with urllib.request.urlopen(req, timeout=45) as response:
            payload = response.read()
            return response.status, json.loads(payload) if payload else None
    except urllib.error.HTTPError as error:
        payload = error.read()
        try:
            value = json.loads(payload)
        except ValueError:
            value = {"message": payload[:300].decode(errors="replace")}
        return error.code, value


def safe_pod(pod):
    result = {field: pod.get(field) for field in (
        "id", "name", "desiredStatus", "machineId", "gpuTypeId",
        "gpuCount", "cloudType", "computeType", "costPerHr",
        "adjustedCostPerHr", "createdAt", "imageName", "memoryInGb",
        "vcpuCount", "containerDiskInGb", "publicIp", "portMappings",
    ) if field in pod}
    gpu = pod.get("gpu") or {}
    machine = pod.get("machine") or {}
    result["gpuTypeId"] = (pod.get("gpuTypeId") or gpu.get("id") or
                           machine.get("gpuTypeId"))
    result["gpuCount"] = pod.get("gpuCount", gpu.get("count"))
    if isinstance(machine.get("secureCloud"), bool):
        result["cloudType"] = "SECURE" if machine["secureCloud"] else "COMMUNITY"
    result["imageName"] = pod.get("imageName", pod.get("image"))
    return result


def pod_body(name, public_key):
    boot = "\n".join((
        "set -euo pipefail",
        "export DEBIAN_FRONTEND=noninteractive",
        "apt-get update -qq",
        "apt-get install -y -qq --no-install-recommends openssh-server git build-essential pkg-config libssl-dev ca-certificates",
        "mkdir -p /run/sshd /root/.ssh",
        "printf '%s\\n' \"$PUBLIC_KEY\" > /root/.ssh/authorized_keys",
        "chmod 700 /root/.ssh",
        "chmod 600 /root/.ssh/authorized_keys",
        "printf '%s\\n' 'PasswordAuthentication no' 'PermitRootLogin prohibit-password' >> /etc/ssh/sshd_config",
        "exec /usr/sbin/sshd -D -e",
    ))
    return {"name": name, "computeType": "GPU", "imageName": IMAGE,
            "cloudType": "COMMUNITY", "gpuTypeIds": [GPU],
            "gpuTypePriority": "custom", "gpuCount": 1,
            "containerDiskInGb": 60, "volumeInGb": 0,
            "ports": ["22/tcp"], "dockerEntrypoint": ["bash", "-lc"],
            "dockerStartCmd": [boot], "env": {"PUBLIC_KEY": public_key}}


def delete_until_absent(key, pod_id, name, receipt):
    while True:
        try:
            status, detail = runpod_guard.observe(key, pod_id, name)
        except (OSError, ValueError) as error:
            status, detail = "API_ERROR", {"error_type": type(error).__name__}
        if status == "POD_ABSENT":
            runpod_guard.write_receipt(receipt, {"at_utc": now(), "status":
                                                     "VERIFIED_POD_ABSENT", **detail})
            return
        code, _ = runpod_guard.request(key, "DELETE", "/pods/" + pod_id)
        runpod_guard.write_receipt(receipt, {"at_utc": now(),
                                             "status": "DELETE_REQUESTED",
                                             "pod_id": pod_id, "http": code,
                                             "observed": status})
        time.sleep(10)


def create_and_guard(key, run_dir, name, public_key, approved_cap, max_seconds):
    run_dir.mkdir(parents=True, exist_ok=False)
    if (not name.startswith("n83-maxk-") or
            not public_key.startswith(("ssh-ed25519 ", "ssh-rsa "))):
        raise ValueError("Pod name or public SSH key differs")
    if (not Decimal("0") < approved_cap <= Decimal("5") or
            not 0 < max_seconds <= runpod_guard.MAX_SECONDS):
        raise ValueError("approval or lifetime cap differs")
    before_code, before = request(key, "GET", "/pods")
    if before_code != 200 or not isinstance(before, list):
        raise RuntimeError("pre-create Pod inventory unavailable")
    if any(pod.get("name") == name for pod in before):
        raise RuntimeError("a Pod with this campaign name already exists")
    body = pod_body(name, public_key)
    started = now()
    try:
        create_code, response = request(key, "POST", "/pods", body)
    except (OSError, ValueError) as error:
        create_code, response = "AMBIGUOUS", {"error_type": type(error).__name__}
    # A failed or ambiguous POST is resolved by inventory, never retried.
    while True:
        try:
            after_code, after = request(key, "GET", "/pods")
        except (OSError, ValueError):
            after_code, after = "API_ERROR", None
        if after_code == 200 and isinstance(after, list):
            break
        print("post-create inventory unavailable; retrying to resolve Pod identity",
              flush=True)
        time.sleep(10)
    named = ([safe_pod(pod) for pod in after if pod.get("name") == name]
             if after_code == 200 and isinstance(after, list) else [])
    receipt = {"schema": "n83.maxk-runpod-create/v1",
               "started_utc": started, "completed_utc": now(),
               "name": name, "image": IMAGE, "gpu_type": GPU,
               "public_key_sha256": hashlib.sha256(public_key.encode()).hexdigest(),
               "request_sha256": hashlib.sha256(json.dumps(body, sort_keys=True,
                                                    separators=(",", ":")).encode()).hexdigest(),
               "approved_cap_usd": str(approved_cap),
               "max_lifetime_seconds": max_seconds,
               "inventory_before_ids": [pod.get("id") for pod in before],
               "create_http": create_code,
               "create_response": (safe_pod(response) if isinstance(response, dict)
                                   else None),
               "inventory_after_http": after_code, "matching_after": named}
    save_new(run_dir / "create.json", receipt)
    ids = {pod.get("id") for pod in named if pod.get("id")}
    if isinstance(response, dict) and response.get("id"):
        ids.add(response["id"])
    if len(ids) != 1:
        raise RuntimeError("Pod identity unresolved; inspect inventory before another create")
    pod_id = ids.pop()
    detail_code, detail = request(key, "GET", "/pods/" + pod_id + "?includeMachine=true")
    pod = safe_pod(detail) if detail_code == 200 and isinstance(detail, dict) else {}
    save_new(run_dir / "post_create.json",
             {"at_utc": now(), "http": detail_code, "pod": pod})
    try:
        rate = Decimal(str(pod.get("costPerHr")))
    except (ValueError, TypeError):
        rate = Decimal("Infinity")
    eligible = (pod.get("id") == pod_id and pod.get("name") == name and
                pod.get("desiredStatus") == "RUNNING" and
                pod.get("cloudType") == "COMMUNITY" and
                pod.get("gpuTypeId") == GPU and pod.get("gpuCount") == 1 and
                isinstance(pod.get("memoryInGb"), (int, float)) and
                pod["memoryInGb"] >= 48 and
                isinstance(pod.get("vcpuCount"), (int, float)) and
                pod["vcpuCount"] >= 4 and
                rate.is_finite() and Decimal("0") < rate <=
                runpod_guard.MAX_RATE and
                rate * Decimal(max_seconds) / Decimal(3600) <= approved_cap)
    if not eligible:
        delete_until_absent(key, pod_id, name, run_dir / "guard.jsonl")
        raise RuntimeError("created Pod failed price or capacity gate and was deleted")
    allocation = {"schema": "n83.maxk-runpod-allocation/v1",
                  "pod_id": pod_id, "pod_name": name,
                  "created_utc": started,
                  "cost_per_hr_usd": str(rate),
                  "approved_cap_usd": str(approved_cap),
                  "max_lifetime_seconds": max_seconds,
                  "machine_id": pod.get("machineId"),
                  "provider_memory_gb": pod["memoryInGb"],
                  "provider_vcpu_count": pod["vcpuCount"],
                  "image": IMAGE, "gpu_type": GPU,
                  "create_receipt_sha256": hashlib.sha256(
                      (run_dir / "create.json").read_bytes()).hexdigest()}
    save_new(run_dir / "allocation.json", allocation)
    print(json.dumps({"status": "ALLOCATED_GUARD_ACTIVE", "pod_id": pod_id,
                      "machine_id": pod.get("machineId"), "rate_usd_hr": str(rate),
                      "allocation": str(run_dir / "allocation.json")}), flush=True)
    return runpod_guard.run(allocation, run_dir / "guard.jsonl", key, 30)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--name", required=True)
    parser.add_argument("--approved-cap-usd", type=Decimal, required=True)
    parser.add_argument("--max-seconds", type=int, default=43200)
    parser.add_argument("--public-key", type=Path,
                        default=Path.home() / ".ssh/id_ed25519.pub")
    args = parser.parse_args()
    public_key = args.public_key.read_text().strip()
    key = getpass.getpass("RunPod key: ")
    if not key:
        parser.error("empty API key")
    return create_and_guard(key, args.run_dir.resolve(), args.name,
                            public_key, args.approved_cap_usd, args.max_seconds)


if __name__ == "__main__":
    raise SystemExit(main())
