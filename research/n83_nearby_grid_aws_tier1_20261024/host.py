"""AWS Spot Tier 1 host identity and cgroup receipts for the N83 grid."""

import json
import os
from pathlib import Path
import platform
import urllib.request

from research.n83_highk_online_distribution_20261020 import run as library

parent = library.parent
prior = library.holdout_parent
ROOT_CGROUP = Path("/sys/fs/cgroup")
INSTANCE_TYPE = "g5g.metal"
ZONE = "us-east-1b"
REGION = "us-east-1"
STEAL_FRACTION_CAP = 0.005


def read_optional(path):
    try:
        return Path(path).read_text()
    except OSError:
        return None


def imds(path, token):
    request = urllib.request.Request(
        f"http://169.254.169.254/latest/{path}",
        headers={"X-aws-ec2-metadata-token": token})
    with urllib.request.urlopen(request, timeout=3) as response:
        return response.read().decode()


def instance_identity():
    request = urllib.request.Request(
        "http://169.254.169.254/latest/api/token", method="PUT",
        headers={"X-aws-ec2-metadata-token-ttl-seconds": "21600"})
    with urllib.request.urlopen(request, timeout=3) as response:
        token = response.read().decode()
    document = json.loads(imds("dynamic/instance-identity/document", token))
    signature = imds("dynamic/instance-identity/pkcs7", token)
    life_cycle = imds("meta-data/instance-life-cycle", token).strip()
    if (document.get("instanceType") != INSTANCE_TYPE or
            document.get("availabilityZone") != ZONE or
            document.get("region") != REGION or
            life_cycle != "spot" or not signature.strip()):
        raise ValueError("AWS Spot identity or placement differs")
    return {"document": document, "pkcs7": signature,
            "instance_life_cycle": life_cycle}


def quota_chain(group):
    chain = []
    path = Path(group).resolve()
    if path != ROOT_CGROUP and ROOT_CGROUP not in path.parents:
        raise ValueError("cgroup path escapes the v2 mount")
    while True:
        raw = read_optional(path / "cpu.max")
        chain.append({"path": str(path), "cpu_max": raw})
        if path == ROOT_CGROUP:
            return chain
        path = path.parent


def unbounded(chain):
    return bool(chain) and all(
        item["cpu_max"] is None or
        (item["cpu_max"].split() and item["cpu_max"].split()[0] == "max")
        for item in chain)


def host_receipt(group):
    host = parent.host_receipt(group)
    identity = instance_identity()
    document = identity["document"]
    host.update({
        "schema": "n83.nearby-grid-aws-tier1-host/v1",
        "provider": "aws_ec2_spot", "instance_id": document["instanceId"],
        "run_url": None, "runner_name": None, "image_version": None,
        "instance_type": document["instanceType"],
        "availability_zone": document["availabilityZone"],
        "region": document["region"], "account_id": document["accountId"],
        "image_id": document["imageId"],
        "instance_identity": identity,
        "git_sha": os.environ["AWS_EXPECTED_GIT_SHA"],
        "runner_os": platform.system(),
        "runner_arch": platform.machine(),
        "cgroup_cpu_quota_chain_before": quota_chain(group),
        "proc_self_cgroup": read_optional("/proc/self/cgroup"),
        "clock_ticks_per_second": os.sysconf("SC_CLK_TCK"),
    })
    return host


def cpu_telemetry():
    return prior.cpu_telemetry()


def topology_receipt(visible_cpus):
    return prior.topology_receipt(visible_cpus)


def run_with_telemetry(folder, command, environment, cap_seconds, group):
    before = cpu_telemetry()
    quota_before = quota_chain(group)
    usage = parent.one_run(folder, command, environment, cap_seconds, group)
    after = cpu_telemetry()
    quota_after = quota_chain(group)
    parent.write_json(folder / "host_telemetry.json", {
        "before": before, "after": after,
        "quota_chain_before": quota_before,
        "quota_chain_after": quota_after,
    })
    return usage
